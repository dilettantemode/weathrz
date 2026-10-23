use crate::weather::WeatherCondition;
use crossterm::style::Color;

type Rgb = (u8, u8, u8);

const NIGHT_DEG: f64 = -12.0;
const DAY_DEG: f64 = 20.0;
const STOPS: [(f64, Rgb, Rgb); 4] = [
    (NIGHT_DEG, (10, 14, 30), (22, 28, 52)),
    (-4.0, (16, 20, 46), (58, 38, 74)),
    (6.0, (24, 36, 76), (112, 66, 54)),
    (DAY_DEG, (22, 52, 98), (52, 96, 142)),
];

fn lerp(from: Rgb, to: Rgb, amount: f64) -> Rgb {
    let channel =
        |a: u8, b: u8| (f64::from(a) + (f64::from(b) - f64::from(a)) * amount).round() as u8;
    (
        channel(from.0, to.0),
        channel(from.1, to.1),
        channel(from.2, to.2),
    )
}

fn ends_at(elevation: f64) -> (Rgb, Rgb) {
    let elevation = elevation.clamp(NIGHT_DEG, DAY_DEG);
    let upper = STOPS
        .iter()
        .position(|(deg, _, _)| *deg >= elevation)
        .unwrap_or(STOPS.len() - 1)
        .max(1);
    let (low_deg, low_top, low_horizon) = STOPS[upper - 1];
    let (high_deg, high_top, high_horizon) = STOPS[upper];
    let amount = (elevation - low_deg) / (high_deg - low_deg);
    (
        lerp(low_top, high_top, amount),
        lerp(low_horizon, high_horizon, amount),
    )
}

fn grey_share(condition: WeatherCondition) -> f64 {
    match condition {
        WeatherCondition::Clear => 0.0,
        WeatherCondition::PartlyCloudy => 0.15,
        WeatherCondition::Cloudy => 0.35,
        WeatherCondition::Fog => 0.8,
        _ => 0.55,
    }
}

fn greyed(color: Rgb, share: f64) -> Rgb {
    let level = (0.3 * f64::from(color.0) + 0.59 * f64::from(color.1) + 0.11 * f64::from(color.2))
        .round() as u8;
    lerp(color, (level, level, level), share)
}

pub fn sky_rows(
    height: u16,
    horizon: u16,
    sun_elevation: Option<f64>,
    is_day: bool,
    condition: WeatherCondition,
) -> Vec<Color> {
    let elevation = sun_elevation.unwrap_or(if is_day { DAY_DEG } else { NIGHT_DEG });
    let (top, low) = ends_at(elevation);
    let share = grey_share(condition);
    let span = f64::from(horizon.saturating_sub(1).max(1));
    (0..height)
        .map(|y| {
            if y >= horizon {
                return Color::Reset;
            }
            let depth = f64::from(y) / span;
            let (r, g, b) = greyed(lerp(top, low, depth * depth), share);
            Color::Rgb { r, g, b }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb(color: Color) -> Rgb {
        match color {
            Color::Rgb { r, g, b } => (r, g, b),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn the_sky_stops_at_the_ground_and_lightens_toward_the_horizon() {
        let rows = sky_rows(32, 25, Some(45.0), true, WeatherCondition::Clear);
        assert_eq!(rows.len(), 32);
        assert!(rows[25..].iter().all(|row| *row == Color::Reset));
        let (top, horizon) = (rgb(rows[0]), rgb(rows[24]));
        assert_eq!(top, STOPS[3].1);
        assert_eq!(horizon, STOPS[3].2);
        assert!(horizon.2 > top.2, "bluer and lighter low down");
    }

    #[test]
    fn dusk_glows_warm_and_night_is_dark() {
        let dusk = rgb(sky_rows(32, 25, Some(2.0), false, WeatherCondition::Clear)[24]);
        assert!(dusk.0 > dusk.2, "{dusk:?} is warm at the horizon");
        let night = rgb(sky_rows(32, 25, Some(-30.0), false, WeatherCondition::Clear)[0]);
        assert_eq!(night, STOPS[0].1);
        let fallback = rgb(sky_rows(32, 25, None, false, WeatherCondition::Clear)[0]);
        assert_eq!(fallback, night);
    }

    #[test]
    fn clouds_and_fog_grey_the_sky() {
        let clear = rgb(sky_rows(32, 25, Some(45.0), true, WeatherCondition::Clear)[10]);
        let rain = rgb(sky_rows(32, 25, Some(45.0), true, WeatherCondition::Rain)[10]);
        let spread = |(r, g, b): Rgb| r.max(g).max(b) - r.min(g).min(b);
        assert!(spread(rain) < spread(clear), "{rain:?} vs {clear:?}");
    }
}
