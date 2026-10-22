use crate::app::provider_for;
use crate::app_state::condition_label;
use crate::cache;
use crate::config::{Config, IconMode};
use crate::hud::{Greeting, HudRole, HudSegment, ascii_segments, greeting_segments, nerd_segments};
use crate::weather::{WeatherClient, WeatherData, WeatherLocation};
use std::time::Duration;

const FRESH_SECS: u64 = 600;
const FETCH_TIMEOUT: Duration = Duration::from_secs(8);
const STALE_OUTLOOK_SECS: u64 = 3600;

pub fn compose(
    weather: &WeatherData,
    config: &Config,
    greeting: Option<&Greeting>,
    age_secs: Option<u64>,
) -> String {
    let location = (!config.location.hide).then(|| {
        config.location.city.clone().unwrap_or_else(|| {
            format!(
                "{:.2}, {:.2}",
                config.location.latitude, config.location.longitude
            )
        })
    });
    let label = condition_label(weather.condition);
    let mode = config.icon_mode().unwrap_or_default();
    let body = match mode {
        IconMode::Nerd => nerd_segments(
            weather,
            label,
            &config.units,
            false,
            location.as_deref(),
            None,
        ),
        IconMode::Ascii => ascii_segments(
            weather,
            label,
            &config.units,
            false,
            location.as_deref(),
            None,
        ),
    };
    let mut parts: Vec<HudSegment> = greeting_segments(greeting, IconMode::Nerd)
        .into_iter()
        .filter(|_| mode == IconMode::Nerd)
        .chain(body)
        .filter(|segment| !matches!(segment.role, HudRole::Separator | HudRole::Hint))
        .collect();
    if mode == IconMode::Ascii
        && let Some(greeting) = greeting
    {
        parts.insert(
            0,
            crate::hud::segment(HudRole::Greeting, greeting.text.clone()),
        );
    }
    let joiner = match mode {
        IconMode::Nerd => "  ",
        IconMode::Ascii => " | ",
    };
    let mut line = parts
        .iter()
        .map(|segment| segment.text.trim())
        .collect::<Vec<_>>()
        .join(joiner);
    if let Some(age) = age_secs.filter(|age| *age >= FRESH_SECS) {
        line.push_str(&format!("{joiner}{} ago", age_words(age)));
    }
    line
}

fn without_old_outlook(data: WeatherData, age_secs: u64) -> WeatherData {
    if age_secs < STALE_OUTLOOK_SECS {
        return data;
    }
    WeatherData {
        outlook: Vec::new(),
        ..data
    }
}

fn age_words(seconds: u64) -> String {
    if seconds < 3_600 {
        format!("{} min", seconds / 60)
    } else {
        format!("{} h", seconds / 3_600)
    }
}

pub async fn fetch(config: &Config) -> Result<(WeatherData, Option<u64>), String> {
    let (kind, provider) = provider_for(config);
    let location = WeatherLocation {
        latitude: config.location.latitude,
        longitude: config.location.longitude,
        elevation: None,
    };
    let last = cache::last_weather(location.latitude, location.longitude, kind);
    if let Some((data, age)) = &last
        && *age < FRESH_SECS
        && !crate::weather::client::predates_the_outlook(data, kind)
    {
        return Ok((data.clone(), None));
    }
    let client = WeatherClient::new(provider, Duration::from_secs(FRESH_SECS));
    match tokio::time::timeout(
        FETCH_TIMEOUT,
        client.fetch_fresh(&location, &config.units, kind),
    )
    .await
    {
        Ok(Ok(data)) => {
            cache::write_weather_now(&data, location.latitude, location.longitude, kind).await;
            Ok((data, None))
        }
        _ => last
            .map(|(data, age)| (without_old_outlook(data, age), Some(age)))
            .ok_or_else(|| "weathrz: offline, and no saved reading from the last 48 h".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weather::WeatherCondition;
    use crate::weather::types::CelestialEvents;

    fn reading() -> WeatherData {
        WeatherData {
            condition: WeatherCondition::Clear,
            temperature: 31.0,
            precipitation: 0.0,
            wind_speed: 2.0,
            wind_direction: 0.0,
            sun: CelestialEvents::from_bool(true),
            moon_phase: None,
            humidity: Some(60.0),
            timestamp: "2026-10-21T11:40".to_string(),
            attribution: String::new(),
            feels_like: Some(36.0),
            uv_index: None,
            outlook: Vec::new(),
        }
    }

    fn config(icons: &str) -> Config {
        let mut config: Config = toml::from_str(&format!(
            "icons = \"{icons}\"\n[location]\nlatitude = -6.2\nlongitude = 106.8\nauto = false\ncity = \"Jakarta\"\n"
        ))
        .unwrap();
        config.location.city = Some("Jakarta".to_string());
        config
    }

    #[test]
    fn the_line_joins_the_bar_without_separators_or_hints() {
        let nerd = compose(&reading(), &config("nerd"), None, None);
        assert!(
            nerd.starts_with("\u{e30d} Clear  \u{e350} 31.0°C (feels 36°)"),
            "{nerd}"
        );
        assert!(nerd.ends_with("\u{f041} Jakarta"), "{nerd}");
        let ascii = compose(&reading(), &config("ascii"), None, None);
        assert!(
            ascii.starts_with("Weather: Clear | Temp: 31.0°C (feels 36°)"),
            "{ascii}"
        );
        assert!(!ascii.contains("quit"), "{ascii}");
    }

    #[test]
    fn a_greeting_leads_and_an_old_reading_says_its_age() {
        let greeting = Greeting {
            icon: '\u{f023b}',
            text: "Happy 81st Independence Day, Indonesia".to_string(),
            short: None,
        };
        let line = compose(&reading(), &config("nerd"), Some(&greeting), Some(7_300));
        assert!(line.starts_with("\u{f023b} Happy 81st"), "{line}");
        assert!(line.ends_with("  2 h ago"), "{line}");
        let fresh = compose(&reading(), &config("ascii"), Some(&greeting), Some(30));
        assert!(
            fresh.starts_with("Happy 81st Independence Day, Indonesia | Weather"),
            "{fresh}"
        );
    }
}
