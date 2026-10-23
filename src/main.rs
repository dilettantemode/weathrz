use clap::{CommandFactory, Parser};
use clap_complete::generate;
use config::Config;
use crossterm::{
    cursor, execute,
    style::ResetColor,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use hide::HideSet;
use render::TerminalRenderer;
use std::{io, panic};
use theme::ThemeRegistry;
use weathr::animation::flyers::{catalogue, settings::FlyerSettings};
use weathr::cli::{self, Cli};
use weathr::clock::{Clock, parse_date_arg};
use weathr::shutdown::Shutdown;
use weathr::{app, config, events, geolocation, hide, places, render, text, theme, weather};

const LINE_GREETING_TURN_SECS: i64 = 60;

fn info(silent: bool, msg: &str) {
    if !silent {
        println!("{}", msg);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show, ResetColor);
        default_hook(info);
    }));

    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            let err = cli::extract_simulate_missing_value(err);
            eprintln!("{}", err);
            eprintln!();
            cli::print_simulate_help();
            std::process::exit(1);
        }
    };

    if let Some(shell) = cli.completions {
        let mut cmd = Cli::command();
        let mut out = io::stdout();
        generate(shell, &mut cmd, "weathrz", &mut out);
        return Ok(());
    }

    let user_themes = Config::themes_dir();

    if cli.list_themes {
        let listing = theme::load::list_themes(user_themes.as_deref());
        println!("{}", theme::load::format_listing(&listing));
        for entry in &listing {
            for warning in theme::load::resolve(&entry.id, user_themes.as_deref()).warnings {
                eprintln!("Warning: {}: {warning}", entry.id);
            }
        }
        return Ok(());
    }

    if cli.list_flyers {
        println!("{}", catalogue::format_listing());
        return Ok(());
    }

    let mut config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            eprintln!("\nFix or recreate it at:");
            eprintln!(
                "  Linux: ~/.config/weathrz/config.toml (or $XDG_CONFIG_HOME/weathrz/config.toml)"
            );
            eprintln!("  macOS: ~/Library/Application Support/weathrz/config.toml");
            eprintln!("  Windows: %APPDATA%\\weathrz\\config.toml");
            eprintln!("\nExample config.toml:");
            eprintln!("  [location]");
            eprintln!("  latitude = 52.52");
            eprintln!("  longitude = 13.41");
            eprintln!("  auto = false  # Set to true to auto-detect location");
            eprintln!();
            Config::default()
        }
    };
    if cli.line || cli.json || cli.where_am_i {
        config.silent = true;
    }
    if let Some(query) = cli.locate.as_deref() {
        match places::search(query).await {
            Ok(found) if found.is_empty() => println!("No place matches '{query}'."),
            Ok(found) => {
                for (index, place) in found.iter().enumerate() {
                    println!("{:>2}  {}", index + 1, place.describe());
                }
                println!(
                    "\nPut one in config.toml as [location] name = \"<name>, <region or country code>\",\n\
                     or use latitude/longitude."
                );
            }
            Err(error) => {
                eprintln!("--locate: {}", error.user_friendly_message());
                std::process::exit(1);
            }
        }
        return Ok(());
    }
    let env_location =
        std::env::var(config::ENV_LATITUDE).is_ok() || std::env::var(config::ENV_LONGITUDE).is_ok();
    let place_query = match (&cli.location, env_location) {
        (Some(query), _) => Some(("--location", query.clone())),
        (None, false) => config
            .location
            .name
            .clone()
            .map(|name| ("name in config.toml", name)),
        (None, true) => None,
    }
    .filter(|(_, query)| !query.trim().is_empty());
    let mut location_source = if env_location {
        "WEATHR_LATITUDE / WEATHR_LONGITUDE"
    } else if config.location.auto || cli.auto_location {
        "your IP address (ipinfo.io)"
    } else if config.location.latitude == config::default_latitude()
        && config.location.longitude == config::default_longitude()
    {
        "the default (Berlin)"
    } else {
        "latitude/longitude in config.toml"
    };
    let mut named_place = None;
    if let Some((source, query)) = place_query {
        match places::resolve(&query).await {
            Ok(place) => {
                config.location.latitude = place.latitude;
                config.location.longitude = place.longitude;
                config.location.auto = false;
                config.location.city = places::parse_coordinates(&query)
                    .is_none()
                    .then(|| place.name.clone());
                named_place = place
                    .country_code
                    .as_deref()
                    .map(|code| events::NamedPlace {
                        latitude: place.latitude,
                        longitude: place.longitude,
                        country: events::country::known(code),
                    });
                location_source = source;
            }
            Err(message) => eprintln!("Warning: {message}; using the configured location."),
        }
    }
    let (user_events, user_warnings) = match Config::events_path() {
        Some(path) => events::user::load(&path),
        None => (events::user::UserEvents::default(), Vec::new()),
    };
    let (mut event_settings, mut event_warnings) =
        events::EventSettings::from_config_with(&config.events, events::user::leak(user_events));
    event_warnings.extend(user_warnings);
    event_settings.named = named_place;
    if let Some(code) = cli.country.as_deref() {
        match events::country::parse_setting(code) {
            Ok(setting) => event_settings.country_setting = setting,
            Err(message) => {
                eprintln!("--{}", message.trim_end_matches("; using \"auto\""));
                std::process::exit(2);
            }
        }
    }
    event_settings.locate(config.location.latitude, config.location.longitude);
    let preview = cli.event.as_deref().map(|name| {
        event_settings.find(name).unwrap_or_else(|message| {
            eprintln!("--event: {message}");
            std::process::exit(2);
        })
    });
    if let Some(Ok(setting)) = preview
        .and_then(|def| def.country())
        .map(events::country::parse_setting)
    {
        event_settings.country_setting = setting;
        event_settings.locate(config.location.latitude, config.location.longitude);
    }
    let real_now = chrono::Local::now().naive_local();
    let clock = match (cli.date.as_deref(), preview) {
        (Some(when), _) => match parse_date_arg(when, real_now) {
            Ok(start) => Clock::fake(start),
            Err(message) => {
                eprintln!("{message}");
                std::process::exit(2);
            }
        },
        (None, Some(def)) => match events::showcase_after(def, real_now) {
            Some(showcase) => Clock::fake(showcase),
            None => {
                eprintln!(
                    "No known date for {} after {}: its dates are bundled through {} and moon days calculated {} years past that. Use --date to preview it.",
                    def.id,
                    real_now.format("%Y-%m-%d"),
                    def.country()
                        .map_or(events::LAST_YEAR, events::last_year_for),
                    events::lunar::YEARS_AHEAD
                );
                std::process::exit(2);
            }
        },
        (None, None) => Clock::Real,
    };
    let (mut flyer_settings, flyer_warnings) = FlyerSettings::from_config(&config.sky);
    flyer_settings.preview = cli
        .flyer
        .as_deref()
        .and_then(catalogue::find_any)
        .map(|def| def.id);
    // CLI Overrides
    if cli.auto_location {
        config.location.auto = true;
    }
    if cli.hide_location {
        config.location.hide = true;
    }
    if cli.hide_hud {
        config.hide_hud = true;
    }
    if cli.imperial {
        config.units = weather::WeatherUnits::imperial();
    }
    if cli.metric {
        config.units = weather::WeatherUnits::metric();
    }
    if cli.silent {
        config.silent = true;
    }

    let lat_from_env = std::env::var(config::ENV_LATITUDE).is_ok();
    let lon_from_env = std::env::var(config::ENV_LONGITUDE).is_ok();
    if lat_from_env || lon_from_env {
        info(
            config.silent,
            &format!(
                "Location overridden via environment: ({:.4}, {:.4})",
                config.location.latitude, config.location.longitude
            ),
        );
    }

    if !config.location.auto
        && config.location.latitude == config::default_latitude()
        && config.location.longitude == config::default_longitude()
        && !lat_from_env
        && !lon_from_env
    {
        eprintln!("Warning: No location set, defaulting to Berlin (52.52, 13.41).");
    }

    if config.location.auto
        || config.location.hide
        || lat_from_env
        || lon_from_env
        || cli.simulate.is_some()
    {
        config.location.ip_check = false;
    }

    // Auto-detect location if enabled
    if config.location.auto {
        info(config.silent, "Auto-detecting location...");
        match geolocation::detect_location().await {
            Ok(geo_loc) => {
                if let Some(city) = &geo_loc.city {
                    info(
                        config.silent,
                        &format!(
                            "Location detected: {} ({:.4}, {:.4})",
                            city, geo_loc.latitude, geo_loc.longitude
                        ),
                    );
                } else {
                    info(
                        config.silent,
                        &format!(
                            "Location detected: {:.4}, {:.4}",
                            geo_loc.latitude, geo_loc.longitude
                        ),
                    );
                }
                config.location.latitude = geo_loc.latitude;
                config.location.longitude = geo_loc.longitude;
                config.location.city = geo_loc.city;
            }
            Err(e) => {
                eprintln!("{}", e.user_friendly_message());
                location_source = "latitude/longitude in config.toml (the IP lookup failed)";
            }
        }
    }

    if cli.list_events {
        event_settings.locate(config.location.latitude, config.location.longitude);
        println!("{}", events::format_listing(clock.now(), &event_settings));
        for warning in &event_warnings {
            eprintln!("Warning: {warning}");
        }
        return Ok(());
    }

    // Resolve city name via reverse geocoding when needed but not yet known
    if config.location.city.is_none()
        && !config.location.hide
        && matches!(
            config.location.display,
            config::LocationDisplay::City | config::LocationDisplay::Mixed
        )
    {
        info(config.silent, "Resolving city name...");
        if let Some(city) = geolocation::reverse_geocode(
            config.location.latitude,
            config.location.longitude,
            &config.location.city_name_language,
        )
        .await
        {
            info(config.silent, &format!("City resolved: {}", city));
            config.location.city = Some(city);
        }
    }

    if cli.where_am_i {
        event_settings.locate(config.location.latitude, config.location.longitude);
        let label = config.location.city.as_deref().unwrap_or("(no place name)");
        println!(
            "{label} ({:.4}, {:.4}) · from {location_source}",
            config.location.latitude, config.location.longitude
        );
        match event_settings.country.and_then(events::country::name_of) {
            Some(country) => println!("Holidays: {country}"),
            None => println!("Holidays: the yearly ones only"),
        }
        return Ok(());
    }

    if cli.line || cli.json {
        event_settings.locate(config.location.latitude, config.location.longitude);
        let greeting = events::resolve(clock.now(), &event_settings)
            .greeting_turning_every(LINE_GREETING_TURN_SECS)
            .map(|(def, text)| weathr::hud::Greeting {
                icon: def.nerd_icon,
                short: events::short_greeting(def, &text),
                text,
            });
        let reading = match cli.simulate.as_deref() {
            Some(condition) => weathr::line::simulated(condition, cli.night),
            None => weathr::line::fetch(&config).await,
        };
        match reading {
            Ok((weather, age)) if cli.json => {
                let shown = !config.location.hide;
                let reading = weathr::details::json(
                    &weather,
                    (
                        config.location.city.clone().filter(|_| shown),
                        shown.then_some(config.location.latitude),
                        shown.then_some(config.location.longitude),
                    ),
                    weathr::details::provider_or_computed(
                        &weather,
                        (config.location.latitude, config.location.longitude),
                        clock.now(),
                        clock.now_utc(),
                    ),
                    age,
                    greeting.map(|greeting| greeting.text),
                );
                match serde_json::to_string_pretty(&reading) {
                    Ok(text) => println!("{text}"),
                    Err(error) => {
                        eprintln!("weathrz: {error}");
                        std::process::exit(1);
                    }
                }
                return Ok(());
            }
            Ok((weather, age)) => {
                println!(
                    "{}",
                    weathr::line::compose(&weather, &config, greeting.as_ref(), age)
                );
                return Ok(());
            }
            Err(message) => {
                eprintln!("{message}");
                std::process::exit(1);
            }
        }
    }

    let requested_theme = cli.theme.as_deref().unwrap_or(config.normalized_theme());
    let resolved_theme = theme::load::resolve(requested_theme, user_themes.as_deref());
    for warning in &resolved_theme.warnings {
        eprintln!("Warning: {warning}");
    }
    let (hidden, hide_warnings) = HideSet::parse(&config.hide);
    for warning in &hide_warnings {
        eprintln!("Warning: {warning}");
    }
    for warning in config.frame_rates().1 {
        eprintln!("Warning: {warning}");
    }
    for warning in &event_warnings {
        eprintln!("Warning: {warning}");
    }
    for warning in &flyer_warnings {
        eprintln!("Warning: {warning}");
    }
    if let Some(def) = preview {
        if !event_settings.allows(def.id) {
            eprintln!("note: event '{}' is disabled in config.toml", def.id);
        }
    }
    if let Err(warning) = config.trees_setting() {
        eprintln!("Warning: {warning}");
    }
    if let Err(warning) = config.language() {
        eprintln!("Warning: {warning}");
    }
    if let Err(warning) = config.glyph_mode() {
        eprintln!("Warning: {warning}");
    }
    if let Err(warning) = config.icon_mode() {
        eprintln!("Warning: {warning}");
    }
    let mut theme_registry = ThemeRegistry::new();
    theme_registry.activate_resolved(resolved_theme);

    let mut renderer = match TerminalRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("\n{}\n", e.user_friendly_message());
            std::process::exit(1);
        }
    };

    let shutdown = Shutdown::new();
    if let Err(e) = shutdown.listen_for_signals() {
        eprintln!("Warning: cannot listen for termination signals: {e}");
    }

    if let Err(e) = renderer.init() {
        eprintln!("\n{}\n", e.user_friendly_message());
        std::process::exit(1);
    };

    let (term_width, term_height) = renderer.get_size();

    event_settings.locate(config.location.latitude, config.location.longitude);
    let home = dirs::home_dir();
    let shown_path = |path: Option<std::path::PathBuf>| {
        path.map_or("(no config directory)".to_string(), |path| {
            match home
                .as_deref()
                .and_then(|home| path.strip_prefix(home).ok())
            {
                Some(rest) => format!("~/{}", rest.display()),
                None => path.display().to_string(),
            }
        })
    };
    let own = &event_settings.user;
    let help_info = vec![
        (
            text::HELP_LOCATION.to_string(),
            format!(
                "{} ({:.4}, {:.4})",
                config.location.city.as_deref().unwrap_or("unnamed"),
                config.location.latitude,
                config.location.longitude
            ),
        ),
        (text::HELP_FROM.to_string(), location_source.to_string()),
        (
            text::HELP_HOLIDAYS.to_string(),
            event_settings
                .country
                .and_then(events::country::name_of)
                .unwrap_or(text::HELP_YEARLY_ONLY)
                .to_string(),
        ),
        (
            text::HELP_CONFIG.to_string(),
            shown_path(Config::config_path()),
        ),
        (
            text::HELP_OWN_DAYS.to_string(),
            format!(
                "{} · {} days, {} moved",
                shown_path(Config::events_path()),
                own.days.len(),
                own.moved.len()
            ),
        ),
        (
            text::HELP_DATA.to_string(),
            "Open-Meteo (CC BY 4.0), CAMS air quality, OpenStreetMap".to_string(),
        ),
        (
            text::HELP_VERSION.to_string(),
            format!("weathrz {} (a weathr fork)", env!("CARGO_PKG_VERSION")),
        ),
    ];
    let mut app = app::App::new(
        &config,
        app::AppOptions {
            simulate_condition: cli.simulate,
            simulate_night: cli.night,
            show_leaves: cli.leaves,
            clock,
            event_settings,
            flyer_settings,
            aqi_preview: cli.aqi,
            kite_preview: cli.flyer.as_deref() == Some(catalogue::KITE),
            hidden,
        },
        term_width,
        term_height,
        theme_registry,
    );

    app.set_help(&help_info);
    let result = app.run(&mut renderer, &shutdown).await;

    renderer.cleanup()?;

    if let Err(e) = result {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
