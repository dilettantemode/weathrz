use chrono::{
    Datelike, Duration, Local, NaiveDate, NaiveDateTime, Offset, TimeZone, Timelike, Weekday,
};

pub mod country;
mod dates;
pub mod lunar;
pub mod user;
mod world;

pub use country::CountrySetting;
pub use dates::{LAST_YEAR, SOURCES};

pub const HALLOWEEN: &str = "halloween";
pub const CHRISTMAS_LIGHTS: &str = "christmas_lights";
pub const CHRISTMAS: &str = "christmas";
pub const CHRISTMAS_SEASON: &str = "christmas_season";
pub const LIGHTS_TO_JANUARY_7: &str = "lights_to_january_7";
pub const NEW_YEAR_LIGHTS: &str = "new_year_lights";
pub const NEW_YEAR: &str = "new_year";
pub const ISRA_MIKRAJ: &str = "id:isra_mikraj";
pub const IMLEK: &str = "id:imlek";
pub const RAMADAN: &str = "id:ramadan";
pub const NYEPI: &str = "id:nyepi";
pub const MUDIK: &str = "id:mudik";
pub const IDULFITRI: &str = "id:idulfitri";
pub const JUMAT_AGUNG: &str = "id:jumat_agung";
pub const PASKAH: &str = "id:paskah";
pub const KENAIKAN: &str = "id:kenaikan";
pub const IDULADHA: &str = "id:iduladha";
pub const WAISAK: &str = "id:waisak";
pub const MUHARAM: &str = "id:muharam";
pub const MAULID: &str = "id:maulid";
pub const KEMERDEKAAN: &str = "id:kemerdekaan";
pub const AGUSTUSAN: &str = "id:agustusan";
pub const PANCASILA: &str = "id:pancasila";
pub const HARI_BURUH: &str = "id:hari_buruh";

const EVE_HOURS: i64 = 6;
const FIRST_DAYS: i64 = 3;
const SUNSET_STAND_IN_HOUR: u32 = 18;
const GREETING_TURN_SECS: i64 = 10;
const SUNSET_REACH_HOURS: i64 = 6;
const STAR_AND_CRESCENT: char = '\u{f0979}';
const RAMADAN_CRESCENT: char = '\u{f0f67}';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moment {
    pub month: u32,
    pub day: u32,
    pub hour: u32,
}

const fn moment(month: u32, day: u32, hour: u32) -> Moment {
    Moment { month, day, hour }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stamp {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dated {
    pub start: Stamp,
    pub end: Stamp,
    pub era: u32,
    pub tentative: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Spring,
    Autumn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayRule {
    Fixed(u32, u32),
    Nth(u32, Weekday, u8),
    Last(u32, Weekday),
    Easter(i64),
    Orthodox(i64),
    Equinox(Season),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Dates {
    Yearly {
        start: Moment,
        end: Moment,
        showcase: Moment,
    },
    Rule {
        rule: DayRule,
        days: i64,
        eve: bool,
        showcase_hour: i64,
    },
    Listed {
        dates: &'static [Dated],
        showcase_hours: i64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Look {
    Halloween,
    ChristmasLights,
    Christmas,
    NewYear,
    LunarNewYear,
    Lanterns,
    Ramadan,
    Eid,
    Ketupat,
    Takbiran,
    Torches,
    Calm,
    Mudik,
    Waisak,
    Easter,
    Diwali,
    Fireworks,
    Candles,
    Flag,
    Bunting,
    Kites,
    Bonfire,
    Holi,
    Koinobori,
    HalfMast,
    Songkran,
    HarvestMoon,
    Carnival,
    Parol,
    Wreath,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EventDef {
    pub id: &'static str,
    pub dates: Dates,
    pub greeting: &'static str,
    pub nerd_icon: char,
    pub looks: &'static [Look],
}

const LUNAR_EVE_HOUR: u32 = 18;
const LUNAR_EVE_REACH_DAYS: i64 = 40;

const NOT_KEPT: &[(&str, &[&str])] = &[
    (CHRISTMAS_LIGHTS, &["EG", "ET", "PH", "RU", "TR"]),
    (CHRISTMAS, &["EG", "ET", "RU", "TR"]),
    (NEW_YEAR, &["ET"]),
];

const ONLY_IN: &[(&str, &[&str])] = &[
    (CHRISTMAS_SEASON, &["PH"]),
    (LIGHTS_TO_JANUARY_7, &["EG", "RU"]),
    (NEW_YEAR_LIGHTS, &["TR"]),
];

impl EventDef {
    pub fn country(&self) -> Option<&'static str> {
        self.id.split_once(':').map(|(country, _)| country)
    }

    pub fn name(&self) -> &'static str {
        self.id.split_once(':').map_or(self.id, |(_, name)| name)
    }

    pub fn belongs_to(&self, country: Option<&str>) -> bool {
        let only_in = ONLY_IN
            .iter()
            .find(|(id, _)| *id == self.id)
            .map(|(_, countries)| *countries);
        match (self.country(), country) {
            (None, Some(wanted)) => match only_in {
                Some(countries) => countries.contains(&wanted),
                None => !NOT_KEPT
                    .iter()
                    .any(|(id, countries)| *id == self.id && countries.contains(&wanted)),
            },
            (None, None) => only_in.is_none(),
            (Some(own), Some(wanted)) => own.eq_ignore_ascii_case(wanted),
            (Some(_), None) => false,
        }
    }
}

const fn rule(rule: DayRule, days: i64, eve: bool, showcase_hour: i64) -> Dates {
    Dates::Rule {
        rule,
        days,
        eve,
        showcase_hour,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occurrence {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub showcase: NaiveDateTime,
    pub era: u32,
    pub tentative: bool,
}

const fn listed(dates: &'static [Dated], showcase_hours: i64) -> Dates {
    Dates::Listed {
        dates,
        showcase_hours,
    }
}

pub const BUILTIN: &[EventDef] = &[
    EventDef {
        id: NYEPI,
        dates: listed(dates::NYEPI, 15),
        greeting: "Day of Silence · Saka New Year {era}",
        nerd_icon: '\u{f0594}',
        looks: &[Look::Calm],
    },
    EventDef {
        id: IDULFITRI,
        dates: listed(dates::IDULFITRI, 3),
        greeting: "Happy Eid al-Fitr {era} AH",
        nerd_icon: '\u{f0979}',
        looks: &[Look::Ketupat, Look::Takbiran],
    },
    EventDef {
        id: IDULADHA,
        dates: listed(dates::IDULADHA, 16),
        greeting: "Happy Eid al-Adha {era} AH",
        nerd_icon: '\u{f0979}',
        looks: &[Look::Ketupat],
    },
    EventDef {
        id: ISRA_MIKRAJ,
        dates: listed(dates::ISRA_MIKRAJ, 3),
        greeting: "Isra and Mi'raj",
        nerd_icon: '\u{f0979}',
        looks: &[],
    },
    EventDef {
        id: MUHARAM,
        dates: listed(dates::MUHARAM, 3),
        greeting: "Happy Islamic New Year {era} AH",
        nerd_icon: '\u{f0979}',
        looks: &[Look::Torches],
    },
    EventDef {
        id: MAULID,
        dates: listed(dates::MAULID, 10),
        greeting: "Mawlid of the Prophet",
        nerd_icon: '\u{f0979}',
        looks: &[],
    },
    EventDef {
        id: WAISAK,
        dates: listed(dates::WAISAK, 21),
        greeting: "Happy Vesak {era} BE",
        nerd_icon: '\u{f05e2}',
        looks: &[Look::Waisak],
    },
    EventDef {
        id: JUMAT_AGUNG,
        dates: rule(DayRule::Easter(-2), 1, false, 10),
        greeting: "Good Friday",
        nerd_icon: '\u{f0144}',
        looks: &[],
    },
    EventDef {
        id: PASKAH,
        dates: rule(DayRule::Easter(0), 1, false, 10),
        greeting: "Happy Easter",
        nerd_icon: '\u{f0ab0}',
        looks: &[Look::Easter],
    },
    EventDef {
        id: KENAIKAN,
        dates: rule(DayRule::Easter(39), 1, false, 10),
        greeting: "Ascension Day",
        nerd_icon: '\u{f0144}',
        looks: &[],
    },
    EventDef {
        id: KEMERDEKAAN,
        dates: rule(DayRule::Fixed(8, 17), 1, false, 10),
        greeting: "Happy {nth since 1945} Independence Day, Indonesia",
        nerd_icon: '\u{f023b}',
        looks: &[Look::Flag, Look::Bunting, Look::Kites],
    },
    EventDef {
        id: PANCASILA,
        dates: rule(DayRule::Fixed(6, 1), 1, false, 10),
        greeting: "Pancasila Day",
        nerd_icon: '\u{f023b}',
        looks: &[Look::Flag],
    },
    EventDef {
        id: HARI_BURUH,
        dates: rule(DayRule::Fixed(5, 1), 1, false, 10),
        greeting: "Happy Labour Day",
        nerd_icon: '\u{f1323}',
        looks: &[],
    },
    EventDef {
        id: IMLEK,
        dates: listed(dates::IMLEK, 5),
        greeting: "Happy Chinese New Year {era} · Year of the {animal}",
        nerd_icon: '\u{eef8}',
        looks: &[Look::LunarNewYear],
    },
    EventDef {
        id: RAMADAN,
        dates: listed(dates::RAMADAN, 2),
        greeting: "Ramadan Mubarak",
        nerd_icon: '\u{f0f67}',
        looks: &[Look::Ramadan],
    },
    EventDef {
        id: MUDIK,
        dates: listed(dates::MUDIK, 106),
        greeting: "",
        nerd_icon: '\u{f010b}',
        looks: &[Look::Mudik],
    },
    EventDef {
        id: AGUSTUSAN,
        dates: Dates::Yearly {
            start: moment(8, 1, 0),
            end: moment(9, 1, 0),
            showcase: moment(8, 10, 10),
        },
        greeting: "",
        nerd_icon: '\u{f023b}',
        looks: &[Look::Flag, Look::Bunting],
    },
    EventDef {
        id: HALLOWEEN,
        dates: Dates::Yearly {
            start: moment(10, 31, 0),
            end: moment(11, 1, 6),
            showcase: moment(10, 31, 21),
        },
        greeting: "Happy Halloween",
        nerd_icon: '\u{f0bbf}',
        looks: &[Look::Halloween],
    },
    EventDef {
        id: CHRISTMAS_LIGHTS,
        dates: Dates::Yearly {
            start: moment(12, 15, 0),
            end: moment(12, 27, 0),
            showcase: moment(12, 20, 21),
        },
        greeting: "",
        nerd_icon: '\u{f0405}',
        looks: &[Look::ChristmasLights],
    },
    EventDef {
        id: CHRISTMAS_SEASON,
        dates: Dates::Yearly {
            start: moment(12, 1, 0),
            end: moment(1, 7, 0),
            showcase: moment(12, 20, 21),
        },
        greeting: "",
        nerd_icon: '\u{f0405}',
        looks: &[Look::ChristmasLights, Look::Parol],
    },
    EventDef {
        id: LIGHTS_TO_JANUARY_7,
        dates: Dates::Yearly {
            start: moment(12, 15, 0),
            end: moment(1, 8, 0),
            showcase: moment(1, 6, 21),
        },
        greeting: "",
        nerd_icon: '\u{f0405}',
        looks: &[Look::ChristmasLights],
    },
    EventDef {
        id: NEW_YEAR_LIGHTS,
        dates: Dates::Yearly {
            start: moment(12, 15, 0),
            end: moment(1, 1, 6),
            showcase: moment(12, 30, 21),
        },
        greeting: "",
        nerd_icon: '\u{f0405}',
        looks: &[Look::ChristmasLights],
    },
    EventDef {
        id: CHRISTMAS,
        dates: Dates::Yearly {
            start: moment(12, 24, 0),
            end: moment(12, 27, 0),
            showcase: moment(12, 24, 21),
        },
        greeting: "Merry Christmas",
        nerd_icon: '\u{f0405}',
        looks: &[Look::Christmas],
    },
    EventDef {
        id: NEW_YEAR,
        dates: Dates::Yearly {
            start: moment(12, 31, 20),
            end: moment(1, 2, 0),
            showcase: moment(12, 31, 23),
        },
        greeting: "Happy New Year",
        nerd_icon: '\u{f0e30}',
        looks: &[Look::NewYear],
    },
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Place {
    pub latitude: f64,
    pub longitude: f64,
    pub utc_offset_secs: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventSettings {
    pub enabled: bool,
    pub disabled: Vec<&'static str>,
    pub country_setting: CountrySetting,
    pub country: Option<&'static str>,
    pub user: &'static user::UserEvents,
    pub place: Option<Place>,
    pub named: Option<NamedPlace>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NamedPlace {
    pub latitude: f64,
    pub longitude: f64,
    pub country: Option<&'static str>,
}

impl Default for EventSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            disabled: Vec::new(),
            country_setting: CountrySetting::Auto,
            country: None,
            user: &user::NONE,
            place: None,
            named: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActiveEvents {
    events: Vec<&'static EventDef>,
    now: NaiveDateTime,
    country: Option<&'static str>,
    place: Option<Place>,
    lunar_eves: &'static [Dated],
}

impl EventSettings {
    pub fn in_country(code: &'static str) -> Self {
        Self {
            country_setting: CountrySetting::Fixed(code),
            country: Some(code),
            ..Self::default()
        }
    }

    pub fn allows(&self, id: &str) -> bool {
        self.enabled && !self.disabled.contains(&id)
    }

    pub fn catalogue(&self) -> impl Iterator<Item = &'static EventDef> + '_ {
        let user = self.user;
        user.days
            .iter()
            .copied()
            .chain(all().map(move |def| user.moved_for(def.id).unwrap_or(def)))
    }

    pub fn find(&self, name: &str) -> Result<&'static EventDef, String> {
        find_in(self.catalogue().collect(), name, self.country)
    }

    pub fn locate(&mut self, latitude: f64, longitude: f64) {
        self.place = Some(Place {
            latitude,
            longitude,
            utc_offset_secs: None,
        });
        let named = self.named.filter(|named| {
            (named.latitude - latitude).abs() < 1e-6 && (named.longitude - longitude).abs() < 1e-6
        });
        self.country = match (self.country_setting, named) {
            (CountrySetting::Auto, Some(named)) => named.country,
            (setting, _) => country::resolve(setting, latitude, longitude),
        };
    }

    pub fn from_config(config: &crate::config::EventsConfig) -> (Self, Vec<String>) {
        Self::from_config_with(config, &user::NONE)
    }

    pub fn from_config_with(
        config: &crate::config::EventsConfig,
        user: &'static user::UserEvents,
    ) -> (Self, Vec<String>) {
        let mut warnings = config.warnings.clone();
        let mut disabled: Vec<&'static str> = Vec::new();
        for name in &config.disable {
            let name = name.trim();
            let mut matching = matching(name);
            matching.extend(user.days.iter().copied().filter(|def| def.id == name));
            if matching.is_empty() {
                warnings.push(format!(
                    "events.disable: unknown event '{name}' ignored (weathrz --list-events shows the ids)"
                ));
            }
            for def in matching {
                if !disabled.contains(&def.id) {
                    disabled.push(def.id);
                }
            }
        }
        if !config.observances {
            for id in world::OBSERVANCES {
                if !disabled.contains(id) {
                    disabled.push(id);
                }
            }
        }
        let country_setting = country::parse_setting(&config.country).unwrap_or_else(|warning| {
            warnings.push(format!("events.{warning}"));
            CountrySetting::Auto
        });
        (
            Self {
                enabled: config.enabled,
                disabled,
                country_setting,
                country: None,
                user,
                place: None,
                named: None,
            },
            warnings,
        )
    }
}

impl ActiveEvents {
    pub fn contains(&self, id: &str) -> bool {
        self.events.iter().any(|def| def.id == id)
    }

    pub fn has(&self, look: Look) -> bool {
        self.events.iter().any(|def| def.looks.contains(&look))
    }

    pub fn calm(&self) -> bool {
        self.has(Look::Calm)
    }

    pub fn country(&self) -> Option<&'static str> {
        self.country
    }

    pub fn greetings(&self) -> Vec<(&'static EventDef, String)> {
        let all: Vec<(&'static EventDef, String)> = self
            .events
            .iter()
            .filter_map(|def| {
                greeting_at(def, self.now, self.country, self.place).map(|text| (*def, text))
            })
            .collect();
        let day_off = all
            .iter()
            .any(|(def, _)| def.country().is_some() && !is_observance(def));
        if day_off {
            all.into_iter()
                .filter(|(def, _)| !is_observance(def))
                .collect()
        } else {
            all
        }
    }

    pub fn greeting(&self) -> Option<(&'static EventDef, String)> {
        self.greeting_turning_every(GREETING_TURN_SECS)
    }

    pub fn greeting_turning_every(&self, seconds: i64) -> Option<(&'static EventDef, String)> {
        let mut all = self.greetings();
        if all.is_empty() {
            return None;
        }
        let turn = self.now.and_utc().timestamp().div_euclid(seconds.max(1)) as usize % all.len();
        Some(all.swap_remove(turn))
    }
    pub fn country_of(&self, look: Look) -> Option<&'static str> {
        let def = self.events.iter().find(|def| def.looks.contains(&look))?;
        def.country().or(self.country)
    }

    pub fn since_start(&self, id: &str) -> Option<Duration> {
        let def = self.events.iter().find(|def| def.id == id)?;
        occurrence_at(def, self.now, self.place).map(|occurrence| self.now - occurrence.start)
    }

    pub fn since_start_of(&self, look: Look) -> Option<Duration> {
        let def = self.events.iter().find(|def| def.looks.contains(&look))?;
        occurrence_at(def, self.now, self.place).map(|occurrence| self.now - occurrence.start)
    }

    pub fn since_lunar_new_year_eve(&self) -> Option<Duration> {
        let def = self
            .events
            .iter()
            .find(|def| def.looks.contains(&Look::LunarNewYear))?;
        let tabled = (def.id == IMLEK)
            .then(|| {
                self.lunar_eves
                    .iter()
                    .filter_map(|dated| at_stamp(dated.start))
                    .rfind(|eve| *eve <= self.now)
            })
            .flatten();
        let today = self.now.date();
        let computed = lunar::rule_for(def).and_then(|rule| {
            lunar::days(
                rule.start,
                today - Duration::days(LUNAR_EVE_REACH_DAYS),
                today + Duration::days(1),
            )
            .into_iter()
            .filter_map(|(day, _)| (day - Duration::days(1)).and_hms_opt(LUNAR_EVE_HOUR, 0, 0))
            .filter(|eve| *eve <= self.now)
            .max()
        });
        tabled.max(computed).map(|eve| self.now - eve)
    }

    pub fn now(&self) -> NaiveDateTime {
        self.now
    }
}

pub fn all() -> impl Iterator<Item = &'static EventDef> {
    BUILTIN.iter().chain(world::WORLD.iter())
}

pub fn find(id: &str) -> Option<&'static EventDef> {
    all().find(|def| def.id == id)
}

fn matching(name: &str) -> Vec<&'static EventDef> {
    match find(name) {
        Some(def) => vec![def],
        None => all().filter(|def| def.name() == name).collect(),
    }
}

pub fn find_for(name: &str, country: Option<&str>) -> Result<&'static EventDef, String> {
    find_in(all().collect(), name, country)
}

fn find_in(
    catalogue: Vec<&'static EventDef>,
    name: &str,
    country: Option<&str>,
) -> Result<&'static EventDef, String> {
    let name = name.trim();
    let candidates: Vec<&'static EventDef> = match catalogue.iter().find(|def| def.id == name) {
        Some(def) => vec![*def],
        None => catalogue
            .iter()
            .copied()
            .filter(|def| def.name() == name)
            .collect(),
    };
    match candidates.as_slice() {
        [] => Err(format!(
            "unknown event '{name}' (weathrz --list-events --country <code> shows the ids)"
        )),
        [only] => Ok(only),
        several => several
            .iter()
            .copied()
            .find(|def| def.belongs_to(country) && def.country().is_some())
            .ok_or_else(|| {
                format!(
                    "'{name}' is a holiday in several countries; pick one of {}",
                    several
                        .iter()
                        .map(|def| def.id)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }),
    }
}

pub fn last_year_for(country: &str) -> i32 {
    if country.eq_ignore_ascii_case("ID") {
        return LAST_YEAR;
    }
    world::LAST_YEARS
        .iter()
        .find(|(code, _)| code.eq_ignore_ascii_case(country))
        .map_or(LAST_YEAR, |(_, year)| *year)
}

pub fn easter(year: i32) -> Option<NaiveDate> {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let (d, e) = (b / 4, b % 4);
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let (i, k) = (c / 4, c % 4);
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let n = h + l - 7 * m + 114;
    NaiveDate::from_ymd_opt(year, (n / 31) as u32, (n % 31 + 1) as u32)
}

pub fn orthodox_easter(year: i32) -> Option<NaiveDate> {
    let (a, b, c) = (year % 4, year % 7, year % 19);
    let d = (19 * c + 15) % 30;
    let e = (2 * a + 4 * b - d + 34) % 7;
    let n = d + e + 114;
    let julian = NaiveDate::from_ymd_opt(year, (n / 31) as u32, (n % 31 + 1) as u32)?;
    Some(julian + Duration::days(13))
}

fn equinox_day(year: i32, season: Season) -> Option<NaiveDate> {
    let since = f64::from(year - 1980);
    let (month, base) = match season {
        Season::Spring => (3, 20.8431),
        Season::Autumn => (9, 23.2488),
    };
    let day = (base + 0.242194 * since - (since / 4.0).floor()).floor();
    NaiveDate::from_ymd_opt(year, month, day as u32)
}

pub fn rule_day(rule: DayRule, year: i32) -> Option<NaiveDate> {
    match rule {
        DayRule::Fixed(month, day) => NaiveDate::from_ymd_opt(year, month, day),
        DayRule::Nth(month, weekday, n) => {
            NaiveDate::from_weekday_of_month_opt(year, month, weekday, n)
        }
        DayRule::Last(month, weekday) => {
            let (next_year, next_month) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
            let mut day = NaiveDate::from_ymd_opt(next_year, next_month, 1)?.pred_opt()?;
            while day.weekday() != weekday {
                day = day.pred_opt()?;
            }
            Some(day)
        }
        DayRule::Easter(offset) => Some(easter(year)? + Duration::days(offset)),
        DayRule::Orthodox(offset) => Some(orthodox_easter(year)? + Duration::days(offset)),
        DayRule::Equinox(season) => equinox_day(year, season),
    }
}

fn at_moment(year: i32, moment: Moment) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(year, moment.month, moment.day)?.and_hms_opt(moment.hour, 0, 0)
}

fn at_stamp(stamp: Stamp) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(stamp.year, stamp.month, stamp.day)?.and_hms_opt(stamp.hour, 0, 0)
}

fn before(a: Moment, b: Moment) -> bool {
    (a.month, a.day, a.hour) < (b.month, b.day, b.hour)
}

fn yearly(start: Moment, end: Moment, showcase: Moment, year: i32) -> Option<Occurrence> {
    let following = |moment: Moment| {
        if before(moment, start) {
            year + 1
        } else {
            year
        }
    };
    Some(Occurrence {
        start: at_moment(year, start)?,
        end: at_moment(following(end), end)?,
        showcase: at_moment(following(showcase), showcase)?,
        era: 0,
        tentative: false,
    })
}

fn ruled(rule: DayRule, days: i64, eve: bool, showcase_hour: i64, year: i32) -> Option<Occurrence> {
    let day = rule_day(rule, year)?.and_hms_opt(0, 0, 0)?;
    let start = if eve {
        day - Duration::hours(EVE_HOURS)
    } else {
        day
    };
    Some(Occurrence {
        start,
        end: day + Duration::days(days),
        showcase: day + Duration::hours(showcase_hour),
        era: 0,
        tentative: false,
    })
}

fn from_table(dated: &Dated, showcase_hours: i64) -> Option<Occurrence> {
    let start = at_stamp(dated.start)?;
    Some(Occurrence {
        start,
        end: at_stamp(dated.end)?,
        showcase: start + Duration::hours(showcase_hours),
        era: dated.era,
        tentative: dated.tentative,
    })
}

pub fn next_occurrence(def: &EventDef, now: NaiveDateTime) -> Option<Occurrence> {
    let years = now.year() - 1..=now.year() + 8;
    match def.dates {
        Dates::Yearly {
            start,
            end,
            showcase,
        } => years
            .filter_map(|year| yearly(start, end, showcase, year))
            .find(|occurrence| now < occurrence.end),
        Dates::Rule {
            rule,
            days,
            eve,
            showcase_hour,
        } => years
            .filter_map(|year| ruled(rule, days, eve, showcase_hour, year))
            .find(|occurrence| now < occurrence.end),
        Dates::Listed {
            dates,
            showcase_hours,
        } => dates
            .iter()
            .filter_map(|dated| from_table(dated, showcase_hours))
            .find(|occurrence| now < occurrence.end)
            .or_else(|| {
                lunar::calculated(def, bundled_rows(def).unwrap_or(dates), showcase_hours)
                    .into_iter()
                    .find(|occurrence| now < occurrence.end)
            }),
    }
}

fn follows_sunset(def: &EventDef) -> bool {
    matches!(def.nerd_icon, STAR_AND_CRESCENT | RAMADAN_CRESCENT) || def.id == MUDIK
}

type SunsetKey = (i32, u64, u64, i32);

static SUNSETS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<SunsetKey, Option<NaiveDateTime>>>,
> = std::sync::OnceLock::new();
const REMEMBERED_SUNSETS: usize = 256;

pub fn local_sunset(place: Place, date: NaiveDate) -> Option<NaiveDateTime> {
    let noon = date.and_hms_opt(12, 0, 0)?;
    let offset = match place.utc_offset_secs {
        Some(seconds) => seconds,
        None => Local
            .offset_from_local_datetime(&noon)
            .earliest()?
            .fix()
            .local_minus_utc(),
    };
    let key = (
        date.num_days_from_ce(),
        place.latitude.to_bits(),
        place.longitude.to_bits(),
        offset,
    );
    let memo = SUNSETS.get_or_init(Default::default);
    if let Some(found) = memo.lock().ok().and_then(|memo| memo.get(&key).copied()) {
        return found;
    }
    let shift = Duration::seconds(i64::from(offset));
    let found =
        crate::astro::sunset_after(place.latitude, place.longitude, (noon - shift).and_utc())
            .map(|set| set.naive_utc() + shift);
    if let Ok(mut memo) = memo.lock() {
        if memo.len() >= REMEMBERED_SUNSETS {
            memo.clear();
        }
        memo.insert(key, found);
    }
    found
}

fn at_sunset(moment: NaiveDateTime, place: Place) -> NaiveDateTime {
    if moment.hour() != SUNSET_STAND_IN_HOUR || moment.minute() != 0 {
        return moment;
    }
    local_sunset(place, moment.date())
        .filter(|sunset| (*sunset - moment).abs() <= Duration::hours(SUNSET_REACH_HOURS))
        .unwrap_or(moment)
}

pub fn occurrence_at(
    def: &EventDef,
    now: NaiveDateTime,
    place: Option<Place>,
) -> Option<Occurrence> {
    let Some(place) = place.filter(|_| follows_sunset(def)) else {
        return next_occurrence(def, now);
    };
    let timed = |occurrence: Occurrence| Occurrence {
        start: at_sunset(occurrence.start, place),
        end: at_sunset(occurrence.end, place),
        ..occurrence
    };
    let mut from = now - Duration::hours(SUNSET_REACH_HOURS + 1);
    for _ in 0..4 {
        let found = next_occurrence(def, from)?;
        let shifted = timed(found);
        if now < shifted.end {
            return Some(shifted);
        }
        from = found.end;
    }
    None
}

fn bundled_rows(def: &EventDef) -> Option<&'static [Dated]> {
    match all().find(|bundled| bundled.id == def.id)?.dates {
        Dates::Listed { dates, .. } => Some(dates),
        _ => None,
    }
}

pub fn next_window(def: &EventDef, now: NaiveDateTime) -> Option<(NaiveDateTime, NaiveDateTime)> {
    next_occurrence(def, now).map(|occurrence| (occurrence.start, occurrence.end))
}

pub fn showcase_after(def: &EventDef, now: NaiveDateTime) -> Option<NaiveDateTime> {
    next_occurrence(def, now).map(|occurrence| occurrence.showcase)
}

pub fn is_christmas_eve_night(now: NaiveDateTime) -> bool {
    now.month() == 12
        && ((now.day() == 24 && now.hour() >= 19) || (now.day() == 25 && now.hour() < 5))
}

struct Phrases {
    christmas: Option<&'static str>,
    new_year: &'static str,
    countdown: &'static str,
}

fn phrases(country: Option<&str>) -> Phrases {
    let keeps_western_christmas = !matches!(country, Some("RU" | "ET"));
    Phrases {
        christmas: keeps_western_christmas.then_some(crate::text::MERRY_CHRISTMAS),
        new_year: crate::text::HAPPY_NEW_YEAR,
        countdown: crate::text::NEW_YEAR_COUNTDOWN,
    }
}

fn new_year_text(now: NaiveDateTime, country: Option<&str>) -> Option<String> {
    let words = phrases(country);
    if now.month() != 12 {
        return Some(words.new_year.replace("{year}", &now.year().to_string()));
    }
    let next_year = now.year() + 1;
    let midnight = NaiveDate::from_ymd_opt(next_year, 1, 1)?.and_hms_opt(0, 0, 0)?;
    let minute = now.with_second(0)?.with_nanosecond(0)?;
    let minutes = (midnight - minute).num_minutes().max(0);
    let time = format!("{:02}:{:02}", minutes / 60, minutes % 60);
    Some(
        words
            .countdown
            .replace("{year}", &next_year.to_string())
            .replace("{time}", &time),
    )
}

pub fn greeting_text(def: &EventDef, now: NaiveDateTime, country: Option<&str>) -> Option<String> {
    greeting_at(def, now, country, None)
}

fn greeting_at(
    def: &EventDef,
    now: NaiveDateTime,
    country: Option<&str>,
    place: Option<Place>,
) -> Option<String> {
    match def.id {
        NEW_YEAR => return new_year_text(now, country),
        CHRISTMAS => return phrases(country).christmas.map(str::to_string),
        _ => {}
    }
    if def.greeting.is_empty() {
        return None;
    }
    let occurrence = occurrence_at(def, now, place).filter(|occurrence| occurrence.start <= now)?;
    let since = now - occurrence.start;
    let eve = since < Duration::hours(EVE_HOURS);
    let first_days = since < Duration::hours(EVE_HOURS) + Duration::days(FIRST_DAYS);
    let day_year = (occurrence.start + Duration::hours(EVE_HOURS)).year();
    let full = with_since(
        &def.greeting
            .replace("{era}", &occurrence.era.to_string())
            .replace("{animal}", zodiac(day_year, def.country() == Some("vn"))),
        day_year,
    );
    match def.id {
        IDULFITRI | IDULADHA if eve => Some(crate::text::TAKBIRAN_NIGHT.to_string()),
        IMLEK if eve => Some(crate::text::CHINESE_NEW_YEARS_EVE.to_string()),
        IMLEK if first_days => Some(full),
        IMLEK if occurrence.end - now <= Duration::days(1) => {
            Some(crate::text::CAP_GO_MEH.to_string())
        }
        IMLEK => None,
        _ if def.looks.contains(&Look::Ramadan) => first_days.then_some(full),
        _ => Some(full),
    }
}

const ZODIAC: [&str; 12] = [
    "Rat", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster", "Dog",
    "Pig",
];

fn zodiac(year: i32, vietnamese: bool) -> &'static str {
    match ZODIAC[(year - 4).rem_euclid(12) as usize] {
        "Rabbit" if vietnamese => "Cat",
        animal => animal,
    }
}

fn ordinal(number: i32) -> String {
    let suffix = match (number % 10, number % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{number}{suffix}")
}

fn with_since(text: &str, year: i32) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let tail = &rest[open + 1..];
        let parsed = tail.split_once('}').and_then(|(inside, after)| {
            let (nth, base) = match inside.strip_prefix("nth since ") {
                Some(base) => (true, base),
                None => (false, inside.strip_prefix("since ")?),
            };
            let years = year - base.trim().parse::<i32>().ok()?;
            Some((
                if nth {
                    ordinal(years)
                } else {
                    years.to_string()
                },
                after,
            ))
        });
        match parsed {
            Some((value, after)) => {
                out.push_str(&value);
                rest = after;
            }
            None => {
                out.push('{');
                rest = tail;
            }
        }
    }
    out.push_str(rest);
    out
}

pub fn short_greeting(def: &EventDef, text: &str) -> Option<String> {
    if !def.greeting.contains(" · ") {
        return None;
    }
    text.split_once(" · ").map(|(head, _)| head.to_string())
}

fn is_active_at(def: &EventDef, now: NaiveDateTime, place: Option<Place>) -> bool {
    occurrence_at(def, now, place).is_some_and(|occurrence| occurrence.start <= now)
}

pub fn is_observance(def: &EventDef) -> bool {
    world::OBSERVANCES.contains(&def.id)
}

fn priority(
    index: usize,
    def: &EventDef,
    now: NaiveDateTime,
    settings: &EventSettings,
) -> (u8, bool, i64, u8, usize) {
    if settings.user.is_own(def) {
        return (0, false, 0, 0, index);
    }
    let group = match def.country() {
        _ if def.id == NEW_YEAR => 0,
        Some(_) => 1,
        None => 2,
    };
    let observance = is_observance(def);
    if def.country() != Some("id") && def.country().is_some() {
        let span = next_window(def, now).map_or(0, |(start, end)| (end - start).num_hours());
        let kind = match def.dates {
            Dates::Listed { .. } => 0,
            _ => 1,
        };
        return (group, observance, span, kind, index);
    }
    (group, observance, 0, 0, index)
}

pub fn resolve(now: NaiveDateTime, settings: &EventSettings) -> ActiveEvents {
    let mut active: Vec<(usize, &'static EventDef)> = settings
        .catalogue()
        .enumerate()
        .filter(|(_, def)| {
            def.belongs_to(settings.country)
                && settings.allows(def.id)
                && is_active_at(def, now, settings.place)
        })
        .collect();
    active.sort_by_key(|(index, def)| priority(*index, def, now, settings));
    ActiveEvents {
        events: active.into_iter().map(|(_, def)| def).collect(),
        now,
        country: settings.country,
        place: settings.place,
        lunar_eves: match settings.user.moved_for(IMLEK).map(|def| &def.dates) {
            Some(Dates::Listed { dates, .. }) => dates,
            _ => dates::IMLEK,
        },
    }
}

fn country_line(settings: &EventSettings) -> String {
    let how = match settings.country_setting {
        CountrySetting::Auto => "from your location",
        CountrySetting::Fixed(_) => "chosen with country",
        CountrySetting::Nowhere => "country = \"none\"",
    };
    match settings.country.and_then(country::name_of) {
        Some(name) => format!("Holidays for {name} ({how}), plus the yearly ones:\n"),
        None => format!("No country's holidays ({how}); only the yearly ones:\n"),
    }
}

pub fn format_listing(now: NaiveDateTime, settings: &EventSettings) -> String {
    let (marked, days): (Vec<&EventDef>, Vec<&EventDef>) = settings
        .catalogue()
        .filter(|def| def.belongs_to(settings.country))
        .partition(|def| is_observance(def));
    let mut lines: Vec<String> = vec![country_line(settings)];
    lines.extend(listing_lines(days, now, settings));
    let marked = listing_lines(marked, now, settings);
    if !marked.is_empty() {
        lines.push(format!("\n{}", crate::text::LISTING_OBSERVANCES));
        lines.extend(marked);
    }
    if let Some(code) = settings.country {
        let last_year = last_year_for(code);
        lines.push(format!(
            "\nDates that follow the moon or an announcement are bundled through {last_year}; moon days are calculated for {} more years after that. Tentative ones can move a day.\nSources: {}.",
            lunar::YEARS_AHEAD,
            sources_for(code)
        ));
    }
    lines.join("\n")
}

fn listing_lines(
    shown: Vec<&EventDef>,
    now: NaiveDateTime,
    settings: &EventSettings,
) -> Vec<String> {
    let width = shown.iter().map(|def| def.id.len()).max().unwrap_or(0) + 1;
    let mut upcoming: Vec<(&EventDef, Occurrence)> = shown
        .into_iter()
        .filter_map(|def| Some((def, occurrence_at(def, now, settings.place)?)))
        .collect();
    upcoming.sort_by_key(|(_, occurrence)| occurrence.start);
    upcoming
        .into_iter()
        .map(|(def, occurrence)| {
            let mut notes = String::new();
            if occurrence.tentative {
                notes.push_str("  (tentative)");
            }
            if !settings.allows(def.id) {
                notes.push_str("  (disabled)");
            }
            format!(
                "{:<width$} {} → {}{notes}",
                def.id,
                occurrence.start.format("%Y-%m-%d %H:%M"),
                occurrence.end.format("%m-%d %H:%M"),
            )
        })
        .collect()
}

fn sources_for(code: &str) -> String {
    let researched = world::SOURCES
        .iter()
        .find(|(country, _)| *country == code)
        .map(|(_, sources)| *sources);
    match (code, researched) {
        ("ID", Some(observances)) => format!("{SOURCES}; observances: {observances}"),
        ("ID", None) => SOURCES.to_string(),
        (_, Some(sources)) => sources.to_string(),
        (_, None) => "see README".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(text: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap()
    }

    fn halloween() -> &'static EventDef {
        find(HALLOWEEN).expect("halloween is built in")
    }

    fn table(id: &str) -> &'static [Dated] {
        match find(id).unwrap().dates {
            Dates::Listed { dates, .. } => dates,
            _ => panic!("{id} is not listed"),
        }
    }

    fn greets(when: &str) -> Option<String> {
        resolve(at(when), &EventSettings::in_country("ID"))
            .greeting()
            .map(|(_, text)| text)
    }

    #[test]
    fn halloween_runs_from_midnight_to_six_the_next_morning() {
        let on = EventSettings::in_country("ID");
        for (when, expected) in [
            ("2026-10-30T23:59", false),
            ("2026-10-31T00:00", true),
            ("2026-10-31T21:00", true),
            ("2026-11-01T05:59", true),
            ("2026-11-01T06:00", false),
            ("2027-10-31T12:00", true),
            ("2026-07-01T12:00", false),
        ] {
            assert_eq!(
                resolve(at(when), &on).contains(HALLOWEEN),
                expected,
                "{when}"
            );
        }
    }

    #[test]
    fn settings_switch_events_off() {
        let night = at("2026-10-31T21:00");
        let off = EventSettings {
            enabled: false,
            disabled: Vec::new(),
            ..EventSettings::in_country("ID")
        };
        assert!(!resolve(night, &off).contains(HALLOWEEN));
        assert_eq!(resolve(night, &off).greeting(), None);
        let disabled = EventSettings {
            enabled: true,
            disabled: vec![HALLOWEEN],
            ..EventSettings::in_country("ID")
        };
        assert!(!resolve(night, &disabled).contains(HALLOWEEN));
        assert!(!disabled.allows(HALLOWEEN));
        assert!(EventSettings::in_country("ID").allows(HALLOWEEN));
    }

    #[test]
    fn next_window_is_the_current_or_the_coming_one() {
        let this_year = (at("2026-10-31T00:00"), at("2026-11-01T06:00"));
        assert_eq!(
            next_window(halloween(), at("2026-10-13T09:00")),
            Some(this_year)
        );
        assert_eq!(
            next_window(halloween(), at("2026-10-31T22:00")),
            Some(this_year)
        );
        assert_eq!(
            next_window(halloween(), at("2026-11-01T07:00")),
            Some((at("2027-10-31T00:00"), at("2027-11-01T06:00")))
        );
    }

    #[test]
    fn christmas_lights_and_christmas_run_their_days() {
        let on = EventSettings::in_country("ID");
        for (when, lights, christmas) in [
            ("2026-12-14T23:59", false, false),
            ("2026-12-15T00:00", true, false),
            ("2026-12-23T23:59", true, false),
            ("2026-12-24T00:00", true, true),
            ("2026-12-26T23:59", true, true),
            ("2026-12-27T00:00", false, false),
        ] {
            let active = resolve(at(when), &on);
            assert_eq!(active.contains(CHRISTMAS_LIGHTS), lights, "{when}");
            assert_eq!(active.contains(CHRISTMAS), christmas, "{when}");
        }
    }

    #[test]
    fn new_year_crosses_into_the_next_year() {
        let on = EventSettings::in_country("ID");
        for (when, expected) in [
            ("2026-12-31T19:59", false),
            ("2026-12-31T20:00", true),
            ("2027-01-01T00:00", true),
            ("2027-01-01T23:59", true),
            ("2027-01-02T00:00", false),
        ] {
            assert_eq!(
                resolve(at(when), &on).contains(NEW_YEAR),
                expected,
                "{when}"
            );
        }
        let new_year = find(NEW_YEAR).unwrap();
        assert_eq!(
            next_window(new_year, at("2027-01-01T10:00")),
            Some((at("2026-12-31T20:00"), at("2027-01-02T00:00")))
        );
        assert_eq!(
            showcase_after(new_year, at("2026-12-01T00:00")),
            Some(at("2026-12-31T23:00"))
        );
    }

    #[test]
    fn the_new_year_greeting_counts_down_then_wishes() {
        let new_year = find(NEW_YEAR).unwrap();
        assert_eq!(
            greeting_text(new_year, at("2026-12-31T23:12"), Some("ID")),
            Some("2027 in 00:48".to_string())
        );
        assert_eq!(
            greeting_text(new_year, at("2026-12-31T20:00"), Some("ID")),
            Some("2027 in 04:00".to_string())
        );
        assert_eq!(
            greeting_text(new_year, at("2027-01-01T00:05"), Some("ID")),
            Some("Happy New Year 2027".to_string())
        );
        assert_eq!(
            greeting_text(find(CHRISTMAS).unwrap(), at("2026-12-24T21:00"), Some("ID")),
            Some("Merry Christmas".to_string())
        );
        assert_eq!(
            greeting_text(
                find(CHRISTMAS_LIGHTS).unwrap(),
                at("2026-12-20T21:00"),
                Some("ID")
            ),
            None
        );
    }

    #[test]
    fn christmas_eve_night_runs_from_seven_to_five() {
        for (when, expected) in [
            ("2026-12-24T18:59", false),
            ("2026-12-24T19:00", true),
            ("2026-12-25T04:59", true),
            ("2026-12-25T05:00", false),
            ("2026-12-25T21:00", false),
        ] {
            assert_eq!(is_christmas_eve_night(at(when)), expected, "{when}");
        }
    }

    #[test]
    fn only_events_with_words_greet() {
        let on = EventSettings::in_country("ID");
        assert_eq!(resolve(at("2026-12-20T21:00"), &on).greeting(), None);
        assert_eq!(
            resolve(at("2026-12-24T21:00"), &on)
                .greeting()
                .map(|(def, _)| def.id),
            Some(CHRISTMAS)
        );
        assert_eq!(
            resolve(at("2026-12-24T21:00"), &on).now(),
            at("2026-12-24T21:00")
        );
    }

    #[test]
    fn showcase_is_the_next_occurrence_highlight() {
        let now = at("2026-10-20T09:00");
        assert_eq!(
            showcase_after(halloween(), now),
            Some(at("2026-10-31T21:00"))
        );
        assert_eq!(
            showcase_after(find(IMLEK).unwrap(), now),
            Some(at("2027-02-05T23:00"))
        );
        assert_eq!(
            showcase_after(find(PASKAH).unwrap(), now),
            Some(at("2027-03-28T10:00"))
        );
        assert_eq!(
            showcase_after(find(IDULFITRI).unwrap(), now),
            Some(at("2027-03-09T21:00"))
        );
        assert_eq!(
            showcase_after(find(NYEPI).unwrap(), now),
            Some(at("2027-03-08T21:00"))
        );
        assert_eq!(
            showcase_after(find(MUDIK).unwrap(), now),
            Some(at("2027-03-07T10:00"))
        );
        for def in BUILTIN.iter().filter(|def| def.belongs_to(Some("ID"))) {
            let showcase = showcase_after(def, now).expect(def.id);
            assert!(
                resolve(showcase, &EventSettings::in_country("ID")).contains(def.id),
                "{} showcase {showcase} is outside its window",
                def.id
            );
        }
    }

    #[test]
    fn greeting_comes_from_the_active_event() {
        let active = resolve(at("2026-10-31T10:00"), &EventSettings::in_country("ID"));
        let (def, text) = active.greeting().expect("halloween greets");
        assert_eq!(
            (text.as_str(), def.nerd_icon),
            ("Happy Halloween", '\u{f0bbf}')
        );
        assert_eq!(ActiveEvents::default().greeting(), None);
    }

    #[test]
    fn listing_is_sorted_by_date_and_marks_tentative_and_disabled_days() {
        let now = at("2026-10-13T09:00");
        let listing = format_listing(now, &EventSettings::in_country("ID"));
        let lines: Vec<&str> = listing.lines().collect();
        assert_eq!(
            lines[..8],
            [
                "Holidays for Indonesia (chosen with country), plus the yearly ones:",
                "",
                "halloween         2026-10-31 00:00 → 11-01 06:00",
                "christmas_lights  2026-12-15 00:00 → 12-27 00:00",
                "christmas         2026-12-24 00:00 → 12-27 00:00",
                "new_year          2026-12-31 20:00 → 01-02 00:00",
                "id:isra_mikraj    2027-01-04 18:00 → 01-06 00:00",
                "id:imlek          2027-02-05 18:00 → 02-21 00:00",
            ]
        );
        assert!(
            listing.contains("id:ramadan        2027-02-07 18:00 → 03-09 18:00  (tentative)\n"),
            "{listing}"
        );
        assert!(
            listing.contains("id:maulid         2027-08-15 00:00 → 08-16 00:00\n"),
            "{listing}"
        );
        assert!(
            listing.ends_with(
                "\n\nDates that follow the moon or an announcement are bundled through 2028; moon days are calculated for 20 more years after that. Tentative ones can move a day.\n\
                 Sources: SKB 3 Menteri 2026 (No. 1497/2/5 of 2025) and 2027 (No. 1205/3/2 of 2026), Kemenag isbat; \
                 2028 estimated from the neo-MABIMS hisab, HKO and the Saka calendar until its SKB; \
                 observances: jdih.kemenkoinfra.go.id, detik.com, id.wikisource.org, katadata.co.id, news.detik.com, metrotvnews.com."
            ),
            "{listing}"
        );
        let observances = lines
            .iter()
            .skip_while(|line| **line != crate::text::LISTING_OBSERVANCES)
            .skip(1)
            .take_while(|line| line.starts_with("id:"))
            .count();
        assert_eq!(
            observances,
            world::OBSERVANCES
                .iter()
                .filter(|id| id.starts_with("id:"))
                .count()
        );
        let yearly = BUILTIN
            .iter()
            .filter(|def| def.belongs_to(Some("ID")))
            .count();
        assert_eq!(lines.len(), yearly + 5 + 2 + observances);
        let off = EventSettings {
            disabled: vec![HALLOWEEN, RAMADAN],
            ..EventSettings::in_country("ID")
        };
        let listing = format_listing(now, &off);
        assert!(
            listing.contains("\nhalloween         2026-10-31 00:00 → 11-01 06:00  (disabled)\n"),
            "{listing}"
        );
        assert!(
            listing.contains("03-09 18:00  (tentative)  (disabled)\n"),
            "{listing}"
        );
        let nowhere = EventSettings::default();
        let listing = format_listing(now, &nowhere);
        assert!(
            listing
                .starts_with("No country's holidays (from your location); only the yearly ones:\n"),
            "{listing}"
        );
        assert!(!listing.contains("id:"), "{listing}");
        assert!(!listing.contains("Sources"), "{listing}");
    }

    #[test]
    fn past_the_table_moon_days_are_calculated_and_announced_ones_go_quiet() {
        let later = at("2029-01-10T09:00");
        let imlek = next_occurrence(find(IMLEK).unwrap(), later).unwrap();
        assert_eq!(imlek.start, at("2029-02-12T18:00"));
        assert_eq!(imlek.era, 2580);
        assert!(imlek.tentative);
        assert!(next_occurrence(find(ISRA_MIKRAJ).unwrap(), later).is_some());
        assert_eq!(next_occurrence(find("in:diwali").unwrap(), later), None);
        assert!(next_occurrence(find(PASKAH).unwrap(), later).is_some());
        let listing = format_listing(later, &EventSettings::in_country("ID"));
        assert!(
            listing.contains("id:imlek          2029-02-12 18:00 → 02-28 00:00  (tentative)"),
            "{listing}"
        );
        assert!(
            listing.contains("id:paskah         2029-04-01 00:00"),
            "{listing}"
        );
    }

    #[test]
    fn islamic_eves_begin_at_the_real_sunset() {
        let place = Place {
            latitude: -6.2088,
            longitude: 106.8456,
            utc_offset_secs: Some(7 * 3600),
        };
        let mut jakarta = EventSettings::in_country("ID");
        jakarta.place = Some(place);
        let sunset = local_sunset(place, NaiveDate::from_ymd_opt(2027, 3, 9).unwrap()).unwrap();
        assert_ne!((sunset.hour(), sunset.minute()), (18, 0));
        let before = resolve(sunset - Duration::minutes(3), &jakarta);
        let after = resolve(sunset + Duration::minutes(3), &jakarta);
        assert!(
            !before.contains(IDULFITRI) && before.contains(RAMADAN),
            "{before:?}"
        );
        assert!(
            after.contains(IDULFITRI) && !after.contains(RAMADAN),
            "{after:?}"
        );
        assert_eq!(
            after.greeting().map(|(_, text)| text).as_deref(),
            Some("Takbiran Night")
        );
        let imlek = resolve(at("2027-02-05T18:00"), &jakarta);
        assert!(imlek.contains(IMLEK));
        let unplaced = resolve(at("2027-03-09T18:00"), &EventSettings::in_country("ID"));
        assert!(unplaced.contains(IDULFITRI));
    }

    #[test]
    fn the_listing_shows_islamic_eves_at_the_real_sunset() {
        let place = Place {
            latitude: -6.2088,
            longitude: 106.8456,
            utc_offset_secs: Some(7 * 3600),
        };
        let mut jakarta = EventSettings::in_country("ID");
        jakarta.place = Some(place);
        let sunset = local_sunset(place, NaiveDate::from_ymd_opt(2027, 3, 9).unwrap()).unwrap();
        let listing = format_listing(at("2027-03-01T12:00"), &jakarta);
        let idulfitri = listing
            .lines()
            .find(|line| line.starts_with(IDULFITRI))
            .expect("idulfitri is listed");
        assert!(
            idulfitri.contains(&sunset.format("%Y-%m-%d %H:%M").to_string()),
            "{idulfitri}"
        );
        let unplaced = format_listing(at("2027-03-01T12:00"), &EventSettings::in_country("ID"));
        assert!(unplaced.contains("2027-03-09 18:00"), "{unplaced}");
    }

    #[test]
    fn a_far_sunset_holds_its_place_all_evening() {
        let london = Place {
            latitude: 51.5,
            longitude: -0.13,
            utc_offset_secs: Some(0),
        };
        let oslo = Place {
            latitude: 59.9,
            longitude: 10.75,
            utc_offset_secs: Some(2 * 3600),
        };
        for (place, id, times) in [
            (
                london,
                ISRA_MIKRAJ,
                ["2027-12-25T16:30", "2027-12-26T00:30", "2027-12-26T01:30"],
            ),
            (
                oslo,
                IDULADHA,
                ["2027-05-16T22:30", "2027-05-17T01:00", "2027-05-17T03:50"],
            ),
        ] {
            let starts: Vec<NaiveDateTime> = times
                .iter()
                .map(|when| {
                    occurrence_at(find(id).unwrap(), at(when), Some(place))
                        .unwrap()
                        .start
                })
                .collect();
            assert!(
                starts.windows(2).all(|pair| pair[0] == pair[1]),
                "{id}: {starts:?}"
            );
            assert_ne!(starts[0].hour(), 18, "{id}");
        }
    }

    #[test]
    fn overlapping_greetings_take_turns_every_ten_seconds() {
        let indonesia = EventSettings::in_country("ID");
        let both = resolve(at("2027-12-25T19:00"), &indonesia);
        let ids: Vec<&str> = both.greetings().iter().map(|(def, _)| def.id).collect();
        assert_eq!(ids, [ISRA_MIKRAJ, CHRISTMAS]);
        let shown = |seconds: i64| {
            resolve(
                at("2027-12-25T19:00") + Duration::seconds(seconds),
                &indonesia,
            )
            .greeting()
            .map(|(def, _)| def.id)
            .unwrap()
        };
        assert_ne!(shown(0), shown(10));
        assert_eq!(shown(0), shown(20));
        assert_eq!(shown(0), shown(9));
        let alone = resolve(at("2027-12-24T19:00"), &indonesia);
        assert!((0..60).all(|seconds| {
            resolve(
                at("2027-12-24T19:00") + Duration::seconds(seconds),
                &indonesia,
            )
            .greeting()
            .map(|(def, _)| def.id)
                == alone.greeting().map(|(def, _)| def.id)
        }));
    }

    #[test]
    fn a_named_place_brings_its_own_country() {
        let johor = (1.4927, 103.7414);
        let mut settings = EventSettings::default();
        settings.locate(johor.0, johor.1);
        let guessed = settings.country;
        settings.named = Some(NamedPlace {
            latitude: johor.0,
            longitude: johor.1,
            country: Some("MY"),
        });
        settings.locate(johor.0, johor.1);
        assert_eq!(settings.country, Some("MY"), "guessed {guessed:?}");
        settings.named = Some(NamedPlace {
            latitude: 1.29,
            longitude: 103.85,
            country: None,
        });
        settings.locate(1.29, 103.85);
        assert_eq!(
            settings.country, None,
            "a place outside the 14 gets the yearly events"
        );
        settings.locate(-6.2088, 106.8456);
        assert_eq!(
            settings.country,
            Some("ID"),
            "moving elsewhere drops the name's country"
        );
        let fixed = EventSettings {
            named: settings.named,
            ..EventSettings::in_country("JP")
        };
        let mut fixed = fixed;
        fixed.locate(1.29, 103.85);
        assert_eq!(fixed.country, Some("JP"));
    }

    #[test]
    fn a_move_never_bends_the_calculated_years() {
        let (moved, warnings) =
            user::parse("[[move]]\nevent = \"id:idulfitri\"\nyear = 2028\nshift = 1\n");
        assert!(warnings.is_empty(), "{warnings:?}");
        let settings = EventSettings {
            user: user::leak(moved),
            ..EventSettings::in_country("ID")
        };
        let fitri = settings.find(IDULFITRI).unwrap();
        let later = next_occurrence(fitri, at("2029-01-01T00:00")).unwrap();
        let bundled = next_occurrence(find(IDULFITRI).unwrap(), at("2029-01-01T00:00")).unwrap();
        assert_eq!(later.start, bundled.start);
        let (far, _) = user::parse("[[move]]\nevent = \"id:idulfitri\"\nyear = 2026\nshift = 2\n");
        let far = EventSettings {
            user: user::leak(far),
            ..EventSettings::in_country("ID")
        };
        assert!(next_occurrence(far.find(IDULFITRI).unwrap(), at("2029-01-01T00:00")).is_some());
    }

    #[test]
    fn the_lunar_new_year_eve_sparkles_past_the_tables_too() {
        let eve = resolve(at("2029-02-12T21:00"), &EventSettings::in_country("ID"));
        assert_eq!(eve.since_lunar_new_year_eve(), Some(Duration::hours(3)));
    }

    #[test]
    fn a_status_line_turns_by_the_minute() {
        let indonesia = EventSettings::in_country("ID");
        let shown = |minute: i64| {
            resolve(
                at("2027-12-25T19:00") + Duration::minutes(minute),
                &indonesia,
            )
            .greeting_turning_every(60)
            .map(|(def, _)| def.id)
            .unwrap()
        };
        assert_ne!(shown(0), shown(1));
    }

    #[test]
    fn christmas_lights_follow_each_country_s_season() {
        let lit = |country: &'static str, when: &str| {
            resolve(at(when), &EventSettings::in_country(country)).has(Look::ChristmasLights)
        };
        assert!(!lit("US", "2026-12-02T20:00"));
        assert!(lit("US", "2026-12-26T20:00"));
        assert!(!lit("US", "2027-01-03T20:00"));
        assert!(
            lit("PH", "2026-12-02T20:00"),
            "the Philippines light up early"
        );
        assert!(lit("PH", "2027-01-06T20:00"), "through Three Kings");
        assert!(!lit("PH", "2027-01-07T20:00"));
        assert!(lit("EG", "2027-01-06T20:00"), "through Coptic Christmas");
        assert!(!lit("EG", "2027-01-08T20:00"));
        assert!(lit("RU", "2027-01-07T12:00"));
        assert!(lit("TR", "2026-12-31T22:00"), "New Year lights");
        assert!(!lit("TR", "2027-01-01T12:00"));
        assert!(!lit("ET", "2026-12-24T20:00"));
        let anywhere = EventSettings::default();
        assert!(!resolve(at("2026-12-20T20:00"), &anywhere).contains(CHRISTMAS_SEASON));
        assert!(resolve(at("2026-12-20T20:00"), &anywhere).contains(CHRISTMAS_LIGHTS));
    }

    #[test]
    fn ids_are_known_and_unique() {
        let ids: Vec<&str> = all().map(|def| def.id).collect();
        let mut unique = ids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), ids.len());
        assert_eq!(BUILTIN.len(), 24);
        for def in all() {
            match def.country() {
                None => assert!(!def.id.contains(':')),
                Some(code) => assert!(
                    country::name_of(&code.to_ascii_uppercase()).is_some(),
                    "{} has an unknown country",
                    def.id
                ),
            }
        }
        assert_eq!(find("easter"), None);
        assert_eq!(find("id:nyepi").map(|def| def.country()), Some(Some("id")));
    }

    #[test]
    fn observances_can_be_switched_off_without_touching_days_off() {
        let quiet = crate::config::EventsConfig {
            observances: false,
            country: "US".to_string(),
            ..crate::config::EventsConfig::default()
        };
        let (mut settings, _) = EventSettings::from_config(&quiet);
        settings.locate(40.7, -74.0);
        let greeting = |when: &str| {
            resolve(at(when), &settings)
                .greeting()
                .map(|(def, _)| def.id)
        };
        assert_eq!(
            greeting("2027-02-02T10:00"),
            None,
            "Groundhog Day is an observance"
        );
        assert_eq!(greeting("2027-07-04T10:00"), Some("us:independence_day"));
        let (mut loud, _) = EventSettings::from_config(&crate::config::EventsConfig {
            country: "US".to_string(),
            ..crate::config::EventsConfig::default()
        });
        loud.locate(40.7, -74.0);
        assert_eq!(
            resolve(at("2027-02-02T10:00"), &loud)
                .greeting()
                .map(|(def, _)| def.id),
            Some("us:groundhog_day")
        );
    }

    #[test]
    fn an_observance_on_a_day_off_does_not_share_the_greeting() {
        let (mut egypt, _) = EventSettings::from_config(&crate::config::EventsConfig {
            country: "EG".to_string(),
            ..crate::config::EventsConfig::default()
        });
        egypt.locate(30.04, 31.24);
        let active = resolve(at("2026-03-21T10:00"), &egypt);
        assert!(active.contains("eg:mothers_day"));
        let ids: Vec<&str> = active.greetings().iter().map(|(def, _)| def.id).collect();
        assert!(ids.contains(&"eg:eid_al_fitr"), "{ids:?}");
        assert!(!ids.contains(&"eg:mothers_day"), "{ids:?}");
        let us = resolve(at("2026-05-25T09:00"), &EventSettings::in_country("US"));
        assert!(us.has(Look::HalfMast), "the twin still lowers the flag");
        let ids: Vec<&str> = us.greetings().iter().map(|(def, _)| def.id).collect();
        assert_eq!(ids, ["us:memorial_day"]);
    }

    #[test]
    fn unknown_disable_ids_warn_and_are_ignored() {
        let config = crate::config::EventsConfig {
            enabled: true,
            observances: true,
            disable: vec![
                "haloween".to_string(),
                " halloween ".to_string(),
                "halloween".to_string(),
                "id:imlek".to_string(),
            ],
            country: "jp".to_string(),
            warnings: vec!["earlier".to_string()],
        };
        let (settings, warnings) = EventSettings::from_config(&config);
        assert_eq!(settings.disabled, vec![HALLOWEEN, IMLEK]);
        assert!(settings.enabled);
        assert_eq!(settings.country_setting, CountrySetting::Fixed("JP"));
        assert_eq!(
            warnings,
            vec![
                "earlier".to_string(),
                "events.disable: unknown event 'haloween' ignored (weathrz --list-events shows the ids)"
                    .to_string(),
            ]
        );
        let narnia = crate::config::EventsConfig {
            country: "Narnia".to_string(),
            ..crate::config::EventsConfig::default()
        };
        let (settings, warnings) = EventSettings::from_config(&narnia);
        assert_eq!(settings.country_setting, CountrySetting::Auto);
        assert!(
            warnings[0].starts_with("events.country = \"Narnia\" is not known"),
            "{warnings:?}"
        );
    }

    #[test]
    fn computus_matches_the_skb_and_known_easters() {
        for (year, expected) in [
            (2008, (3, 23)),
            (2024, (3, 31)),
            (2025, (4, 20)),
            (2026, (4, 5)),
            (2027, (3, 28)),
            (2038, (4, 25)),
            (2285, (3, 22)),
        ] {
            assert_eq!(
                easter(year),
                NaiveDate::from_ymd_opt(year, expected.0, expected.1),
                "{year}"
            );
        }
    }

    #[test]
    fn every_skb_holiday_falls_inside_its_window() {
        let on = EventSettings::in_country("ID");
        for (when, id) in [
            ("2026-01-16T12:00", ISRA_MIKRAJ),
            ("2026-02-17T12:00", IMLEK),
            ("2026-03-19T12:00", NYEPI),
            ("2026-03-21T12:00", IDULFITRI),
            ("2026-03-22T12:00", IDULFITRI),
            ("2026-03-24T23:59", IDULFITRI),
            ("2026-04-03T12:00", JUMAT_AGUNG),
            ("2026-04-05T12:00", PASKAH),
            ("2026-05-14T12:00", KENAIKAN),
            ("2026-05-27T12:00", IDULADHA),
            ("2026-05-31T12:00", WAISAK),
            ("2026-06-16T12:00", MUHARAM),
            ("2026-08-25T12:00", MAULID),
            ("2027-01-05T12:00", ISRA_MIKRAJ),
            ("2027-02-05T21:00", IMLEK),
            ("2027-02-06T12:00", IMLEK),
            ("2027-03-08T12:00", NYEPI),
            ("2027-03-10T12:00", IDULFITRI),
            ("2027-03-11T12:00", IDULFITRI),
            ("2027-03-15T23:59", IDULFITRI),
            ("2027-03-26T12:00", JUMAT_AGUNG),
            ("2027-03-28T12:00", PASKAH),
            ("2027-05-06T12:00", KENAIKAN),
            ("2027-05-17T12:00", IDULADHA),
            ("2027-05-18T12:00", IDULADHA),
            ("2027-05-20T12:00", WAISAK),
            ("2027-06-06T12:00", MUHARAM),
            ("2027-08-15T12:00", MAULID),
            ("2027-12-26T12:00", ISRA_MIKRAJ),
        ] {
            assert!(resolve(at(when), &on).contains(id), "{id} at {when}");
        }
        for (when, id) in [
            ("2027-03-16T00:00", IDULFITRI),
            ("2027-03-09T17:59", IDULFITRI),
            ("2027-03-09T06:00", NYEPI),
            ("2027-02-21T00:00", IMLEK),
            ("2027-03-27T00:00", JUMAT_AGUNG),
            ("2027-05-21T06:00", WAISAK),
        ] {
            assert!(!resolve(at(when), &on).contains(id), "{id} at {when}");
        }
    }

    #[test]
    fn the_tables_are_sorted_and_agree_with_each_other() {
        for def in BUILTIN {
            let Dates::Listed { dates, .. } = def.dates else {
                continue;
            };
            let windows: Vec<Occurrence> = dates
                .iter()
                .map(|dated| from_table(dated, 0).expect(def.id))
                .collect();
            for pair in windows.windows(2) {
                assert!(pair[0].end <= pair[1].start, "{} out of order", def.id);
            }
            assert!(windows.iter().all(|window| window.start < window.end));
            assert_eq!(dates.last().unwrap().end.year, LAST_YEAR, "{}", def.id);
        }
        let day_of = |dated: &Dated| at_stamp(dated.start).unwrap() + Duration::hours(EVE_HOURS);
        for dated in table(IMLEK) {
            assert_eq!(dated.era as i32, day_of(dated).year() + 551);
        }
        for dated in table(NYEPI) {
            assert_eq!(dated.era as i32, dated.start.year - 78);
        }
        for dated in table(WAISAK) {
            assert_eq!(dated.era as i32, dated.start.year + 544);
        }
        for ((fitri, mudik), ramadan) in table(IDULFITRI)
            .iter()
            .zip(table(MUDIK))
            .zip(table(RAMADAN))
        {
            assert_eq!(mudik.end, fitri.start);
            assert_eq!(ramadan.end, fitri.start);
            assert_eq!(
                at_stamp(mudik.start).unwrap(),
                day_of(fitri) - Duration::days(7)
            );
            assert_eq!((mudik.era, ramadan.era), (fitri.era, fitri.era));
            assert_eq!(mudik.tentative, fitri.tentative);
        }
        for def in [IDULFITRI, IDULADHA, ISRA_MIKRAJ, MUHARAM, RAMADAN] {
            assert!(
                table(def).iter().all(|dated| dated.start.hour == 18),
                "{def} eves start at sunset"
            );
        }
    }

    #[test]
    fn greetings_follow_the_phase_of_each_holiday() {
        for (when, expected) in [
            ("2027-03-09T21:00", Some("Takbiran Night")),
            ("2027-03-10T08:00", Some("Happy Eid al-Fitr 1448 AH")),
            ("2027-05-16T20:00", Some("Takbiran Night")),
            ("2027-05-17T08:00", Some("Happy Eid al-Adha 1448 AH")),
            ("2027-02-05T23:00", Some("Chinese New Year's Eve")),
            (
                "2027-02-06T10:00",
                Some("Happy Chinese New Year 2578 · Year of the Goat"),
            ),
            ("2027-02-20T12:00", Some("Happy Cap Go Meh")),
            ("2027-02-10T12:00", Some("Ramadan Mubarak")),
            ("2027-02-15T12:00", None),
            (
                "2027-03-08T21:00",
                Some("Day of Silence · Saka New Year 1949"),
            ),
            ("2027-05-20T21:00", Some("Happy Vesak 2571 BE")),
            ("2027-06-05T21:00", Some("Happy Islamic New Year 1449 AH")),
            ("2027-03-26T10:00", Some("Good Friday")),
            ("2027-03-28T10:00", Some("Happy Easter")),
            ("2027-05-06T10:00", Some("Ascension Day")),
            ("2027-08-15T10:00", Some("Mawlid of the Prophet")),
            ("2027-03-05T10:00", None),
        ] {
            assert_eq!(greets(when).as_deref(), expected, "{when}");
        }
    }

    #[test]
    fn the_quieter_or_rarer_day_wins_the_greeting() {
        for (when, expected) in [
            ("2027-02-08T20:00", IMLEK),
            ("2027-02-09T12:00", RAMADAN),
            ("2027-03-08T12:00", NYEPI),
            ("2027-12-26T12:00", ISRA_MIKRAJ),
            ("2027-12-25T12:00", CHRISTMAS),
        ] {
            let active = resolve(at(when), &EventSettings::in_country("ID"));
            assert_eq!(
                active.greetings().first().map(|(def, _)| def.id),
                Some(expected),
                "{when}"
            );
        }
        let nyepi = resolve(at("2027-03-08T12:00"), &EventSettings::in_country("ID"));
        assert!(nyepi.calm());
        assert!(nyepi.contains(MUDIK) && nyepi.contains(RAMADAN));
        assert!(!resolve(at("2027-03-09T06:00"), &EventSettings::in_country("ID")).calm());
    }

    #[test]
    fn long_greetings_have_a_short_form_and_the_countdown_keeps_its_clock() {
        let short = |id: &str, when: &str| {
            let text = greeting_text(find(id).unwrap(), at(when), Some("ID")).unwrap();
            short_greeting(find(id).unwrap(), &text)
        };
        assert_eq!(
            short(NYEPI, "2027-03-08T10:00").as_deref(),
            Some("Day of Silence")
        );
        assert_eq!(
            short(IMLEK, "2027-02-06T10:00").as_deref(),
            Some("Happy Chinese New Year 2578")
        );
        assert_eq!(short(IDULFITRI, "2027-03-09T21:00"), None);
        assert_eq!(short(NEW_YEAR, "2026-12-31T23:12"), None);
        assert_eq!(short(WAISAK, "2027-05-20T10:00"), None);
    }

    #[test]
    fn lunar_new_year_eve_follows_each_country_s_calendar() {
        let since = |country: &'static str, when: &str| {
            resolve(at(when), &EventSettings::in_country(country))
                .since_lunar_new_year_eve()
                .map(|since| since.num_hours())
        };
        assert_eq!(since("CN", "2027-02-05T20:00"), Some(2));
        assert_eq!(
            since("KR", "2027-02-06T20:00"),
            Some(2),
            "a day after China's"
        );
        assert_ne!(since("KR", "2027-02-05T20:00"), Some(2));
        assert_eq!(since("ID", "2027-02-05T20:00"), Some(2));
    }

    #[test]
    fn world_greetings_carry_their_years() {
        let says = |id: &str, when: &str| {
            let def = find(id).unwrap();
            greeting_text(def, at(when), def.country()).unwrap()
        };
        assert_eq!(
            says("in:independence_day", "2026-08-15T10:00"),
            "Happy 80th Independence Day, India"
        );
        assert_eq!(
            says("us:independence_day", "2026-07-04T10:00"),
            "Happy Fourth of July · America's 250th birthday"
        );
        assert_eq!(
            says("sg:national_day", "2026-08-09T10:00"),
            "Happy 61st National Day, Singapore"
        );
        assert_eq!(
            says("tr:eid_al_fitr", "2027-03-10T10:00"),
            "Happy Eid al-Fitr 1448 AH"
        );
        assert_eq!(
            says("vn:tet", "2027-02-07T10:00"),
            "Happy Lunar New Year · Year of the Goat"
        );
        assert_eq!(
            says("vn:tet", "2035-02-08T10:00"),
            "Happy Lunar New Year · Year of the Cat"
        );
        assert_eq!(
            says("kr:seollal", "2035-02-08T10:00"),
            "Happy Lunar New Year · Year of the Rabbit"
        );
        let eid = find("my:eid_al_fitr").unwrap();
        let Dates::Listed { dates, .. } = &eid.dates else {
            panic!("Eid is listed");
        };
        let later = lunar::calculated(eid, dates, 0);
        assert_eq!(later.first().map(|occurrence| occurrence.era), Some(1450));
    }

    #[test]
    fn every_researched_world_date_falls_inside_its_window() {
        for (id, dates) in world::CHECKS {
            let def = find(id).unwrap_or_else(|| panic!("{id} is generated"));
            let code = country::parse_setting(def.country().unwrap())
                .ok()
                .and_then(|setting| match setting {
                    CountrySetting::Fixed(code) => Some(code),
                    _ => None,
                })
                .unwrap_or_else(|| panic!("{id} has a known country"));
            let settings = EventSettings::in_country(code);
            let days: Vec<NaiveDate> = dates
                .iter()
                .map(|text| NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap())
                .collect();
            for day in &days {
                let late_morning = day.and_hms_opt(11, 0, 0).unwrap();
                assert!(
                    resolve(late_morning, &settings).contains(id),
                    "{id} on {day}"
                );
                for neighbour in [*day - Duration::days(1), *day + Duration::days(1)] {
                    if !days.contains(&neighbour) {
                        let noon = neighbour.and_hms_opt(11, 0, 0).unwrap();
                        assert!(
                            !resolve(noon, &settings).contains(id),
                            "{id} on {neighbour}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn the_lunar_new_year_eve_is_the_same_night_everywhere() {
        for eve in [at("2026-02-16T21:00"), at("2027-02-05T21:00")] {
            for code in ["ID", "CN", "MY"] {
                let active = resolve(eve, &EventSettings::in_country(code));
                assert!(active.has(Look::LunarNewYear), "{code} {eve}");
                assert_eq!(
                    active.since_lunar_new_year_eve(),
                    Some(Duration::hours(3)),
                    "{code}"
                );
            }
        }
        let early_block = resolve(at("2026-02-15T21:00"), &EventSettings::in_country("CN"));
        assert!(early_block.has(Look::LunarNewYear));
        assert_eq!(early_block.since_lunar_new_year_eve(), None);
        assert_eq!(
            resolve(at("2027-02-05T21:00"), &EventSettings::in_country("JP"))
                .since_lunar_new_year_eve(),
            None
        );
    }

    #[test]
    fn bare_names_still_find_indonesia_s_days_and_pick_the_country() {
        assert_eq!(find_for("nyepi", Some("ID")).map(|def| def.id), Ok(NYEPI));
        assert_eq!(find_for("idulfitri", None).map(|def| def.id), Ok(IDULFITRI));
        assert_eq!(
            find_for("halloween", Some("JP")).map(|def| def.id),
            Ok(HALLOWEEN)
        );
        assert_eq!(
            find_for("eid_al_fitr", Some("PK")).map(|def| def.id),
            Ok("pk:eid_al_fitr")
        );
        assert_eq!(
            find_for("in:diwali", Some("ID")).map(|def| def.id),
            Ok("in:diwali")
        );
        let ambiguous = find_for("eid_al_fitr", Some("ID")).unwrap_err();
        assert!(ambiguous.contains("several countries"), "{ambiguous}");
        assert!(ambiguous.contains("ng:eid_al_fitr"), "{ambiguous}");
        assert!(
            find_for("easter", None)
                .unwrap_err()
                .contains("unknown event 'easter'")
        );
        let config = crate::config::EventsConfig {
            disable: vec![
                "mudik".to_string(),
                "imlek".to_string(),
                "boxing_day".to_string(),
            ],
            ..crate::config::EventsConfig::default()
        };
        let (settings, warnings) = EventSettings::from_config(&config);
        assert!(warnings.is_empty(), "{warnings:?}");
        for id in [MUDIK, IMLEK, "gb:boxing_day", "ng:boxing_day"] {
            assert!(!settings.allows(id), "{id}");
        }
        assert!(settings.allows(NYEPI));
    }

    #[test]
    fn a_country_s_own_days_outrank_the_yearly_ones_but_not_the_countdown() {
        let greets = |when: &str, code: &'static str| {
            resolve(at(when), &EventSettings::in_country(code))
                .greetings()
                .first()
                .map(|(def, _)| def.id)
        };
        assert_eq!(greets("2026-12-26T12:00", "GB"), Some("gb:boxing_day"));
        assert_eq!(
            greets("2026-12-25T12:00", "PK"),
            Some("pk:quaid_e_azam_day")
        );
        assert_eq!(greets("2026-12-31T23:30", "RU"), Some(NEW_YEAR));
        assert_eq!(
            greets("2027-01-06T21:00", "RU"),
            Some("ru:orthodox_christmas")
        );
        assert_eq!(
            greets("2027-01-07T21:00", "RU"),
            Some("ru:orthodox_christmas")
        );
        assert_eq!(greets("2026-05-01T12:00", "BD"), Some("bd:vesak"));
        assert_eq!(greets("2026-12-24T21:00", "US"), Some(CHRISTMAS));
        assert_eq!(greets("2027-12-26T12:00", "ID"), Some(ISRA_MIKRAJ));
    }

    #[test]
    fn russia_and_ethiopia_keep_their_own_christmas_and_new_year() {
        let eve = at("2026-12-24T21:00");
        let midnight = at("2026-12-31T23:30");
        let ethiopia = EventSettings::in_country("ET");
        let russia = EventSettings::in_country("RU");
        assert!(!resolve(eve, &ethiopia).has(Look::ChristmasLights));
        assert!(!resolve(eve, &ethiopia).has(Look::Christmas));
        assert!(!resolve(midnight, &ethiopia).has(Look::NewYear));
        assert!(resolve(eve, &russia).has(Look::ChristmasLights));
        assert!(!resolve(eve, &russia).has(Look::Christmas));
        assert!(resolve(midnight, &russia).has(Look::NewYear));
        assert!(resolve(eve, &EventSettings::in_country("US")).has(Look::Christmas));
        assert!(resolve(eve, &EventSettings::default()).has(Look::Christmas));
    }

    #[test]
    fn every_country_day_greets_at_its_showcase_unless_a_shorter_one_shares_it() {
        for from in ["2025-12-31T00:00", "2026-12-31T00:00"] {
            for def in all().filter(|def| def.country().is_some() && !def.greeting.is_empty()) {
                let Some(showcase) = showcase_after(def, at(from)) else {
                    continue;
                };
                let Ok(CountrySetting::Fixed(code)) =
                    country::parse_setting(def.country().unwrap())
                else {
                    panic!("{}", def.id);
                };
                let active = resolve(showcase, &EventSettings::in_country(code));
                assert!(active.contains(def.id), "{} at {showcase}", def.id);
                let Some((winner, _)) = active.greetings().into_iter().next() else {
                    panic!("{} at {showcase}: no greeting", def.id);
                };
                let span =
                    |def: &EventDef| next_window(def, showcase).map(|(start, end)| end - start);
                assert!(
                    winner.id == def.id
                        || winner.id == NEW_YEAR
                        || (is_observance(def) && !is_observance(winner))
                        || (winner.country() == def.country() && span(winner) <= span(def)),
                    "{} at {showcase} greets as {}",
                    def.id,
                    winner.id
                );
            }
        }
    }

    #[test]
    fn the_generated_checks_match_the_holiday_tables() {
        let docs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/holidays");
        let mut tables: std::collections::HashMap<String, Vec<String>> = Default::default();
        for entry in std::fs::read_dir(&docs).unwrap() {
            let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
            for block in text.split("```toml").skip(1) {
                let block = block.split("```").next().unwrap();
                let value: toml::Value = toml::from_str(block).unwrap();
                for holiday in value["holiday"].as_array().unwrap() {
                    let country = holiday["country"].as_str().unwrap().to_ascii_lowercase();
                    let country = if country == "uk" {
                        "gb".to_string()
                    } else {
                        country
                    };
                    let id = format!("{country}:{}", holiday["id"].as_str().unwrap());
                    let mut dates: Vec<String> = holiday
                        .as_table()
                        .unwrap()
                        .iter()
                        .filter(|(key, _)| key.starts_with("dates_"))
                        .flat_map(|(_, list)| list.as_array().unwrap().iter())
                        .map(|date| date.as_str().unwrap().to_string())
                        .collect();
                    dates.sort();
                    tables.insert(id, dates);
                }
            }
        }
        for (id, dates) in world::CHECKS {
            let mut expected: Vec<String> = dates.iter().map(|date| date.to_string()).collect();
            expected.sort();
            assert_eq!(tables.get(*id), Some(&expected), "{id}");
        }
        let generated: std::collections::HashSet<&str> =
            world::CHECKS.iter().map(|(id, _)| *id).collect();
        let left_out: Vec<&String> = tables
            .keys()
            .filter(|id| !generated.contains(id.as_str()))
            .filter(|id| !id.ends_with(":christmas") && !id.ends_with(":new_year"))
            .filter(|id| !id.starts_with("mx:media_asta_"))
            .collect();
        assert_eq!(
            left_out,
            [&"et:derg_downfall_day".to_string()],
            "{left_out:?}"
        );
        let mexico_lowered = world::CHECKS
            .iter()
            .filter(|(id, _)| id.starts_with("mx:media_asta_"))
            .count();
        assert_eq!(
            mexico_lowered, 9,
            "only the best-known Mexican half-mast days"
        );
    }

    #[test]
    fn day_rules_land_on_the_right_dates() {
        let day = |rule, year| rule_day(rule, year).map(|date| date.format("%Y-%m-%d").to_string());
        for (rule, year, expected) in [
            (DayRule::Fixed(7, 4), 2026, "2026-07-04"),
            (DayRule::Nth(11, Weekday::Thu, 4), 2026, "2026-11-26"),
            (DayRule::Nth(11, Weekday::Thu, 4), 2027, "2027-11-25"),
            (DayRule::Nth(1, Weekday::Mon, 3), 2027, "2027-01-18"),
            (DayRule::Last(5, Weekday::Mon), 2026, "2026-05-25"),
            (DayRule::Last(5, Weekday::Mon), 2027, "2027-05-31"),
            (DayRule::Last(12, Weekday::Fri), 2026, "2026-12-25"),
            (DayRule::Easter(1), 2027, "2027-03-29"),
            (DayRule::Easter(-47), 2027, "2027-02-09"),
            (DayRule::Orthodox(0), 2024, "2024-05-05"),
            (DayRule::Orthodox(0), 2025, "2025-04-20"),
            (DayRule::Orthodox(0), 2026, "2026-04-12"),
            (DayRule::Orthodox(-2), 2027, "2027-04-30"),
            (DayRule::Equinox(Season::Spring), 2026, "2026-03-20"),
            (DayRule::Equinox(Season::Autumn), 2026, "2026-09-23"),
            (DayRule::Equinox(Season::Spring), 2027, "2027-03-21"),
            (DayRule::Equinox(Season::Autumn), 2028, "2028-09-22"),
        ] {
            assert_eq!(
                day(rule, year).as_deref(),
                Some(expected),
                "{rule:?} {year}"
            );
        }
        assert_eq!(rule_day(DayRule::Nth(2, Weekday::Mon, 5), 2027), None);
    }

    #[test]
    fn a_ruled_day_with_an_eve_starts_at_six_the_evening_before() {
        let occurrence = ruled(DayRule::Fixed(9, 16), 1, true, -3, 2026).unwrap();
        assert_eq!(occurrence.start, at("2026-09-15T18:00"));
        assert_eq!(occurrence.end, at("2026-09-17T00:00"));
        assert_eq!(occurrence.showcase, at("2026-09-15T21:00"));
    }

    #[test]
    fn only_the_located_country_s_days_resolve() {
        let nyepi = at("2027-03-08T12:00");
        assert!(resolve(nyepi, &EventSettings::in_country("ID")).contains(NYEPI));
        assert!(!resolve(nyepi, &EventSettings::default()).contains(NYEPI));
        assert!(!resolve(nyepi, &EventSettings::in_country("JP")).contains(NYEPI));
        let halloween_night = at("2026-10-31T21:00");
        for settings in [EventSettings::default(), EventSettings::in_country("JP")] {
            assert!(resolve(halloween_night, &settings).contains(HALLOWEEN));
        }
        let fitri = resolve(at("2027-03-10T08:00"), &EventSettings::in_country("ID"));
        assert!(fitri.has(Look::Ketupat) && fitri.has(Look::Takbiran));
        assert!(!fitri.has(Look::Calm));
        assert_eq!(fitri.country(), Some("ID"));
    }

    #[test]
    fn christmas_and_new_year_are_english_everywhere() {
        let christmas = find(CHRISTMAS).unwrap();
        let new_year = find(NEW_YEAR).unwrap();
        let eve = at("2026-12-24T21:00");
        let late = at("2026-12-31T23:12");
        let day = at("2027-01-01T10:00");
        for (country, merry) in [
            (Some("ID"), Some("Merry Christmas")),
            (Some("US"), Some("Merry Christmas")),
            (Some("GB"), Some("Merry Christmas")),
            (Some("BR"), Some("Merry Christmas")),
            (Some("CN"), Some("Merry Christmas")),
            (Some("RU"), None),
            (Some("ET"), None),
            (None, Some("Merry Christmas")),
        ] {
            assert_eq!(
                greeting_text(christmas, eve, country).as_deref(),
                merry,
                "{country:?}"
            );
            assert_eq!(
                greeting_text(new_year, late, country).as_deref(),
                Some("2027 in 00:48"),
                "{country:?}"
            );
            assert_eq!(
                greeting_text(new_year, day, country).as_deref(),
                Some("Happy New Year 2027"),
                "{country:?}"
            );
        }
    }

    #[test]
    fn since_start_measures_from_the_current_window() {
        let eve = resolve(at("2027-03-09T21:30"), &EventSettings::in_country("ID"));
        assert_eq!(
            eve.since_start(IDULFITRI),
            Some(Duration::hours(3) + Duration::minutes(30))
        );
        assert_eq!(eve.since_start(IMLEK), None);
    }
}
