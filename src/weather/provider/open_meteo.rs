use crate::astro;
use crate::error::{NetworkError, WeatherError};
use crate::weather::provider::{WeatherProvider, WeatherProviderResponse};
use crate::weather::types::{
    CelestialEvents, HourPoint, PrecipitationUnit, TemperatureUnit, WeatherLocation, WeatherUnits,
    WindSpeedUnit,
};
use crate::weather::units::{normalize_precipitation, normalize_temperature, normalize_wind_speed};
use async_trait::async_trait;
use serde::Deserialize;
use serde::de::{self, Deserializer};
use std::time::Duration;

const OPEN_METEO_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";
const OUTLOOK_HOURS: u32 = 13;

pub struct OpenMeteoProvider {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    current: CurrentWeather,
    #[serde(default)]
    hourly: Option<Hourly>,
}

#[derive(Debug, Deserialize)]
struct Hourly {
    time: Vec<String>,
    #[serde(default)]
    precipitation_probability: Vec<Option<f64>>,
    #[serde(default)]
    weather_code: Vec<Option<f64>>,
    #[serde(default)]
    precipitation: Vec<Option<f64>>,
}

impl Hourly {
    fn points(self, unit: PrecipitationUnit) -> Vec<HourPoint> {
        self.time
            .into_iter()
            .enumerate()
            .map(|(index, time)| HourPoint {
                time,
                probability: self
                    .precipitation_probability
                    .get(index)
                    .copied()
                    .flatten()
                    .map(|value| value.clamp(0.0, 100.0).round() as u8),
                code: self
                    .weather_code
                    .get(index)
                    .copied()
                    .flatten()
                    .map_or(0, |value| value.round() as i32),
                precipitation: normalize_precipitation(
                    self.precipitation
                        .get(index)
                        .copied()
                        .flatten()
                        .unwrap_or(0.0),
                    unit,
                ),
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    time: String,
    temperature_2m: f64,
    #[serde(deserialize_with = "deserialize_i32_from_number")]
    is_day: i32,
    precipitation: f64,
    #[serde(deserialize_with = "deserialize_i32_from_number")]
    weather_code: i32,
    wind_speed_10m: f64,
    wind_direction_10m: f64,
    #[serde(default)]
    relative_humidity_2m: Option<f64>,
    #[serde(default)]
    apparent_temperature: Option<f64>,
    #[serde(default)]
    uv_index: Option<f64>,
}

fn deserialize_i32_from_number<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Number {
        Integer(i32),
        Float(f64),
    }

    match Number::deserialize(deserializer)? {
        Number::Integer(value) => Ok(value),
        Number::Float(value) => {
            if !value.is_finite() {
                return Err(de::Error::custom("expected a finite numeric value"));
            }
            Ok(value.round() as i32)
        }
    }
}

impl OpenMeteoProvider {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|e| {
                eprintln!("Warning: Failed to create custom HTTP client: {}", e);
                eprintln!("Using default client with standard timeout settings.");
                reqwest::Client::new()
            });

        Self {
            client,
            base_url: OPEN_METEO_BASE_URL.to_string(),
        }
    }

    fn temperature_unit_param(unit: &TemperatureUnit) -> &'static str {
        match unit {
            TemperatureUnit::Celsius => "celsius",
            TemperatureUnit::Fahrenheit => "fahrenheit",
        }
    }

    fn wind_speed_unit_param(unit: &WindSpeedUnit) -> &'static str {
        match unit {
            WindSpeedUnit::Kmh => "kmh",
            WindSpeedUnit::Ms => "ms",
            WindSpeedUnit::Mph => "mph",
            WindSpeedUnit::Kn => "kn",
        }
    }

    fn precipitation_unit_param(unit: &PrecipitationUnit) -> &'static str {
        match unit {
            PrecipitationUnit::Mm => "mm",
            PrecipitationUnit::Inch => "inch",
        }
    }

    fn build_url(&self, location: &WeatherLocation, units: &WeatherUnits) -> String {
        format!(
            "{}?latitude={}&longitude={}&current=temperature_2m,is_day,precipitation,weather_code,wind_speed_10m,wind_direction_10m,relative_humidity_2m,apparent_temperature,uv_index&hourly=precipitation_probability,weather_code,precipitation&forecast_hours={}&temperature_unit={}&wind_speed_unit={}&precipitation_unit={}&timezone=auto",
            self.base_url,
            location.latitude,
            location.longitude,
            OUTLOOK_HOURS,
            Self::temperature_unit_param(&units.temperature),
            Self::wind_speed_unit_param(&units.wind_speed),
            Self::precipitation_unit_param(&units.precipitation)
        )
    }
}

impl Default for OpenMeteoProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WeatherProvider for OpenMeteoProvider {
    fn get_attribution(&self) -> &'static str {
        ""
    }

    async fn get_current_weather(
        &self,
        location: &WeatherLocation,
        units: &WeatherUnits,
    ) -> Result<WeatherProviderResponse, WeatherError> {
        let url = self.build_url(location, units);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .and_then(|resp| resp.error_for_status())
            .map_err(|e| WeatherError::Network(NetworkError::from_reqwest(e, &url, 30)))?;

        let data: OpenMeteoResponse = response
            .json()
            .await
            .map_err(|e| WeatherError::Network(NetworkError::from_reqwest(e, &url, 30)))?;

        let moon_phase = Some(astro::moon_phase(chrono::Utc::now()));

        Ok(WeatherProviderResponse {
            weather_code: data.current.weather_code,
            temperature: normalize_temperature(data.current.temperature_2m, units.temperature),
            precipitation: normalize_precipitation(data.current.precipitation, units.precipitation),
            wind_speed: normalize_wind_speed(data.current.wind_speed_10m, units.wind_speed),
            wind_direction: data.current.wind_direction_10m,
            sun: CelestialEvents::only_day(data.current.is_day),
            moon_phase,
            humidity: data.current.relative_humidity_2m,
            timestamp: data.current.time,
            attribution: self.get_attribution().to_string(),
            feels_like: data
                .current
                .apparent_temperature
                .map(|value| normalize_temperature(value, units.temperature)),
            uv_index: data.current.uv_index,
            outlook: data
                .hourly
                .map(|hourly| hourly.points(units.precipitation))
                .unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_conversion_params() {
        assert_eq!(
            OpenMeteoProvider::temperature_unit_param(&TemperatureUnit::Celsius),
            "celsius"
        );
        assert_eq!(
            OpenMeteoProvider::temperature_unit_param(&TemperatureUnit::Fahrenheit),
            "fahrenheit"
        );
        assert_eq!(
            OpenMeteoProvider::wind_speed_unit_param(&WindSpeedUnit::Kmh),
            "kmh"
        );
        assert_eq!(
            OpenMeteoProvider::wind_speed_unit_param(&WindSpeedUnit::Ms),
            "ms"
        );
        assert_eq!(
            OpenMeteoProvider::precipitation_unit_param(&PrecipitationUnit::Mm),
            "mm"
        );
    }

    #[test]
    fn current_weather_reads_humidity_when_present() {
        let with: OpenMeteoResponse = serde_json::from_str(
            r#"{"current":{"time":"t","temperature_2m":30.1,"is_day":1,"precipitation":0.0,"weather_code":1,"wind_speed_10m":2.0,"wind_direction_10m":90.0,"relative_humidity_2m":58}}"#,
        )
        .unwrap();
        assert_eq!(with.current.relative_humidity_2m, Some(58.0));
        let without: OpenMeteoResponse = serde_json::from_str(
            r#"{"current":{"time":"t","temperature_2m":30.1,"is_day":1,"precipitation":0.0,"weather_code":1,"wind_speed_10m":2.0,"wind_direction_10m":90.0}}"#,
        )
        .unwrap();
        assert_eq!(without.current.relative_humidity_2m, None);
    }

    #[test]
    fn the_request_asks_for_humidity() {
        let provider = OpenMeteoProvider::new();
        let location = WeatherLocation {
            latitude: -6.2,
            longitude: 106.8,
            elevation: None,
        };
        let url = provider.build_url(&location, &WeatherUnits::metric());
        assert!(url.contains("relative_humidity_2m"), "{url}");
        assert!(url.contains("apparent_temperature,uv_index"), "{url}");
        assert!(
            url.contains(
                "hourly=precipitation_probability,weather_code,precipitation&forecast_hours=13"
            ),
            "{url}"
        );
    }

    #[test]
    fn hourly_points_carry_probability_code_and_amount() {
        let data: OpenMeteoResponse = serde_json::from_str(
            r#"{"current":{"time":"2026-10-21T11:45","temperature_2m":31.0,"is_day":1,"precipitation":0.0,"weather_code":2,"wind_speed_10m":9.0,"wind_direction_10m":200.0,"apparent_temperature":36.4,"uv_index":7.55},
                "hourly":{"time":["2026-10-21T11:00","2026-10-21T12:00"],"precipitation_probability":[10,null],"weather_code":[2,95],"precipitation":[0.0,1.4]}}"#,
        )
        .unwrap();
        assert_eq!(data.current.apparent_temperature, Some(36.4));
        assert_eq!(data.current.uv_index, Some(7.55));
        let inches: OpenMeteoResponse = serde_json::from_str(
            r#"{"current":{"time":"2026-10-21T11:45","temperature_2m":88.0,"is_day":1,"precipitation":0.0,"weather_code":2,"wind_speed_10m":5.0,"wind_direction_10m":200.0},
                "hourly":{"time":["2026-10-21T12:00"],"precipitation_probability":[80],"weather_code":[63],"precipitation":[0.12]}}"#,
        )
        .unwrap();
        let wet = inches.hourly.unwrap().points(PrecipitationUnit::Inch);
        assert!((wet[0].precipitation - 3.048).abs() < 1e-9, "{wet:?}");
        let points = data.hourly.unwrap().points(PrecipitationUnit::Mm);
        assert_eq!(
            points,
            vec![
                HourPoint {
                    time: "2026-10-21T11:00".to_string(),
                    probability: Some(10),
                    code: 2,
                    precipitation: 0.0
                },
                HourPoint {
                    time: "2026-10-21T12:00".to_string(),
                    probability: None,
                    code: 95,
                    precipitation: 1.4
                },
            ]
        );
    }
}
