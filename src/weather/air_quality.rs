use crate::error::{NetworkError, WeatherError};
use crate::weather::WeatherLocation;
use std::time::{Duration, SystemTime};

const STALE_AFTER: Duration = Duration::from_secs(3 * 60 * 60);
const MAX_AQI: f64 = 500.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AirQuality {
    pub us_aqi: u16,
    pub fetched_at: Option<SystemTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AqiBand {
    Good,
    Moderate,
    Sensitive,
    Unhealthy,
    VeryUnhealthy,
    Hazardous,
}

pub fn url(location: &WeatherLocation) -> String {
    format!(
        "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&current=us_aqi&timezone=auto",
        location.latitude, location.longitude
    )
}

impl AqiBand {
    pub fn from_aqi(aqi: u16) -> Self {
        match aqi {
            0..=50 => Self::Good,
            51..=100 => Self::Moderate,
            101..=150 => Self::Sensitive,
            151..=200 => Self::Unhealthy,
            201..=300 => Self::VeryUnhealthy,
            _ => Self::Hazardous,
        }
    }

    pub fn label(self) -> &'static str {
        crate::text::aqi_band(self)
    }

    pub fn tier(self) -> usize {
        match self {
            Self::Good => 0,
            Self::Moderate => 1,
            Self::Sensitive => 2,
            Self::Unhealthy => 3,
            Self::VeryUnhealthy => 4,
            Self::Hazardous => 5,
        }
    }
}

impl AirQuality {
    pub fn is_stale(&self, now: SystemTime) -> bool {
        self.fetched_at.is_some_and(|fetched| {
            now.duration_since(fetched)
                .is_ok_and(|age| age > STALE_AFTER)
        })
    }
}

pub fn parse(json: &str, fetched_at: SystemTime) -> Result<AirQuality, WeatherError> {
    let no_data = || WeatherError::Data(crate::error::DataError::NoData);
    let value: serde_json::Value = serde_json::from_str(json).map_err(|_| no_data())?;
    let aqi = value
        .get("current")
        .and_then(|current| current.get("us_aqi"))
        .and_then(serde_json::Value::as_f64)
        .filter(|aqi| aqi.is_finite() && *aqi >= 0.0)
        .ok_or_else(no_data)?;
    Ok(AirQuality {
        us_aqi: aqi.round().min(MAX_AQI) as u16,
        fetched_at: Some(fetched_at),
    })
}

pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()
        .unwrap_or_default()
}

pub async fn fetch(
    client: &reqwest::Client,
    location: &WeatherLocation,
) -> Result<AirQuality, WeatherError> {
    let url = url(location);
    let body = client
        .get(&url)
        .send()
        .await
        .and_then(|response| response.error_for_status())
        .map_err(|e| WeatherError::Network(NetworkError::from_reqwest(e, &url, 30)))?
        .text()
        .await
        .map_err(|e| WeatherError::Network(NetworkError::from_reqwest(e, &url, 30)))?;
    parse(&body, SystemTime::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bands_follow_the_us_scale() {
        for (aqi, band) in [
            (0, AqiBand::Good),
            (50, AqiBand::Good),
            (51, AqiBand::Moderate),
            (100, AqiBand::Moderate),
            (101, AqiBand::Sensitive),
            (150, AqiBand::Sensitive),
            (151, AqiBand::Unhealthy),
            (200, AqiBand::Unhealthy),
            (201, AqiBand::VeryUnhealthy),
            (300, AqiBand::VeryUnhealthy),
            (301, AqiBand::Hazardous),
            (500, AqiBand::Hazardous),
        ] {
            assert_eq!(AqiBand::from_aqi(aqi), band, "{aqi}");
        }
        assert_eq!(AqiBand::Sensitive.label(), "sensitive groups");
        assert_eq!(AqiBand::VeryUnhealthy.label(), "very unhealthy");
        assert_eq!(AqiBand::Good.tier(), 0);
        assert_eq!(AqiBand::Hazardous.tier(), 5);
    }

    #[test]
    fn parse_reads_the_live_reply_shape() {
        let now = SystemTime::now();
        let reading = parse(
            r#"{"latitude":-6.2,"longitude":106.8,"current_units":{"time":"iso8601","us_aqi":"USAQI"},"current":{"time":"2026-10-14T16:00","interval":3600,"us_aqi":212}}"#,
            now,
        )
        .unwrap();
        assert_eq!(
            reading,
            AirQuality {
                us_aqi: 212,
                fetched_at: Some(now)
            }
        );
    }

    #[test]
    fn parse_rejects_or_caps_odd_replies() {
        let now = SystemTime::now();
        for bad in [
            r#"{"current":{"time":"t"}}"#,
            r#"{"current":{"time":"t","us_aqi":null}}"#,
            r#"{"current":{"time":"t","us_aqi":-3}}"#,
            r#"{"error":true,"reason":"rate limited"}"#,
            "not json",
        ] {
            assert!(parse(bad, now).is_err(), "{bad}");
        }
        assert_eq!(
            parse(r#"{"current":{"us_aqi":612}}"#, now).unwrap().us_aqi,
            500
        );
        assert_eq!(
            parse(r#"{"current":{"us_aqi":41.6}}"#, now).unwrap().us_aqi,
            42
        );
    }

    #[test]
    fn readings_go_stale_after_three_hours_but_previews_never() {
        let fetched = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000);
        let reading = AirQuality {
            us_aqi: 80,
            fetched_at: Some(fetched),
        };
        assert!(!reading.is_stale(fetched + Duration::from_secs(2 * 3600 + 59 * 60)));
        assert!(reading.is_stale(fetched + Duration::from_secs(3 * 3600 + 60)));
        let preview = AirQuality {
            us_aqi: 80,
            fetched_at: None,
        };
        assert!(!preview.is_stale(fetched + Duration::from_secs(999_999)));
    }

    #[test]
    fn the_url_asks_for_us_aqi_at_the_location() {
        let url = url(&WeatherLocation {
            latitude: -6.2088,
            longitude: 106.8456,
            elevation: None,
        });
        assert_eq!(
            url,
            "https://air-quality-api.open-meteo.com/v1/air-quality?latitude=-6.2088&longitude=106.8456&current=us_aqi&timezone=auto"
        );
    }
}
