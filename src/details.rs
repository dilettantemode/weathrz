use crate::astro::SunSky;
use crate::text;
use crate::weather::WeatherCondition;
use crate::weather::air_quality::AqiBand;
use crate::weather::types::{HourPoint, WeatherData, WeatherUnits};
use crate::weather::units::{
    format_precipitation, format_temperature, format_wind_speed, ms_to_kmh,
};
use chrono::{DateTime, Duration, NaiveDateTime, NaiveTime, Utc};
use serde::Serialize;

const HOURS_SHOWN: usize = 6;
const COMPASS: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

pub struct Facts<'a> {
    pub weather: &'a WeatherData,
    pub units: &'a WeatherUnits,
    pub aqi: Option<u16>,
    pub sun: Option<SunSky>,
    pub sun_times: Option<(NaiveTime, NaiveTime)>,
    pub moon_phase: Option<f64>,
    pub now: NaiveDateTime,
}

pub fn sun_times(
    (latitude, longitude): (f64, f64),
    now: NaiveDateTime,
    now_utc: DateTime<Utc>,
) -> Option<(NaiveTime, NaiveTime)> {
    let offset = now - now_utc.naive_utc();
    let midnight = now.date().and_hms_opt(0, 0, 0)?;
    let rise = crate::astro::sunrise_after(latitude, longitude, (midnight - offset).and_utc())?;
    let set = crate::astro::sunset_after(latitude, longitude, rise)?;
    let local = |at: DateTime<Utc>| (at.naive_utc() + offset).time();
    Some((local(rise), local(set)))
}

pub fn provider_or_computed(
    weather: &WeatherData,
    place: (f64, f64),
    now: NaiveDateTime,
    now_utc: DateTime<Utc>,
) -> Option<(NaiveTime, NaiveTime)> {
    weather
        .sun
        .rise
        .zip(weather.sun.set)
        .or_else(|| sun_times(place, now, now_utc))
}

pub fn compass(degrees: f64) -> &'static str {
    COMPASS[((degrees.rem_euclid(360.0) + 22.5) / 45.0) as usize % COMPASS.len()]
}

pub fn uv_band(uv: f64) -> &'static str {
    match uv.round() as i64 {
        ..=2 => text::UV_LOW,
        3..=5 => text::UV_MODERATE,
        6..=7 => text::UV_HIGH,
        8..=10 => text::UV_VERY_HIGH,
        _ => text::UV_EXTREME,
    }
}

pub fn moon_lit(phase: f64) -> f64 {
    (1.0 - (phase * std::f64::consts::TAU).cos()) / 2.0
}

pub fn moon_name(phase: f64) -> &'static str {
    let phase = phase.rem_euclid(1.0);
    let index = match phase {
        p if !(0.03..0.97).contains(&p) => 0,
        p if p < 0.22 => 1,
        p if p < 0.28 => 2,
        p if p < 0.47 => 3,
        p if p < 0.53 => 4,
        p if p < 0.72 => 5,
        p if p < 0.78 => 6,
        _ => 7,
    };
    text::MOON_PHASES[index]
}

fn parse(time: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M").ok()
}

fn condition_of(code: i32) -> WeatherCondition {
    crate::weather::normalizer::WeatherNormalizer::wmo_code_to_condition(code)
}

pub fn hours_ahead(weather: &WeatherData, now: NaiveDateTime) -> Vec<(NaiveDateTime, &HourPoint)> {
    weather
        .outlook
        .iter()
        .filter_map(|point| Some((parse(&point.time)? - Duration::hours(1), point)))
        .filter(|(start, _)| *start + Duration::hours(1) > now)
        .take(HOURS_SHOWN)
        .collect()
}

fn daylight(rise: NaiveTime, set: NaiveTime) -> Option<String> {
    let minutes = (set - rise).num_minutes();
    (minutes > 0).then(|| {
        format!(
            "{} h {:02} m {}",
            minutes / 60,
            minutes % 60,
            text::OF_DAYLIGHT
        )
    })
}

pub fn rows(facts: &Facts<'_>) -> Vec<(String, String)> {
    let weather = facts.weather;
    let units = facts.units;
    let (temp, temp_unit) = format_temperature(weather.temperature, units.temperature);
    let mut now = format!(
        "{} · {temp:.1}{temp_unit}",
        text::condition(weather.condition)
    );
    if let Some(feels) = weather.feels_like {
        let (feels, _) = format_temperature(feels, units.temperature);
        now.push_str(&format!(" ({} {}°)", text::FEELS, feels.round() as i64));
    }
    let mut out = vec![(text::DETAILS_NOW.to_string(), now)];
    let (wind, wind_unit) = format_wind_speed(weather.wind_speed, units.wind_speed);
    out.push((
        text::WIND.to_string(),
        format!(
            "{wind:.1} {wind_unit} {} {}",
            text::FROM_THE,
            compass(weather.wind_direction)
        ),
    ));
    if let Some(humidity) = weather.humidity {
        out.push((
            text::HUMIDITY.to_string(),
            format!("{}%", humidity.round() as i64),
        ));
    }
    if let Some(uv) = weather.uv_index.filter(|_| weather.sun.is_day) {
        out.push((
            text::UV.to_string(),
            format!("{} · {}", uv.round() as i64, uv_band(uv)),
        ));
    }
    if let Some(aqi) = facts.aqi {
        out.push((
            text::DETAILS_AIR.to_string(),
            format!(
                "{} {aqi} · {}",
                text::AQI,
                text::aqi_band(AqiBand::from_aqi(aqi))
            ),
        ));
    }
    if let Some((rise, set)) = facts.sun_times {
        let mut sun = format!(
            "{} {} · {} {}",
            text::RISE,
            rise.format("%H:%M"),
            text::SET,
            set.format("%H:%M")
        );
        if let Some(length) = daylight(rise, set) {
            sun.push_str(&format!(" · {length}"));
        }
        out.push((text::DETAILS_SUN.to_string(), sun));
    }
    if let Some(sky) = facts.sun {
        let place = if sky.elevation_deg > 0.0 {
            format!(
                "{} {}° {} {}",
                text::NOW,
                sky.elevation_deg.round() as i64,
                text::UP_IN_THE,
                compass(sky.azimuth_deg)
            )
        } else {
            format!("{} {}", text::NOW, text::BELOW_THE_HORIZON)
        };
        let label = if facts.sun_times.is_some() {
            String::new()
        } else {
            text::DETAILS_SUN.to_string()
        };
        out.push((label, place));
    }
    if let Some(phase) = facts.moon_phase {
        out.push((
            text::DETAILS_MOON.to_string(),
            format!(
                "{} · {}% {}",
                moon_name(phase),
                (moon_lit(phase) * 100.0).round() as i64,
                text::LIT
            ),
        ));
    }
    let hours = hours_ahead(weather, facts.now);
    if !hours.is_empty() {
        out.push((String::new(), String::new()));
        out.push((text::DETAILS_NEXT_HOURS.to_string(), String::new()));
        for (start, point) in hours {
            let (amount, amount_unit) =
                format_precipitation(point.precipitation, units.precipitation);
            let mut hour = text::condition(condition_of(point.code)).to_string();
            if let Some(probability) = point.probability {
                hour.push_str(&format!(" · {probability}%"));
            }
            if point.precipitation > 0.0 {
                hour.push_str(&format!(" · {amount:.1} {amount_unit}"));
            }
            out.push((start.format("%H:%M").to_string(), hour));
        }
    }
    out
}

#[derive(Debug, Serialize)]
pub struct JsonHour {
    pub from: String,
    pub condition: &'static str,
    pub probability_pct: Option<u8>,
    pub precipitation_mm: f64,
}

#[derive(Debug, Serialize)]
pub struct JsonReading {
    pub place: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub reading_time: String,
    pub age_secs: Option<u64>,
    pub condition: &'static str,
    pub temperature_c: f64,
    pub feels_like_c: Option<f64>,
    pub humidity_pct: Option<f64>,
    pub wind_kmh: f64,
    pub wind_from_deg: f64,
    pub wind_from: &'static str,
    pub precipitation_mm: f64,
    pub uv_index: Option<f64>,
    pub is_day: bool,
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
    pub moon_phase: Option<f64>,
    pub moon: Option<&'static str>,
    pub greeting: Option<String>,
    pub hours: Vec<JsonHour>,
}

pub fn json(
    weather: &WeatherData,
    (place, latitude, longitude): (Option<String>, Option<f64>, Option<f64>),
    sun_times: Option<(NaiveTime, NaiveTime)>,
    (now, moon_phase): (NaiveDateTime, f64),
    age_secs: Option<u64>,
    greeting: Option<String>,
) -> JsonReading {
    let round = |value: f64| (value * 10.0).round() / 10.0;
    JsonReading {
        place,
        latitude,
        longitude,
        reading_time: weather.timestamp.clone(),
        age_secs,
        condition: text::condition(weather.condition),
        temperature_c: round(weather.temperature),
        feels_like_c: weather.feels_like.map(round),
        humidity_pct: weather.humidity.map(round),
        wind_kmh: round(ms_to_kmh(weather.wind_speed)),
        wind_from_deg: round(weather.wind_direction),
        wind_from: compass(weather.wind_direction),
        precipitation_mm: round(weather.precipitation),
        uv_index: weather.uv_index.map(round),
        is_day: weather.sun.is_day,
        sunrise: sun_times.map(|(rise, _)| rise.format("%H:%M").to_string()),
        sunset: sun_times.map(|(_, set)| set.format("%H:%M").to_string()),
        moon_phase: Some((moon_phase * 1000.0).round() / 1000.0),
        moon: Some(moon_name(moon_phase)),
        greeting,
        hours: hours_ahead(weather, now)
            .into_iter()
            .map(|(start, point)| JsonHour {
                from: start.format("%Y-%m-%dT%H:%M").to_string(),
                condition: text::condition(condition_of(point.code)),
                probability_pct: point.probability,
                precipitation_mm: round(point.precipitation),
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weather::types::CelestialEvents;

    fn reading() -> WeatherData {
        let mut sun = CelestialEvents::from_bool(true);
        sun.rise = NaiveTime::from_hms_opt(5, 48, 0);
        sun.set = NaiveTime::from_hms_opt(17, 52, 0);
        WeatherData {
            condition: WeatherCondition::RainShowers,
            temperature: 24.0,
            precipitation: 0.4,
            wind_speed: 10.0 / 3.6,
            wind_direction: 225.0,
            sun,
            moon_phase: Some(0.37),
            humidity: Some(81.0),
            timestamp: "2026-10-23T14:20".to_string(),
            attribution: String::new(),
            feels_like: Some(27.0),
            uv_index: Some(6.2),
            outlook: (14..23)
                .map(|hour| HourPoint {
                    time: format!("2026-10-23T{hour:02}:00"),
                    probability: Some(hour as u8 * 3),
                    code: if hour < 18 { 80 } else { 2 },
                    precipitation: if hour < 18 { 0.8 } else { 0.0 },
                })
                .collect(),
        }
    }

    fn at(text: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap()
    }

    fn lines(facts: &Facts<'_>) -> Vec<String> {
        rows(facts)
            .into_iter()
            .map(|(label, value)| format!("{label}|{value}"))
            .collect()
    }

    #[test]
    fn the_card_reads_the_whole_reading() {
        let weather = reading();
        let units = WeatherUnits::metric();
        let sun = SunSky {
            elevation_deg: 52.4,
            azimuth_deg: 300.0,
            noon_elevation_deg: 80.0,
            day_progress: Some(0.7),
        };
        let card = lines(&Facts {
            weather: &weather,
            units: &units,
            aqi: Some(90),
            sun: Some(sun),
            sun_times: weather.sun.rise.zip(weather.sun.set),
            moon_phase: weather.moon_phase,
            now: at("2026-10-23T14:20"),
        });
        assert_eq!(
            card[..8],
            [
                "Now|Rain Showers · 24.0°C (feels 27°)",
                "Wind|10.0 km/h from the SW",
                "Humidity|81%",
                "UV|6 · high",
                "Air|AQI 90 · moderate",
                "Sun|rise 05:48 · set 17:52 · 12 h 04 m of daylight",
                "|now 52° up in the NW",
                "Moon|waxing gibbous · 84% lit",
            ]
        );
        assert_eq!(card[9], "Next hours|");
        assert_eq!(card[10], "14:00|Rain Showers · 45% · 0.8 mm");
        assert_eq!(card.len(), 10 + HOURS_SHOWN);
        assert_eq!(card.last().unwrap(), "19:00|Partly Cloudy · 60%");
    }

    #[test]
    fn imperial_units_and_a_missing_reading_drop_rows_not_the_card() {
        let mut weather = reading();
        weather.sun = CelestialEvents::from_bool(false);
        weather.humidity = None;
        weather.outlook.clear();
        let card = lines(&Facts {
            weather: &weather,
            units: &WeatherUnits::imperial(),
            aqi: None,
            sun: None,
            sun_times: None,
            moon_phase: None,
            now: at("2026-10-23T14:20"),
        });
        assert_eq!(
            card,
            [
                "Now|Rain Showers · 75.2°F (feels 81°)",
                "Wind|6.2 mph from the SW"
            ]
        );
    }

    #[test]
    fn without_provider_times_the_sun_is_worked_out_for_the_place() {
        let at = |text| NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap();
        let now = at("2026-10-23T12:00");
        let (rise, set) = sun_times(
            (-6.2088, 106.8456),
            now,
            (now - Duration::hours(7)).and_utc(),
        )
        .unwrap();
        let minutes = |time: NaiveTime| (time - NaiveTime::MIN).num_minutes();
        assert!(
            (5 * 60 + 20..5 * 60 + 45).contains(&minutes(rise)),
            "{rise}"
        );
        assert!((17 * 60 + 35..18 * 60).contains(&minutes(set)), "{set}");
        let winter = at("2026-12-21T12:00");
        let polar = sun_times(
            (78.2, 15.6),
            winter,
            (winter - Duration::hours(1)).and_utc(),
        );
        assert_eq!(polar, None, "no sunrise in Svalbard's polar night");
    }

    #[test]
    fn compass_moon_and_uv_words() {
        assert_eq!(compass(0.0), "N");
        assert_eq!(compass(359.0), "N");
        assert_eq!(compass(22.6), "NE");
        assert_eq!(compass(-90.0), "W");
        assert_eq!(moon_name(0.0), "new moon");
        assert_eq!(moon_name(0.5), "full moon");
        assert_eq!(moon_name(0.76), "last quarter");
        assert_eq!(moon_name(0.9), "waning crescent");
        assert!((moon_lit(0.5) - 1.0).abs() < 1e-9);
        assert_eq!(uv_band(2.4), "low");
        assert_eq!(uv_band(10.6), "extreme");
    }

    #[test]
    fn json_keeps_metric_keys_whatever_the_display_units() {
        let weather = reading();
        let value = serde_json::to_value(json(
            &weather,
            (Some("Jakarta".to_string()), Some(-6.2), Some(106.8)),
            weather.sun.rise.zip(weather.sun.set),
            (at("2026-10-23T14:20"), 0.37),
            None,
            None,
        ))
        .unwrap();
        assert_eq!(value["wind_kmh"], 10.0);
        assert_eq!(value["wind_from"], "SW");
        assert_eq!(value["sunrise"], "05:48");
        assert_eq!(value["moon"], "waxing gibbous");
        assert_eq!(value["hours"][0]["from"], "2026-10-23T14:00");
        assert_eq!(value["hours"].as_array().unwrap().len(), HOURS_SHOWN);
    }
}
