use crate::weather::types::HourPoint;
use chrono::{Duration, NaiveDateTime};

pub const RAIN_LIKELY: u8 = 50;
pub const DRY_LIKELY: u8 = 30;
const WINDOW_HOURS: i64 = 12;
const STORM_CODES: std::ops::RangeInclusive<i32> = 95..=99;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outlook {
    Rain {
        from: Option<NaiveDateTime>,
        probability: u8,
        storm: bool,
    },
    DryFrom(Option<NaiveDateTime>),
    StaysWet,
    StaysDry,
}

fn parse(time: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M").ok()
}

pub fn outlook(now: &str, raining: bool, points: &[HourPoint]) -> Option<Outlook> {
    let now = parse(now)?;
    let ahead: Vec<(NaiveDateTime, &HourPoint)> = points
        .iter()
        .filter_map(|point| Some((parse(&point.time)?, point)))
        .filter(|(end, _)| *end > now && *end <= now + Duration::hours(WINDOW_HOURS + 1))
        .collect();
    if ahead.iter().all(|(_, point)| point.probability.is_none()) {
        return None;
    }
    let starts = |end: NaiveDateTime| {
        let start = end - Duration::hours(1);
        (start > now).then_some(start)
    };
    if raining {
        return Some(
            ahead
                .iter()
                .find(|(_, point)| point.probability.is_some_and(|p| p < DRY_LIKELY))
                .map_or(Outlook::StaysWet, |(end, _)| Outlook::DryFrom(starts(*end))),
        );
    }
    let Some(first) = ahead
        .iter()
        .position(|(_, point)| point.probability.is_some_and(|p| p >= RAIN_LIKELY))
    else {
        return Some(Outlook::StaysDry);
    };
    let (end, point) = ahead[first];
    Some(Outlook::Rain {
        from: starts(end),
        probability: point.probability.unwrap_or(RAIN_LIKELY),
        storm: ahead[first..]
            .iter()
            .any(|(_, point)| STORM_CODES.contains(&point.code)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hours(from: u32, probabilities: &[Option<u8>], codes: &[i32]) -> Vec<HourPoint> {
        probabilities
            .iter()
            .enumerate()
            .map(|(index, probability)| HourPoint {
                time: format!("2026-10-21T{:02}:00", from + index as u32),
                probability: *probability,
                code: codes.get(index).copied().unwrap_or(2),
                precipitation: 0.0,
            })
            .collect()
    }

    fn at(text: &str) -> NaiveDateTime {
        parse(text).unwrap()
    }

    #[test]
    fn a_dry_morning_names_the_first_likely_wet_hour() {
        let points = hours(11, &[Some(5), Some(20), Some(73), Some(85)], &[]);
        assert_eq!(
            outlook("2026-10-21T11:40", false, &points),
            Some(Outlook::Rain {
                from: Some(at("2026-10-21T12:00")),
                probability: 73,
                storm: false
            })
        );
    }

    #[test]
    fn thunder_anywhere_after_the_first_wet_hour_makes_it_storms() {
        let points = hours(12, &[Some(10), Some(60), Some(80)], &[2, 80, 95]);
        assert!(matches!(
            outlook("2026-10-21T11:40", false, &points),
            Some(Outlook::Rain { storm: true, .. })
        ));
        let before = hours(12, &[Some(10), Some(60)], &[95, 80]);
        assert!(matches!(
            outlook("2026-10-21T11:40", false, &before),
            Some(Outlook::Rain { storm: false, .. })
        ));
    }

    #[test]
    fn rain_in_the_current_hour_is_soon() {
        let points = hours(12, &[Some(70)], &[]);
        assert_eq!(
            outlook("2026-10-21T11:40", false, &points),
            Some(Outlook::Rain {
                from: None,
                probability: 70,
                storm: false
            })
        );
    }

    #[test]
    fn rain_says_when_it_stops() {
        let points = hours(12, &[Some(90), Some(60), Some(20)], &[]);
        assert_eq!(
            outlook("2026-10-21T11:10", true, &points),
            Some(Outlook::DryFrom(Some(at("2026-10-21T13:00"))))
        );
        let all_day = hours(12, &[Some(90); 12], &[]);
        assert_eq!(
            outlook("2026-10-21T11:10", true, &all_day),
            Some(Outlook::StaysWet)
        );
    }

    #[test]
    fn nothing_likely_is_dry_and_no_data_is_unknown() {
        let points = hours(12, &[Some(10), Some(40)], &[]);
        assert_eq!(
            outlook("2026-10-21T11:40", false, &points),
            Some(Outlook::StaysDry)
        );
        assert_eq!(outlook("2026-10-21T11:40", false, &[]), None);
        assert_eq!(
            outlook("2026-10-21T11:40", false, &hours(12, &[None, None], &[])),
            None
        );
        assert_eq!(outlook("garbage", false, &points), None);
    }

    #[test]
    fn hours_already_past_or_beyond_the_window_are_ignored() {
        let mut points = hours(9, &[Some(99), Some(99), Some(10)], &[]);
        points.push(HourPoint {
            time: "2026-10-22T09:00".to_string(),
            probability: Some(99),
            code: 2,
            precipitation: 0.0,
        });
        assert_eq!(outlook("2026-10-21T11:40", false, &points), None);
    }
}
