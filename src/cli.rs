use clap::Parser;
use clap::builder::{PossibleValue, PossibleValuesParser};
use clap_complete::Shell;

use crate::weather::WeatherCondition;

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\n\nWeather data provided by Open-Meteo.com (https://open-meteo.com/)\n",
    "Data licensed under CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/)\n\n",
    "Air quality from the Open-Meteo Air Quality API (CAMS model; air-quality-api.open-meteo.com)\n\n",
    "Geocoding powered by Nominatim/OpenStreetMap (https://nominatim.openstreetmap.org/)\n",
    "Data \u{00a9} OpenStreetMap contributors, ODbL (https://www.openstreetmap.org/copyright)"
);

const ABOUT: &str = concat!(
    "Terminal-based ASCII weather application, a custom-flavoured fork of weathr\n\n",
    "Weather data provided by Open-Meteo.com (https://open-meteo.com/)\n",
    "Data licensed under CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/)\n\n",
    "Air quality from the Open-Meteo Air Quality API (CAMS model; air-quality-api.open-meteo.com)\n\n",
    "Geocoding powered by Nominatim/OpenStreetMap (https://nominatim.openstreetmap.org/)\n",
    "Data \u{00a9} OpenStreetMap contributors, ODbL (https://www.openstreetmap.org/copyright)"
);

fn simulate_parser() -> PossibleValuesParser {
    PossibleValuesParser::new(
        WeatherCondition::ALL
            .iter()
            .map(|c| PossibleValue::new(c.as_str()).help(c.description())),
    )
}

fn flyer_parser() -> PossibleValuesParser {
    PossibleValuesParser::new(
        crate::animation::flyers::catalogue::definitions()
            .map(|def| PossibleValue::new(def.id))
            .chain([PossibleValue::new(
                crate::animation::flyers::catalogue::KITE,
            )]),
    )
}

#[derive(Parser)]
#[command(version, long_version = LONG_VERSION, about = ABOUT, long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "CONDITION",
        value_parser = simulate_parser(),
        help = "Simulate weather condition (clear, rain, drizzle, snow, etc.)"
    )]
    pub simulate: Option<String>,

    #[arg(
        short,
        long,
        help = "Simulate night time (for testing moon, stars, fireflies)"
    )]
    pub night: bool,

    #[arg(short, long, help = "Enable falling autumn leaves")]
    pub leaves: bool,

    #[arg(long, help = "Auto-detect location via IP (uses ipinfo.io)")]
    pub auto_location: bool,

    #[arg(
        long,
        value_name = "PLACE",
        conflicts_with = "auto_location",
        allow_hyphen_values = true,
        help = "Use this place for one run: a name (\"Springfield\", \"Springfield, Illinois\", \"Springfield, US\") or \"LAT,LON\""
    )]
    pub location: Option<String>,

    #[arg(
        long,
        value_name = "PLACE",
        help = "List the places matching a name, then exit"
    )]
    pub locate: Option<String>,

    #[arg(
        long = "where",
        help = "Show the location in use and where it came from, then exit"
    )]
    pub where_am_i: bool,

    #[arg(
        long,
        help = "Print the bar as one line (for tmux, zellij or a status bar), then exit; reuses a reading under 10 minutes old"
    )]
    pub line: bool,

    #[arg(
        long,
        help = "Print the reading as JSON (metric keys, for scripts), then exit; reuses a reading under 10 minutes old"
    )]
    pub json: bool,

    #[arg(long, help = "Hide location coordinates in UI")]
    pub hide_location: bool,

    #[arg(long, help = "Hide HUD (status line)")]
    pub hide_hud: bool,

    #[arg(
        long,
        conflicts_with = "metric",
        help = "Use imperial units (°F, mph, inch)"
    )]
    pub imperial: bool,

    #[arg(
        long,
        conflicts_with = "imperial",
        help = "Use metric units (°C, km/h, mm)"
    )]
    pub metric: bool,

    #[arg(long, help = "Run silently (suppress non-error output)")]
    pub silent: bool,

    #[arg(
        long,
        value_name = "THEME",
        help = "Colour theme to use (overrides config); see --list-themes"
    )]
    pub theme: Option<String>,

    #[arg(long, help = "List available colour themes and exit")]
    pub list_themes: bool,

    #[arg(
        long,
        value_name = "WHEN",
        help = "Pretend it is this local date/time: YYYY-MM-DD or YYYY-MM-DDTHH:MM"
    )]
    pub date: Option<String>,

    #[arg(
        long,
        value_name = "EVENT",
        conflicts_with = "date",
        help = "Preview an event at its showcase time (e.g. halloween, jp:childrens_day, in:diwali; --list-events shows the ids)"
    )]
    pub event: Option<String>,

    #[arg(long, help = "List seasonal events and their next dates, then exit")]
    pub list_events: bool,

    #[arg(
        long,
        value_name = "CODE",
        help = "Use this country's holidays (ID, IN, CN, US, PK, NG, BR, BD, RU, ET, MX, JP, MY, GB; auto; none)"
    )]
    pub country: Option<String>,

    #[arg(
        long,
        value_name = "FLYER",
        value_parser = flyer_parser(),
        help = "Preview one flyer, character or the kite: it comes within a second and keeps going"
    )]
    pub flyer: Option<String>,

    #[arg(long, help = "List the aircraft, characters and the kite, then exit")]
    pub list_flyers: bool,

    #[arg(
        long,
        value_name = "AQI",
        value_parser = clap::value_parser!(u16).range(0..=500),
        help = "Preview a fixed US AQI reading (0-500) instead of fetching air quality"
    )]
    pub aqi: Option<u16>,

    #[arg(long, value_name = "SHELL", value_enum)]
    pub completions: Option<Shell>,
}

pub fn extract_simulate_missing_value(err: clap::Error) -> clap::Error {
    let msg = err.to_string();
    if msg.contains("--simulate") && msg.contains("value is required") {
        err
    } else {
        err.exit()
    }
}

pub fn print_simulate_help() {
    let mut current_group = "";

    eprintln!("Available weather conditions:");
    for condition in WeatherCondition::ALL {
        let group = condition.group();
        if group != current_group {
            eprintln!();
            eprintln!("  {}:", group);
            current_group = group;
        }
        eprintln!(
            "    {:<18} - {}",
            condition.as_str(),
            condition.description()
        );
    }

    eprintln!();
    eprintln!("Examples:");
    eprintln!("  weathrz --simulate rain");
    eprintln!("  weathrz --simulate snow --night");
    eprintln!("  weathrz -s thunderstorm -n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_theme_flags() {
        let cli = Cli::try_parse_from(["weathr", "--theme", "gruvbox"]).unwrap();
        assert_eq!(cli.theme.as_deref(), Some("gruvbox"));
        assert!(!cli.list_themes);
        let cli = Cli::try_parse_from(["weathr", "--list-themes"]).unwrap();
        assert!(cli.list_themes);
        assert_eq!(cli.theme, None);
    }

    #[test]
    fn parses_event_preview_flags() {
        let cli = Cli::try_parse_from(["weathr", "--event", "halloween"]).unwrap();
        assert_eq!(cli.event.as_deref(), Some("halloween"));
        let cli = Cli::try_parse_from(["weathr", "--country", "jp"]).unwrap();
        assert_eq!(cli.country.as_deref(), Some("jp"));
        let cli = Cli::try_parse_from(["weathr", "--event", "id:nyepi"]).unwrap();
        assert_eq!(cli.event.as_deref(), Some("id:nyepi"));
        let cli = Cli::try_parse_from(["weathr", "--date", "2026-10-31T17:55"]).unwrap();
        assert_eq!(cli.date.as_deref(), Some("2026-10-31T17:55"));
        assert!(
            Cli::try_parse_from(["weathr", "--list-events"])
                .unwrap()
                .list_events
        );
        let bare = Cli::try_parse_from(["weathr", "--event", "nyepi"]).unwrap();
        assert_eq!(bare.event.as_deref(), Some("nyepi"));
        assert!(
            Cli::try_parse_from(["weathr", "--event", "halloween", "--date", "2026-10-31"])
                .is_err()
        );
    }

    #[test]
    fn parses_flyer_flags() {
        let cli = Cli::try_parse_from(["weathr", "--flyer", "witch"]).unwrap();
        assert_eq!(cli.flyer.as_deref(), Some("witch"));
        let cli = Cli::try_parse_from(["weathr", "--flyer", "caped-hero"]).unwrap();
        assert_eq!(cli.flyer.as_deref(), Some("caped-hero"));
        let cli = Cli::try_parse_from(["weathr", "--flyer", "kite"]).unwrap();
        assert_eq!(cli.flyer.as_deref(), Some("kite"));
        assert!(
            Cli::try_parse_from(["weathr", "--list-flyers"])
                .unwrap()
                .list_flyers
        );
        let unknown = Cli::try_parse_from(["weathr", "--flyer", "jumbo"])
            .err()
            .expect("unknown flyers are rejected");
        assert!(unknown.to_string().contains("plane"), "{unknown}");
    }

    #[test]
    fn parses_the_aqi_preview() {
        assert_eq!(
            Cli::try_parse_from(["weathr", "--aqi", "0"]).unwrap().aqi,
            Some(0)
        );
        assert_eq!(
            Cli::try_parse_from(["weathr", "--aqi", "500"]).unwrap().aqi,
            Some(500)
        );
        assert!(Cli::try_parse_from(["weathr", "--aqi", "501"]).is_err());
        assert!(Cli::try_parse_from(["weathr", "--aqi", "abc"]).is_err());
        assert!(ABOUT.contains("air-quality-api.open-meteo.com") || ABOUT.contains("CAMS"));
    }
}
