use crate::weather::WeatherCondition;
use crate::weather::air_quality::AqiBand;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    #[default]
    En,
}

pub fn parse_language(text: &str) -> Result<Language, String> {
    match text.trim().to_ascii_lowercase().as_str() {
        "" | "en" => Ok(Language::En),
        other => Err(format!(
            "language = \"{other}\" is not available yet; using \"en\""
        )),
    }
}

pub const WEATHER: &str = "Weather";
pub const TEMP: &str = "Temp";
pub const WIND: &str = "Wind";
pub const PRECIP: &str = "Precip";
pub const HUMIDITY: &str = "Humidity";
pub const AQI: &str = "AQI";
pub const LOCATION: &str = "Location";
pub const UV: &str = "UV";
pub const OFFLINE: &str = "OFFLINE";
pub const LOADING: &str = "Loading";
pub const OFFLINE_RETRYING: &str = "Offline · retrying";
pub const QUIT_HINT: &str = "Press 'q' to quit, '?' for help";
pub const FEELS: &str = "feels";
pub const RAIN: &str = "rain";
pub const STORMS: &str = "storms";
pub const SOON: &str = "soon";
pub const UNTIL: &str = "until";
pub const EASING: &str = "easing";
pub const AGO: &str = "ago";
pub const PANE_TOO_SMALL: &str = "pane too small";
pub const OFFLINE_LAST_READING: &str = "Offline · showing the last reading";
pub const OFFLINE_READING_FROM: &str = "Offline · reading from";

pub const HELP_TITLE: &str = "weathrz";
pub const HELP_KEYS: &[(&str, &str)] = &[
    ("q, Ctrl-C", "quit"),
    ("r", "fetch the weather now"),
    ("i", "details: sun, moon, air, next hours"),
    ("?, Esc", "close this help"),
];

pub const DETAILS_TITLE: &str = "details";
pub const DETAILS_NOW: &str = "Now";
pub const DETAILS_AIR: &str = "Air";
pub const DETAILS_SUN: &str = "Sun";
pub const DETAILS_MOON: &str = "Moon";
pub const DETAILS_NEXT_HOURS: &str = "Next hours";
pub const FROM_THE: &str = "from the";
pub const RISE: &str = "rise";
pub const SET: &str = "set";
pub const NOW: &str = "now";
pub const OF_DAYLIGHT: &str = "of daylight";
pub const UP_IN_THE: &str = "up in the";
pub const BELOW_THE_HORIZON: &str = "below the horizon";
pub const LIT: &str = "lit";
pub const UV_LOW: &str = "low";
pub const UV_MODERATE: &str = "moderate";
pub const UV_HIGH: &str = "high";
pub const UV_VERY_HIGH: &str = "very high";
pub const UV_EXTREME: &str = "extreme";
pub const MOON_PHASES: [&str; 8] = [
    "new moon",
    "waxing crescent",
    "first quarter",
    "waxing gibbous",
    "full moon",
    "waning gibbous",
    "last quarter",
    "waning crescent",
];
pub const HELP_LOCATION: &str = "Location";
pub const HELP_FROM: &str = "From";
pub const HELP_HOLIDAYS: &str = "Holidays";
pub const HELP_CONFIG: &str = "Config";
pub const HELP_OWN_DAYS: &str = "Own days";
pub const HELP_DATA: &str = "Data";
pub const HELP_VERSION: &str = "Version";
pub const HELP_YEARLY_ONLY: &str = "the yearly ones only";

pub const TAKBIRAN_NIGHT: &str = "Takbiran Night";
pub const CHINESE_NEW_YEARS_EVE: &str = "Chinese New Year's Eve";
pub const CAP_GO_MEH: &str = "Happy Cap Go Meh";
pub const MERRY_CHRISTMAS: &str = "Merry Christmas";
pub const HAPPY_NEW_YEAR: &str = "Happy New Year {year}";
pub const NEW_YEAR_COUNTDOWN: &str = "{year} in {time}";

pub fn condition(condition: WeatherCondition) -> &'static str {
    match condition {
        WeatherCondition::Clear => "Clear",
        WeatherCondition::Cloudy => "Cloudy",
        WeatherCondition::PartlyCloudy => "Partly Cloudy",
        WeatherCondition::Overcast => "Overcast",
        WeatherCondition::Fog => "Fog",
        WeatherCondition::Drizzle => "Drizzle",
        WeatherCondition::FreezingRain => "Freezing Rain",
        WeatherCondition::Rain => "Rain",
        WeatherCondition::Snow => "Snow",
        WeatherCondition::SnowGrains => "Snow Grains",
        WeatherCondition::RainShowers => "Rain Showers",
        WeatherCondition::SnowShowers => "Snow Showers",
        WeatherCondition::Thunderstorm => "Thunderstorm",
        WeatherCondition::ThunderstormHail => "Thunderstorm with Hail",
    }
}

pub fn aqi_band(band: AqiBand) -> &'static str {
    match band {
        AqiBand::Good => "good",
        AqiBand::Moderate => "moderate",
        AqiBand::Sensitive => "sensitive groups",
        AqiBand::Unhealthy => "unhealthy",
        AqiBand::VeryUnhealthy => "very unhealthy",
        AqiBand::Hazardous => "hazardous",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_is_the_only_language_and_the_default() {
        assert_eq!(parse_language(""), Ok(Language::En));
        assert_eq!(parse_language(" EN "), Ok(Language::En));
        assert!(
            parse_language("id")
                .unwrap_err()
                .contains("not available yet")
        );
    }

    #[test]
    fn every_condition_has_an_english_name() {
        for condition in WeatherCondition::ALL {
            assert!(!super::condition(*condition).is_empty());
        }
    }
}
