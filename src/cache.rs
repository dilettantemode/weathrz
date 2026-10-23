use crate::weather::WeatherData;
use crate::{config::Provider, geolocation::GeoLocation};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::fs;
use tokio::sync::mpsc;

const LOCATION_CACHE_DURATION_SECS: u64 = 86400;
const WEATHER_CACHE_DURATION_SECS: u64 = 300;
const LAST_READING_MAX_AGE_SECS: u64 = 48 * 3600;
const CACHE_WRITER_BUFFER_SIZE: usize = 32;

#[derive(Debug)]
enum CacheWriteTask {
    Location(GeoLocation),
    Weather(Box<WeatherData>, f64, f64, Provider),
    Geocode(String, f64, f64, String),
}

static CACHE_WRITER: OnceLock<mpsc::Sender<CacheWriteTask>> = OnceLock::new();

fn get_cache_writer() -> &'static mpsc::Sender<CacheWriteTask> {
    CACHE_WRITER.get_or_init(|| {
        let (tx, mut rx) = mpsc::channel::<CacheWriteTask>(CACHE_WRITER_BUFFER_SIZE);

        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    CacheWriteTask::Location(location) => {
                        let _ = write_location_cache(&location).await;
                    }
                    CacheWriteTask::Weather(data, lat, lon, provider) => {
                        let _ = write_weather_cache(&data, lat, lon, provider).await;
                    }
                    CacheWriteTask::Geocode(city, lat, lon, lang) => {
                        let _ = write_geocode_cache(&city, lat, lon, &lang).await;
                    }
                }
            }
        });

        tx
    })
}

#[derive(Serialize, Deserialize)]
struct LocationCache {
    location: GeoLocation,
    cached_at: u64,
}

#[derive(Serialize, Deserialize)]
struct WeatherCache {
    data: WeatherData,
    cached_at: u64,
    location_key: String,
    provider: Provider,
}

fn get_cache_dir() -> Option<PathBuf> {
    Some(dirs::cache_dir()?.join("weathrz"))
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn make_location_key(latitude: f64, longitude: f64) -> String {
    format!("{:.2},{:.2}", latitude, longitude)
}

pub async fn load_cached_location() -> Option<GeoLocation> {
    let cache_path = get_cache_dir()?.join("location.json");
    let contents = fs::read_to_string(&cache_path).await.ok()?;
    let cache: LocationCache = serde_json::from_str(&contents).ok()?;

    let now = current_timestamp();
    if now.saturating_sub(cache.cached_at) < LOCATION_CACHE_DURATION_SECS {
        Some(cache.location)
    } else {
        None
    }
}

async fn write_location_cache(location: &GeoLocation) -> Result<(), std::io::Error> {
    if let Some(cache_dir) = get_cache_dir() {
        fs::create_dir_all(&cache_dir).await?;

        let cache = LocationCache {
            location: location.clone(),
            cached_at: current_timestamp(),
        };

        let json = serde_json::to_string(&cache)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Atomic write: write to temp file, then rename
        let temp_path = cache_dir.join(format!("location.json.{}.tmp", std::process::id()));
        let final_path = cache_dir.join("location.json");
        fs::write(&temp_path, json).await?;
        fs::rename(temp_path, final_path).await?;
    }
    Ok(())
}

pub fn save_location_cache(location: &GeoLocation) {
    let location = location.clone();
    let writer = get_cache_writer();
    let _ = writer.try_send(CacheWriteTask::Location(location));
}

#[derive(Serialize, Deserialize)]
struct GeocodeCache {
    city_name: String,
    cached_at: u64,
    location_key: String,
    language: String,
}

pub async fn load_cached_geocode(latitude: f64, longitude: f64, language: &str) -> Option<String> {
    let location_key = make_location_key(latitude, longitude);
    let filename = format!("geocode_{}_{}.json", location_key, language);
    let cache_path = get_cache_dir()?.join(filename);
    let contents = fs::read_to_string(&cache_path).await.ok()?;
    let cache: GeocodeCache = serde_json::from_str(&contents).ok()?;

    let now = current_timestamp();
    if now.saturating_sub(cache.cached_at) < LOCATION_CACHE_DURATION_SECS {
        Some(cache.city_name)
    } else {
        None
    }
}

async fn write_geocode_cache(
    city_name: &str,
    latitude: f64,
    longitude: f64,
    language: &str,
) -> Result<(), std::io::Error> {
    if let Some(cache_dir) = get_cache_dir() {
        fs::create_dir_all(&cache_dir).await?;

        let location_key = make_location_key(latitude, longitude);
        let cache = GeocodeCache {
            city_name: city_name.to_string(),
            cached_at: current_timestamp(),
            location_key: location_key.clone(),
            language: language.to_string(),
        };

        let json = serde_json::to_string(&cache)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Atomic write with location-specific filename to prevent cache thrashing
        let filename = format!("geocode_{}_{}.json", location_key, language);
        let temp_path = cache_dir.join(format!("{}.{}.tmp", filename, std::process::id()));
        let final_path = cache_dir.join(filename);
        fs::write(&temp_path, json).await?;
        fs::rename(temp_path, final_path).await?;
    }
    Ok(())
}

pub fn save_geocode_cache(city_name: &str, latitude: f64, longitude: f64, language: &str) {
    let city_name = city_name.to_string();
    let language = language.to_string();
    let writer = get_cache_writer();
    let _ = writer.try_send(CacheWriteTask::Geocode(
        city_name, latitude, longitude, language,
    ));
}

fn weather_filename(latitude: f64, longitude: f64, provider: Provider) -> String {
    format!(
        "weather_{}_{:?}.json",
        make_location_key(latitude, longitude),
        provider
    )
}

pub fn last_weather(
    latitude: f64,
    longitude: f64,
    provider: Provider,
) -> Option<(WeatherData, u64)> {
    let path = get_cache_dir()?.join(weather_filename(latitude, longitude, provider));
    read_last_weather(&path, current_timestamp())
}

fn read_last_weather(path: &std::path::Path, now: u64) -> Option<(WeatherData, u64)> {
    let contents = std::fs::read_to_string(path).ok()?;
    let cache: WeatherCache = serde_json::from_str(&contents).ok()?;
    let age = now.saturating_sub(cache.cached_at);
    (age <= LAST_READING_MAX_AGE_SECS).then_some((cache.data, age))
}

pub async fn load_cached_weather(
    latitude: f64,
    longitude: f64,
    provider: Provider,
) -> Option<WeatherData> {
    let cache_path = get_cache_dir()?.join(weather_filename(latitude, longitude, provider));
    let contents = fs::read_to_string(&cache_path).await.ok()?;
    let cache: WeatherCache = serde_json::from_str(&contents).ok()?;

    let now = current_timestamp();
    if now.saturating_sub(cache.cached_at) < WEATHER_CACHE_DURATION_SECS {
        Some(cache.data)
    } else {
        None
    }
}

async fn write_weather_cache(
    weather: &WeatherData,
    latitude: f64,
    longitude: f64,
    provider: Provider,
) -> Result<(), std::io::Error> {
    if let Some(cache_dir) = get_cache_dir() {
        fs::create_dir_all(&cache_dir).await?;

        let location_key = make_location_key(latitude, longitude);
        let cache = WeatherCache {
            data: weather.clone(),
            cached_at: current_timestamp(),
            location_key: location_key.clone(),
            provider,
        };

        let json = serde_json::to_string(&cache)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Atomic write with location-specific filename to prevent cache thrashing
        let filename = format!("weather_{}_{:?}.json", location_key, provider);
        let temp_path = cache_dir.join(format!("{}.{}.tmp", filename, std::process::id()));
        let final_path = cache_dir.join(filename);
        fs::write(&temp_path, json).await?;
        fs::rename(temp_path, final_path).await?;
    }
    Ok(())
}

pub async fn write_weather_now(
    weather: &WeatherData,
    latitude: f64,
    longitude: f64,
    provider: Provider,
) {
    let _ = write_weather_cache(weather, latitude, longitude, provider).await;
}

pub fn save_weather_cache(
    weather: &WeatherData,
    latitude: f64,
    longitude: f64,
    provider: Provider,
) {
    let weather = Box::new(weather.clone());
    let writer = get_cache_writer();
    let _ = writer.try_send(CacheWriteTask::Weather(
        weather, latitude, longitude, provider,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weather::WeatherCondition;
    use crate::weather::types::CelestialEvents;

    fn reading() -> WeatherData {
        WeatherData {
            condition: WeatherCondition::Cloudy,
            temperature: 29.0,
            precipitation: 0.0,
            wind_speed: 3.0,
            wind_direction: 90.0,
            sun: CelestialEvents::from_bool(true),
            moon_phase: Some(0.4),
            humidity: Some(70.0),
            timestamp: "2026-10-20T10:00".to_string(),
            attribution: "Open-Meteo".to_string(),
            feels_like: None,
            uv_index: None,
            outlook: Vec::new(),
        }
    }

    #[test]
    fn the_last_reading_is_kept_for_two_days_with_its_age() {
        let dir = std::env::temp_dir().join(format!("weathrz-last-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("weather.json");
        let cache = WeatherCache {
            data: reading(),
            cached_at: 1_000_000,
            location_key: make_location_key(-6.2, 106.8),
            provider: Provider::OpenMeteo,
        };
        std::fs::write(&path, serde_json::to_string(&cache).unwrap()).unwrap();
        let (data, age) = read_last_weather(&path, 1_000_000 + 7_200).expect("two hours old");
        assert_eq!((data.temperature, age), (29.0, 7_200));
        assert!(read_last_weather(&path, 1_000_000 + 49 * 3_600).is_none());
        assert!(read_last_weather(&dir.join("missing.json"), 1_000_000).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
