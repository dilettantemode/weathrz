use crate::astro::SunSky;
use crate::config::{IconMode, LocationDisplay};
use crate::hud::{Greeting, HudRole, HudSegment, ascii_segments, greeting_segments, nerd_segments};
use crate::weather::air_quality::AirQuality;
use crate::weather::{
    WeatherCondition, WeatherConditions, WeatherData, WeatherLocation, WeatherUnits,
};
use std::time::{Instant, SystemTime};

pub struct AppState {
    pub current_weather: Option<WeatherData>,
    pub is_offline: bool,
    pub weather_conditions: WeatherConditions,
    pub loading_state: LoadingState,
    pub cached_weather_info: String,
    pub weather_info_needs_update: bool,
    pub location: WeatherLocation,
    pub city_name: Option<String>,
    pub location_display: LocationDisplay,
    pub hide_location: bool,
    pub units: WeatherUnits,
    pub icon_mode: IconMode,
    pub cached_hud_segments: Vec<HudSegment>,
    pub greeting: Option<Greeting>,
    pub air_quality: Option<AirQuality>,
    pub sun_sky: Option<SunSky>,
    pub moon_sky: Option<crate::astro::MoonSky>,
    pub glyphs: crate::glyphs::Glyphs,
    pub reading_at: Option<SystemTime>,
    offline_age: Option<String>,
    fitted_hud: Option<(u16, Vec<HudSegment>)>,
}

fn with_age(mut segments: Vec<HudSegment>, age: Option<&str>) -> Vec<HudSegment> {
    if let Some(age) = age {
        for segment in &mut segments {
            if segment.role == HudRole::Offline {
                segment.text = format!("{} · {age}", segment.text);
            }
        }
    }
    segments
}

impl AppState {
    pub fn new(
        location: WeatherLocation,
        city_name: Option<String>,
        location_display: LocationDisplay,
        hide_location: bool,
        units: WeatherUnits,
    ) -> Self {
        Self {
            current_weather: None,
            is_offline: false,
            weather_conditions: WeatherConditions::default(),
            loading_state: LoadingState::new(),
            cached_weather_info: String::new(),
            weather_info_needs_update: true,
            location,
            city_name,
            location_display,
            hide_location,
            units,
            icon_mode: IconMode::Ascii,
            cached_hud_segments: Vec::new(),
            greeting: None,
            air_quality: None,
            sun_sky: None,
            moon_sky: None,
            glyphs: crate::glyphs::Glyphs::Unicode,
            reading_at: None,
            offline_age: None,
            fitted_hud: None,
        }
    }

    pub fn update_weather(&mut self, weather: WeatherData) {
        self.weather_conditions.is_thunderstorm = weather.condition.is_thunderstorm();
        self.weather_conditions.is_snowing = weather.condition.is_snowing();
        self.weather_conditions.is_raining =
            weather.condition.is_raining() && !self.weather_conditions.is_thunderstorm;
        self.weather_conditions.is_cloudy = weather.condition.is_cloudy();
        self.weather_conditions.is_foggy = weather.condition.is_foggy();
        self.weather_conditions.sun = weather.sun;

        self.current_weather = Some(weather);
        self.is_offline = false;
        self.weather_info_needs_update = true;
    }

    pub fn set_location(&mut self, location: WeatherLocation, city_name: Option<String>) {
        self.location = location;
        self.city_name = city_name;
        self.current_weather = None;
        self.air_quality = None;
        self.is_offline = false;
        self.weather_info_needs_update = true;
    }

    pub fn set_offline_mode(&mut self, offline: bool) {
        self.is_offline = offline;
        self.weather_info_needs_update = true;
    }

    pub fn set_icon_mode(&mut self, mode: IconMode) {
        self.icon_mode = mode;
        self.weather_info_needs_update = true;
    }

    pub fn set_air_quality(&mut self, reading: AirQuality) {
        self.air_quality = Some(reading);
        self.weather_info_needs_update = true;
    }

    pub fn fresh_aqi(&self, now: SystemTime) -> Option<u16> {
        self.air_quality
            .filter(|reading| !reading.is_stale(now))
            .map(|reading| reading.us_aqi)
    }

    pub fn drop_stale_air_quality(&mut self, now: SystemTime) -> bool {
        if self
            .air_quality
            .is_some_and(|reading| reading.is_stale(now))
        {
            self.air_quality = None;
            self.weather_info_needs_update = true;
            true
        } else {
            false
        }
    }

    pub fn set_offline_age(&mut self, age: Option<String>) {
        if self.offline_age != age {
            self.offline_age = age;
            self.weather_info_needs_update = true;
        }
    }

    pub fn set_greeting(&mut self, greeting: Option<Greeting>) {
        if self.greeting != greeting {
            self.greeting = greeting;
            self.weather_info_needs_update = true;
        }
    }

    pub fn update_loading_animation(&mut self) {
        if self.loading_state.should_update() {
            self.loading_state.next_frame();
            self.weather_info_needs_update = true;
        }
    }

    pub fn get_condition_text(&self) -> &str {
        match self.current_weather {
            Some(ref weather) => condition_label(weather.condition),
            None => crate::text::LOADING,
        }
    }

    fn location_label(&self) -> Option<String> {
        if self.hide_location {
            return None;
        }
        let (lat_value, lat_dir) = if self.location.latitude >= 0.0 {
            (self.location.latitude, "N")
        } else {
            (-self.location.latitude, "S")
        };
        let (lon_value, lon_dir) = if self.location.longitude >= 0.0 {
            (self.location.longitude, "E")
        } else {
            (-self.location.longitude, "W")
        };
        let coords = format!("{:.2}°{}, {:.2}°{}", lat_value, lat_dir, lon_value, lon_dir);
        Some(match self.location_display {
            LocationDisplay::Coordinates => coords,
            LocationDisplay::City => match &self.city_name {
                Some(city) => city.clone(),
                None => coords,
            },
            LocationDisplay::Mixed => match &self.city_name {
                Some(city) => format!("{} ({})", city, coords),
                None => coords,
            },
        })
    }

    pub fn update_cached_info(&mut self) {
        if !self.weather_info_needs_update {
            return;
        }

        let location_label = self.location_label();
        let aqi = self.fresh_aqi(SystemTime::now());
        let shown = self.current_weather.as_ref().map(|weather| {
            if self.is_offline {
                WeatherData {
                    outlook: Vec::new(),
                    ..weather.clone()
                }
            } else {
                weather.clone()
            }
        });

        let ascii = shown.as_ref().map(|weather| {
            with_age(
                ascii_segments(
                    weather,
                    self.get_condition_text(),
                    &self.units,
                    self.is_offline,
                    location_label.as_deref(),
                    aqi,
                ),
                self.offline_age.as_deref(),
            )
        });
        self.cached_weather_info = match &ascii {
            Some(parts) => parts.iter().map(|part| part.text.as_str()).collect(),
            None if self.is_offline => crate::text::OFFLINE_RETRYING.to_string(),
            None => format!(
                "{}: {}... {}",
                crate::text::WEATHER,
                crate::text::LOADING,
                self.loading_state.current_char()
            ),
        };

        let body = match (&shown, self.icon_mode, ascii) {
            (Some(weather), IconMode::Nerd, _) => with_age(
                nerd_segments(
                    weather,
                    self.get_condition_text(),
                    &self.units,
                    self.is_offline,
                    location_label.as_deref(),
                    aqi,
                ),
                self.offline_age.as_deref(),
            ),
            (_, IconMode::Ascii, Some(parts)) => parts,
            _ => vec![crate::hud::segment(
                HudRole::Plain,
                self.cached_weather_info.clone(),
            )],
        };

        self.cached_hud_segments = greeting_segments(self.greeting.as_ref(), self.icon_mode)
            .into_iter()
            .chain(body)
            .collect();
        self.fitted_hud = None;

        self.weather_info_needs_update = false;
    }

    pub fn hud_bar(&mut self, width: u16) -> &[HudSegment] {
        if self
            .fitted_hud
            .as_ref()
            .is_none_or(|(fitted_width, _)| *fitted_width != width)
        {
            let bar = crate::hud::fit(&self.cached_hud_segments, width, self.icon_mode);
            self.fitted_hud = Some((width, bar));
        }
        self.fitted_hud
            .as_ref()
            .map_or(&[], |(_, bar)| bar.as_slice())
    }

    pub fn should_show_sun(&self) -> bool {
        if !self.weather_conditions.sun.is_day {
            return false;
        }

        if let Some(ref weather) = self.current_weather {
            matches!(
                weather.condition,
                WeatherCondition::Clear | WeatherCondition::PartlyCloudy | WeatherCondition::Cloudy
            )
        } else {
            false
        }
    }

    pub fn should_show_fireflies(&self) -> bool {
        if self.weather_conditions.sun.is_day {
            return false;
        }

        if let Some(ref weather) = self.current_weather {
            let is_warm = weather.temperature > 15.0;
            let is_clear_night = matches!(
                weather.condition,
                WeatherCondition::Clear | WeatherCondition::PartlyCloudy
            );
            is_warm
                && is_clear_night
                && !self.weather_conditions.is_raining
                && !self.weather_conditions.is_thunderstorm
                && !self.weather_conditions.is_snowing
        } else {
            false
        }
    }
}

pub struct LoadingState {
    pub frame: usize,
    pub last_update: Instant,
    pub loading_chars: [char; 4],
}

impl LoadingState {
    pub fn new() -> Self {
        Self {
            frame: 0,
            last_update: Instant::now(),
            loading_chars: ['|', '/', '-', '\\'],
        }
    }

    pub fn should_update(&self) -> bool {
        self.last_update.elapsed() >= std::time::Duration::from_millis(100)
    }

    pub fn next_frame(&mut self) {
        self.frame = (self.frame + 1) % self.loading_chars.len();
        self.last_update = Instant::now();
    }

    pub fn current_char(&self) -> char {
        self.loading_chars[self.frame]
    }
}

impl Default for LoadingState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn condition_label(condition: WeatherCondition) -> &'static str {
    crate::text::condition(condition)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LocationDisplay;
    use crate::weather::types::{
        CelestialEvents, PrecipitationUnit, TemperatureUnit, WindSpeedUnit,
    };

    fn create_app_state(lat: f64, lon: f64) -> AppState {
        create_app_state_full(lat, lon, None, LocationDisplay::Coordinates)
    }

    fn create_app_state_full(
        lat: f64,
        lon: f64,
        city: Option<String>,
        display: LocationDisplay,
    ) -> AppState {
        let location = WeatherLocation {
            latitude: lat,
            longitude: lon,
            elevation: None,
        };
        let units = WeatherUnits {
            temperature: TemperatureUnit::Celsius,
            wind_speed: WindSpeedUnit::Kmh,
            precipitation: PrecipitationUnit::Mm,
        };
        let mut app = AppState::new(location, city, display, false, units);

        let weather = WeatherData {
            condition: WeatherCondition::Clear,
            temperature: 20.0,
            precipitation: 0.0,
            wind_speed: 10.0,
            wind_direction: 0.0,
            moon_phase: Some(0.5),
            humidity: None,
            timestamp: "2024-01-01T12:00:00Z".to_string(),
            attribution: "".to_string(),
            feels_like: None,
            uv_index: None,
            outlook: Vec::new(),
            sun: CelestialEvents::from_bool(true),
        };
        app.update_weather(weather);

        app
    }

    #[test]
    fn test_new_york_coordinates() {
        // New York: 40.7128°N, 74.0060°W (positive lat, negative lon)
        let mut app = create_app_state(40.7128, -74.0060);
        app.update_cached_info();

        println!("NYC: {}", app.cached_weather_info);
        assert!(app.cached_weather_info.contains("40.71°N"));
        assert!(app.cached_weather_info.contains("74.01°W"));
    }

    #[test]
    fn test_sydney_coordinates() {
        // Sydney: 33.8688°S, 151.2093°E (negative lat, positive lon)
        let mut app = create_app_state(-33.8688, 151.2093);
        app.update_cached_info();

        println!("Sydney: {}", app.cached_weather_info);
        assert!(app.cached_weather_info.contains("33.87°S"));
        assert!(app.cached_weather_info.contains("151.21°E"));
    }

    #[test]
    fn test_london_coordinates() {
        // London: 51.5074°N, 0.1278°W (positive lat, negative lon near 0)
        let mut app = create_app_state(51.5074, -0.1278);
        app.update_cached_info();

        println!("London: {}", app.cached_weather_info);
        assert!(app.cached_weather_info.contains("51.51°N"));
        assert!(app.cached_weather_info.contains("0.13°W"));
    }

    #[test]
    fn test_sao_paulo_coordinates() {
        // São Paulo: 23.5505°S, 46.6333°W (negative lat, negative lon)
        let mut app = create_app_state(-23.5505, -46.6333);
        app.update_cached_info();

        println!("São Paulo: {}", app.cached_weather_info);
        assert!(app.cached_weather_info.contains("23.55°S"));
        assert!(app.cached_weather_info.contains("46.63°W"));
    }

    #[test]
    fn test_tokyo_coordinates() {
        // Tokyo: 35.6762°N, 139.6503°E (positive lat, positive lon)
        let mut app = create_app_state(35.6762, 139.6503);
        app.update_cached_info();

        println!("Tokyo: {}", app.cached_weather_info);
        assert!(app.cached_weather_info.contains("35.68°N"));
        assert!(app.cached_weather_info.contains("139.65°E"));
    }

    #[test]
    fn test_equator_prime_meridian() {
        // Null Island: 0°, 0° (exactly at equator and prime meridian)
        let mut app = create_app_state(0.0, 0.0);
        app.update_cached_info();

        println!("Null Island: {}", app.cached_weather_info);
        assert!(app.cached_weather_info.contains("0.00°N"));
        assert!(app.cached_weather_info.contains("0.00°E"));
    }

    #[test]
    fn test_display_coordinates_mode() {
        let mut app = create_app_state_full(
            34.0754,
            -84.2941,
            Some("Alpharetta".to_string()),
            LocationDisplay::Coordinates,
        );
        app.update_cached_info();

        assert!(
            app.cached_weather_info
                .contains("Location: 34.08°N, 84.29°W")
        );
        assert!(!app.cached_weather_info.contains("Alpharetta"));
    }

    #[test]
    fn test_set_location_switches_place_and_waits_for_fresh_weather() {
        let mut app = create_app_state_full(
            -6.2088,
            106.8456,
            Some("Jakarta".to_string()),
            LocationDisplay::City,
        );
        app.set_offline_mode(true);
        app.update_cached_info();

        app.set_location(
            WeatherLocation {
                latitude: 1.2897,
                longitude: 103.8501,
                elevation: None,
            },
            Some("Singapore".to_string()),
        );

        assert_eq!(app.location.latitude, 1.2897);
        assert_eq!(app.location.longitude, 103.8501);
        assert_eq!(app.location_label().as_deref(), Some("Singapore"));
        assert!(app.current_weather.is_none());
        assert!(!app.is_offline);
        app.update_cached_info();
        assert!(app.cached_weather_info.starts_with("Weather: Loading"));
    }

    #[test]
    fn test_display_city_mode_with_city() {
        let mut app = create_app_state_full(
            34.0754,
            -84.2941,
            Some("Alpharetta".to_string()),
            LocationDisplay::City,
        );
        app.update_cached_info();

        assert!(app.cached_weather_info.contains("Location: Alpharetta"));
        assert!(!app.cached_weather_info.contains("34.08°N"));
    }

    #[test]
    fn test_display_city_mode_without_city_falls_back() {
        let mut app = create_app_state_full(34.0754, -84.2941, None, LocationDisplay::City);
        app.update_cached_info();

        assert!(
            app.cached_weather_info
                .contains("Location: 34.08°N, 84.29°W")
        );
    }

    #[test]
    fn test_display_mixed_mode_with_city() {
        let mut app = create_app_state_full(
            34.0754,
            -84.2941,
            Some("Alpharetta".to_string()),
            LocationDisplay::Mixed,
        );
        app.update_cached_info();

        assert!(
            app.cached_weather_info
                .contains("Location: Alpharetta (34.08°N, 84.29°W)")
        );
    }

    #[test]
    fn test_display_mixed_mode_without_city_falls_back() {
        let mut app = create_app_state_full(34.0754, -84.2941, None, LocationDisplay::Mixed);
        app.update_cached_info();

        assert!(
            app.cached_weather_info
                .contains("Location: 34.08°N, 84.29°W")
        );
        assert!(!app.cached_weather_info.contains("("));
    }

    #[test]
    fn test_nerd_mode_builds_coloured_segments() {
        let mut app = create_app_state_full(
            -6.2,
            106.8,
            Some("Jakarta".to_string()),
            LocationDisplay::City,
        );
        app.set_icon_mode(crate::config::IconMode::Nerd);
        app.update_cached_info();
        let texts: Vec<&str> = app
            .cached_hud_segments
            .iter()
            .map(|s| s.text.as_str())
            .collect();
        assert_eq!(
            texts,
            [
                "\u{e30d} Clear",
                "   ",
                "\u{e350} 20.0°C",
                "   ",
                "\u{e34b} 36.0 km/h",
                "   ",
                "\u{e371} 0.0 mm",
                "   ",
                "\u{f041} Jakarta"
            ]
        );
        assert!(app.cached_weather_info.contains("Press 'q' to quit"));
    }

    #[test]
    fn test_ascii_mode_splits_the_bar_into_parts_that_spell_the_line() {
        let mut app = create_app_state(40.7128, -74.0060);
        app.update_cached_info();
        assert!(app.cached_hud_segments.len() > 1);
        let line: String = app
            .cached_hud_segments
            .iter()
            .map(|s| s.text.as_str())
            .collect();
        assert_eq!(line, app.cached_weather_info);
        assert!(
            line.ends_with(" | Press 'q' to quit, '?' for help"),
            "{line}"
        );
    }

    #[test]
    fn test_nerd_mode_while_loading_shows_loading_text() {
        let location = WeatherLocation {
            latitude: 0.0,
            longitude: 0.0,
            elevation: None,
        };
        let mut app = AppState::new(
            location,
            None,
            LocationDisplay::Coordinates,
            false,
            WeatherUnits::metric(),
        );
        app.set_icon_mode(crate::config::IconMode::Nerd);
        app.update_cached_info();
        assert_eq!(app.cached_hud_segments.len(), 1);
        assert!(
            app.cached_hud_segments[0]
                .text
                .starts_with("Weather: Loading...")
        );
    }

    fn pumpkin() -> crate::hud::Greeting {
        crate::hud::Greeting {
            icon: '\u{f0bbf}',
            text: "Happy Halloween".to_string(),
            short: None,
        }
    }

    fn jakarta_nerd() -> AppState {
        let mut app = create_app_state_full(
            -6.2,
            106.8,
            Some("Jakarta".to_string()),
            LocationDisplay::City,
        );
        app.set_icon_mode(crate::config::IconMode::Nerd);
        app
    }

    #[test]
    fn test_greeting_leads_the_nerd_bar() {
        let mut app = jakarta_nerd();
        app.set_greeting(Some(pumpkin()));
        app.update_cached_info();
        let texts: Vec<&str> = app
            .cached_hud_segments
            .iter()
            .map(|s| s.text.as_str())
            .collect();
        assert_eq!(
            texts[..3],
            ["\u{f0bbf} Happy Halloween", "   ", "\u{e30d} Clear"]
        );
        assert_eq!(
            app.cached_hud_segments[0].role,
            crate::hud::HudRole::Greeting
        );
    }

    #[test]
    fn test_greeting_comes_before_offline() {
        let mut app = jakarta_nerd();
        app.set_greeting(Some(pumpkin()));
        app.set_offline_mode(true);
        app.update_cached_info();
        let roles: Vec<crate::hud::HudRole> =
            app.cached_hud_segments.iter().map(|s| s.role).collect();
        assert_eq!(
            roles[..3],
            [
                crate::hud::HudRole::Greeting,
                crate::hud::HudRole::Separator,
                crate::hud::HudRole::Offline
            ]
        );
    }

    #[test]
    fn test_greeting_leads_the_ascii_bar() {
        let mut app = create_app_state(40.7128, -74.0060);
        app.set_greeting(Some(pumpkin()));
        app.update_cached_info();
        assert_eq!(app.cached_hud_segments[0].text, "Happy Halloween | ");
        let rest: String = app.cached_hud_segments[1..]
            .iter()
            .map(|s| s.text.as_str())
            .collect();
        assert_eq!(rest, app.cached_weather_info);
    }

    #[test]
    fn test_the_fitted_bar_follows_the_width_and_every_rebuild() {
        let mut app = jakarta_nerd();
        app.update_cached_info();
        let fitted =
            |app: &AppState, width| crate::hud::fit(&app.cached_hud_segments, width, app.icon_mode);
        assert_eq!(app.hud_bar(200).to_vec(), fitted(&app, 200));
        assert_eq!(app.hud_bar(30).to_vec(), fitted(&app, 30));
        app.set_greeting(Some(pumpkin()));
        app.update_cached_info();
        assert_eq!(app.hud_bar(30).to_vec(), fitted(&app, 30));
        assert_eq!(app.hud_bar(200)[0].role, crate::hud::HudRole::Greeting);
    }

    #[test]
    fn test_greeting_change_marks_the_bar_for_rebuild() {
        let mut app = jakarta_nerd();
        app.update_cached_info();
        assert!(!app.weather_info_needs_update);
        app.set_greeting(Some(pumpkin()));
        assert!(app.weather_info_needs_update);
        app.update_cached_info();
        app.set_greeting(Some(pumpkin()));
        assert!(!app.weather_info_needs_update);
        app.set_greeting(None);
        assert!(app.weather_info_needs_update);
        app.update_cached_info();
        assert!(
            app.cached_hud_segments
                .iter()
                .all(|s| s.role != crate::hud::HudRole::Greeting)
        );
    }

    #[test]
    fn test_greeting_shows_while_loading() {
        let location = WeatherLocation {
            latitude: 0.0,
            longitude: 0.0,
            elevation: None,
        };
        let mut app = AppState::new(
            location,
            None,
            LocationDisplay::Coordinates,
            false,
            WeatherUnits::metric(),
        );
        app.set_icon_mode(crate::config::IconMode::Nerd);
        app.set_greeting(Some(pumpkin()));
        app.update_cached_info();
        assert_eq!(
            app.cached_hud_segments[0].role,
            crate::hud::HudRole::Greeting
        );
        assert!(
            app.cached_hud_segments
                .last()
                .unwrap()
                .text
                .starts_with("Weather: Loading...")
        );
    }

    use crate::weather::air_quality::AirQuality;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_air_quality_and_humidity_join_the_bars() {
        let mut app = jakarta_nerd();
        if let Some(weather) = app.current_weather.as_mut() {
            weather.humidity = Some(58.0);
        }
        app.set_air_quality(AirQuality {
            us_aqi: 212,
            fetched_at: None,
        });
        app.update_cached_info();
        let texts: Vec<&str> = app
            .cached_hud_segments
            .iter()
            .map(|s| s.text.as_str())
            .collect();
        assert!(texts.contains(&"\u{e373} 58%"), "{texts:?}");
        assert!(texts.contains(&"\u{f0d43} 212 \u{f157c}"), "{texts:?}");
        assert!(
            app.cached_weather_info
                .contains(" | Humidity: 58% | AQI: 212 very unhealthy | Location: Jakarta"),
            "{}",
            app.cached_weather_info
        );
    }

    #[test]
    fn test_stale_air_quality_leaves_the_bar() {
        let mut app = jakarta_nerd();
        let now = SystemTime::now();
        app.set_air_quality(AirQuality {
            us_aqi: 90,
            fetched_at: Some(now - Duration::from_secs(4 * 3600)),
        });
        assert_eq!(app.fresh_aqi(now), None);
        app.set_air_quality(AirQuality {
            us_aqi: 90,
            fetched_at: Some(now - Duration::from_secs(600)),
        });
        assert_eq!(app.fresh_aqi(now), Some(90));
        app.set_air_quality(AirQuality {
            us_aqi: 90,
            fetched_at: Some(now - Duration::from_secs(4 * 3600)),
        });
        app.update_cached_info();
        assert!(
            app.cached_hud_segments
                .iter()
                .all(|s| !matches!(s.role, crate::hud::HudRole::Aqi(_)))
        );
    }

    #[test]
    fn test_set_location_clears_air_quality() {
        let mut app = jakarta_nerd();
        app.set_air_quality(AirQuality {
            us_aqi: 50,
            fetched_at: None,
        });
        app.set_location(
            WeatherLocation {
                latitude: 1.3,
                longitude: 103.8,
                elevation: None,
            },
            Some("Singapore".to_string()),
        );
        assert_eq!(app.air_quality, None);
    }

    #[test]
    fn test_stale_air_quality_is_dropped_without_a_weather_update() {
        let mut app = jakarta_nerd();
        let now = SystemTime::now();
        app.set_air_quality(AirQuality {
            us_aqi: 120,
            fetched_at: Some(now - Duration::from_secs(600)),
        });
        app.update_cached_info();
        assert!(!app.drop_stale_air_quality(now));
        assert!(!app.weather_info_needs_update);

        let later = now + Duration::from_secs(4 * 3600);
        assert!(app.drop_stale_air_quality(later));
        assert_eq!(app.air_quality, None);
        assert!(app.weather_info_needs_update);
        assert!(!app.drop_stale_air_quality(later));

        app.set_air_quality(AirQuality {
            us_aqi: 120,
            fetched_at: None,
        });
        assert!(!app.drop_stale_air_quality(later));
    }
}
