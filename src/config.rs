use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Table;

use crate::error::ConfigError;
use crate::weather::types::WeatherUnits;

pub const ENV_LATITUDE: &str = "WEATHR_LATITUDE";
pub const ENV_LONGITUDE: &str = "WEATHR_LONGITUDE";
pub const DEFAULT_THEME: &str = "default";
pub const CONFIG_DIR: &str = "weathrz";
pub const UPSTREAM_CONFIG_DIR: &str = "weathr";

#[derive(Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LocationDisplay {
    #[default]
    Coordinates,
    City,
    Mixed,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum IconMode {
    #[default]
    Ascii,
    Nerd,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub location: Location,
    #[serde(default)]
    pub hide_hud: bool,
    #[serde(default)]
    pub units: WeatherUnits,
    #[serde(default)]
    pub silent: bool,
    #[serde(default)]
    pub provider: HashMap<Provider, Table>,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default, deserialize_with = "one_or_many")]
    pub hide: Vec<String>,
    #[serde(default = "default_icons", deserialize_with = "lenient_string")]
    pub icons: String,
    #[serde(default, deserialize_with = "lenient_string")]
    pub glyphs: String,
    #[serde(default, deserialize_with = "lenient_string")]
    pub language: String,
    #[serde(default)]
    pub events: EventsConfig,
    #[serde(default)]
    pub sky: SkyConfig,
    #[serde(default, deserialize_with = "lenient_string")]
    pub trees: String,
    #[serde(default, deserialize_with = "lenient_count")]
    pub fps: Option<i64>,
    #[serde(default, deserialize_with = "lenient_count")]
    pub idle_fps: Option<i64>,
}

fn default_theme() -> String {
    DEFAULT_THEME.to_string()
}

fn default_icons() -> String {
    "ascii".to_string()
}

fn one_or_many<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(match toml::Value::deserialize(deserializer)? {
        toml::Value::Array(items) => items.into_iter().map(value_text).collect(),
        other => vec![value_text(other)],
    })
}

fn lenient_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(value_text(toml::Value::deserialize(deserializer)?))
}

fn lenient_count<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(match toml::Value::deserialize(deserializer)? {
        toml::Value::Integer(number) => Some(number),
        _ => None,
    })
}

fn value_text(value: toml::Value) -> String {
    match value {
        toml::Value::String(text) => text,
        other => other.to_string(),
    }
}

fn text_list(value: &toml::Value) -> Vec<String> {
    match value.clone() {
        toml::Value::Array(items) => items.into_iter().map(value_text).collect(),
        other => vec![value_text(other)],
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventsConfig {
    pub enabled: bool,
    pub observances: bool,
    pub disable: Vec<String>,
    pub country: String,
    pub warnings: Vec<String>,
}

impl Default for EventsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            observances: true,
            disable: Vec::new(),
            country: String::new(),
            warnings: Vec::new(),
        }
    }
}

impl<'de> Deserialize<'de> for EventsConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_value(toml::Value::deserialize(deserializer)?))
    }
}

impl EventsConfig {
    fn from_value(value: toml::Value) -> Self {
        let mut config = Self::default();
        let toml::Value::Table(table) = value else {
            config
                .warnings
                .push("events must be a table like [events]; using the defaults".to_string());
            return config;
        };
        match table.get("enabled") {
            None => {}
            Some(toml::Value::Boolean(enabled)) => config.enabled = *enabled,
            Some(other) => config.warnings.push(format!(
                "events.enabled = {other} is not true or false; using true"
            )),
        }
        match table.get("observances") {
            None => {}
            Some(toml::Value::Boolean(observances)) => config.observances = *observances,
            Some(other) => config.warnings.push(format!(
                "events.observances = {other} is not true or false; using true"
            )),
        }
        if let Some(disable) = table.get("disable") {
            config.disable = match disable.clone() {
                toml::Value::Array(items) => items.into_iter().map(value_text).collect(),
                other => vec![value_text(other)],
            };
        }
        if let Some(country) = table.get("country") {
            config.country = value_text(country.clone());
        }
        config
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkyConfig {
    pub flyers: Vec<String>,
    pub characters: Vec<String>,
    pub every: Option<String>,
    pub specials: Option<String>,
    pub banner: Option<String>,
    pub warnings: Vec<String>,
}

impl<'de> Deserialize<'de> for SkyConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_value(toml::Value::deserialize(deserializer)?))
    }
}

impl SkyConfig {
    fn from_value(value: toml::Value) -> Self {
        let mut config = Self::default();
        let toml::Value::Table(table) = value else {
            config
                .warnings
                .push("sky must be a table like [sky]; using the defaults".to_string());
            return config;
        };
        config.flyers = table.get("flyers").map(text_list).unwrap_or_default();
        config.characters = table.get("characters").map(text_list).unwrap_or_default();
        config.every = table.get("every").cloned().map(value_text);
        config.specials = table.get("specials").cloned().map(value_text);
        config.banner = table.get("banner").cloned().map(value_text);
        config
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Copy)]
pub enum Provider {
    #[default]
    OpenMeteo,
    MetOffice,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    #[serde(default = "default_latitude")]
    pub latitude: f64,
    #[serde(default = "default_longitude")]
    pub longitude: f64,
    #[serde(default)]
    pub auto: bool,
    #[serde(default)]
    pub hide: bool,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub display: LocationDisplay,
    #[serde(default = "default_city_name_language")]
    pub city_name_language: String,
    #[serde(default)]
    pub ip_check: bool,
}

fn default_city_name_language() -> String {
    "en".to_string()
}

pub fn default_latitude() -> f64 {
    52.52
}

pub fn default_longitude() -> f64 {
    13.41
}

impl Default for Location {
    fn default() -> Self {
        Self {
            latitude: default_latitude(),
            longitude: default_longitude(),
            auto: true,
            hide: false,
            city: None,
            name: None,
            display: LocationDisplay::default(),
            city_name_language: default_city_name_language(),
            ip_check: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            eprintln!(
                "Warning: Config file not found. Create one at {:?} to customize settings.",
                config_path
            );
            let default = Self::default();
            if default.location.auto {
                eprintln!(
                    "Tip: Set latitude and longitude in config.toml for more accurate weather."
                );
            }
            let mut config = default;
            config.apply_env_overrides()?;
            return Ok(config);
        }

        let mut config = Self::load_from_path(&config_path)?;
        config.apply_env_overrides()?;
        config.validate()?;
        Ok(config)
    }

    fn apply_env_overrides(&mut self) -> Result<(), ConfigError> {
        if let Ok(val) = env::var(ENV_LATITUDE) {
            let lat = val
                .trim()
                .parse::<f64>()
                .map_err(|_| ConfigError::InvalidEnvVar {
                    name: ENV_LATITUDE,
                    value: val.clone(),
                })?;
            self.location.latitude = lat;
            self.location.auto = false;
        }

        if let Ok(val) = env::var(ENV_LONGITUDE) {
            let lon = val
                .trim()
                .parse::<f64>()
                .map_err(|_| ConfigError::InvalidEnvVar {
                    name: ENV_LONGITUDE,
                    value: val.clone(),
                })?;
            self.location.longitude = lon;
            self.location.auto = false;
        }

        Ok(())
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.location.latitude < -90.0 || self.location.latitude > 90.0 {
            return Err(ConfigError::InvalidLatitude(self.location.latitude));
        }

        if self.location.longitude < -180.0 || self.location.longitude > 180.0 {
            return Err(ConfigError::InvalidLongitude(self.location.longitude));
        }

        Ok(())
    }

    pub fn normalized_theme(&self) -> &str {
        let theme = self.theme.trim();
        if theme.is_empty() {
            DEFAULT_THEME
        } else {
            theme
        }
    }

    pub fn frame_rates(&self) -> ((u32, u32), Vec<String>) {
        let mut warnings = Vec::new();
        let mut rate = |key: &str, value: Option<i64>, default: u32, allow_off: bool| match value {
            None => default,
            Some(0) if allow_off => 0,
            Some(number) if (5..=30).contains(&number) => number as u32,
            Some(number) => {
                let clamped = number.clamp(5, 30) as u32;
                warnings.push(format!(
                    "{key} = {number} is out of range (5-30); using {clamped}"
                ));
                clamped
            }
        };
        let fps = rate("fps", self.fps, 30, false);
        let idle_fps = rate("idle_fps", self.idle_fps, 5, true);
        ((fps, idle_fps), warnings)
    }

    pub fn trees_setting(&self) -> Result<crate::scene::world::trees::TreesSetting, String> {
        crate::scene::world::trees::parse_setting(&self.trees)
    }

    pub fn language(&self) -> Result<crate::text::Language, String> {
        crate::text::parse_language(&self.language)
    }

    pub fn glyph_mode(&self) -> Result<crate::glyphs::Glyphs, String> {
        crate::glyphs::parse(&self.glyphs)
    }

    pub fn icon_mode(&self) -> Result<IconMode, String> {
        match self.icons.trim() {
            "" | "ascii" => Ok(IconMode::Ascii),
            "nerd" => Ok(IconMode::Nerd),
            other => Err(format!(
                "icons = \"{other}\" is not valid (use \"ascii\" or \"nerd\"); using \"ascii\""
            )),
        }
    }

    pub fn load_from_path(path: &PathBuf) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path).map_err(|e| ConfigError::ReadError {
            path: path.display().to_string(),
            source: e,
        })?;

        let value: toml::Value = toml::from_str(&content).map_err(ConfigError::ParseError)?;

        if let Some(loc) = value.get("location") {
            let has_lat = loc.get("latitude").is_some();
            let has_lon = loc.get("longitude").is_some();
            if has_lat && !has_lon {
                eprintln!(
                    "Warning: latitude is set but longitude is missing, defaulting longitude to 13.41 (Berlin)."
                );
            } else if has_lon && !has_lat {
                eprintln!(
                    "Warning: longitude is set but latitude is missing, defaulting latitude to 52.52 (Berlin)."
                );
            }
        }

        toml::Value::try_into(value).map_err(ConfigError::ParseError)
    }

    fn get_config_path() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .ok_or(ConfigError::NoConfigDir)?;

        Ok(Self::config_in(&config_dir))
    }

    fn config_in(config_dir: &Path) -> PathBuf {
        let own = config_dir.join(CONFIG_DIR).join("config.toml");
        let upstream = config_dir.join(UPSTREAM_CONFIG_DIR).join("config.toml");
        if !own.exists() && upstream.exists() {
            upstream
        } else {
            own
        }
    }

    pub fn config_path() -> Option<PathBuf> {
        Self::get_config_path().ok()
    }

    pub fn events_path() -> Option<PathBuf> {
        Self::get_config_path().ok().and_then(|path| {
            path.parent()
                .map(|dir| dir.join(crate::events::user::FILE_NAME))
        })
    }

    pub fn themes_dir() -> Option<PathBuf> {
        Self::get_config_path()
            .ok()?
            .parent()
            .map(|dir| dir.join("themes"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_config_deserialize_valid() {
        let toml_content = r#"
[location]
latitude = 52.52
longitude = 13.41
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, 52.52);
        assert_eq!(config.location.longitude, 13.41);
    }

    #[test]
    fn test_config_deserialize_negative_coordinates() {
        let toml_content = r#"
[location]
latitude = -33.8688
longitude = 151.2093
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, -33.8688);
        assert_eq!(config.location.longitude, 151.2093);
    }

    #[test]
    fn test_config_load_from_path_success() {
        let toml_content = r#"
[location]
latitude = 40.7128
longitude = -74.0060
"#;
        let temp_dir = std::env::temp_dir();
        let test_config_path = temp_dir.join("weathr_test_config.toml");
        fs::write(&test_config_path, toml_content).unwrap();

        let config = Config::load_from_path(&test_config_path).unwrap();
        assert_eq!(config.location.latitude, 40.7128);
        assert_eq!(config.location.longitude, -74.0060);

        fs::remove_file(test_config_path).ok();
    }

    #[test]
    fn test_config_load_from_path_file_not_found() {
        let nonexistent_path = PathBuf::from("/tmp/nonexistent_weathr_config_12345.toml");
        let result = Config::load_from_path(&nonexistent_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "ReadError");
    }

    #[test]
    fn test_config_load_from_path_invalid_toml() {
        let toml_content = "this is not valid toml {{{{";
        let temp_dir = std::env::temp_dir();
        let test_config_path = temp_dir.join("weathr_test_invalid.toml");
        fs::write(&test_config_path, toml_content).unwrap();

        let result = Config::load_from_path(&test_config_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "ParseError");

        fs::remove_file(test_config_path).ok();
    }

    #[test]
    fn test_normalized_theme_defaults_when_blank() {
        let config = Config {
            theme: "   ".to_string(),
            ..Config::default()
        };

        assert_eq!(config.normalized_theme(), "default");
    }

    #[test]
    fn test_normalized_theme_keeps_value() {
        let config = Config {
            theme: "retro".to_string(),
            ..Config::default()
        };

        assert_eq!(config.normalized_theme(), "retro");
    }

    #[test]
    fn test_config_missing_latitude() {
        let toml_content = r#"
[location]
longitude = 13.41
"#;
        let temp_dir = std::env::temp_dir();
        let test_config_path = temp_dir.join("weathr_test_missing_lat.toml");
        fs::write(&test_config_path, toml_content).unwrap();

        let config = Config::load_from_path(&test_config_path).unwrap();
        assert_eq!(config.location.latitude, default_latitude());
        assert_eq!(config.location.longitude, 13.41);

        fs::remove_file(test_config_path).ok();
    }

    #[test]
    fn test_config_missing_longitude() {
        let toml_content = r#"
[location]
latitude = 52.52
"#;
        let temp_dir = std::env::temp_dir();
        let test_config_path = temp_dir.join("weathr_test_missing_lon.toml");
        fs::write(&test_config_path, toml_content).unwrap();

        let config = Config::load_from_path(&test_config_path).unwrap();
        assert_eq!(config.location.latitude, 52.52);
        assert_eq!(config.location.longitude, default_longitude());

        fs::remove_file(test_config_path).ok();
    }

    #[test]
    fn test_location_boundary_values() {
        let toml_content = r#"
[location]
latitude = 90.0
longitude = 180.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, 90.0);
        assert_eq!(config.location.longitude, 180.0);
    }

    #[test]
    fn test_location_zero_coordinates() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.latitude, 0.0);
        assert_eq!(config.location.longitude, 0.0);
    }

    #[test]
    fn test_validation_invalid_latitude_high() {
        let config = Config {
            location: Location {
                latitude: 91.0,
                longitude: 0.0,
                auto: false,
                hide: false,
                city: None,
                name: None,
                display: LocationDisplay::default(),
                city_name_language: "auto".to_string(),
                ip_check: false,
            },
            hide_hud: false,
            units: WeatherUnits::default(),
            silent: false,
            provider: HashMap::new(),
            theme: "default".to_string(),
            ..Config::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "InvalidLatitude");
    }

    #[test]
    fn test_validation_invalid_latitude_low() {
        let config = Config {
            location: Location {
                latitude: -91.0,
                longitude: 0.0,
                auto: false,
                hide: false,
                city: None,
                name: None,
                display: LocationDisplay::default(),
                city_name_language: "auto".to_string(),
                ip_check: false,
            },
            hide_hud: false,
            units: WeatherUnits::default(),
            silent: false,
            provider: HashMap::new(),
            theme: "default".to_string(),
            ..Config::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "InvalidLatitude");
    }

    #[test]
    fn test_validation_invalid_longitude_high() {
        let config = Config {
            location: Location {
                latitude: 0.0,
                longitude: 181.0,
                auto: false,
                hide: false,
                city: None,
                name: None,
                display: LocationDisplay::default(),
                city_name_language: "auto".to_string(),
                ip_check: false,
            },
            hide_hud: false,
            units: WeatherUnits::default(),
            silent: false,
            provider: HashMap::new(),
            theme: "default".to_string(),
            ..Config::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "InvalidLongitude");
    }

    #[test]
    fn test_validation_invalid_longitude_low() {
        let config = Config {
            location: Location {
                latitude: 0.0,
                longitude: -181.0,
                auto: false,
                hide: false,
                city: None,
                name: None,
                display: LocationDisplay::default(),
                city_name_language: "auto".to_string(),
                ip_check: false,
            },
            hide_hud: false,
            units: WeatherUnits::default(),
            silent: false,
            provider: HashMap::new(),
            theme: "default".to_string(),
            ..Config::default()
        };
        let result = config.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "InvalidLongitude");
    }

    #[test]
    fn test_validation_valid_config() {
        let config = Config {
            location: Location {
                latitude: 52.52,
                longitude: 13.41,
                auto: false,
                hide: false,
                city: None,
                name: None,
                display: LocationDisplay::default(),
                city_name_language: "auto".to_string(),
                ip_check: false,
            },
            hide_hud: false,
            units: WeatherUnits::default(),
            silent: false,
            provider: HashMap::new(),
            theme: "default".to_string(),
            ..Config::default()
        };
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_units_default() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(
            config.units.temperature,
            crate::weather::types::TemperatureUnit::Celsius
        );
        assert_eq!(
            config.units.wind_speed,
            crate::weather::types::WindSpeedUnit::Kmh
        );
        assert_eq!(
            config.units.precipitation,
            crate::weather::types::PrecipitationUnit::Mm
        );
    }

    #[test]
    fn test_config_units_custom() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0

[units]
temperature = "fahrenheit"
wind_speed = "mph"
precipitation = "inch"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(
            config.units.temperature,
            crate::weather::types::TemperatureUnit::Fahrenheit
        );
        assert_eq!(
            config.units.wind_speed,
            crate::weather::types::WindSpeedUnit::Mph
        );
        assert_eq!(
            config.units.precipitation,
            crate::weather::types::PrecipitationUnit::Inch
        );
    }

    #[test]
    fn test_location_display_default() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.display, LocationDisplay::Coordinates);
    }

    #[test]
    fn test_location_display_coordinates() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
display = "coordinates"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.display, LocationDisplay::Coordinates);
    }

    #[test]
    fn test_location_display_city() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
display = "city"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.display, LocationDisplay::City);
    }

    #[test]
    fn test_location_display_mixed() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
display = "mixed"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.display, LocationDisplay::Mixed);
    }

    #[test]
    fn test_location_city_field() {
        let toml_content = r#"
[location]
latitude = 53.9
longitude = 27.5667
city = "Minsk"
display = "city"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.city, Some("Minsk".to_string()));
        assert_eq!(config.location.display, LocationDisplay::City);
    }

    #[test]
    fn test_location_city_field_default_none() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.city, None);
    }

    #[test]
    fn test_location_ip_check_defaults_off() {
        let toml_content = r#"
[location]
latitude = -6.2088
longitude = 106.8456
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert!(!config.location.ip_check);
        assert!(!Location::default().ip_check);
    }

    #[test]
    fn test_location_ip_check_enabled() {
        let toml_content = r#"
[location]
latitude = -6.2088
longitude = 106.8456
ip_check = true
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert!(config.location.ip_check);
    }

    #[test]
    fn test_city_name_language_default() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.city_name_language, "en");
    }

    #[test]
    fn test_city_name_language_explicit_auto() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
city_name_language = "auto"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.city_name_language, "auto");
    }

    #[test]
    fn test_city_name_language_explicit_en() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
city_name_language = "en"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.city_name_language, "en");
    }

    #[test]
    fn test_city_name_language_explicit_ru() {
        let toml_content = r#"
[location]
latitude = 0.0
longitude = 0.0
city_name_language = "ru"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.location.city_name_language, "ru");
    }

    #[test]
    fn test_env_var_latitude_override() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("WEATHR_LATITUDE", "48.8566");
            env::remove_var("WEATHR_LONGITUDE");
        }
        let mut config = Config::default();
        config.apply_env_overrides().unwrap();
        assert_eq!(config.location.latitude, 48.8566);
        assert!(!config.location.auto);
        unsafe { env::remove_var("WEATHR_LATITUDE") };
    }

    #[test]
    fn test_env_var_longitude_override() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("WEATHR_LATITUDE");
            env::set_var("WEATHR_LONGITUDE", "2.3522");
        }
        let mut config = Config::default();
        config.apply_env_overrides().unwrap();
        assert_eq!(config.location.longitude, 2.3522);
        assert!(!config.location.auto);
        unsafe { env::remove_var("WEATHR_LONGITUDE") };
    }

    #[test]
    fn test_env_var_both_override() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("WEATHR_LATITUDE", "35.6762");
            env::set_var("WEATHR_LONGITUDE", "139.6503");
        }
        let mut config = Config::default();
        config.apply_env_overrides().unwrap();
        assert_eq!(config.location.latitude, 35.6762);
        assert_eq!(config.location.longitude, 139.6503);
        assert!(!config.location.auto);
        unsafe {
            env::remove_var("WEATHR_LATITUDE");
            env::remove_var("WEATHR_LONGITUDE");
        }
    }

    #[test]
    fn test_env_var_invalid_latitude() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("WEATHR_LATITUDE", "not-a-number");
            env::remove_var("WEATHR_LONGITUDE");
        }
        let mut config = Config::default();
        let result = config.apply_env_overrides();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "InvalidEnvVar");
        unsafe { env::remove_var("WEATHR_LATITUDE") };
    }

    #[test]
    fn test_env_var_invalid_longitude() {
        let _guard = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("WEATHR_LATITUDE");
            env::set_var("WEATHR_LONGITUDE", "abc");
        }
        let mut config = Config::default();
        let result = config.apply_env_overrides();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), "InvalidEnvVar");
        unsafe { env::remove_var("WEATHR_LONGITUDE") };
    }

    #[test]
    fn test_env_var_overrides_config_file_values() {
        let _guard = ENV_MUTEX.lock().unwrap();
        let toml_content = r#"
[location]
latitude = 52.52
longitude = 13.41
auto = false
"#;
        unsafe {
            env::set_var("WEATHR_LATITUDE", "-33.8688");
            env::set_var("WEATHR_LONGITUDE", "151.2093");
        }
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("weathr_test_env_override.toml");
        fs::write(&path, toml_content).unwrap();
        let mut config = Config::load_from_path(&path).unwrap();
        config.apply_env_overrides().unwrap();
        assert_eq!(config.location.latitude, -33.8688);
        assert_eq!(config.location.longitude, 151.2093);
        assert!(!config.location.auto);
        fs::remove_file(path).ok();
        unsafe {
            env::remove_var("WEATHR_LATITUDE");
            env::remove_var("WEATHR_LONGITUDE");
        }
    }

    #[test]
    fn test_themes_dir_sits_next_to_config() {
        let dir = Config::themes_dir().expect("a config dir exists in tests");
        assert!(
            dir.ends_with("weathrz/themes") || dir.ends_with("weathr/themes"),
            "{dir:?}"
        );
    }

    #[test]
    fn test_config_falls_back_to_the_upstream_dir_until_its_own_exists() {
        let root = env::temp_dir().join(format!("weathrz-config-dirs-{}", std::process::id()));
        fs::remove_dir_all(&root).ok();
        assert_eq!(
            Config::config_in(&root),
            root.join("weathrz/config.toml"),
            "neither exists"
        );
        fs::create_dir_all(root.join("weathr")).unwrap();
        fs::write(root.join("weathr/config.toml"), "").unwrap();
        assert_eq!(Config::config_in(&root), root.join("weathr/config.toml"));
        fs::create_dir_all(root.join("weathrz")).unwrap();
        fs::write(root.join("weathrz/config.toml"), "").unwrap();
        assert_eq!(Config::config_in(&root), root.join("weathrz/config.toml"));
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn test_hide_accepts_a_list_or_a_single_name() {
        let config: Config = toml::from_str("hide = [\"birds\", \"mailbox\"]").unwrap();
        assert_eq!(config.hide, vec!["birds", "mailbox"]);
        let config: Config =
            toml::from_str("hide = \"birds\"\n[location]\nlatitude = 1.0\nlongitude = 2.0\n")
                .unwrap();
        assert_eq!(config.hide, vec!["birds"]);
        assert_eq!(config.location.latitude, 1.0);
        let config: Config = toml::from_str("").unwrap();
        assert!(config.hide.is_empty());
    }

    #[test]
    fn test_icons_mode() {
        let config: Config =
            toml::from_str("icons = \"nerd\"\n[location]\nlatitude = 1.0\nlongitude = 2.0\n")
                .unwrap();
        assert_eq!(config.icon_mode(), Ok(IconMode::Nerd));
        assert_eq!(config.location.latitude, 1.0);
        let config: Config =
            toml::from_str("icons = \"emoji\"\n[location]\nlatitude = 1.0\nlongitude = 2.0\n")
                .unwrap();
        assert!(config.icon_mode().is_err());
        assert_eq!(config.location.latitude, 1.0);
        let config: Config = toml::from_str("").unwrap();
        assert_eq!(config.icon_mode(), Ok(IconMode::Ascii));
        assert_eq!(Config::default().icon_mode(), Ok(IconMode::Ascii));
    }

    #[test]
    fn test_wrong_typed_new_keys_keep_the_rest_of_config() {
        let config: Config = toml::from_str(
            "hide = true\nicons = 1\ntheme = \"gruvbox\"\n[location]\nlatitude = 1.0\nlongitude = 2.0\n",
        )
        .unwrap();
        assert_eq!(config.location.latitude, 1.0);
        assert_eq!(config.theme, "gruvbox");
        assert!(config.icon_mode().is_err());
        assert_eq!(config.hide, vec!["true"]);
        let config: Config = toml::from_str("hide = [1, \"birds\"]").unwrap();
        assert_eq!(config.hide, vec!["1", "birds"]);
    }

    #[test]
    fn test_events_table_defaults_to_on() {
        let config: Config = toml::from_str("").unwrap();
        assert_eq!(config.events, EventsConfig::default());
        assert!(config.events.enabled);
        assert!(Config::default().events.enabled);
        let config: Config =
            toml::from_str("[events]\nenabled = false\ndisable = [\"halloween\"]\n").unwrap();
        assert!(!config.events.enabled);
        assert_eq!(config.events.disable, vec!["halloween"]);
        assert!(config.events.warnings.is_empty());
        assert!(config.events.observances);
        let config: Config = toml::from_str("[events]\nobservances = false\n").unwrap();
        assert!(!config.events.observances);
        let config: Config = toml::from_str("[events]\nobservances = \"no\"\n").unwrap();
        assert!(config.events.observances);
        assert!(config.events.warnings[0].contains("observances"));
        let config: Config = toml::from_str("[events]\ncountry = \"MY\"\n").unwrap();
        assert_eq!(config.events.country, "MY");
        assert_eq!(Config::default().events.country, "");
    }

    #[test]
    fn test_wrong_typed_events_keep_the_rest_of_config() {
        let config: Config = toml::from_str(
            "[events]\nenabled = \"yes\"\ndisable = \"halloween\"\n[location]\nlatitude = 1.0\nlongitude = 2.0\n",
        )
        .unwrap();
        assert_eq!(config.location.latitude, 1.0);
        assert!(config.events.enabled);
        assert_eq!(config.events.disable, vec!["halloween"]);
        assert_eq!(
            config.events.warnings.len(),
            1,
            "{:?}",
            config.events.warnings
        );
        assert!(config.events.warnings[0].contains("events.enabled"));
        let config: Config =
            toml::from_str("events = 1\n[location]\nlatitude = 1.0\nlongitude = 2.0\n").unwrap();
        assert_eq!(config.location.latitude, 1.0);
        assert!(config.events.enabled);
        assert_eq!(
            config.events.warnings.len(),
            1,
            "{:?}",
            config.events.warnings
        );
    }

    #[test]
    fn test_sky_table_reads_flyers_every_and_banner() {
        let config: Config = toml::from_str("").unwrap();
        assert_eq!(config.sky, SkyConfig::default());
        let config: Config = toml::from_str(
            "[sky]\nflyers = [\"plane\", \"heli\"]\nevery = 90\nbanner = \"Jakarta\"\n",
        )
        .unwrap();
        assert_eq!(config.sky.flyers, vec!["plane", "heli"]);
        assert_eq!(config.sky.every.as_deref(), Some("90"));
        assert_eq!(config.sky.banner.as_deref(), Some("Jakarta"));
        assert!(config.sky.warnings.is_empty());
    }

    #[test]
    fn test_wrong_typed_sky_keeps_the_rest_of_config() {
        let config: Config =
            toml::from_str("sky = 1\n[location]\nlatitude = 1.0\nlongitude = 2.0\n").unwrap();
        assert_eq!(config.location.latitude, 1.0);
        assert_eq!(config.sky.warnings.len(), 1, "{:?}", config.sky.warnings);
        let config: Config =
            toml::from_str("[sky]\nflyers = \"plane\"\nevery = \"busy\"\n").unwrap();
        assert_eq!(config.sky.flyers, vec!["plane"]);
        assert_eq!(config.sky.every.as_deref(), Some("busy"));
    }

    #[test]
    fn test_frame_rates_default_and_clamp() {
        let config: Config =
            toml::from_str("[location]\nlatitude = 1.0\nlongitude = 2.0\n").unwrap();
        assert_eq!(config.frame_rates(), ((30, 5), Vec::<String>::new()));
        let config: Config =
            toml::from_str("fps = 15\nidle_fps = 0\n[location]\nlatitude = 1.0\nlongitude = 2.0\n")
                .unwrap();
        assert_eq!(config.frame_rates(), ((15, 0), Vec::<String>::new()));
        let config: Config = toml::from_str(
            "fps = 120\nidle_fps = 2\n[location]\nlatitude = 1.0\nlongitude = 2.0\n",
        )
        .unwrap();
        let ((fps, idle), warnings) = config.frame_rates();
        assert_eq!((fps, idle), (30, 5));
        assert_eq!(warnings.len(), 2, "{warnings:?}");
    }

    #[test]
    fn test_trees_default_to_auto_and_read_a_climate() {
        use crate::scene::world::trees::{Climate, TreesSetting};
        let config: Config =
            toml::from_str("[location]\nlatitude = 1.0\nlongitude = 2.0\n").unwrap();
        assert_eq!(config.trees_setting(), Ok(TreesSetting::Auto));
        let config: Config =
            toml::from_str("trees = \"dry\"\n[location]\nlatitude = 1.0\nlongitude = 2.0\n")
                .unwrap();
        assert_eq!(
            config.trees_setting(),
            Ok(TreesSetting::Fixed(Climate::Dry))
        );
    }

    #[test]
    fn test_sky_table_reads_characters_and_specials() {
        let config: Config =
            toml::from_str("[sky]\ncharacters = [\"ufo\", \"moon-biker\"]\nspecials = 6\n")
                .unwrap();
        assert_eq!(config.sky.characters, vec!["ufo", "moon-biker"]);
        assert_eq!(config.sky.specials.as_deref(), Some("6"));
        let config: Config = toml::from_str("[sky]\ncharacters = \"ufo\"\n").unwrap();
        assert_eq!(config.sky.characters, vec!["ufo"]);
        assert_eq!(config.sky.specials, None);
    }
}
