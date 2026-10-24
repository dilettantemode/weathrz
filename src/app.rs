use crate::animation::RenderLayer;
use crate::animation::flyers::settings::FlyerSettings;
use crate::animation_manager::{AnimationManager, FrameInputs};
use crate::app_state::AppState;
use crate::astro;
use crate::cache;
use crate::clock::Clock;
use crate::config::{Config, Provider};
use crate::error::WeatherError;
use crate::events::{self, EventSettings};
use crate::geolocation;
use crate::hide::HideSet;
use crate::hud::{self, Greeting, role_color};
use crate::location_check::{self, Choice, IpCheck};
use crate::render::TerminalRenderer;
use crate::scene::overlay::OverlayRegistry;
use crate::scene::world::WorldScene;
use crate::scene::world::trees::{self, Garden, TreesSetting};
use crate::scene::world::yard::{self, Yard};
use crate::scene::{SceneContext, SceneRegistry};
use crate::shutdown::Shutdown;
use crate::theme::ThemeRegistry;
use crate::theme::colors::ElementColors;
use crate::weather::air_quality::{self, AirQuality};

use crate::weather::provider::WeatherProvider;
use crate::weather::provider::met_office::{MetOfficeProvider, MetOfficeProviderConfig};
use crate::weather::types::CelestialEvents;
use crate::weather::units::{kmh_to_ms, ms_to_kmh};
use crate::weather::{
    OpenMeteoProvider, WeatherClient, WeatherCondition, WeatherData, WeatherLocation, WeatherUnits,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use serde::Deserialize;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

const REFRESH_INTERVAL: Duration = Duration::from_secs(300);
const FIRST_RETRY: Duration = Duration::from_secs(15);
const MIN_SCENE_WIDTH: u16 = 40;
const MIN_SCENE_HEIGHT: u16 = 12;
const AIR_REFRESH: Duration = Duration::from_secs(3600);
const REFRESH_DEBOUNCE: Duration = Duration::from_secs(5);
const AIR_RETRY: Duration = Duration::from_secs(120);
const SIMULATION_FPS: u32 = 30;
const DEFAULT_THEME_ID: &str = "default";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThemeBindings {
    theme_id: &'static str,
    scene_id: &'static str,
    overlay_id: Option<&'static str>,
}

fn resolve_theme_bindings(
    themes: &ThemeRegistry,
    scenes: &SceneRegistry,
    overlays: &OverlayRegistry,
) -> ThemeBindings {
    let active_theme = themes.active();
    let mut theme_id = active_theme.id;
    let mut scene_id = active_theme.scene_id;
    let mut overlay_id = active_theme.overlay_id;

    let scene_missing = scenes.get(scene_id).is_none();
    if scene_missing {
        if theme_id != DEFAULT_THEME_ID {
            eprintln!(
                "Warning: theme '{}' references missing scene '{}'. Falling back to '{}'.",
                theme_id, scene_id, DEFAULT_THEME_ID
            );
            let fallback_theme = themes
                .get(DEFAULT_THEME_ID)
                .expect("default theme must be registered");
            theme_id = fallback_theme.id;
            scene_id = fallback_theme.scene_id;
            overlay_id = fallback_theme.overlay_id;
        } else {
            panic!("default theme references missing scene '{}'.", scene_id);
        }
    }

    if scenes.get(scene_id).is_none() {
        panic!(
            "theme '{}' references missing scene '{}', and no fallback scene is available",
            theme_id, scene_id
        );
    }

    let validated_overlay = overlay_id.and_then(|id| {
        if overlays.get(id).is_some() {
            Some(id)
        } else {
            eprintln!(
                "Warning: theme '{}' references missing overlay '{}'. Overlay disabled.",
                theme_id, id
            );
            None
        }
    });

    ThemeBindings {
        theme_id,
        scene_id,
        overlay_id: validated_overlay,
    }
}

fn calm_colors(mut colors: ElementColors, calm: bool) -> ElementColors {
    if calm {
        colors.window = colors.palette[1];
        colors.hud_greeting = colors.hud_location;
    }
    colors
}

pub(crate) fn frame_plan(focused: bool, fps: u32, idle_fps: u32) -> (Duration, u32) {
    let rate = if focused || idle_fps == 0 {
        fps
    } else {
        idle_fps.min(fps)
    };
    let rate = rate.clamp(1, SIMULATION_FPS);
    let ticks = ((SIMULATION_FPS + rate / 2) / rate).max(1);
    (
        Duration::from_millis(u64::from(ticks) * 1000 / u64::from(SIMULATION_FPS)),
        ticks,
    )
}

pub(crate) fn next_wait(failures: u32) -> Duration {
    if failures == 0 {
        return REFRESH_INTERVAL;
    }
    let shift = (failures - 1).min(8);
    (FIRST_RETRY * (1 << shift)).min(REFRESH_INTERVAL)
}

fn age_text(seconds: u64) -> String {
    if seconds < 60 {
        "under a minute".to_string()
    } else if seconds < 3_600 {
        format!("{} min", seconds / 60)
    } else {
        format!("{} h", seconds / 3_600)
    }
}

fn sky_at(
    at: chrono::DateTime<chrono::Utc>,
    location: WeatherLocation,
    force_night: bool,
) -> (bool, f64) {
    let is_day = !force_night && astro::sun_is_up(location.latitude, location.longitude, at);
    (is_day, astro::moon_phase(at))
}

fn scene_moon(
    fixed_sky: bool,
    location: WeatherLocation,
    at: chrono::DateTime<chrono::Utc>,
) -> Option<astro::MoonSky> {
    (!fixed_sky).then(|| astro::moon_sky(location.latitude, location.longitude, at))
}

type MoonKey = (i64, u64, u64);

fn scene_sun(
    fixed_sun: bool,
    location: WeatherLocation,
    at: chrono::DateTime<chrono::Utc>,
) -> Option<astro::SunSky> {
    (!fixed_sun).then(|| astro::sun_sky(location.latitude, location.longitude, at))
}

const ORNAMENT_TWINKLE_FRAMES: u64 = 15;

type WeatherResult = Result<WeatherData, WeatherError>;

struct WeatherSource {
    provider: Arc<dyn WeatherProvider>,
    kind: Provider,
    units: WeatherUnits,
}

impl WeatherSource {
    fn spawn(
        &self,
        location: WeatherLocation,
        fresh: bool,
    ) -> (mpsc::Receiver<WeatherResult>, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel(1);
        let weather_client = WeatherClient::new(self.provider.clone(), REFRESH_INTERVAL);
        let units = self.units;
        let kind = self.kind;
        let handle = tokio::spawn(async move {
            let mut failures = 0;
            let mut fresh = fresh;
            loop {
                let result = if fresh {
                    fresh = false;
                    weather_client.fetch_fresh(&location, &units, kind).await
                } else {
                    weather_client
                        .get_current_weather(&location, &units, kind)
                        .await
                };
                failures = if result.is_ok() { 0 } else { failures + 1 };
                if tx.send(result).await.is_err() {
                    break;
                }
                tokio::time::sleep(next_wait(failures)).await;
            }
        });
        (rx, handle)
    }
}

type AirResult = Result<AirQuality, WeatherError>;

struct AirSource {
    client: reqwest::Client,
}

impl AirSource {
    fn spawn(&self, location: WeatherLocation) -> (mpsc::Receiver<AirResult>, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel(1);
        let client = self.client.clone();
        let handle = tokio::spawn(async move {
            loop {
                let result = air_quality::fetch(&client, &location).await;
                let wait = if result.is_ok() {
                    AIR_REFRESH
                } else {
                    AIR_RETRY
                };
                if tx.send(result).await.is_err() {
                    break;
                }
                tokio::time::sleep(wait).await;
            }
        });
        (rx, handle)
    }
}

pub fn provider_for(config: &Config) -> (Provider, Arc<dyn WeatherProvider>) {
    let wanted_provider = config
        .provider
        .keys()
        .next()
        .cloned()
        .unwrap_or(Provider::default());
    let provider: Arc<dyn WeatherProvider> = match wanted_provider {
        Provider::OpenMeteo => Arc::new(OpenMeteoProvider::new()),
        Provider::MetOffice => {
            let provider_config = {
                if let Some(provider_config) = config.provider.get(&wanted_provider) {
                    MetOfficeProviderConfig::deserialize(provider_config.clone()).unwrap()
                } else {
                    MetOfficeProviderConfig::default()
                }
            };
            Arc::new(MetOfficeProvider::new(provider_config).unwrap())
        }
    };
    (wanted_provider, provider)
}

fn animation_wind(weather: &WeatherData) -> (f32, f32) {
    (
        ms_to_kmh(weather.wind_speed) as f32,
        weather.wind_direction as f32,
    )
}

pub(crate) fn simulated_weather(condition: WeatherCondition, night: bool) -> WeatherData {
    WeatherData {
        condition,
        temperature: if condition.is_snowing() || condition == WeatherCondition::FreezingRain {
            -3.0
        } else {
            20.0
        },
        precipitation: if condition.is_raining() { 2.5 } else { 0.0 },
        wind_speed: kmh_to_ms(if condition.is_thunderstorm() {
            45.0
        } else {
            10.0
        }),
        wind_direction: 225.0,
        sun: CelestialEvents::from_bool(!night),
        moon_phase: Some(0.5),
        humidity: Some(75.0),
        timestamp: "simulated".to_string(),
        attribution: "".to_string(),
        feels_like: None,
        uv_index: None,
        outlook: Vec::new(),
    }
}

pub struct AppOptions {
    pub simulate_condition: Option<String>,
    pub simulate_night: bool,
    pub show_leaves: bool,
    pub clock: Clock,
    pub event_settings: EventSettings,
    pub flyer_settings: FlyerSettings,
    pub aqi_preview: Option<u16>,
    pub kite_preview: bool,
    pub hidden: HideSet,
}

pub struct App {
    state: AppState,
    animations: AnimationManager,
    scenes: SceneRegistry,
    overlays: OverlayRegistry,
    themes: ThemeRegistry,
    active_scene_id: &'static str,
    active_overlay_id: Option<&'static str>,
    weather_receiver: mpsc::Receiver<WeatherResult>,
    weather_source: Option<WeatherSource>,
    weather_task: Option<JoinHandle<()>>,
    air_receiver: mpsc::Receiver<AirResult>,
    air_source: Option<AirSource>,
    air_task: Option<JoinHandle<()>>,
    aqi_preview: Option<u16>,
    ip_check: Option<IpCheck>,
    hide_hud: bool,
    clock: Clock,
    event_settings: EventSettings,
    force_night: bool,
    simulated: bool,
    trees_setting: TreesSetting,
    frame: u64,
    fps: u32,
    idle_fps: u32,
    help_open: bool,
    help_rows: Vec<String>,
    details_open: bool,
    painted_sky: bool,
    moon_cache: Option<(MoonKey, Option<astro::MoonSky>)>,
    last_refresh: Option<Instant>,
}

impl App {
    pub fn new(
        config: &Config,
        options: AppOptions,
        term_width: u16,
        term_height: u16,
        themes: ThemeRegistry,
    ) -> Self {
        let AppOptions {
            simulate_condition,
            simulate_night,
            show_leaves,
            clock,
            event_settings,
            flyer_settings,
            aqi_preview,
            kite_preview,
            hidden,
        } = options;
        let simulated = simulate_condition.is_some();
        let location = WeatherLocation {
            latitude: config.location.latitude,
            longitude: config.location.longitude,
            elevation: None,
        };

        let mut state = AppState::new(
            location,
            config.location.city.clone(),
            config.location.display,
            config.location.hide,
            config.units,
        );
        state.set_icon_mode(config.icon_mode().unwrap_or_default());
        state.glyphs = config.glyph_mode().unwrap_or_default();
        let mut animations = AnimationManager::new(
            term_width,
            term_height,
            show_leaves,
            &hidden,
            &flyer_settings,
            kite_preview,
        );
        animations.update_moon_phase(astro::moon_phase(clock.now_utc()));

        let mut scenes = SceneRegistry::new();
        scenes.register(Box::new(WorldScene::new(
            term_width,
            term_height,
            hidden.decorations,
        )));

        let overlays = OverlayRegistry::new();
        let bindings = resolve_theme_bindings(&themes, &scenes, &overlays);

        let (_, mut weather_receiver) = mpsc::channel(1);
        let mut weather_source = None;
        let mut weather_task = None;

        if let Some(ref condition_str) = simulate_condition {
            let simulated_condition =
                condition_str
                    .parse::<WeatherCondition>()
                    .unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        WeatherCondition::Clear
                    });

            let weather = simulated_weather(simulated_condition, simulate_night);

            let rain_intensity = weather.condition.rain_intensity();
            let snow_intensity = weather.condition.snow_intensity();
            let (wind_kmh, wind_direction) = animation_wind(&weather);

            state.update_weather(weather);
            animations.update_rain_intensity(rain_intensity);
            animations.update_snow_intensity(snow_intensity);
            animations.update_wind(wind_kmh, wind_direction);
        } else {
            let (wanted_provider, provider) = provider_for(config);

            let source = WeatherSource {
                provider,
                kind: wanted_provider,
                units: config.units,
            };
            let (rx, handle) = source.spawn(location, false);
            weather_receiver = rx;
            weather_task = Some(handle);
            weather_source = Some(source);
        }

        let (_, mut air_receiver) = mpsc::channel(1);
        let mut air_source = None;
        let mut air_task = None;
        if let Some(us_aqi) = aqi_preview {
            state.set_air_quality(AirQuality {
                us_aqi,
                fetched_at: None,
            });
        } else if simulate_condition.is_none() && !config.hide_hud {
            let source = AirSource {
                client: air_quality::client(),
            };
            let (rx, handle) = source.spawn(location);
            air_receiver = rx;
            air_task = Some(handle);
            air_source = Some(source);
        }

        let ip_check = config.location.ip_check.then(|| {
            let (tx, rx) = oneshot::channel();
            tokio::spawn(async move {
                let _ = tx.send(geolocation::detect_location_fresh().await);
            });
            IpCheck::new(
                rx,
                (config.location.latitude, config.location.longitude),
                location_check::place_label(
                    config.location.city.as_deref(),
                    config.location.latitude,
                    config.location.longitude,
                ),
            )
        });

        Self {
            state,
            animations,
            scenes,
            overlays,
            themes,
            active_scene_id: bindings.scene_id,
            active_overlay_id: bindings.overlay_id,
            weather_receiver,
            weather_source,
            weather_task,
            air_receiver,
            air_source,
            air_task,
            aqi_preview,
            ip_check,
            hide_hud: config.hide_hud,
            clock,
            event_settings,
            force_night: simulate_night,
            simulated,
            trees_setting: config.trees_setting().unwrap_or_default(),
            frame: 0,
            fps: config.frame_rates().0.0,
            idle_fps: config.frame_rates().0.1,
            help_open: false,
            help_rows: crate::help::rows(&[]),
            details_open: false,
            painted_sky: config.sky.painted,
            moon_cache: None,
            last_refresh: None,
        }
    }

    pub fn set_help(&mut self, info: &[(String, String)]) {
        self.help_rows = crate::help::rows(info);
    }

    fn details_rows(&self) -> Vec<String> {
        let Some(weather) = self.state.current_weather.as_ref() else {
            return vec![crate::text::LOADING.to_string()];
        };
        crate::help::aligned(&crate::details::rows(&crate::details::Facts {
            weather,
            units: &self.state.units,
            aqi: self.state.fresh_aqi(SystemTime::now()),
            sun: self.state.sun_sky,
            sun_times: crate::details::provider_or_computed(
                weather,
                (self.state.location.latitude, self.state.location.longitude),
                self.clock.now(),
                self.clock.now_utc(),
            ),
            moon_phase: Some(astro::moon_phase(self.clock.now_utc())),
            now: crate::details::local_now(weather, self.clock.now(), self.clock.now_utc()),
        }))
    }

    fn refresh_now(&mut self) {
        if self.simulated
            || self
                .last_refresh
                .is_some_and(|at| at.elapsed() < REFRESH_DEBOUNCE)
        {
            return;
        }
        self.last_refresh = Some(Instant::now());
        if let Some(source) = &self.weather_source {
            if let Some(task) = self.weather_task.take() {
                task.abort();
            }
            let (receiver, task) = source.spawn(self.state.location, true);
            self.weather_receiver = receiver;
            self.weather_task = Some(task);
        }
        self.restart_air_task();
    }

    fn switch_to_ip_location(&mut self) {
        let Some(prompt) = self.ip_check.as_mut().and_then(IpCheck::take_prompt) else {
            return;
        };
        let location = WeatherLocation {
            latitude: prompt.ip.latitude,
            longitude: prompt.ip.longitude,
            elevation: None,
        };
        let simulated_weather = self
            .simulated
            .then(|| self.state.current_weather.clone())
            .flatten();
        self.state.set_location(location, prompt.ip.city);
        self.event_settings
            .locate(location.latitude, location.longitude);
        if let Some(weather) = simulated_weather {
            self.state.update_weather(weather);
        }
        if let Some(us_aqi) = self.aqi_preview {
            self.state.set_air_quality(AirQuality {
                us_aqi,
                fetched_at: None,
            });
        }
        if let Some(source) = &self.weather_source {
            if let Some(task) = self.weather_task.take() {
                task.abort();
            }
            let (receiver, task) = source.spawn(location, false);
            self.weather_receiver = receiver;
            self.weather_task = Some(task);
        }
        self.restart_air_task();
    }

    fn place_the_sky(&mut self) {
        let fixed = self.simulated && !self.clock.is_fake();
        let at = self.clock.now_utc();
        let location = self.state.location;
        self.state.sun_sky = scene_sun(fixed, location, at);
        let key = (
            at.timestamp().div_euclid(60) * i64::from(!fixed),
            location.latitude.to_bits(),
            location.longitude.to_bits(),
        );
        let moon = match self.moon_cache {
            Some((cached, moon)) if cached == key => moon,
            _ => {
                let moon = scene_moon(fixed, location, at);
                self.moon_cache = Some((key, moon));
                moon
            }
        };
        self.state.moon_sky = moon;
    }

    fn advance_quietly(&mut self, (width, height): (u16, u16), rng: &mut impl rand::Rng) {
        if width < MIN_SCENE_WIDTH || height < MIN_SCENE_HEIGHT {
            return;
        }
        self.follow_sky();
        self.place_the_sky();
        let active_events = events::resolve(self.clock.now(), &self.event_settings);
        let colors = *self
            .themes
            .active()
            .colors
            .for_time(self.state.weather_conditions.sun.is_day);
        self.frame = self.frame.wrapping_add(1);
        let Some(scene) = self.scenes.get_mut(self.active_scene_id) else {
            return;
        };
        scene.update_size(width, height);
        let layout = scene.layout();
        let frame = FrameInputs {
            conditions: &self.state.weather_conditions,
            state: &self.state,
            layout: &layout,
            colors: &colors,
            events: &active_events,
        };
        self.animations.update_only(&frame, rng);
    }

    fn apply_weather(&mut self, weather: WeatherData) {
        let rain_intensity = weather.condition.rain_intensity();
        let snow_intensity = weather.condition.snow_intensity();
        let fog_intensity = weather.condition.fog_intensity();
        let (wind_kmh, wind_direction) = animation_wind(&weather);
        if let Some(moon_phase) = weather.moon_phase {
            self.animations.update_moon_phase(moon_phase);
        }
        self.state.update_weather(weather);
        self.state.reading_at = Some(SystemTime::now());
        self.animations.update_rain_intensity(rain_intensity);
        self.animations.update_snow_intensity(snow_intensity);
        self.animations.update_fog_intensity(fog_intensity);
        self.animations.update_wind(wind_kmh, wind_direction);
    }

    fn weather_failed(&mut self, message: String, last: Option<(WeatherData, u64)>) -> String {
        let note = if self.state.current_weather.is_some() {
            format!("{} · {message}", crate::text::OFFLINE_LAST_READING)
        } else if let Some((weather, age)) = last {
            self.apply_weather(weather);
            self.state.reading_at = SystemTime::now().checked_sub(Duration::from_secs(age));
            format!(
                "{} {} {} · {message}",
                crate::text::OFFLINE_READING_FROM,
                age_text(age),
                crate::text::AGO
            )
        } else {
            format!("{} · {message}", crate::text::OFFLINE_RETRYING)
        };
        self.state.set_offline_mode(true);
        note
    }

    fn garden(&self, active_events: &events::ActiveEvents) -> Garden {
        let weather = self.state.current_weather.as_ref();
        let freezing = weather.is_some_and(|weather| weather.temperature < 0.0);
        let wind_kmh = weather.map_or(0.0, |weather| animation_wind(weather).0);
        let (main, second) =
            trees::trees_for(self.trees_setting, self.state.location.latitude, freezing);
        let wind_from = weather.map_or(270.0, |weather| animation_wind(weather).1);
        let conditions = &self.state.weather_conditions;
        Garden {
            main,
            second,
            sway: trees::sway(wind_kmh, self.frame),
            ornaments: active_events
                .has(events::Look::ChristmasLights)
                .then_some(self.frame / ORNAMENT_TWINKLE_FRAMES),
            yard: Yard {
                wind_kmh,
                toward_east: -wind_from.to_radians().sin(),
                frame: self.frame,
                wet: conditions.is_raining || conditions.is_thunderstorm,
                road: active_events.has(events::Look::Mudik),
                seed: yard::seed_for(self.state.location.latitude, self.state.location.longitude),
            },
        }
    }

    fn restart_air_task(&mut self) {
        if let Some(source) = &self.air_source {
            if let Some(task) = self.air_task.take() {
                task.abort();
            }
            let (receiver, task) = source.spawn(self.state.location);
            self.air_receiver = receiver;
            self.air_task = Some(task);
        }
    }

    fn follow_sky(&mut self) {
        let follows_the_sky =
            self.clock.is_fake() || self.state.current_weather.is_none() || self.state.is_offline;
        if !follows_the_sky {
            return;
        }
        let (is_day, moon_phase) =
            sky_at(self.clock.now_utc(), self.state.location, self.force_night);
        self.animations.update_moon_phase(moon_phase);
        if self.state.weather_conditions.sun.is_day != is_day {
            self.state.weather_conditions.sun.is_day = is_day;
            if let Some(weather) = self.state.current_weather.as_mut() {
                weather.sun.is_day = is_day;
            }
            self.state.weather_info_needs_update = true;
        }
    }

    fn draw_frame(
        &mut self,
        renderer: &mut TerminalRenderer,
        rng: &mut impl rand::Rng,
        attribution: &str,
        location_prompt: Option<&str>,
    ) -> io::Result<()> {
        self.follow_sky();
        self.state.set_offline_age(
            self.state
                .is_offline
                .then_some(self.state.reading_at)
                .flatten()
                .and_then(|at| SystemTime::now().duration_since(at).ok())
                .map(|age| format!("{} {}", age_text(age.as_secs()), crate::text::AGO)),
        );
        self.place_the_sky();
        let active_events = events::resolve(self.clock.now(), &self.event_settings);

        renderer.clear()?;

        let colors = calm_colors(
            *self
                .themes
                .active()
                .colors
                .for_time(self.state.weather_conditions.sun.is_day),
            active_events.calm(),
        );

        let (term_width, term_height) = renderer.get_size();
        if term_width < MIN_SCENE_WIDTH || term_height < MIN_SCENE_HEIGHT {
            let lines = [
                crate::text::PANE_TOO_SMALL.to_string(),
                format!("{MIN_SCENE_WIDTH}x{MIN_SCENE_HEIGHT} or more"),
            ];
            let top = term_height.saturating_sub(lines.len() as u16) / 2;
            for (row, line) in lines.iter().enumerate() {
                let x = term_width.saturating_sub(line.len() as u16) / 2;
                renderer.render_line_colored(x, top + row as u16, line, colors.hud_location)?;
            }
            return Ok(());
        }
        self.frame = self.frame.wrapping_add(1);
        let garden = self.garden(&active_events);
        let scene = self
            .scenes
            .get_mut(self.active_scene_id)
            .expect("active scene must be registered");
        scene.update_size(term_width, term_height);

        let layout = scene.layout();
        let painted = self.painted_sky && renderer.can_paint();
        let is_day = self.state.weather_conditions.sun.is_day;
        if painted {
            let condition = self
                .state
                .current_weather
                .as_ref()
                .map_or(WeatherCondition::Clear, |weather| weather.condition);
            let elevation = self
                .state
                .sun_sky
                .filter(|_| !self.force_night)
                .map(|sky| sky.elevation_deg);
            if self.state.glyphs == crate::glyphs::Glyphs::Ascii {
                renderer.paint_backdrop(&crate::scene::sky_paint::sky_rows(
                    term_height,
                    layout.ground_y,
                    elevation,
                    is_day,
                    condition,
                ));
            } else {
                let bands = crate::scene::sky_paint::sky_rows(
                    term_height * 2,
                    layout.ground_y * 2,
                    elevation,
                    is_day,
                    condition,
                );
                let (upper, lower): (Vec<_>, Vec<_>) =
                    bands.chunks(2).map(|pair| (pair[0], pair[1])).unzip();
                renderer.paint_backdrop_halves(&upper, &lower);
            }
        }
        let ctx = SceneContext {
            colors: &colors,
            garden,
            glow_windows: painted && !is_day && !active_events.calm(),
        };
        let frame = FrameInputs {
            conditions: &self.state.weather_conditions,
            state: &self.state,
            layout: &layout,
            colors: &colors,
            events: &active_events,
        };

        self.animations
            .render(renderer, RenderLayer::Background, &frame, rng)?;

        scene.render(renderer, &ctx)?;

        if let Some(ov_id) = self.active_overlay_id {
            if let Some(overlay) = self.overlays.get_mut(ov_id) {
                overlay.update_size(term_width, term_height);
                overlay.render(renderer, &ctx, &layout)?;
            }
        }

        self.animations
            .render(renderer, RenderLayer::PostScene, &frame, rng)?;
        self.animations
            .render(renderer, RenderLayer::Foreground, &frame, rng)?;

        self.state
            .set_greeting(active_events.greeting().map(|(def, text)| Greeting {
                icon: def.nerd_icon,
                short: events::short_greeting(def, &text),
                text,
            }));
        self.state.update_loading_animation();
        self.state.update_cached_info();

        if !self.hide_hud {
            let mut x = hud::HUD_X;
            for segment in self.state.hud_bar(term_width) {
                renderer.render_line_colored(
                    x,
                    1,
                    &segment.text,
                    role_color(segment.role, &colors),
                )?;
                x = x.saturating_add(segment.text.chars().count() as u16);
            }
        }

        if let Some(prompt) = location_prompt {
            renderer.render_line_colored(2, 2, prompt, colors.hud_location)?;
        }

        let card = if self.help_open {
            Some((crate::text::HELP_TITLE, self.help_rows.clone()))
        } else if self.details_open {
            Some((crate::text::DETAILS_TITLE, self.details_rows()))
        } else {
            None
        };
        if let Some((title, rows)) = card {
            let lines = crate::help::framed(title, &rows, term_width, term_height);
            let top = term_height.saturating_sub(lines.len() as u16) / 2;
            for (row, line) in lines.iter().enumerate() {
                let width = line.chars().count() as u16;
                let x = term_width.saturating_sub(width) / 2;
                for column in x..x + width {
                    renderer.mark_solid(column, top + row as u16);
                }
                renderer.render_line_colored(x, top + row as u16, line, colors.hud)?;
            }
        }

        let attribution_x = term_width.saturating_sub(attribution.chars().count() as u16 + 2);
        let attribution_y = if term_height > 0 { term_height - 1 } else { 0 };
        renderer.render_line_colored(
            attribution_x,
            attribution_y,
            attribution,
            colors.attribution,
        )
    }

    pub async fn run(
        &mut self,
        renderer: &mut TerminalRenderer,
        shutdown: &Shutdown,
    ) -> io::Result<()> {
        let mut rng = rand::rng();
        let mut attribution = "Awaiting weather data".to_string();
        let mut focused = true;

        loop {
            if shutdown.is_requested() {
                break;
            }

            let now = Instant::now();
            let location_prompt = self
                .ip_check
                .as_mut()
                .and_then(|check| check.poll(now))
                .map(|prompt| prompt.text(now));

            match self.weather_receiver.try_recv() {
                Ok(Ok(weather)) => {
                    attribution = weather.attribution.clone();
                    self.apply_weather(weather);
                }
                Ok(Err(error)) => {
                    let message = match &error {
                        WeatherError::Network(net_err) => net_err.user_friendly_message(),
                        _ => format!("Failed to fetch weather: {}", error),
                    };
                    let last = match (&self.weather_source, &self.state.current_weather) {
                        (Some(source), None) => cache::last_weather(
                            self.state.location.latitude,
                            self.state.location.longitude,
                            source.kind,
                        ),
                        _ => None,
                    };
                    attribution = self.weather_failed(message, last);
                }
                Err(e) => {
                    if e == mpsc::error::TryRecvError::Disconnected {
                        attribution = "".to_string();
                    }
                }
            }
            if let Ok(Ok(reading)) = self.air_receiver.try_recv() {
                self.state.set_air_quality(reading);
            }
            if self.state.drop_stale_air_quality(SystemTime::now()) {
                self.restart_air_task();
            }

            let (wait, ticks) = frame_plan(focused, self.fps, self.idle_fps);
            for _ in 1..ticks {
                self.advance_quietly(renderer.get_size(), &mut rng);
            }
            self.draw_frame(renderer, &mut rng, &attribution, location_prompt.as_deref())?;

            renderer.flush()?;

            if event::poll(wait)? {
                match event::read()? {
                    Event::FocusLost => focused = false,
                    Event::FocusGained => focused = true,
                    Event::Resize(width, height) => {
                        renderer.manual_resize(width, height)?;
                        let (new_width, new_height) = renderer.get_size();
                        self.animations.on_resize(new_width, new_height);
                    }
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {}
                    Event::Key(key_event) if location_prompt.is_some() => {
                        match location_check::choice_for_key(&key_event) {
                            Some(Choice::Ip) => self.switch_to_ip_location(),
                            Some(Choice::Config) => {
                                if let Some(check) = self.ip_check.as_mut() {
                                    check.dismiss();
                                }
                            }
                            Some(Choice::Quit) => break,
                            None => {}
                        }
                    }
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            break;
                        }
                        KeyCode::Char('?') => {
                            self.help_open = !self.help_open;
                            self.details_open = false;
                        }
                        KeyCode::Char('i') | KeyCode::Char('I') => {
                            self.details_open = !self.details_open;
                            self.help_open = false;
                        }
                        KeyCode::Esc => {
                            self.help_open = false;
                            self.details_open = false;
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => self.refresh_now(),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nyepi_darkens_the_windows_and_dims_the_greeting() {
        let night = crate::theme::catalogue::default_colors().night;
        let calm = calm_colors(night, true);
        assert_ne!(night.window, night.palette[1]);
        assert_eq!(calm.window, night.palette[1]);
        assert_eq!(calm.hud_greeting, night.hud_location);
        assert_eq!(calm_colors(night, false), night);
    }

    fn jakarta() -> WeatherLocation {
        WeatherLocation {
            latitude: -6.2088,
            longitude: 106.8456,
            elevation: None,
        }
    }

    fn utc(text: &str) -> chrono::DateTime<chrono::Utc> {
        at(text).and_utc()
    }

    #[test]
    fn the_garden_follows_the_place_and_the_weather() {
        let mut app = test_app(Clock::Real, "clear", false);
        let none = crate::events::ActiveEvents::default();
        assert_eq!(app.garden(&none).main.id, "round");
        assert_eq!(app.garden(&none).ornaments, None);
        let christmas =
            crate::events::resolve(at("2026-12-20T21:00"), &EventSettings::in_country("ID"));
        assert!(app.garden(&christmas).ornaments.is_some());
        app.state.location = jakarta();
        assert_eq!(app.garden(&none).main.id, "round");
        app.state.location.latitude = 52.52;
        if let Some(weather) = app.state.current_weather.as_mut() {
            weather.temperature = -4.0;
            weather.wind_speed = 50.0 / 3.6;
        }
        assert_eq!(app.garden(&none).main.id, "bare");
        let swings: Vec<i16> = (0..60)
            .map(|frame| {
                app.frame = frame;
                app.garden(&none).sway
            })
            .collect();
        assert!(swings.iter().any(|sway| *sway != 0), "{swings:?}");
        app.trees_setting = TreesSetting::Fixed(trees::Climate::Dry);
        assert_eq!(app.garden(&none).main.id, "saguaro");
    }

    fn cached_reading() -> WeatherData {
        WeatherData {
            temperature: 29.0,
            ..simulated_weather(WeatherCondition::Cloudy, false)
        }
    }

    #[test]
    fn retries_back_off_from_fifteen_seconds_to_the_refresh() {
        let waits: Vec<u64> = (0..8)
            .map(|failures| next_wait(failures).as_secs())
            .collect();
        assert_eq!(waits, [300, 15, 30, 60, 120, 240, 300, 300]);
    }

    #[test]
    fn offline_with_a_saved_reading_shows_it_and_its_age() {
        let mut app = test_app(Clock::Real, "clear", false);
        app.state.current_weather = None;
        let note = app.weather_failed("timed out".to_string(), Some((cached_reading(), 7_300)));
        assert_eq!(
            app.state
                .current_weather
                .as_ref()
                .map(|weather| weather.temperature),
            Some(29.0)
        );
        assert!(app.state.is_offline);
        assert!(note.contains("reading from 2 h ago"), "{note}");
    }

    #[test]
    fn the_bar_shows_how_old_the_offline_reading_is_and_keeps_counting() {
        let mut app = test_app(Clock::Real, "clear", false);
        app.state.current_weather = None;
        app.weather_failed("timed out".to_string(), Some((cached_reading(), 7_300)));
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(
            renderer.text().contains("OFFLINE · 2 h ago"),
            "{}",
            renderer.text()
        );
        app.state.reading_at = app
            .state
            .reading_at
            .and_then(|at| at.checked_sub(Duration::from_secs(3_600)));
        app.weather_failed("timed out".to_string(), None);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(
            renderer.text().contains("OFFLINE · 3 h ago"),
            "{}",
            renderer.text()
        );
        app.apply_weather(cached_reading());
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(!renderer.text().contains("OFFLINE"), "{}", renderer.text());
    }

    #[test]
    fn offline_the_sky_follows_the_real_sun_not_the_old_reading() {
        let mut app = test_app(Clock::Real, "clear", false);
        let now = Clock::Real.now_utc();
        let real_day = astro::sun_is_up(
            app.state.location.latitude,
            app.state.location.longitude,
            now,
        );
        let mut stale = cached_reading();
        stale.sun.is_day = !real_day;
        app.apply_weather(stale);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert_eq!(app.state.weather_conditions.sun.is_day, !real_day);
        app.weather_failed("timed out".to_string(), None);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert_eq!(app.state.weather_conditions.sun.is_day, real_day);
        assert_eq!(
            app.state
                .current_weather
                .as_ref()
                .map(|weather| weather.sun.is_day),
            Some(real_day)
        );
    }

    #[test]
    fn a_long_attribution_never_pushes_past_the_left_edge() {
        let mut app = test_app(Clock::Real, "clear", false);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        for length in [107, 108, 109, 110, 200] {
            let attribution = "x".repeat(length);
            app.draw_frame(&mut renderer, &mut rng, &attribution, None)
                .unwrap();
        }
        let short = "Offline · retrying";
        app.draw_frame(&mut renderer, &mut rng, short, None)
            .unwrap();
        let last = renderer.text().lines().last().unwrap().to_string();
        assert!(last.trim_end().ends_with(short), "{last:?}");
        assert_eq!(last.trim_end().chars().count(), 110 - 2, "{last:?}");
    }

    #[test]
    fn offline_without_a_reading_invents_nothing() {
        let mut app = test_app(Clock::Real, "clear", false);
        app.state.current_weather = None;
        let note = app.weather_failed("timed out".to_string(), None);
        assert!(app.state.current_weather.is_none());
        assert!(app.state.is_offline);
        assert!(note.contains("retrying"), "{note}");
        app.state.update_cached_info();
        assert!(
            app.state
                .cached_weather_info
                .starts_with("Offline · retrying"),
            "{}",
            app.state.cached_weather_info
        );
    }

    #[test]
    fn offline_keeps_the_reading_already_shown() {
        let mut app = test_app(Clock::Real, "clear", false);
        let shown = |app: &App| {
            app.state
                .current_weather
                .as_ref()
                .map(|weather| (weather.temperature, weather.timestamp.clone()))
        };
        let before = shown(&app);
        let note = app.weather_failed("timed out".to_string(), Some((cached_reading(), 60)));
        assert_eq!(shown(&app), before);
        assert!(app.state.is_offline);
        assert!(note.contains("last reading"), "{note}");
    }

    #[test]
    fn a_simulated_scene_on_the_real_clock_keeps_the_fixed_sun() {
        assert_eq!(scene_sun(true, jakarta(), utc("2026-10-31T11:30")), None);
        assert!(scene_sun(false, jakarta(), utc("2026-10-31T11:30")).is_some());
        let mut app = test_app(Clock::Real, "clear", false);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert_eq!(app.state.sun_sky, None);
    }

    #[test]
    fn a_winter_evening_preview_is_night() {
        let cet = chrono::FixedOffset::east_opt(3600).unwrap();
        let mut app = test_app(Clock::fake_at(at("2026-12-21T17:30"), cet), "clear", false);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(!app.state.weather_conditions.sun.is_day);
    }

    #[test]
    fn animation_wind_converts_metres_per_second_to_kmh() {
        let weather = simulated_weather(WeatherCondition::Clear, false);
        let weather = WeatherData {
            wind_speed: 10.0,
            wind_direction: 90.0,
            ..weather
        };
        let (speed_kmh, direction) = animation_wind(&weather);
        assert!((speed_kmh - 36.0).abs() < 1e-3, "got {speed_kmh}");
        assert_eq!(direction, 90.0);
    }

    #[test]
    fn simulated_wind_is_stored_in_metres_per_second() {
        let calm = simulated_weather(WeatherCondition::Rain, false);
        let storm = simulated_weather(WeatherCondition::Thunderstorm, false);
        assert!((animation_wind(&calm).0 - 10.0).abs() < 1e-3);
        assert!((animation_wind(&storm).0 - 45.0).abs() < 1e-3);
    }

    #[test]
    fn simulated_snow_and_freezing_rain_read_below_zero() {
        for condition in [
            WeatherCondition::Snow,
            WeatherCondition::SnowGrains,
            WeatherCondition::SnowShowers,
            WeatherCondition::FreezingRain,
        ] {
            assert!(
                simulated_weather(condition, false).temperature < 0.0,
                "{condition:?}"
            );
        }
        assert_eq!(
            simulated_weather(WeatherCondition::Rain, false).temperature,
            20.0
        );
    }
    use crate::render::TerminalRenderer;
    use crate::scene::overlay::SceneOverlay;
    use crate::scene::{Scene, SceneContext, SceneLayout};
    use crate::theme::catalogue::default_colors;
    use crate::theme::{Theme, ThemeRegistry};
    use std::io;

    struct TestScene {
        id: &'static str,
    }

    impl TestScene {
        fn new(id: &'static str) -> Self {
            Self { id }
        }
    }

    impl Scene for TestScene {
        fn id(&self) -> &'static str {
            self.id
        }

        fn update_size(&mut self, _width: u16, _height: u16) {}

        fn render(
            &self,
            _renderer: &mut TerminalRenderer,
            _ctx: &SceneContext<'_>,
        ) -> io::Result<()> {
            Ok(())
        }

        fn layout(&self) -> SceneLayout {
            SceneLayout {
                ground_y: 0,
                skyline_y: 0,
                chimney_pos: None,
                door: None,
                width: 0,
                height: 0,
            }
        }
    }

    struct TestOverlay {
        id: &'static str,
    }

    impl TestOverlay {
        fn new(id: &'static str) -> Self {
            Self { id }
        }
    }

    impl SceneOverlay for TestOverlay {
        fn id(&self) -> &'static str {
            self.id
        }

        fn update_size(&mut self, _width: u16, _height: u16) {}

        fn render(
            &self,
            _renderer: &mut TerminalRenderer,
            _ctx: &SceneContext<'_>,
            _layout: &SceneLayout,
        ) -> io::Result<()> {
            Ok(())
        }
    }

    fn scene_registry_with_world() -> SceneRegistry {
        let mut scenes = SceneRegistry::new();
        scenes.register(Box::new(TestScene::new("world")));
        scenes
    }

    #[test]
    fn bindings_fall_back_to_default_when_scene_missing() {
        let scenes = scene_registry_with_world();
        let overlays = OverlayRegistry::new();
        let mut themes = ThemeRegistry::new();
        themes.register(Theme {
            id: "custom",
            display_name: "Custom",
            scene_id: "unknown",
            overlay_id: None,
            colors: default_colors(),
        });
        themes.set_active("custom").unwrap();

        let bindings = resolve_theme_bindings(&themes, &scenes, &overlays);

        assert_eq!(bindings.theme_id, DEFAULT_THEME_ID);
        assert_eq!(bindings.scene_id, "world");
        assert_eq!(bindings.overlay_id, None);
    }

    #[test]
    fn bindings_disable_unregistered_overlay() {
        let scenes = scene_registry_with_world();
        let overlays = OverlayRegistry::new();
        let mut themes = ThemeRegistry::new();
        themes.register(Theme {
            id: "overlay-theme",
            display_name: "Overlay Theme",
            scene_id: "world",
            overlay_id: Some("hud"),
            colors: default_colors(),
        });
        themes.set_active("overlay-theme").unwrap();

        let bindings = resolve_theme_bindings(&themes, &scenes, &overlays);

        assert_eq!(bindings.theme_id, "overlay-theme");
        assert_eq!(bindings.scene_id, "world");
        assert_eq!(bindings.overlay_id, None);
    }

    #[test]
    fn bindings_keep_registered_overlay() {
        let scenes = scene_registry_with_world();
        let mut overlays = OverlayRegistry::new();
        overlays.register(Box::new(TestOverlay::new("hud")));
        let mut themes = ThemeRegistry::new();
        themes.register(Theme {
            id: "overlay",
            display_name: "Overlay",
            scene_id: "world",
            overlay_id: Some("hud"),
            colors: default_colors(),
        });
        themes.set_active("overlay").unwrap();

        let bindings = resolve_theme_bindings(&themes, &scenes, &overlays);

        assert_eq!(bindings.theme_id, "overlay");
        assert_eq!(bindings.overlay_id, Some("hud"));
    }

    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn test_app(clock: Clock, condition: &str, night: bool) -> App {
        let mut themes = ThemeRegistry::new();
        themes.activate_resolved(crate::theme::load::resolve("tokyo-night", None));
        App::new(
            &Config::default(),
            AppOptions {
                simulate_condition: Some(condition.to_string()),
                simulate_night: night,
                show_leaves: false,
                clock,
                event_settings: EventSettings::in_country("ID"),
                flyer_settings: FlyerSettings::default(),
                aqi_preview: None,
                kite_preview: false,
                hidden: HideSet::default(),
            },
            110,
            32,
            themes,
        )
    }

    #[test]
    fn quiet_ticks_in_a_tiny_pane_never_reach_the_fireworks() {
        let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
        let mut themes = ThemeRegistry::new();
        themes.activate_resolved(crate::theme::load::resolve("tokyo-night", None));
        let mut app = App::new(
            &Config::default(),
            AppOptions {
                simulate_condition: Some("clear".to_string()),
                simulate_night: true,
                show_leaves: false,
                clock: Clock::fake_at(at("2026-12-31T23:30"), wib),
                event_settings: EventSettings::in_country("ID"),
                flyer_settings: FlyerSettings::default(),
                aqi_preview: None,
                kite_preview: false,
                hidden: HideSet::default(),
            },
            12,
            30,
            themes,
        );
        let mut rng = StdRng::seed_from_u64(3);
        for _ in 0..2000 {
            app.advance_quietly((12, 30), &mut rng);
        }
        assert_eq!(app.frame, 0);
    }

    #[test]
    fn quiet_ticks_follow_the_pane_after_a_resize() {
        let mut app = test_app(Clock::Real, "clear", false);
        let mut rng = StdRng::seed_from_u64(4);
        app.advance_quietly((60, 20), &mut rng);
        let layout = app.scenes.get(app.active_scene_id).unwrap().layout();
        assert_eq!((layout.width, layout.height), (60, 20));
    }

    #[test]
    fn draw_frame_draws_the_scene_without_a_terminal() {
        let mut app = test_app(Clock::Real, "clear", false);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        let text = renderer.text();
        assert!(text.contains("[___]"), "{text}");
        assert!(text.contains("Weather: Clear"), "{text}");
    }

    fn at(text: &str) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap()
    }

    #[test]
    fn fake_sky_follows_the_sun_at_the_location() {
        for (when, force_night, day) in [
            ("2026-10-30T22:10", false, false),
            ("2026-10-30T22:45", false, true),
            ("2026-10-31T10:35", false, true),
            ("2026-10-31T11:00", false, false),
            ("2026-10-31T03:00", true, false),
        ] {
            assert_eq!(
                sky_at(utc(when), jakarta(), force_night).0,
                day,
                "{when} {force_night}"
            );
        }
    }

    #[test]
    fn the_sun_follows_the_clock_and_the_location() {
        let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
        let mut app = test_app(Clock::fake_at(at("2026-10-19T17:00"), wib), "clear", false);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        let ten_utc = chrono::NaiveDate::from_ymd_opt(2026, 10, 19)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc();
        let expected = crate::astro::sun_sky(
            app.state.location.latitude,
            app.state.location.longitude,
            ten_utc,
        );
        let sky = app.state.sun_sky.expect("the app places the sun");
        assert!(
            (sky.elevation_deg - expected.elevation_deg).abs() < 0.05,
            "{sky:?} {expected:?}"
        );
    }

    #[test]
    fn nyepi_night_keeps_the_sky_empty() {
        let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
        let sees_the_ufo = |when: &str| {
            let mut themes = ThemeRegistry::new();
            themes.activate_resolved(crate::theme::load::resolve("tokyo-night", None));
            let mut app = App::new(
                &Config::default(),
                AppOptions {
                    simulate_condition: Some("clear".to_string()),
                    simulate_night: true,
                    show_leaves: false,
                    clock: Clock::fake_at(at(when), wib),
                    event_settings: EventSettings::in_country("ID"),
                    flyer_settings: FlyerSettings {
                        every: crate::animation::flyers::settings::Every::Busy,
                        specials: 1,
                        characters: vec!["ufo"],
                        ..FlyerSettings::default()
                    },
                    aqi_preview: None,
                    kite_preview: false,
                    hidden: HideSet::default(),
                },
                110,
                32,
                themes,
            );
            let mut renderer = TerminalRenderer::headless(110, 32);
            let mut rng = StdRng::seed_from_u64(8);
            (0..1500).any(|_| {
                app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
                renderer.text().contains("(_________)")
            })
        };
        assert!(!sees_the_ufo("2027-03-08T21:00"));
        assert!(sees_the_ufo("2027-03-09T21:00"));
    }

    #[test]
    fn hiding_the_moon_keeps_the_moon_biker_home() {
        let cet = chrono::FixedOffset::east_opt(3600).unwrap();
        let sees_the_biker = |hide: &[&str]| {
            let mut themes = ThemeRegistry::new();
            themes.activate_resolved(crate::theme::load::resolve("tokyo-night", None));
            let names: Vec<String> = hide.iter().map(|name| name.to_string()).collect();
            let mut app = App::new(
                &Config::default(),
                AppOptions {
                    simulate_condition: Some("clear".to_string()),
                    simulate_night: false,
                    show_leaves: false,
                    clock: Clock::fake_at(at("2026-10-26T21:00"), cet),
                    event_settings: EventSettings::in_country("ID"),
                    flyer_settings: FlyerSettings {
                        every: crate::animation::flyers::settings::Every::Busy,
                        specials: 1,
                        characters: vec!["moon-biker"],
                        ..FlyerSettings::default()
                    },
                    aqi_preview: None,
                    kite_preview: false,
                    hidden: HideSet::parse(&names).0,
                },
                110,
                32,
                themes,
            );
            let mut renderer = TerminalRenderer::headless(110, 32);
            let mut rng = StdRng::seed_from_u64(26);
            (0..1500).any(|_| {
                app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
                renderer.text().contains("*@*")
            })
        };
        assert!(sees_the_biker(&[]));
        assert!(!sees_the_biker(&["moon"]));
    }

    #[test]
    fn an_unfocused_pane_draws_less_often_but_keeps_the_pace() {
        let ms = |plan: (Duration, u32)| (plan.0.as_millis(), plan.1);
        assert_eq!(ms(frame_plan(true, 30, 10)), (33, 1));
        assert_eq!(ms(frame_plan(false, 30, 10)), (100, 3));
        assert_eq!(ms(frame_plan(false, 30, 0)), (33, 1));
        assert_eq!(ms(frame_plan(true, 15, 10)), (66, 2));
        assert_eq!(ms(frame_plan(false, 15, 30)), (66, 2));
        assert_eq!(ms(frame_plan(true, 20, 10)), (66, 2));
        assert_eq!(ms(frame_plan(true, 24, 10)), (33, 1));
        assert_eq!(ms(frame_plan(true, 12, 10)), (100, 3));
        assert_eq!(ms(frame_plan(true, 7, 10)), (133, 4));
        assert_eq!(ms(frame_plan(false, 30, 6)), (166, 5));
    }

    #[test]
    fn quiet_ticks_move_the_scene_without_drawing() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut quiet =
            snapshot_app_with("2026-10-20T10:00", 110, 32, FlyerSettings::default(), true);
        quiet.animations.update_wind(20.0, 225.0);
        for _ in 0..200 {
            quiet.advance_quietly((110, 32), &mut rng);
        }
        let mut renderer = TerminalRenderer::headless(110, 32);
        quiet.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(renderer.text().contains("+-|"), "{}", renderer.text());

        let mut fresh =
            snapshot_app_with("2026-10-20T10:00", 110, 32, FlyerSettings::default(), true);
        fresh.animations.update_wind(20.0, 225.0);
        let mut renderer = TerminalRenderer::headless(110, 32);
        fresh.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(!renderer.text().contains("+-|"));
    }

    #[test]
    fn a_tiny_pane_shows_a_note_instead_of_the_scene() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut app = test_app(Clock::Real, "clear", false);
        app.animations.on_resize(30, 8);
        let mut tiny = TerminalRenderer::headless(30, 8);
        app.draw_frame(&mut tiny, &mut rng, "", None).unwrap();
        assert!(tiny.text().contains("pane too small"), "{}", tiny.text());
        app.animations.on_resize(40, 12);
        let mut usable = TerminalRenderer::headless(40, 12);
        app.draw_frame(&mut usable, &mut rng, "", None).unwrap();
        assert!(
            !usable.text().contains("pane too small"),
            "{}",
            usable.text()
        );
    }

    #[test]
    fn the_bar_counts_down_to_the_new_year() {
        let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
        let mut app = test_app(Clock::fake_at(at("2026-12-31T23:12"), wib), "clear", false);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert_eq!(
            app.state
                .greeting
                .as_ref()
                .map(|greeting| greeting.text.as_str()),
            Some("2027 in 00:48")
        );
    }

    #[test]
    fn fake_sky_moon_is_waning_gibbous_on_halloween_2026() {
        let (_, phase) = sky_at(utc("2026-10-31T14:00"), jakarta(), false);
        assert!((0.6..0.75).contains(&phase), "{phase}");
    }

    #[test]
    fn a_fake_clock_overrides_the_simulated_daylight() {
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(1);

        let cet = chrono::FixedOffset::east_opt(3600).unwrap();
        let mut app = test_app(Clock::fake_at(at("2026-10-31T21:00"), cet), "clear", false);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(!app.state.weather_conditions.sun.is_day);
        assert!(!app.state.current_weather.as_ref().unwrap().sun.is_day);

        let mut app = test_app(Clock::Real, "clear", false);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        assert!(app.state.weather_conditions.sun.is_day);
    }

    fn snapshot_app(when: &str, width: u16, height: u16) -> App {
        snapshot_app_with(when, width, height, FlyerSettings::default(), false)
    }

    fn snapshot_app_with(
        when: &str,
        width: u16,
        height: u16,
        flyer_settings: FlyerSettings,
        kite_preview: bool,
    ) -> App {
        let mut themes = ThemeRegistry::new();
        themes.activate_resolved(crate::theme::load::resolve("tokyo-night", None));
        let config = Config {
            hide: vec!["stars".to_string(), "clouds".to_string()],
            icons: "nerd".to_string(),
            glyphs: "ascii".to_string(),
            ..Config::default()
        };
        App::new(
            &config,
            AppOptions {
                simulate_condition: Some("clear".to_string()),
                simulate_night: false,
                show_leaves: false,
                clock: Clock::fake_at(at(when), chrono::FixedOffset::east_opt(3600).unwrap()),
                event_settings: EventSettings::in_country("ID"),
                flyer_settings,
                aqi_preview: None,
                kite_preview,
                hidden: HideSet::parse(&config.hide).0,
            },
            width,
            height,
            themes,
        )
    }

    #[test]
    fn the_kite_never_panics_on_small_or_resized_panes() {
        let mut rng = StdRng::seed_from_u64(10);
        for (width, height) in [(20, 5), (40, 10), (70, 20), (80, 24)] {
            let mut app = snapshot_app_with(
                "2026-10-20T10:00",
                width,
                height,
                FlyerSettings::default(),
                true,
            );
            app.animations.update_wind(30.0, 225.0);
            let mut renderer = TerminalRenderer::headless(width, height);
            for _ in 0..1200 {
                app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
            }
        }
        let mut app =
            snapshot_app_with("2026-10-20T10:00", 110, 32, FlyerSettings::default(), true);
        app.animations.update_wind(30.0, 225.0);
        let mut renderer = TerminalRenderer::headless(110, 32);
        for _ in 0..400 {
            app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        }
        assert!(renderer.text().contains("+-|"), "the kite flyer is out");
        app.animations.on_resize(20, 5);
        let mut small = TerminalRenderer::headless(20, 5);
        for _ in 0..600 {
            app.draw_frame(&mut small, &mut rng, "", None).unwrap();
        }
    }

    fn middle_column(text: &str, marks: &[&str]) -> Option<usize> {
        text.lines()
            .find_map(|line| marks.iter().find_map(|mark| line.find(mark)))
            .filter(|col| (30..80).contains(col))
    }

    fn frame_until(when: &str, done: impl Fn(&str) -> bool) -> String {
        let mut app = snapshot_app(when, 110, 32);
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..1500 {
            app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
            let text = renderer.text();
            if done(&text) {
                return text;
            }
        }
        panic!("the frame never showed what the snapshot waits for");
    }

    #[test]
    fn snapshot_halloween_night() {
        let text = frame_until("2026-10-31T21:00", |text| {
            middle_column(text, &[",_,"]).is_some()
        });
        assert!(text.contains("( ^  ^ )"), "{text}");
        assert!(text.contains("\u{f0bbf} Happy Halloween"), "{text}");
        insta::assert_snapshot!(text);
    }

    #[test]
    fn snapshot_halloween_day() {
        let text = frame_until("2026-10-31T10:00", |_| true);
        assert!(text.contains("( ^  ^ )"), "{text}");
        assert!(!text.contains(",_,"), "{text}");
        insta::assert_snapshot!(text);
    }

    #[test]
    fn snapshot_halloween_witch() {
        let text = frame_until("2026-10-31T21:00", |text| {
            middle_column(text, &["<===<<<", ">>>===>"]).is_some()
        });
        insta::assert_snapshot!(text);
    }

    fn preview(id: &'static str) -> FlyerSettings {
        FlyerSettings {
            preview: Some(id),
            ..FlyerSettings::default()
        }
    }

    fn run_until(mut app: App, width: u16, height: u16, done: impl Fn(&str) -> bool) -> String {
        let mut renderer = TerminalRenderer::headless(width, height);
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..1500 {
            app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
            let text = renderer.text();
            if done(&text) {
                return text;
            }
        }
        panic!("the frame never showed what the snapshot waits for");
    }

    #[test]
    fn snapshot_caped_hero_by_day() {
        let app = snapshot_app_with("2026-10-20T10:00", 110, 32, preview("caped-hero"), false);
        let text = run_until(app, 110, 32, |text| {
            middle_column(text, &["* =="]).is_some()
        });
        insta::assert_snapshot!(text);
    }

    #[test]
    fn snapshot_moon_biker_crossing_the_full_moon() {
        let app = snapshot_app_with("2026-10-26T21:00", 110, 32, preview("moon-biker"), false);
        let text = run_until(app, 110, 32, |text| {
            let lines: Vec<&str> = text.lines().collect();
            let moon_left = lines
                .iter()
                .find_map(|line| line.find("_..._"))
                .map(|col| col.saturating_sub(7));
            let biker = lines.get(9).and_then(|line| line.find("*@*"));
            matches!((moon_left, biker), (Some(left), Some(col)) if (left..=left + 12).contains(&col))
        });
        insta::assert_snapshot!(text);
    }

    #[test]
    fn snapshot_kite_at_twenty_kmh() {
        let mut app =
            snapshot_app_with("2026-10-20T10:00", 110, 32, FlyerSettings::default(), true);
        app.animations.update_wind(20.0, 225.0);
        let text = run_until(app, 110, 32, |text| {
            let lines: Vec<&str> = text.lines().collect();
            lines[8].rfind("/\\").is_some_and(|col| col > 80) && lines[25].contains("+-|")
        });
        insta::assert_snapshot!(text);
    }

    #[test]
    fn snapshot_short_pane_has_no_flyers_or_kite() {
        let mut app = snapshot_app_with("2026-10-20T10:00", 110, 20, preview("caped-hero"), true);
        app.animations.update_wind(20.0, 225.0);
        let mut renderer = TerminalRenderer::headless(110, 20);
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..900 {
            app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        }
        let text = renderer.text();
        assert!(!text.contains("* =="), "{text}");
        assert!(!text.contains("+-|"), "{text}");
        insta::assert_snapshot!(text);
    }

    #[test]
    fn the_details_card_sits_over_the_scene() {
        let mut app = snapshot_app("2026-10-20T10:00", 110, 32);
        app.details_open = true;
        let mut renderer = TerminalRenderer::headless(110, 32);
        let mut rng = StdRng::seed_from_u64(7);
        app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        let text = renderer.text();
        for shown in [
            "+- details -",
            "Clear · 20.0°C",
            "10.0 km/h from the SW",
            "| Sun       rise ",
            "waxing gibbous",
        ] {
            assert!(text.contains(shown), "{shown}\n{text}");
        }
    }

    #[test]
    fn small_and_resized_panes_never_panic() {
        let mut rng = StdRng::seed_from_u64(9);
        for (width, height) in [(20, 5), (40, 10), (70, 20), (80, 24)] {
            let mut app = snapshot_app("2026-10-31T21:00", width, height);
            let mut renderer = TerminalRenderer::headless(width, height);
            for _ in 0..1200 {
                app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
            }
        }
        let mut app = snapshot_app("2026-10-31T21:00", 110, 32);
        let mut renderer = TerminalRenderer::headless(110, 32);
        for _ in 0..1000 {
            app.draw_frame(&mut renderer, &mut rng, "", None).unwrap();
        }
        app.animations.on_resize(20, 5);
        let mut small = TerminalRenderer::headless(20, 5);
        for _ in 0..600 {
            app.draw_frame(&mut small, &mut rng, "", None).unwrap();
        }
    }

    #[test]
    fn switching_to_the_ip_location_moves_the_app_and_keeps_the_aqi_preview() {
        let mut themes = ThemeRegistry::new();
        themes.activate_resolved(crate::theme::load::resolve("tokyo-night", None));
        let mut app = App::new(
            &Config::default(),
            AppOptions {
                simulate_condition: Some("clear".to_string()),
                simulate_night: false,
                show_leaves: false,
                clock: Clock::Real,
                event_settings: EventSettings::in_country("ID"),
                flyer_settings: FlyerSettings::default(),
                aqi_preview: Some(175),
                kite_preview: false,
                hidden: HideSet::default(),
            },
            110,
            32,
            themes,
        );
        let (sender, receiver) = tokio::sync::oneshot::channel();
        sender
            .send(Ok(crate::geolocation::GeoLocation {
                latitude: 1.29,
                longitude: 103.85,
                city: Some("Singapore".to_string()),
            }))
            .unwrap();
        let mut check = IpCheck::new(receiver, (52.52, 13.41), "Berlin".to_string());
        assert!(check.poll(std::time::Instant::now()).is_some());
        app.ip_check = Some(check);

        app.switch_to_ip_location();

        assert_eq!(
            (app.state.location.latitude, app.state.location.longitude),
            (1.29, 103.85)
        );
        assert_eq!(app.state.city_name.as_deref(), Some("Singapore"));
        assert_eq!(
            app.state.air_quality.map(|reading| reading.us_aqi),
            Some(175)
        );
        assert!(
            app.state.current_weather.is_some(),
            "the simulated weather stays"
        );
        app.state.update_cached_info();
        assert!(
            !app.state.cached_weather_info.contains("Loading"),
            "{}",
            app.state.cached_weather_info
        );
    }

    #[test]
    fn switching_to_the_ip_location_brings_that_country_s_holidays() {
        let mut settings = EventSettings::default();
        settings.locate(-6.2088, 106.8456);
        assert_eq!(settings.country, Some("ID"));
        let mut app = test_app(Clock::Real, "clear", false);
        app.event_settings = settings;
        let (sender, receiver) = tokio::sync::oneshot::channel();
        sender
            .send(Ok(crate::geolocation::GeoLocation {
                latitude: 3.15,
                longitude: 101.69,
                city: Some("Kuala Lumpur".to_string()),
            }))
            .unwrap();
        let mut check = IpCheck::new(receiver, (-6.2088, 106.8456), "Jakarta".to_string());
        assert!(check.poll(std::time::Instant::now()).is_some());
        app.ip_check = Some(check);
        app.switch_to_ip_location();
        assert_eq!(app.event_settings.country, Some("MY"));
    }

    #[test]
    fn the_aqi_preview_is_a_fixed_reading_without_a_task() {
        let mut themes = ThemeRegistry::new();
        themes.activate_resolved(crate::theme::load::resolve("tokyo-night", None));
        let app = App::new(
            &Config::default(),
            AppOptions {
                simulate_condition: Some("clear".to_string()),
                simulate_night: false,
                show_leaves: false,
                clock: Clock::Real,
                event_settings: EventSettings::in_country("ID"),
                flyer_settings: FlyerSettings::default(),
                aqi_preview: Some(175),
                kite_preview: false,
                hidden: HideSet::default(),
            },
            110,
            32,
            themes,
        );
        assert_eq!(
            app.state.air_quality,
            Some(crate::weather::air_quality::AirQuality {
                us_aqi: 175,
                fetched_at: None
            })
        );
        assert!(app.air_source.is_none());
        assert!(app.air_task.is_none());
    }
}
