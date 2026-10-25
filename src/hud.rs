use crate::config::IconMode;
use crate::text;
use crate::theme::colors::ElementColors;
use crate::weather::air_quality::AqiBand;
use crate::weather::outlook::{Outlook, outlook};
use crate::weather::{
    WeatherCondition, WeatherData, WeatherUnits, format_precipitation, format_temperature,
    format_wind_speed,
};
use crossterm::style::Color;

pub const ICON_TEMP: char = '\u{e350}';
pub const ICON_WIND: char = '\u{e34b}';
pub const ICON_PRECIP: char = '\u{e371}';
pub const ICON_LOCATION: char = '\u{f041}';
pub const ICON_HUMIDITY: char = '\u{e373}';
pub const ICON_AQI: char = '\u{f0d43}';
pub const ICON_RAIN_AHEAD: char = '\u{e37c}';
pub const ICON_STORM_AHEAD: char = '\u{e31d}';
pub const ICON_UV: char = '\u{f17fe}';
const FEELS_APART_C: f64 = 2.0;
const UV_SHOWN: f64 = 3.0;
const SEPARATOR: &str = "   ";
const ASCII_SEPARATOR: &str = " | ";

pub const HUD_X: u16 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Greeting {
    pub icon: char,
    pub text: String,
    pub short: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudRole {
    Greeting,
    Plain,
    Offline,
    Condition,
    Temp,
    Wind,
    Precip,
    Humidity,
    Aqi(AqiBand),
    Outlook,
    Uv(usize),
    Location,
    Hint,
    Separator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudSegment {
    pub role: HudRole,
    pub text: String,
    pub short: Option<String>,
}

pub fn segment(role: HudRole, text: String) -> HudSegment {
    HudSegment {
        role,
        text,
        short: None,
    }
}

fn separator() -> HudSegment {
    segment(HudRole::Separator, SEPARATOR.to_string())
}

pub fn greeting_segments(greeting: Option<&Greeting>, mode: IconMode) -> Vec<HudSegment> {
    let Some(greeting) = greeting else {
        return Vec::new();
    };
    let shown = |text: &str| match mode {
        IconMode::Nerd => format!("{} {text}", greeting.icon),
        IconMode::Ascii => format!("{text} | "),
    };
    let mut segments = vec![HudSegment {
        role: HudRole::Greeting,
        text: shown(&greeting.text),
        short: greeting.short.as_deref().map(shown),
    }];
    if mode == IconMode::Nerd {
        segments.push(separator());
    }
    segments
}

pub fn fit(segments: &[HudSegment], width: u16, mode: IconMode) -> Vec<HudSegment> {
    let fits = |bar: &[HudSegment]| {
        HUD_X as usize + bar.iter().map(|s| s.text.chars().count()).sum::<usize>() <= width as usize
    };
    let mut bar = segments.to_vec();
    if fits(&bar) {
        return bar;
    }
    drop_role(&mut bar, HudRole::Hint);
    if !fits(&bar) {
        drop_uv(&mut bar);
    }
    if !fits(&bar)
        && let Some(first) = bar.first_mut().filter(|s| s.role == HudRole::Greeting)
        && let Some(short) = first.short.take()
    {
        first.text = short;
    }
    let greeting_first =
        |bar: &[HudSegment]| bar.first().is_some_and(|s| s.role == HudRole::Greeting);
    if mode == IconMode::Nerd && !fits(&bar) && greeting_first(&bar) {
        let icon = bar[0].text.chars().next().unwrap_or(' ');
        bar[0] = segment(HudRole::Greeting, icon.to_string());
    }
    for role in [
        HudRole::Location,
        HudRole::Wind,
        HudRole::Precip,
        HudRole::Outlook,
    ] {
        if fits(&bar) {
            break;
        }
        drop_role(&mut bar, role);
    }
    if mode == IconMode::Ascii && !fits(&bar) && greeting_first(&bar) {
        bar.remove(0);
    }
    bar
}

fn drop_uv(bar: &mut Vec<HudSegment>) {
    if let Some(role) = bar
        .iter()
        .map(|s| s.role)
        .find(|role| matches!(role, HudRole::Uv(_)))
    {
        drop_role(bar, role);
    }
}

fn drop_role(bar: &mut Vec<HudSegment>, role: HudRole) {
    if let Some(index) = bar.iter().position(|s| s.role == role) {
        let start = if index > 0 && bar[index - 1].role == HudRole::Separator {
            index - 1
        } else {
            index
        };
        bar.drain(start..=index);
    }
}

fn precipitating(condition: WeatherCondition) -> bool {
    !matches!(
        condition,
        WeatherCondition::Clear
            | WeatherCondition::PartlyCloudy
            | WeatherCondition::Cloudy
            | WeatherCondition::Overcast
            | WeatherCondition::Fog
    )
}

pub fn uv_tier(uv: f64) -> Option<usize> {
    let uv = uv.round();
    if uv < UV_SHOWN {
        None
    } else if uv < 6.0 {
        Some(1)
    } else if uv < 8.0 {
        Some(2)
    } else if uv < 11.0 {
        Some(3)
    } else {
        Some(4)
    }
}

fn feels_text(weather: &WeatherData, units: &WeatherUnits) -> String {
    match weather.feels_like {
        Some(feels) if (feels - weather.temperature).abs() >= FEELS_APART_C => {
            let (value, _) = format_temperature(feels, units.temperature);
            format!(" ({} {}°)", text::FEELS, value.round() as i64)
        }
        _ => String::new(),
    }
}

fn clock(at: Option<chrono::NaiveDateTime>) -> String {
    at.map_or(text::SOON.to_string(), |at| {
        format!("~{}", at.format("%H:%M"))
    })
}

enum Wet {
    Amount(String),
    Ahead { text: String, storm: bool },
    Nothing,
}

fn wet_text(weather: &WeatherData, amount: String) -> Wet {
    let raining = precipitating(weather.condition);
    match outlook(&weather.timestamp, raining, &weather.outlook) {
        None | Some(Outlook::StaysWet) => Wet::Amount(amount),
        Some(Outlook::StaysDry) => Wet::Nothing,
        Some(Outlook::DryFrom(at)) => Wet::Amount(match at {
            Some(_) => format!("{amount} · {} {}", text::UNTIL, clock(at)),
            None => format!("{amount} · {}", text::EASING),
        }),
        Some(Outlook::Rain {
            from,
            probability,
            storm,
        }) => Wet::Ahead {
            text: format!(
                "{} {} · {probability}%",
                if storm { text::STORMS } else { text::RAIN },
                clock(from)
            ),
            storm,
        },
    }
}

fn uv_segment(weather: &WeatherData, nerd: bool) -> Option<HudSegment> {
    let uv = weather.uv_index.filter(|_| weather.sun.is_day)?;
    let tier = uv_tier(uv)?;
    let text = if nerd {
        format!("{ICON_UV} {} {}", text::UV, uv.round() as i64)
    } else {
        format!("{}: {}", text::UV, uv.round() as i64)
    };
    Some(segment(HudRole::Uv(tier), text))
}

pub fn ascii_segments(
    weather: &WeatherData,
    condition_text: &str,
    units: &WeatherUnits,
    is_offline: bool,
    location: Option<&str>,
    aqi: Option<u16>,
) -> Vec<HudSegment> {
    let (temp, temp_unit) = format_temperature(weather.temperature, units.temperature);
    let (wind, wind_unit) = format_wind_speed(weather.wind_speed, units.wind_speed);
    let (precip, precip_unit) = format_precipitation(weather.precipitation, units.precipitation);
    let mut parts = Vec::new();
    if is_offline {
        parts.push(segment(HudRole::Offline, text::OFFLINE.to_string()));
    }
    parts.push(segment(
        HudRole::Condition,
        format!("{}: {condition_text}", text::WEATHER),
    ));
    parts.push(segment(
        HudRole::Temp,
        format!(
            "{}: {temp:.1}{temp_unit}{}",
            text::TEMP,
            feels_text(weather, units)
        ),
    ));
    parts.push(segment(
        HudRole::Wind,
        format!("{}: {wind:.1}{wind_unit}", text::WIND),
    ));
    match wet_text(weather, format!("{precip:.1}{precip_unit}")) {
        Wet::Amount(amount) => parts.push(segment(
            HudRole::Precip,
            format!("{}: {amount}", text::PRECIP),
        )),
        Wet::Ahead { text, .. } => parts.push(segment(HudRole::Outlook, capitalised(&text))),
        Wet::Nothing => {}
    }
    if let Some(humidity) = weather.humidity {
        parts.push(segment(
            HudRole::Humidity,
            format!("{}: {}%", text::HUMIDITY, humidity.round() as i64),
        ));
    }
    parts.extend(uv_segment(weather, false));
    if let Some(aqi) = aqi {
        let band = AqiBand::from_aqi(aqi);
        parts.push(segment(
            HudRole::Aqi(band),
            format!("{}: {aqi} {}", text::AQI, band.label()),
        ));
    }
    if let Some(label) = location {
        parts.push(segment(
            HudRole::Location,
            format!("{}: {label}", text::LOCATION),
        ));
    }
    parts.push(segment(HudRole::Hint, text::QUIT_HINT.to_string()));
    let mut segments = Vec::with_capacity(parts.len() * 2);
    for (index, part) in parts.into_iter().enumerate() {
        if index > 0 {
            segments.push(segment(HudRole::Separator, ASCII_SEPARATOR.to_string()));
        }
        segments.push(part);
    }
    segments
}

pub fn condition_icon(condition: WeatherCondition, is_day: bool) -> char {
    match condition {
        WeatherCondition::Clear => {
            if is_day {
                '\u{e30d}'
            } else {
                '\u{e32b}'
            }
        }
        WeatherCondition::PartlyCloudy => {
            if is_day {
                '\u{e302}'
            } else {
                '\u{e37e}'
            }
        }
        WeatherCondition::Cloudy | WeatherCondition::Overcast => '\u{e312}',
        WeatherCondition::Fog => '\u{e313}',
        WeatherCondition::Drizzle => '\u{e31b}',
        WeatherCondition::Rain => '\u{e318}',
        WeatherCondition::RainShowers => '\u{e319}',
        WeatherCondition::FreezingRain => '\u{e316}',
        WeatherCondition::Snow | WeatherCondition::SnowGrains | WeatherCondition::SnowShowers => {
            '\u{e31a}'
        }
        WeatherCondition::Thunderstorm => '\u{e31d}',
        WeatherCondition::ThunderstormHail => '\u{e314}',
    }
}

pub fn nerd_segments(
    weather: &WeatherData,
    condition_text: &str,
    units: &WeatherUnits,
    is_offline: bool,
    location: Option<&str>,
    aqi: Option<u16>,
) -> Vec<HudSegment> {
    let (temp, temp_unit) = format_temperature(weather.temperature, units.temperature);
    let (wind, wind_unit) = format_wind_speed(weather.wind_speed, units.wind_speed);
    let (precip, precip_unit) = format_precipitation(weather.precipitation, units.precipitation);

    let mut segments = Vec::new();
    if is_offline {
        segments.push(segment(HudRole::Offline, text::OFFLINE.to_string()));
        segments.push(separator());
    }
    segments.push(segment(
        HudRole::Condition,
        format!(
            "{} {condition_text}",
            condition_icon(weather.condition, weather.sun.is_day)
        ),
    ));
    segments.push(separator());
    segments.push(segment(
        HudRole::Temp,
        format!(
            "{ICON_TEMP} {temp:.1}{temp_unit}{}",
            feels_text(weather, units)
        ),
    ));
    segments.push(separator());
    segments.push(segment(
        HudRole::Wind,
        format!("{ICON_WIND} {wind:.1} {wind_unit}"),
    ));
    match wet_text(weather, format!("{precip:.1} {precip_unit}")) {
        Wet::Amount(text) => {
            segments.push(separator());
            segments.push(segment(HudRole::Precip, format!("{ICON_PRECIP} {text}")));
        }
        Wet::Ahead { text, storm } => {
            let icon = if storm {
                ICON_STORM_AHEAD
            } else {
                ICON_RAIN_AHEAD
            };
            segments.push(separator());
            segments.push(segment(HudRole::Outlook, format!("{icon} {text}")));
        }
        Wet::Nothing => {}
    }
    if let Some(humidity) = weather.humidity {
        segments.push(separator());
        segments.push(segment(
            HudRole::Humidity,
            format!("{ICON_HUMIDITY} {}%", humidity.round() as i64),
        ));
    }
    if let Some(uv) = uv_segment(weather, true) {
        segments.push(separator());
        segments.push(uv);
    }
    if let Some(aqi) = aqi {
        let band = AqiBand::from_aqi(aqi);
        segments.push(separator());
        segments.push(segment(
            HudRole::Aqi(band),
            format!("{ICON_AQI} {aqi} {}", aqi_face(band)),
        ));
    }
    if let Some(label) = location {
        segments.push(separator());
        segments.push(segment(
            HudRole::Location,
            format!("{ICON_LOCATION} {label}"),
        ));
    }
    segments
}

fn capitalised(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

pub fn aqi_face(band: AqiBand) -> char {
    match band {
        AqiBand::Good => '\u{f0c71}',
        AqiBand::Moderate => '\u{f0c74}',
        AqiBand::Sensitive => '\u{f0f4c}',
        AqiBand::Unhealthy => '\u{f1586}',
        AqiBand::VeryUnhealthy => '\u{f157c}',
        AqiBand::Hazardous => '\u{f068c}',
    }
}

pub fn role_color(role: HudRole, colors: &ElementColors) -> Color {
    match role {
        HudRole::Greeting => colors.hud_greeting,
        HudRole::Plain => colors.hud,
        HudRole::Offline => colors.hud_offline,
        HudRole::Condition => colors.hud_condition,
        HudRole::Temp => colors.hud_temp,
        HudRole::Wind => colors.hud_wind,
        HudRole::Precip => colors.hud_precip,
        HudRole::Humidity => colors.hud_humidity,
        HudRole::Aqi(band) => colors.aqi[band.tier()],
        HudRole::Outlook => colors.hud_precip,
        HudRole::Uv(tier) => colors.aqi[tier.min(colors.aqi.len() - 1)],
        HudRole::Location | HudRole::Hint | HudRole::Separator => colors.hud_location,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weather::types::CelestialEvents;

    fn weather(condition: WeatherCondition, is_day: bool) -> WeatherData {
        WeatherData {
            condition,
            temperature: 28.0,
            precipitation: 0.2,
            wind_speed: 2.5,
            wind_direction: 0.0,
            sun: CelestialEvents::from_bool(is_day),
            moon_phase: None,
            humidity: None,
            timestamp: String::new(),
            attribution: String::new(),
            feels_like: None,
            uv_index: None,
            outlook: Vec::new(),
        }
    }

    #[test]
    fn every_condition_has_an_icon_for_day_and_night() {
        let expected = [
            (WeatherCondition::Clear, '\u{e30d}', '\u{e32b}'),
            (WeatherCondition::PartlyCloudy, '\u{e302}', '\u{e37e}'),
            (WeatherCondition::Cloudy, '\u{e312}', '\u{e312}'),
            (WeatherCondition::Overcast, '\u{e312}', '\u{e312}'),
            (WeatherCondition::Fog, '\u{e313}', '\u{e313}'),
            (WeatherCondition::Drizzle, '\u{e31b}', '\u{e31b}'),
            (WeatherCondition::Rain, '\u{e318}', '\u{e318}'),
            (WeatherCondition::RainShowers, '\u{e319}', '\u{e319}'),
            (WeatherCondition::FreezingRain, '\u{e316}', '\u{e316}'),
            (WeatherCondition::Snow, '\u{e31a}', '\u{e31a}'),
            (WeatherCondition::SnowGrains, '\u{e31a}', '\u{e31a}'),
            (WeatherCondition::SnowShowers, '\u{e31a}', '\u{e31a}'),
            (WeatherCondition::Thunderstorm, '\u{e31d}', '\u{e31d}'),
            (WeatherCondition::ThunderstormHail, '\u{e314}', '\u{e314}'),
        ];
        assert_eq!(expected.len(), WeatherCondition::ALL.len());
        for (condition, day, night) in expected {
            assert_eq!(condition_icon(condition, true), day, "{condition:?} day");
            assert_eq!(
                condition_icon(condition, false),
                night,
                "{condition:?} night"
            );
        }
    }

    #[test]
    fn segments_follow_the_mockup_order() {
        let segments = nerd_segments(
            &weather(WeatherCondition::Rain, true),
            "Rain",
            &WeatherUnits::metric(),
            false,
            Some("Jakarta"),
            None,
        );
        let roles: Vec<HudRole> = segments.iter().map(|s| s.role).collect();
        assert_eq!(
            roles,
            [
                HudRole::Condition,
                HudRole::Separator,
                HudRole::Temp,
                HudRole::Separator,
                HudRole::Wind,
                HudRole::Separator,
                HudRole::Precip,
                HudRole::Separator,
                HudRole::Location
            ]
        );
        let texts: Vec<&str> = segments.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(
            texts,
            [
                "\u{e318} Rain",
                "   ",
                "\u{e350} 28.0°C",
                "   ",
                "\u{e34b} 9.0 km/h",
                "   ",
                "\u{e371} 0.2 mm",
                "   ",
                "\u{f041} Jakarta"
            ]
        );
    }

    #[test]
    fn offline_and_hidden_location_change_the_ends() {
        let segments = nerd_segments(
            &weather(WeatherCondition::Clear, false),
            "Clear",
            &WeatherUnits::imperial(),
            true,
            None,
            None,
        );
        assert_eq!(
            segments.first().map(|s| (s.role, s.text.as_str())),
            Some((HudRole::Offline, "OFFLINE"))
        );
        assert_eq!(segments.last().map(|s| s.role), Some(HudRole::Precip));
        assert!(segments.iter().any(|s| s.text == "\u{e32b} Clear"));
        assert!(segments.iter().any(|s| s.text.ends_with("°F")));
        assert!(segments.iter().any(|s| s.text.ends_with(" mph")));
        assert!(segments.iter().any(|s| s.text.ends_with(" in")));
    }

    #[test]
    fn roles_map_to_their_element_colours() {
        let colors = crate::theme::catalogue::default_colors().day;
        assert_eq!(role_color(HudRole::Plain, &colors), colors.hud);
        assert_eq!(
            role_color(HudRole::Condition, &colors),
            colors.hud_condition
        );
        assert_eq!(role_color(HudRole::Temp, &colors), colors.hud_temp);
        assert_eq!(role_color(HudRole::Wind, &colors), colors.hud_wind);
        assert_eq!(role_color(HudRole::Precip, &colors), colors.hud_precip);
        assert_eq!(role_color(HudRole::Location, &colors), colors.hud_location);
        assert_eq!(role_color(HudRole::Offline, &colors), colors.hud_offline);
    }

    fn pumpkin() -> Greeting {
        Greeting {
            icon: '\u{f0bbf}',
            text: "Happy Halloween".to_string(),
            short: None,
        }
    }

    #[test]
    fn a_long_greeting_shortens_before_anything_else_goes() {
        let eid = Greeting {
            icon: '\u{f0594}',
            text: "Day of Silence · Saka New Year 1949".to_string(),
            short: Some("Day of Silence".to_string()),
        };
        let mut ascii = greeting_segments(Some(&eid), IconMode::Ascii);
        ascii.push(segment(
            HudRole::Plain,
            "Weather: Clear | Temp: 28.0°C".to_string(),
        ));
        assert_eq!(texts(&fit(&ascii, 120, IconMode::Ascii)), texts(&ascii));
        let shorter = fit(&ascii, 60, IconMode::Ascii);
        assert_eq!(
            texts(&shorter),
            ["Day of Silence | ", "Weather: Clear | Temp: 28.0°C"]
        );
        assert_eq!(texts(&fit(&ascii, 40, IconMode::Ascii)), texts(&ascii[1..]));
        let mut nerd = greeting_segments(Some(&eid), IconMode::Nerd);
        nerd.extend(nerd_segments(
            &weather(WeatherCondition::Clear, false),
            "Clear",
            &WeatherUnits::metric(),
            false,
            Some("Jakarta"),
            None,
        ));
        let nerd_width = HUD_X
            + texts(&nerd)[1..]
                .iter()
                .map(|t| t.chars().count() as u16)
                .sum::<u16>()
            + 20;
        assert_eq!(
            fit(&nerd, nerd_width, IconMode::Nerd)[0].text,
            "\u{f0594} Day of Silence"
        );
    }

    fn texts(segments: &[HudSegment]) -> Vec<&str> {
        segments.iter().map(|s| s.text.as_str()).collect()
    }

    #[test]
    fn greeting_segments_follow_the_icon_mode() {
        assert!(greeting_segments(None, IconMode::Nerd).is_empty());
        let nerd = greeting_segments(Some(&pumpkin()), IconMode::Nerd);
        assert_eq!(texts(&nerd), ["\u{f0bbf} Happy Halloween", "   "]);
        assert_eq!(nerd[0].role, HudRole::Greeting);
        let ascii = greeting_segments(Some(&pumpkin()), IconMode::Ascii);
        assert_eq!(texts(&ascii), ["Happy Halloween | "]);
    }

    #[test]
    fn fit_shrinks_or_drops_the_greeting_only_when_too_wide() {
        let mut nerd = greeting_segments(Some(&pumpkin()), IconMode::Nerd);
        nerd.extend(nerd_segments(
            &weather(WeatherCondition::Clear, false),
            "Clear",
            &WeatherUnits::metric(),
            false,
            Some("Jakarta"),
            None,
        ));
        assert_eq!(fit(&nerd, 200, IconMode::Nerd), nerd);
        let narrow = fit(&nerd, 60, IconMode::Nerd);
        assert_eq!(narrow[0].text, "\u{f0bbf}");
        assert_eq!(narrow[1..], nerd[1..]);

        let mut ascii = greeting_segments(Some(&pumpkin()), IconMode::Ascii);
        ascii.push(segment(
            HudRole::Plain,
            "Weather: Clear | Temp: 28.0°C".to_string(),
        ));
        assert_eq!(fit(&ascii, 200, IconMode::Ascii), ascii);
        assert_eq!(fit(&ascii, 30, IconMode::Ascii), ascii[1..]);

        let plain = vec![segment(HudRole::Plain, "x".repeat(100))];
        assert_eq!(fit(&plain, 40, IconMode::Ascii), plain);
    }

    #[test]
    fn greeting_role_uses_its_own_colour() {
        let colors = crate::theme::catalogue::default_colors().day;
        assert_eq!(role_color(HudRole::Greeting, &colors), colors.hud_greeting);
    }

    fn jakarta_bar(aqi: Option<u16>) -> Vec<HudSegment> {
        let mut weather = weather(WeatherCondition::Clear, true);
        weather.temperature = 33.1;
        weather.wind_speed = 9.1 / 3.6;
        weather.precipitation = 0.0;
        weather.humidity = Some(58.4);
        let mut bar = greeting_segments(Some(&pumpkin()), IconMode::Nerd);
        bar.extend(nerd_segments(
            &weather,
            "Clear",
            &WeatherUnits::metric(),
            false,
            Some("Jakarta"),
            aqi,
        ));
        bar
    }

    fn jakarta_ascii_bar(aqi: Option<u16>) -> Vec<HudSegment> {
        let mut weather = weather(WeatherCondition::Clear, true);
        weather.temperature = 33.1;
        weather.wind_speed = 9.1 / 3.6;
        weather.precipitation = 0.0;
        weather.humidity = Some(58.4);
        let mut bar = greeting_segments(Some(&pumpkin()), IconMode::Ascii);
        bar.extend(ascii_segments(
            &weather,
            "Clear",
            &WeatherUnits::metric(),
            false,
            Some("Jakarta"),
            aqi,
        ));
        bar
    }

    #[test]
    fn ascii_segments_spell_the_classic_line() {
        let bar = jakarta_ascii_bar(Some(212));
        let line: String = bar[1..].iter().map(|s| s.text.as_str()).collect();
        assert_eq!(
            line,
            "Weather: Clear | Temp: 33.1°C | Wind: 9.1km/h | Precip: 0.0mm | Humidity: 58% \
             | AQI: 212 very unhealthy | Location: Jakarta | Press 'q' to quit, '?' for help"
        );
        assert_eq!(bar.last().map(|s| s.role), Some(HudRole::Hint));
    }

    #[test]
    fn the_ascii_bar_drops_the_quit_hint_first_and_the_greeting_last() {
        let full = jakarta_ascii_bar(Some(212));
        let width = bar_width(&full);
        assert_eq!(fit(&full, width as u16, IconMode::Ascii), full);

        let no_hint = fit(&full, (width - 1) as u16, IconMode::Ascii);
        assert!(!roles_of(&no_hint).contains(&HudRole::Hint));
        assert!(roles_of(&no_hint).contains(&HudRole::Location));
        assert_eq!(no_hint[0].role, HudRole::Greeting);

        let no_location = fit(&full, (bar_width(&no_hint) - 1) as u16, IconMode::Ascii);
        assert!(!roles_of(&no_location).contains(&HudRole::Location));
        assert_eq!(no_location[0].role, HudRole::Greeting);

        let no_wind = fit(&full, (bar_width(&no_location) - 1) as u16, IconMode::Ascii);
        let no_precip = fit(&full, (bar_width(&no_wind) - 1) as u16, IconMode::Ascii);
        assert_eq!(no_precip[0].role, HudRole::Greeting);
        assert!(!roles_of(&no_precip).contains(&HudRole::Precip));

        let minimal = fit(&full, (bar_width(&no_precip) - 1) as u16, IconMode::Ascii);
        assert_eq!(
            roles_of(&minimal),
            [
                HudRole::Condition,
                HudRole::Temp,
                HudRole::Humidity,
                HudRole::Aqi(AqiBand::VeryUnhealthy)
            ]
        );
        assert_eq!(fit(&full, 110, IconMode::Ascii)[0].role, HudRole::Greeting);
    }

    fn roles_of(segments: &[HudSegment]) -> Vec<HudRole> {
        segments
            .iter()
            .map(|s| s.role)
            .filter(|role| *role != HudRole::Separator)
            .collect()
    }

    fn bar_width(segments: &[HudSegment]) -> usize {
        HUD_X as usize
            + segments
                .iter()
                .map(|s| s.text.chars().count())
                .sum::<usize>()
    }

    #[test]
    fn humidity_and_aqi_sit_between_precipitation_and_location() {
        let bar = jakarta_bar(Some(212));
        assert_eq!(
            roles_of(&bar),
            [
                HudRole::Greeting,
                HudRole::Condition,
                HudRole::Temp,
                HudRole::Wind,
                HudRole::Precip,
                HudRole::Humidity,
                HudRole::Aqi(AqiBand::VeryUnhealthy),
                HudRole::Location
            ]
        );
        let texts: Vec<&str> = bar.iter().map(|s| s.text.as_str()).collect();
        assert!(texts.contains(&"\u{e373} 58%"), "{texts:?}");
        assert!(texts.contains(&"\u{f0d43} 212 \u{f157c}"), "{texts:?}");
        let without = jakarta_bar(None);
        assert!(!roles_of(&without).contains(&HudRole::Aqi(AqiBand::Good)));
        assert_eq!(roles_of(&without).len(), 7);
    }

    #[test]
    fn every_band_has_its_face() {
        assert_eq!(aqi_face(AqiBand::Good), '\u{f0c71}');
        assert_eq!(aqi_face(AqiBand::Moderate), '\u{f0c74}');
        assert_eq!(aqi_face(AqiBand::Sensitive), '\u{f0f4c}');
        assert_eq!(aqi_face(AqiBand::Unhealthy), '\u{f1586}');
        assert_eq!(aqi_face(AqiBand::VeryUnhealthy), '\u{f157c}');
        assert_eq!(aqi_face(AqiBand::Hazardous), '\u{f068c}');
    }

    #[test]
    fn new_roles_use_their_colours() {
        let colors = crate::theme::catalogue::default_colors().day;
        assert_eq!(role_color(HudRole::Humidity, &colors), colors.hud_humidity);
        assert_eq!(
            role_color(HudRole::Aqi(AqiBand::Good), &colors),
            colors.aqi[0]
        );
        assert_eq!(
            role_color(HudRole::Aqi(AqiBand::Hazardous), &colors),
            colors.aqi[5]
        );
    }

    #[test]
    fn fit_sheds_decorations_in_order() {
        let full = jakarta_bar(Some(212));
        let w_full = bar_width(&full);
        assert_eq!(fit(&full, w_full as u16, IconMode::Nerd), full);

        let icon = fit(&full, (w_full - 1) as u16, IconMode::Nerd);
        assert_eq!(icon[0].text, "\u{f0bbf}");
        assert!(roles_of(&icon).contains(&HudRole::Location));

        let no_location = fit(&full, (bar_width(&icon) - 1) as u16, IconMode::Nerd);
        assert!(!roles_of(&no_location).contains(&HudRole::Location));
        assert!(roles_of(&no_location).contains(&HudRole::Wind));

        let no_wind = fit(&full, (bar_width(&no_location) - 1) as u16, IconMode::Nerd);
        assert!(!roles_of(&no_wind).contains(&HudRole::Wind));
        assert!(roles_of(&no_wind).contains(&HudRole::Precip));

        let minimal = fit(&full, (bar_width(&no_wind) - 1) as u16, IconMode::Nerd);
        assert_eq!(
            roles_of(&minimal),
            [
                HudRole::Greeting,
                HudRole::Condition,
                HudRole::Temp,
                HudRole::Humidity,
                HudRole::Aqi(AqiBand::VeryUnhealthy)
            ]
        );
        assert_eq!(fit(&full, 10, IconMode::Nerd), minimal);
        assert!(minimal.windows(2).all(
            |pair| !(pair[0].role == HudRole::Separator && pair[1].role == HudRole::Separator)
        ));
    }

    fn rainy_noon(probabilities: &[Option<u8>], codes: &[i32]) -> WeatherData {
        let mut reading = weather(WeatherCondition::PartlyCloudy, true);
        reading.timestamp = "2026-10-21T11:40".to_string();
        reading.outlook = probabilities
            .iter()
            .enumerate()
            .map(|(index, probability)| crate::weather::types::HourPoint {
                time: format!("2026-10-21T{:02}:00", 12 + index),
                probability: *probability,
                code: codes.get(index).copied().unwrap_or(2),
                precipitation: 0.0,
            })
            .collect();
        reading
    }

    fn words(segments: &[HudSegment]) -> Vec<&str> {
        segments
            .iter()
            .filter(|s| s.role != HudRole::Separator)
            .map(|s| s.text.as_str())
            .collect()
    }

    #[test]
    fn feels_like_shows_only_when_it_differs_by_two_degrees() {
        let mut reading = weather(WeatherCondition::Clear, true);
        reading.feels_like = Some(33.4);
        let nerd = nerd_segments(
            &reading,
            "Clear",
            &WeatherUnits::metric(),
            false,
            None,
            None,
        );
        assert!(words(&nerd).contains(&"\u{e350} 28.0°C (feels 33°)"));
        reading.feels_like = Some(29.5);
        let nerd = nerd_segments(
            &reading,
            "Clear",
            &WeatherUnits::metric(),
            false,
            None,
            None,
        );
        assert!(words(&nerd).contains(&"\u{e350} 28.0°C"));
        reading.feels_like = Some(33.4);
        let ascii = ascii_segments(
            &reading,
            "Clear",
            &WeatherUnits::imperial(),
            false,
            None,
            None,
        );
        assert!(
            words(&ascii).contains(&"Temp: 82.4°F (feels 92°)"),
            "{:?}",
            words(&ascii)
        );
    }

    #[test]
    fn rain_ahead_replaces_the_dry_amount() {
        let reading = rainy_noon(&[Some(20), Some(73)], &[]);
        let nerd = nerd_segments(
            &reading,
            "Partly Cloudy",
            &WeatherUnits::metric(),
            false,
            None,
            None,
        );
        assert!(
            words(&nerd).contains(&"\u{e37c} rain ~12:00 · 73%"),
            "{:?}",
            words(&nerd)
        );
        assert!(!roles_of(&nerd).contains(&HudRole::Precip));
        let storm = rainy_noon(&[Some(60), Some(80)], &[80, 95]);
        let nerd = nerd_segments(
            &storm,
            "Partly Cloudy",
            &WeatherUnits::metric(),
            false,
            None,
            None,
        );
        assert!(
            words(&nerd).contains(&"\u{e31d} storms soon · 60%"),
            "{:?}",
            words(&nerd)
        );
        let ascii = ascii_segments(
            &reading,
            "Partly Cloudy",
            &WeatherUnits::metric(),
            false,
            None,
            None,
        );
        assert!(
            words(&ascii).contains(&"Rain ~12:00 · 73%"),
            "{:?}",
            words(&ascii)
        );
    }

    #[test]
    fn a_dry_outlook_hides_the_amount_and_no_outlook_keeps_it() {
        let dry = rainy_noon(&[Some(10), Some(20)], &[]);
        let nerd = nerd_segments(&dry, "Clear", &WeatherUnits::metric(), false, None, None);
        assert!(!roles_of(&nerd).contains(&HudRole::Precip));
        assert!(!roles_of(&nerd).contains(&HudRole::Outlook));
        let unknown = weather(WeatherCondition::Clear, true);
        let nerd = nerd_segments(
            &unknown,
            "Clear",
            &WeatherUnits::metric(),
            false,
            None,
            None,
        );
        assert!(roles_of(&nerd).contains(&HudRole::Precip));
    }

    #[test]
    fn rain_now_says_when_it_eases() {
        let mut wet = rainy_noon(&[Some(90), Some(20)], &[]);
        wet.condition = WeatherCondition::Rain;
        let nerd = nerd_segments(&wet, "Rain", &WeatherUnits::metric(), false, None, None);
        assert!(
            words(&nerd).contains(&"\u{e371} 0.2 mm · until ~12:00"),
            "{:?}",
            words(&nerd)
        );
    }

    #[test]
    fn uv_shows_by_day_from_three_and_leaves_first_when_narrow() {
        let mut reading = weather(WeatherCondition::Clear, true);
        reading.uv_index = Some(7.6);
        let nerd = nerd_segments(
            &reading,
            "Clear",
            &WeatherUnits::metric(),
            false,
            Some("Jakarta"),
            None,
        );
        assert!(roles_of(&nerd).contains(&HudRole::Uv(3)));
        assert!(words(&nerd).contains(&"\u{f17fe} UV 8"));
        let narrow = fit(&nerd, (bar_width(&nerd) - 1) as u16, IconMode::Nerd);
        assert!(
            !roles_of(&narrow)
                .iter()
                .any(|role| matches!(role, HudRole::Uv(_)))
        );
        assert!(roles_of(&narrow).contains(&HudRole::Location));
        reading.uv_index = Some(2.4);
        let low = nerd_segments(
            &reading,
            "Clear",
            &WeatherUnits::metric(),
            false,
            None,
            None,
        );
        assert!(
            !roles_of(&low)
                .iter()
                .any(|role| matches!(role, HudRole::Uv(_)))
        );
        let mut night = weather(WeatherCondition::Clear, false);
        night.uv_index = Some(9.0);
        let dark = nerd_segments(&night, "Clear", &WeatherUnits::metric(), false, None, None);
        assert!(
            !roles_of(&dark)
                .iter()
                .any(|role| matches!(role, HudRole::Uv(_)))
        );
        assert_eq!(uv_tier(5.4), Some(1));
        assert_eq!(uv_tier(5.6), Some(2));
        assert_eq!(uv_tier(11.0), Some(4));
    }
}
