use super::{Dated, EventDef, Occurrence, Stamp};
use crate::astro::{self, Phase};
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

const ACEH: (f64, f64) = (5.55, 95.32);
const WIB: i64 = 7;
const WITA: i64 = 8;
const BEIJING: i64 = 8;
const MABIMS_ALTITUDE: f64 = 3.0;
const MABIMS_ELONGATION: f64 = 6.4;
const MUHARRAM_1448: (i32, u32, u32) = (2026, 6, 15);
const HIJRI_YEAR_1448: i64 = 1448;
const WAISAK_TARGET: (u32, u32) = (5, 21);
const NYEPI_EARLIEST_MARCH_DAY: u32 = 3;
pub const YEARS_AHEAD: i32 = 20;
const FIT_HOURS: i64 = 24;
const ANCHOR_REACH_DAYS: i64 = 40;
const EVE_HOURS: i64 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lunar {
    Hijri { month: u32, day: u32 },
    LastFridayOfRamadan,
    Chinese { month: u32, day: u32 },
    Nyepi,
    Waisak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    pub start: Lunar,
    pub end: Lunar,
}

const fn same(day: Lunar) -> Rule {
    Rule {
        start: day,
        end: day,
    }
}

const fn hijri(month: u32, day: u32) -> Lunar {
    Lunar::Hijri { month, day }
}

pub fn rule_for(def: &EventDef) -> Option<Rule> {
    Some(match def.name() {
        "idulfitri" | "eid_al_fitr" => same(hijri(10, 1)),
        "iduladha" | "eid_al_adha" => same(hijri(12, 10)),
        "isra_mikraj" => same(hijri(7, 27)),
        "muharam" | "islamic_new_year" => same(hijri(1, 1)),
        "ashura" => same(hijri(1, 10)),
        "maulid" | "mawlid" => same(hijri(3, 12)),
        "shab_e_barat" => same(hijri(8, 15)),
        "shab_e_qadr" => same(hijri(9, 27)),
        "jumatul_bida" => same(Lunar::LastFridayOfRamadan),
        "ramadan" => Rule {
            start: hijri(9, 1),
            end: hijri(10, 1),
        },
        "mudik" => same(hijri(10, 1)),
        "imlek" | "spring_festival" | "chinese_new_year" => {
            same(Lunar::Chinese { month: 1, day: 1 })
        }
        "mid_autumn" => same(Lunar::Chinese { month: 8, day: 15 }),
        "dragon_boat" => same(Lunar::Chinese { month: 5, day: 5 }),
        "nyepi" => same(Lunar::Nyepi),
        "waisak" | "vesak" => same(Lunar::Waisak),
        _ => return None,
    })
}

fn utc_of(date: NaiveDate, hour: u32, zone: i64) -> DateTime<Utc> {
    Utc.from_utc_datetime(&date.and_hms_opt(hour, 0, 0).unwrap_or_default()) - Duration::hours(zone)
}

fn local_date(at: DateTime<Utc>, zone: i64) -> NaiveDate {
    (at + Duration::hours(zone)).date_naive()
}

fn new_moons(from: NaiveDate, to: NaiveDate) -> Vec<(i64, DateTime<Utc>)> {
    let first = astro::lunation_near(utc_of(from, 0, 0)) - 2;
    let last = astro::lunation_near(utc_of(to, 0, 0)) + 2;
    (first..=last)
        .map(|k| (k, astro::lunation(k, Phase::New)))
        .collect()
}

fn hijri_month_start(k: i64) -> NaiveDate {
    let conjunction = astro::lunation(k, Phase::New);
    let day = local_date(conjunction, WIB);
    let visible = astro::sunset_after(ACEH.0, ACEH.1, utc_of(day, 12, WIB)).is_some_and(|sunset| {
        conjunction < sunset
            && astro::moon_altitude(ACEH.0, ACEH.1, sunset) >= MABIMS_ALTITUDE
            && astro::moon_sun_separation(sunset) >= MABIMS_ELONGATION
    });
    day + Duration::days(if visible { 1 } else { 2 })
}

fn hijri_month_of(k: i64) -> (i64, u32) {
    let (year, month, day) = MUHARRAM_1448;
    let anchor = astro::lunation_near(utc_of(
        NaiveDate::from_ymd_opt(year, month, day).unwrap_or_default(),
        0,
        0,
    ));
    let index = HIJRI_YEAR_1448 * 12 + (k - anchor);
    (index.div_euclid(12), index.rem_euclid(12) as u32 + 1)
}

fn solstice_date(year: i32) -> Option<NaiveDate> {
    let around = NaiveDate::from_ymd_opt(year, 12, 21)?;
    Some(local_date(
        astro::sun_reaches(270.0, utc_of(around, 0, 0)),
        BEIJING,
    ))
}

fn principal_terms(after: NaiveDate) -> Vec<NaiveDate> {
    (1..=12)
        .map(|step| {
            let longitude = (270.0 + 30.0 * f64::from(step)).rem_euclid(360.0);
            let around = after + Duration::days((f64::from(step) * 30.44).round() as i64);
            local_date(astro::sun_reaches(longitude, utc_of(around, 0, 0)), BEIJING)
        })
        .collect()
}

fn chinese_months(year: i32) -> Option<Vec<(u32, bool, NaiveDate)>> {
    let first_solstice = solstice_date(year - 1)?;
    let last_solstice = solstice_date(year)?;
    let starts: Vec<NaiveDate> = new_moons(
        first_solstice - Duration::days(35),
        last_solstice + Duration::days(35),
    )
    .into_iter()
    .map(|(_, at)| local_date(at, BEIJING))
    .collect();
    let eleventh =
        |solstice: NaiveDate| starts.iter().copied().filter(|day| *day <= solstice).max();
    let (from, to) = (eleventh(first_solstice)?, eleventh(last_solstice)?);
    let months: Vec<(NaiveDate, NaiveDate)> = starts
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .filter(|(start, _)| *start >= from && *start < to)
        .collect();
    let terms = principal_terms(first_solstice);
    let leap = (months.len() == 13)
        .then(|| {
            months
                .iter()
                .skip(1)
                .position(|(start, next)| !terms.iter().any(|term| term >= start && term < next))
        })
        .flatten()
        .map(|index| index + 1);
    let mut number = 10;
    Some(
        months
            .iter()
            .enumerate()
            .map(|(index, (start, _))| {
                let is_leap = leap == Some(index);
                if !is_leap {
                    number = number % 12 + 1;
                }
                (number, is_leap, *start)
            })
            .collect(),
    )
}

fn chinese_day(month: u32, day: u32, year: i32) -> Option<NaiveDate> {
    let start = chinese_months(year)?
        .into_iter()
        .find(|(number, leap, _)| *number == month && !leap)?
        .2;
    Some(start + Duration::days(i64::from(day) - 1))
}

fn nyepi(year: i32) -> Option<NaiveDate> {
    let earliest = NaiveDate::from_ymd_opt(year, 3, NYEPI_EARLIEST_MARCH_DAY)?;
    new_moons(earliest, earliest + Duration::days(31))
        .into_iter()
        .map(|(_, at)| local_date(at, WITA))
        .find(|date| *date >= earliest)
}

fn waisak(year: i32) -> Option<NaiveDate> {
    let target = NaiveDate::from_ymd_opt(year, WAISAK_TARGET.0, WAISAK_TARGET.1)?;
    let first = astro::lunation_near(utc_of(target, 0, 0)) - 2;
    (first..first + 5)
        .map(|k| local_date(astro::lunation(k, Phase::Full), WIB))
        .min_by_key(|date| (*date - target).num_days().abs())
}

fn hijri_days(month: u32, day: u32, from: NaiveDate, to: NaiveDate) -> Vec<(NaiveDate, i64)> {
    new_moons(from - Duration::days(40), to)
        .into_iter()
        .filter_map(|(k, _)| {
            let (year, number) = hijri_month_of(k);
            (number == month).then(|| {
                (
                    hijri_month_start(k) + Duration::days(i64::from(day) - 1),
                    year,
                )
            })
        })
        .filter(|(date, _)| *date >= from && *date <= to)
        .collect()
}

fn last_friday_of_ramadan(from: NaiveDate, to: NaiveDate) -> Vec<(NaiveDate, i64)> {
    hijri_days(10, 1, from, to + Duration::days(31))
        .into_iter()
        .map(|(eid, year)| {
            let mut day = eid - Duration::days(1);
            while day.weekday() != chrono::Weekday::Fri {
                day -= Duration::days(1);
            }
            (day, year)
        })
        .filter(|(date, _)| *date >= from && *date <= to)
        .collect()
}

pub fn days(lunar: Lunar, from: NaiveDate, to: NaiveDate) -> Vec<(NaiveDate, i64)> {
    let mut out = match lunar {
        Lunar::Hijri { month, day } => hijri_days(month, day, from, to),
        Lunar::LastFridayOfRamadan => last_friday_of_ramadan(from, to),
        _ => (from.year()..=to.year())
            .filter_map(|year| {
                let date = match lunar {
                    Lunar::Chinese { month, day } => chinese_day(month, day, year),
                    Lunar::Nyepi => nyepi(year),
                    Lunar::Waisak => waisak(year),
                    _ => None,
                }?;
                Some((date, 0))
            })
            .filter(|(date, _)| *date >= from && *date <= to)
            .collect(),
    };
    out.sort();
    out.dedup();
    out
}

fn at_stamp(stamp: Stamp) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(stamp.year, stamp.month, stamp.day)?.and_hms_opt(stamp.hour, 0, 0)
}

fn nearest(lunar: Lunar, moment: NaiveDateTime) -> Option<NaiveDate> {
    let around = (moment + Duration::hours(EVE_HOURS)).date();
    days(
        lunar,
        around - Duration::days(ANCHOR_REACH_DAYS),
        around + Duration::days(ANCHOR_REACH_DAYS),
    )
    .into_iter()
    .map(|(date, _)| date)
    .min_by_key(|date| (*date - around).num_days().abs())
}

fn midnight(date: NaiveDate) -> NaiveDateTime {
    date.and_hms_opt(0, 0, 0).unwrap_or_default()
}

#[derive(Debug, Clone, Copy)]
struct Shape {
    start: Duration,
    end: Duration,
}

fn shape_of(rule: Rule, row: &Dated) -> Option<Shape> {
    let start = at_stamp(row.start)?;
    let end = at_stamp(row.end)?;
    Some(Shape {
        start: start - midnight(nearest(rule.start, start)?),
        end: end - midnight(nearest(rule.end, end)?),
    })
}

fn fits(rule: Rule, rows: &[Dated], shape: Shape) -> bool {
    rows.iter().all(|row| {
        at_stamp(row.start).is_some_and(|start| {
            nearest(rule.start, start).is_some_and(|day| {
                (midnight(day) + shape.start - start).num_hours().abs() <= FIT_HOURS
            })
        })
    })
}

fn calculate(def: &EventDef, rows: &[Dated], showcase_hours: i64) -> Vec<Occurrence> {
    let Some(rule) = rule_for(def) else {
        return Vec::new();
    };
    let Some(last) = rows
        .iter()
        .max_by_key(|row| (row.start.year, row.start.month, row.start.day))
    else {
        return Vec::new();
    };
    let Some(shape) = shape_of(rule, last) else {
        return Vec::new();
    };
    if !fits(rule, rows, shape) {
        return Vec::new();
    }
    let (Some(last_start), Some(last_end)) = (at_stamp(last.start), at_stamp(last.end)) else {
        return Vec::new();
    };
    let from = last_start.date();
    let to = NaiveDate::from_ymd_opt(last.start.year + YEARS_AHEAD, 12, 31).unwrap_or(from);
    days(rule.start, from, to)
        .into_iter()
        .filter_map(|(day, hijri_year)| {
            let start = midnight(day) + shape.start;
            let end_day = if rule.end == rule.start {
                day
            } else {
                days(rule.end, day, day + Duration::days(ANCHOR_REACH_DAYS))
                    .first()
                    .map(|(date, _)| *date)?
            };
            let end = midnight(end_day) + shape.end;
            let era = match (last.era, rule.start) {
                (0, _) => 0,
                (_, Lunar::Hijri { .. } | Lunar::LastFridayOfRamadan) => hijri_year as u32,
                (era, _) => (era as i32 + (day.year() - last_start.year())) as u32,
            };
            (start > last_start && start > last_end - Duration::days(1)).then_some(Occurrence {
                start,
                end,
                showcase: start + Duration::hours(showcase_hours),
                era,
                tentative: true,
            })
        })
        .collect()
}

type Key = (&'static str, usize, i64);

pub fn calculated(def: &EventDef, rows: &'static [Dated], showcase_hours: i64) -> Vec<Occurrence> {
    static MEMO: OnceLock<Mutex<HashMap<Key, Vec<Occurrence>>>> = OnceLock::new();
    let key = (def.id, rows.as_ptr() as usize, showcase_hours);
    let memo = MEMO.get_or_init(Default::default);
    if let Some(found) = memo.lock().ok().and_then(|memo| memo.get(&key).cloned()) {
        return found;
    }
    let found = calculate(def, rows, showcase_hours);
    if let Ok(mut memo) = memo.lock() {
        memo.insert(key, found.clone());
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{self, Dates};

    fn day(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn first(lunar: Lunar, year: i32) -> NaiveDate {
        days(
            lunar,
            day(&format!("{year}-01-01")),
            day(&format!("{year}-12-31")),
        )[0]
        .0
    }

    #[test]
    fn chinese_dates_match_the_hong_kong_observatory_through_the_leap_months() {
        for (year, new_year, dragon_boat, mid_autumn) in [
            (2026, "2026-02-17", "2026-06-19", "2026-09-25"),
            (2028, "2028-01-26", "2028-05-28", "2028-10-03"),
            (2029, "2029-02-13", "2029-06-16", "2029-09-22"),
            (2030, "2030-02-03", "2030-06-05", "2030-09-12"),
            (2031, "2031-01-23", "2031-06-24", "2031-10-01"),
            (2033, "2033-01-31", "2033-06-01", "2033-09-08"),
            (2034, "2034-02-19", "2034-06-20", "2034-09-27"),
            (2036, "2036-01-28", "2036-05-30", "2036-10-04"),
            (2039, "2039-01-24", "2039-05-27", "2039-10-02"),
            (2040, "2040-02-12", "2040-06-14", "2040-09-20"),
        ] {
            assert_eq!(
                first(Lunar::Chinese { month: 1, day: 1 }, year),
                day(new_year),
                "{year}"
            );
            assert_eq!(
                first(Lunar::Chinese { month: 5, day: 5 }, year),
                day(dragon_boat),
                "{year}"
            );
            assert_eq!(
                first(Lunar::Chinese { month: 8, day: 15 }, year),
                day(mid_autumn),
                "{year}"
            );
        }
        for (year, expected) in [
            (2020, "2020-01-25"),
            (2023, "2023-01-22"),
            (2025, "2025-01-29"),
        ] {
            assert_eq!(
                first(Lunar::Chinese { month: 1, day: 1 }, year),
                day(expected),
                "{year}"
            );
        }
    }

    #[test]
    fn indonesia_s_islamic_dates_come_from_neo_mabims_at_banda_aceh() {
        for (lunar, expected) in [
            (hijri(9, 1), "2026-02-19"),
            (hijri(10, 1), "2026-03-21"),
            (hijri(12, 10), "2026-05-27"),
            (hijri(1, 1), "2026-06-16"),
            (hijri(3, 12), "2026-08-25"),
            (hijri(9, 1), "2027-02-08"),
            (hijri(10, 1), "2027-03-10"),
            (hijri(12, 10), "2027-05-17"),
            (hijri(1, 1), "2027-06-06"),
            (hijri(3, 12), "2027-08-15"),
        ] {
            let around = day(expected);
            let found = days(
                lunar,
                around - Duration::days(20),
                around + Duration::days(20),
            );
            assert_eq!(
                found.first().map(|(date, _)| *date),
                Some(around),
                "{lunar:?}"
            );
        }
        let fitri = days(hijri(10, 1), day("2027-01-01"), day("2027-12-31"));
        assert_eq!(fitri, vec![(day("2027-03-10"), 1448)]);
    }

    #[test]
    fn nyepi_and_waisak_follow_the_moon_within_a_day() {
        for (year, expected) in [
            (2020, "2020-03-25"),
            (2021, "2021-03-14"),
            (2022, "2022-03-03"),
            (2023, "2023-03-22"),
            (2024, "2024-03-11"),
            (2025, "2025-03-29"),
            (2026, "2026-03-19"),
            (2027, "2027-03-08"),
            (2014, "2014-03-31"),
            (2033, "2033-03-31"),
        ] {
            let found = first(Lunar::Nyepi, year);
            assert!(
                (found - day(expected)).num_days().abs() <= 1,
                "{year}: {found}"
            );
        }
        for (year, expected) in [
            (2020, "2020-05-07"),
            (2021, "2021-05-26"),
            (2022, "2022-05-16"),
            (2023, "2023-06-04"),
            (2024, "2024-05-23"),
            (2025, "2025-05-12"),
            (2026, "2026-05-31"),
            (2027, "2027-05-20"),
        ] {
            let found = first(Lunar::Waisak, year);
            assert!(
                (found - day(expected)).num_days().abs() <= 1,
                "{year}: {found}"
            );
        }
    }

    fn rows(def: &EventDef) -> &'static [Dated] {
        match def.dates {
            Dates::Listed { dates, .. } => dates,
            _ => &[],
        }
    }

    #[test]
    fn every_indonesian_moon_day_carries_on_past_the_table() {
        for id in [
            events::IDULFITRI,
            events::IDULADHA,
            events::ISRA_MIKRAJ,
            events::MUHARAM,
            events::MAULID,
            events::RAMADAN,
            events::MUDIK,
            events::IMLEK,
            events::NYEPI,
            events::WAISAK,
        ] {
            let def = events::find(id).unwrap();
            let later = calculated(def, rows(def), 0);
            assert!(later.len() >= 10, "{id}: {}", later.len());
            assert!(later.iter().all(|occurrence| occurrence.tentative));
            assert!(
                later.windows(2).all(|pair| pair[0].start < pair[1].start),
                "{id}"
            );
        }
        let fitri = calculated(
            events::find(events::IDULFITRI).unwrap(),
            rows(events::find(events::IDULFITRI).unwrap()),
            0,
        );
        assert_eq!(fitri[0].era, 1450);
        let ramadan = calculated(
            events::find(events::RAMADAN).unwrap(),
            rows(events::find(events::RAMADAN).unwrap()),
            0,
        );
        assert_eq!(
            ramadan[0].end, fitri[0].start,
            "Ramadan ends on the takbiran eve"
        );
    }

    #[test]
    fn days_that_no_rule_fits_stay_table_only() {
        let diwali = events::find("in:diwali").unwrap();
        assert!(calculated(diwali, rows(diwali), 0).is_empty());
        let indian_vesak = events::find("in:vesak").unwrap();
        assert!(
            calculated(indian_vesak, rows(indian_vesak), 0).is_empty(),
            "India's Buddha Purnima falls a month away from Indonesia's Waisak rule in 2026"
        );
    }
}
