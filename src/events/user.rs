use super::{BUILTIN, Dated, Dates, DayRule, EventDef, Look, Stamp, world};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};
use serde::Deserialize;
use std::path::Path;

pub const FILE_NAME: &str = "events.toml";

const DAY_HOURS: i64 = 6;
const SHOWCASE_HOUR: i64 = 10;
const DEFAULT_ICON: char = '\u{f09d3}';
const ICONS: &[(&str, char)] = &[
    ("cake", '\u{f00eb}'),
    ("gift", '\u{f0e44}'),
    ("heart", '\u{f02d1}'),
    ("party", '\u{f1056}'),
    ("star", '\u{f09d3}'),
    ("flag", '\u{f023b}'),
    ("fire", '\u{f0edd}'),
    ("flower", '\u{f024a}'),
];
pub const LOOKS: &[(&str, &[Look])] = &[
    ("none", &[]),
    ("flag", &[Look::Flag]),
    ("bunting", &[Look::Bunting]),
    ("kites", &[Look::Kites]),
    ("bonfire", &[Look::Bonfire]),
    ("holi", &[Look::Holi]),
    ("koinobori", &[Look::Koinobori]),
    ("songkran", &[Look::Songkran]),
    ("harvest_moon", &[Look::HarvestMoon]),
    ("carnival", &[Look::Carnival, Look::Bunting]),
    ("parol", &[Look::Parol]),
    ("wreath", &[Look::Wreath]),
    ("fireworks", &[Look::Fireworks]),
    ("lanterns", &[Look::Lanterns]),
    ("lights", &[Look::Diwali]),
    ("candles", &[Look::Candles]),
    ("sky_lanterns", &[Look::Waisak]),
    ("ketupat", &[Look::Ketupat]),
    ("eggs", &[Look::Easter]),
    ("torches", &[Look::Torches]),
    ("pumpkins", &[Look::Halloween]),
    ("calm", &[Look::Calm]),
];

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UserEvents {
    pub days: Vec<&'static EventDef>,
    pub moved: Vec<&'static EventDef>,
}

pub static NONE: UserEvents = UserEvents {
    days: Vec::new(),
    moved: Vec::new(),
};

impl UserEvents {
    pub fn moved_for(&self, id: &str) -> Option<&'static EventDef> {
        self.moved.iter().copied().find(|def| def.id == id)
    }

    pub fn is_own(&self, def: &EventDef) -> bool {
        self.days.iter().any(|own| own.id == def.id)
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    #[serde(default, rename = "move")]
    moves: Vec<Move>,
    #[serde(default, rename = "day")]
    days: Vec<Day>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Move {
    event: String,
    year: Option<i32>,
    from: Option<String>,
    to: Option<String>,
    shift: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Day {
    name: String,
    date: String,
    days: Option<i64>,
    greeting: Option<String>,
    look: Option<String>,
    icon: Option<String>,
}

pub fn load(path: &Path) -> (UserEvents, Vec<String>) {
    match std::fs::read_to_string(path) {
        Ok(text) => parse(&text),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            (UserEvents::default(), Vec::new())
        }
        Err(error) => (
            UserEvents::default(),
            vec![format!("{}: {error}", path.display())],
        ),
    }
}

pub fn leak(events: UserEvents) -> &'static UserEvents {
    Box::leak(Box::new(events))
}

pub fn parse(text: &str) -> (UserEvents, Vec<String>) {
    let file: File = match toml::from_str(text) {
        Ok(file) => file,
        Err(error) => {
            return (
                UserEvents::default(),
                vec![format!("{FILE_NAME}: {}", error.message())],
            );
        }
    };
    let mut warnings = Vec::new();
    let mut events = UserEvents::default();
    for entry in &file.days {
        match own_day(entry, &events) {
            Ok(def) => events.days.push(def),
            Err(message) => warnings.push(format!("{FILE_NAME}: day '{}': {message}", entry.name)),
        }
    }
    for entry in &file.moves {
        if let Err(message) = apply_move(entry, &mut events) {
            warnings.push(format!("{FILE_NAME}: move '{}': {message}", entry.event));
        }
    }
    (events, warnings)
}

fn leak_str(text: String) -> &'static str {
    Box::leak(text.into_boxed_str())
}

pub fn slug(name: &str) -> String {
    let mut out = String::new();
    for ch in name.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_')
            && !out.is_empty()
            && (ch.is_whitespace() || ch == '-' || ch == '_')
        {
            out.push('_');
        }
    }
    out.trim_end_matches('_').to_string()
}

fn builtin(id: &str) -> bool {
    BUILTIN
        .iter()
        .chain(world::WORLD.iter())
        .any(|def| def.id == id || def.name() == id)
}

fn looks_of(text: &str) -> Result<&'static [Look], String> {
    let parts: Vec<&str> = text.split('+').map(str::trim).collect();
    let mut looks: Vec<Look> = Vec::new();
    for part in &parts {
        let found = LOOKS.iter().find(|(name, _)| name == part).ok_or_else(|| {
            format!(
                "unknown look '{part}' (one of {})",
                LOOKS
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
        for look in found.1 {
            if !looks.contains(look) {
                looks.push(*look);
            }
        }
    }
    Ok(Box::leak(looks.into_boxed_slice()))
}

fn icon_of(text: Option<&str>) -> Result<char, String> {
    let Some(text) = text.map(str::trim) else {
        return Ok(DEFAULT_ICON);
    };
    if let Some((_, icon)) = ICONS.iter().find(|(name, _)| *name == text) {
        return Ok(*icon);
    }
    let mut chars = text.chars();
    let picked = match (chars.next(), chars.next()) {
        (Some(only), None) => Some(only),
        _ => u32::from_str_radix(text.trim_start_matches("U+").trim_start_matches("0x"), 16)
            .ok()
            .and_then(char::from_u32),
    };
    picked
        .filter(|icon| !icon.is_control() && !icon.is_whitespace())
        .ok_or_else(|| {
            format!(
                "icon '{text}' is not one of {}, a single character or a hex code point",
                ICONS
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

enum When {
    Yearly(u32, u32),
    Once(NaiveDate),
}

fn when_of(text: &str) -> Result<When, String> {
    let text = text.trim();
    if let Ok(date) = NaiveDate::parse_from_str(text, "%Y-%m-%d") {
        return Ok(When::Once(date));
    }
    let (month, day) = text
        .split_once('-')
        .and_then(|(month, day)| Some((month.parse::<u32>().ok()?, day.parse::<u32>().ok()?)))
        .ok_or_else(|| {
            format!("date '{text}' is neither MM-DD (every year) nor YYYY-MM-DD (once)")
        })?;
    NaiveDate::from_ymd_opt(2024, month, day)
        .map(|_| When::Yearly(month, day))
        .ok_or_else(|| format!("date '{text}' is not a calendar day"))
}

fn own_day(entry: &Day, events: &UserEvents) -> Result<&'static EventDef, String> {
    let id = slug(&entry.name);
    if id.is_empty() {
        return Err("the name needs at least one letter or digit".to_string());
    }
    if builtin(&id) || events.days.iter().any(|def| def.id == id) {
        return Err(format!("the id '{id}' is taken; pick another name"));
    }
    let length = entry.days.unwrap_or(1);
    if !(1..=366).contains(&length) {
        return Err(format!("days = {length} must be 1 to 366"));
    }
    let looks = looks_of(entry.look.as_deref().unwrap_or("none"))?;
    let nerd_icon = icon_of(entry.icon.as_deref())?;
    let greeting = entry
        .greeting
        .clone()
        .unwrap_or_else(|| entry.name.trim().to_string());
    let dates = match when_of(&entry.date)? {
        When::Yearly(month, day) => Dates::Rule {
            rule: DayRule::Fixed(month, day),
            days: length,
            eve: false,
            showcase_hour: SHOWCASE_HOUR,
        },
        When::Once(day) => {
            let end = day + Duration::days(length);
            let rows: &'static [Dated] = Box::leak(Box::new([Dated {
                start: stamp_of(day.and_hms_opt(0, 0, 0).unwrap_or_default()),
                end: stamp_of(end.and_hms_opt(0, 0, 0).unwrap_or_default()),
                era: 0,
                tentative: false,
            }]));
            Dates::Listed {
                dates: rows,
                showcase_hours: SHOWCASE_HOUR,
            }
        }
    };
    Ok(Box::leak(Box::new(EventDef {
        id: leak_str(id),
        dates,
        greeting: leak_str(greeting),
        nerd_icon,
        looks,
    })))
}

fn stamp_of(at: NaiveDateTime) -> Stamp {
    Stamp {
        year: at.year(),
        month: at.month(),
        day: at.day(),
        hour: at.hour(),
    }
}

fn at_stamp(stamp: Stamp) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(stamp.year, stamp.month, stamp.day)?.and_hms_opt(stamp.hour, 0, 0)
}

fn day_of(dated: &Dated) -> Option<NaiveDate> {
    at_stamp(dated.start).map(|start| (start + Duration::hours(DAY_HOURS)).date())
}

fn date_arg(text: &str, key: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(text.trim(), "%Y-%m-%d")
        .map_err(|_| format!("{key} = '{text}' is not a YYYY-MM-DD date"))
}

fn apply_move(entry: &Move, events: &mut UserEvents) -> Result<(), String> {
    let name = entry.event.trim();
    let candidates: Vec<&'static EventDef> = BUILTIN
        .iter()
        .chain(world::WORLD.iter())
        .filter(|def| def.id == name || (!name.contains(':') && def.name() == name))
        .collect();
    let original = match candidates.as_slice() {
        [] => return Err("unknown event (weathrz --list-events shows the ids)".to_string()),
        [only] => *only,
        several => {
            return Err(format!(
                "several countries have it; use one of {}",
                several
                    .iter()
                    .map(|def| def.id)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    };
    let current = events.moved_for(original.id).unwrap_or(original);
    let Dates::Listed { dates, .. } = current.dates else {
        return Err(
            "only holidays that follow the moon or an announcement can move; add a [[day]] for anything else"
                .to_string(),
        );
    };
    let picked: Vec<usize> = match (entry.from.as_deref(), entry.year) {
        (Some(_), Some(_)) => return Err("give year or from, not both".to_string()),
        (Some(from), None) => {
            let from = date_arg(from, "from")?
                .and_hms_opt(12, 0, 0)
                .unwrap_or_default();
            dates
                .iter()
                .enumerate()
                .filter(|(_, dated)| {
                    at_stamp(dated.start).is_some_and(|start| start <= from)
                        && at_stamp(dated.end).is_some_and(|end| from < end)
                })
                .map(|(index, _)| index)
                .collect()
        }
        (None, Some(year)) => dates
            .iter()
            .enumerate()
            .filter(|(_, dated)| day_of(dated).is_some_and(|day| day.year() == year))
            .map(|(index, _)| index)
            .collect(),
        (None, None) => {
            return Err("say which one with year = YYYY or from = \"YYYY-MM-DD\"".to_string());
        }
    };
    let index = match picked.as_slice() {
        [only] => *only,
        [] => return Err("no bundled date matches; check weathrz --list-events".to_string()),
        _ => {
            return Err(
                "more than one date matches that year; use from = \"YYYY-MM-DD\"".to_string(),
            );
        }
    };
    let row = &dates[index];
    let shift = match (entry.to.as_deref(), entry.shift) {
        (Some(to), None) => {
            let first = day_of(row).ok_or("the bundled date is broken")?;
            (date_arg(to, "to")? - first).num_days()
        }
        (None, Some(shift)) => shift,
        _ => return Err("give exactly one of to = \"YYYY-MM-DD\" or shift = <days>".to_string()),
    };
    if shift.abs() > 30 {
        return Err(format!("a shift of {shift} days is more than a month"));
    }
    let eid_start = at_stamp(row.start).ok_or("the bundled date is broken")?;
    store(
        events,
        shifted(current, original, index, shift, Span::Whole)?,
    );
    for (linked, span) in linked_to(original.id) {
        let Some(base) = BUILTIN
            .iter()
            .chain(world::WORLD.iter())
            .find(|def| def.id == linked)
        else {
            continue;
        };
        let current = events.moved_for(base.id).unwrap_or(base);
        let Dates::Listed { dates, .. } = current.dates else {
            continue;
        };
        let ends_at_the_eid = |dated: &Dated| {
            at_stamp(dated.end).is_some_and(|end| (end - eid_start).num_hours().abs() <= LINK_HOURS)
        };
        if let Some(index) = dates.iter().position(ends_at_the_eid) {
            store(events, shifted(current, base, index, shift, span)?);
        }
    }
    Ok(())
}

const LINK_HOURS: i64 = 48;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Span {
    Whole,
    End,
}

fn linked_to(id: &str) -> Vec<(String, Span)> {
    if id == super::IDULFITRI {
        return vec![
            (super::RAMADAN.to_string(), Span::End),
            (super::MUDIK.to_string(), Span::Whole),
        ];
    }
    match id.strip_suffix(":eid_al_fitr") {
        Some(country) => vec![(format!("{country}:ramadan"), Span::End)],
        None => Vec::new(),
    }
}

fn store(events: &mut UserEvents, moved: &'static EventDef) {
    events.moved.retain(|def| def.id != moved.id);
    events.moved.push(moved);
}

fn shifted(
    current: &'static EventDef,
    original: &'static EventDef,
    index: usize,
    shift: i64,
    span: Span,
) -> Result<&'static EventDef, String> {
    let Dates::Listed {
        dates,
        showcase_hours,
    } = current.dates
    else {
        return Err("only listed dates can move".to_string());
    };
    let moved_row = |stamp: Stamp| -> Result<Stamp, String> {
        at_stamp(stamp)
            .map(|at| stamp_of(at + Duration::days(shift)))
            .ok_or_else(|| "the bundled date is broken".to_string())
    };
    let mut rows: Vec<Dated> = dates
        .iter()
        .map(|dated| Dated {
            start: dated.start,
            end: dated.end,
            era: dated.era,
            tentative: dated.tentative,
        })
        .collect();
    let row = &dates[index];
    rows[index] = Dated {
        start: match span {
            Span::Whole => moved_row(row.start)?,
            Span::End => row.start,
        },
        end: moved_row(row.end)?,
        era: row.era,
        tentative: false,
    };
    Ok(Box::leak(Box::new(EventDef {
        id: original.id,
        dates: Dates::Listed {
            dates: Box::leak(rows.into_boxed_slice()),
            showcase_hours,
        },
        greeting: original.greeting,
        nerd_icon: original.nerd_icon,
        looks: original.looks,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{self, EventSettings, next_occurrence};

    fn at(text: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap()
    }

    fn settings(user: UserEvents) -> EventSettings {
        EventSettings {
            user: leak(user),
            ..EventSettings::in_country("ID")
        }
    }

    #[test]
    fn a_move_brings_a_holiday_one_day_early() {
        let (user, warnings) = parse(
            r#"
            [[move]]
            event = "id:idulfitri"
            year = 2027
            to = "2027-03-09"
            "#,
        );
        assert_eq!(warnings, Vec::<String>::new());
        let moved = user.moved_for(events::IDULFITRI).unwrap();
        let occurrence = next_occurrence(moved, at("2027-03-01T00:00")).unwrap();
        assert_eq!(occurrence.start, at("2027-03-08T18:00"));
        assert_eq!(occurrence.end, at("2027-03-15T00:00"));
        assert!(!occurrence.tentative);
        let settings = settings(user);
        assert!(events::resolve(at("2027-03-08T20:00"), &settings).contains(events::IDULFITRI));
        assert!(
            !events::resolve(at("2027-03-08T20:00"), &EventSettings::in_country("ID"))
                .contains(events::IDULFITRI)
        );
        let other_year = next_occurrence(moved, at("2026-03-01T00:00")).unwrap();
        assert_eq!(other_year.start, at("2026-03-20T18:00"));
    }

    #[test]
    fn moving_idulfitri_carries_ramadan_s_end_and_the_mudik_week() {
        let (user, warnings) =
            parse("[[move]]\nevent = \"id:idulfitri\"\nyear = 2027\nto = \"2027-03-09\"\n");
        assert_eq!(warnings, Vec::<String>::new());
        let window = |id: &str| {
            let def = user.moved_for(id).unwrap();
            let occurrence = next_occurrence(def, at("2027-03-01T00:00")).unwrap();
            (occurrence.start, occurrence.end)
        };
        assert_eq!(window(events::IDULFITRI).0, at("2027-03-08T18:00"));
        assert_eq!(
            window(events::RAMADAN),
            (at("2027-02-07T18:00"), at("2027-03-08T18:00"))
        );
        assert_eq!(
            window(events::MUDIK),
            (at("2027-03-02T00:00"), at("2027-03-08T18:00"))
        );
        let (malaysia, _) = parse("[[move]]\nevent = \"my:eid_al_fitr\"\nyear = 2027\nshift = 1\n");
        let ramadan = malaysia
            .moved_for("my:ramadan")
            .expect("Malaysia's Ramadan follows");
        let fast = next_occurrence(ramadan, at("2027-03-01T00:00")).unwrap();
        assert_eq!(fast.end, at("2027-03-11T00:00"));
    }

    #[test]
    fn shift_and_from_pick_the_same_row() {
        let (by_from, _) =
            parse("[[move]]\nevent = \"idulfitri\"\nfrom = \"2027-03-11\"\nshift = -1\n");
        let (by_year, _) = parse("[[move]]\nevent = \"id:idulfitri\"\nyear = 2027\nshift = -1\n");
        assert_eq!(by_from, by_year);
    }

    #[test]
    fn bad_moves_warn_and_change_nothing() {
        let (user, warnings) = parse(
            r#"
            [[move]]
            event = "christmas"
            year = 2027
            shift = 1

            [[move]]
            event = "eid_al_fitr"
            year = 2027
            shift = 1

            [[move]]
            event = "id:isra_mikraj"
            year = 2027
            shift = 1

            [[move]]
            event = "id:waisak"
            year = 2027

            [[move]]
            event = "nonsense"
            year = 2027
            shift = 1

            [[move]]
            event = "id:idulfitri"
            year = 2027
            from = "2027-03-10"
            shift = 1
            "#,
        );
        assert!(user.moved.is_empty());
        assert_eq!(warnings.len(), 6, "{warnings:#?}");
        assert!(warnings[5].contains("not both"));
        assert!(warnings[0].contains("only holidays that follow the moon"));
        assert!(warnings[1].contains("several countries"));
        assert!(warnings[2].contains("more than one date"));
        assert!(warnings[3].contains("exactly one of"));
        assert!(warnings[4].contains("unknown event"));
    }

    #[test]
    fn own_days_repeat_yearly_or_happen_once() {
        let (user, warnings) = parse(
            r#"
            [[day]]
            name = "Mum's birthday"
            date = "03-14"
            greeting = "Happy birthday, Mum · {since 1960}"
            look = "candles+bunting"
            icon = "cake"

            [[day]]
            name = "Moving day"
            date = "2027-01-02"
            days = 2
            look = "fireworks"
            "#,
        );
        assert_eq!(warnings, Vec::<String>::new());
        assert_eq!(user.days[0].id, "mums_birthday");
        assert_eq!(user.days[0].looks, &[Look::Candles, Look::Bunting]);
        assert_eq!(user.days[0].nerd_icon, '\u{f00eb}');
        assert_eq!(user.days[1].greeting, "Moving day");
        let settings = settings(user);
        let birthday = events::resolve(at("2027-03-14T09:00"), &settings);
        assert_eq!(
            birthday.greeting().map(|(_, text)| text).as_deref(),
            Some("Happy birthday, Mum · 67")
        );
        assert!(birthday.has(Look::Bunting));
        assert!(events::resolve(at("2028-03-14T09:00"), &settings).contains("mums_birthday"));
        assert!(events::resolve(at("2027-01-03T20:00"), &settings).contains("moving_day"));
        assert!(!events::resolve(at("2028-01-03T20:00"), &settings).contains("moving_day"));
    }

    #[test]
    fn an_own_day_greets_before_the_holidays_it_shares() {
        let (user, _) = parse("[[day]]\nname = \"Anniversary\"\ndate = \"08-17\"\n");
        let settings = settings(user);
        let both = events::resolve(at("2027-08-17T09:00"), &settings);
        assert!(both.contains(events::KEMERDEKAAN));
        assert_eq!(both.greeting().map(|(def, _)| def.id), Some("anniversary"));
        assert!(settings.find("anniversary").is_ok());
        assert!(events::format_listing(at("2027-08-01T00:00"), &settings).contains("anniversary"));
    }

    #[test]
    fn own_days_can_be_disabled_by_id() {
        let (user, _) = parse("[[day]]\nname = \"Mum's birthday\"\ndate = \"03-14\"\n");
        let config = crate::config::EventsConfig {
            disable: vec!["mums_birthday".to_string()],
            ..crate::config::EventsConfig::default()
        };
        let (settings, warnings) = EventSettings::from_config_with(&config, leak(user));
        assert!(warnings.is_empty(), "{warnings:?}");
        assert!(!settings.allows("mums_birthday"));
    }

    #[test]
    fn bad_days_warn() {
        let (user, warnings) = parse(
            r#"
            [[day]]
            name = "christmas"
            date = "12-25"

            [[day]]
            name = "Nope"
            date = "02-30"

            [[day]]
            name = "Odd look"
            date = "01-01"
            look = "confetti"

            [[day]]
            name = "!!!"
            date = "01-01"

            [[day]]
            name = "Ramadan"
            date = "02-01"

            [[day]]
            name = "Escape"
            date = "02-02"
            icon = "1b"
            "#,
        );
        assert!(user.days.is_empty());
        assert_eq!(warnings.len(), 6, "{warnings:#?}");
    }

    #[test]
    fn a_broken_file_is_one_warning() {
        let (user, warnings) = parse("[[day]]\nname = 3\n");
        assert_eq!(user, UserEvents::default());
        assert_eq!(warnings.len(), 1);
        let (_, warnings) = parse("[[party]]\nname = \"x\"\n");
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn a_missing_file_is_silent() {
        let (user, warnings) = load(Path::new("/nonexistent/weathr/events.toml"));
        assert_eq!(user, UserEvents::default());
        assert!(warnings.is_empty());
    }

    #[test]
    fn the_example_file_loads_cleanly_when_uncommented() {
        let example = include_str!("../../events.example.toml");
        let active: Vec<&str> = example
            .lines()
            .map(|line| line.strip_prefix("# ").unwrap_or(line))
            .filter(|body| {
                body.starts_with("[[")
                    || body.split_once(" = ").is_some_and(|(key, _)| {
                        key.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_')
                    })
            })
            .collect();
        let (user, warnings) = parse(&active.join("\n"));
        assert_eq!(warnings, Vec::<String>::new());
        assert_eq!(user.days.len(), 2);
        assert_eq!(user.moved.len(), 3);
    }

    #[test]
    fn slugs_are_plain_ids() {
        assert_eq!(slug("Mum's birthday"), "mums_birthday");
        assert_eq!(slug("  Hari Jadi -- Kota  "), "hari_jadi_kota");
        assert_eq!(slug("Ünï"), "n");
    }
}
