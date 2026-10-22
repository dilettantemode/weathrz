use super::catalogue::{self, KITE, WITCH};
use super::specials::DEFAULT_SPECIALS;
use crate::config::SkyConfig;
use std::ops::RangeInclusive;

const FPS: f32 = 30.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Every {
    Calm,
    Normal,
    Busy,
    Seconds(f32),
}

impl Every {
    pub fn parse(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().as_str() {
            "calm" => Some(Self::Calm),
            "normal" => Some(Self::Normal),
            "busy" => Some(Self::Busy),
            other => other
                .parse::<f32>()
                .ok()
                .filter(|seconds| seconds.is_finite() && *seconds >= 1.0)
                .map(Self::Seconds),
        }
    }

    pub fn gap_frames(self) -> RangeInclusive<u32> {
        let seconds = |low: f32, high: f32| ((low * FPS) as u32)..=((high * FPS) as u32);
        match self {
            Self::Calm => seconds(120.0, 180.0),
            Self::Normal => seconds(40.0, 80.0),
            Self::Busy => seconds(20.0, 40.0),
            Self::Seconds(gap) => seconds(gap * 0.75, gap * 1.25),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlyerSettings {
    pub enabled: Vec<&'static str>,
    pub characters: Vec<&'static str>,
    pub every: Every,
    pub specials: u32,
    pub banner: Option<String>,
    pub preview: Option<&'static str>,
    pub moon_hidden: bool,
}

impl Default for FlyerSettings {
    fn default() -> Self {
        Self {
            enabled: catalogue::aircraft_ids(),
            characters: catalogue::character_ids(),
            every: Every::Calm,
            specials: DEFAULT_SPECIALS,
            banner: None,
            preview: None,
            moon_hidden: false,
        }
    }
}

impl FlyerSettings {
    pub fn from_config(config: &SkyConfig) -> (Self, Vec<String>) {
        let mut warnings = config.warnings.clone();
        let mut enabled: Vec<&'static str> = Vec::new();
        for name in &config.flyers {
            let name = name.trim();
            match catalogue::find(name) {
                Some(def) if def.id == WITCH => warnings.push(
                    "sky.flyers: 'witch' only flies on Halloween nights; ignored".to_string(),
                ),
                Some(def) => {
                    if !enabled.contains(&def.id) {
                        enabled.push(def.id);
                    }
                }
                None => warnings.push(format!(
                    "sky.flyers: unknown flyer '{name}' ignored (known: {})",
                    catalogue::known_aircraft()
                )),
            }
        }
        if enabled.is_empty() {
            if !config.flyers.is_empty() {
                warnings.push("sky.flyers: no known flyers listed; using all of them".to_string());
            }
            enabled = catalogue::aircraft_ids();
        }
        let mut characters: Vec<&'static str> = Vec::new();
        for name in &config.characters {
            let name = name.trim();
            match catalogue::find_character(name) {
                Some(def) => {
                    if !characters.contains(&def.id) {
                        characters.push(def.id);
                    }
                }
                None if name == WITCH => warnings.push(
                    "sky.characters: 'witch' only flies on Halloween nights; ignored".to_string(),
                ),
                None if name == KITE => warnings.push(
                    "sky.characters: 'kite' is not a flyer; it comes out on windy days (hide it with hide = [\"kite\"]); ignored"
                        .to_string(),
                ),
                None => warnings.push(format!(
                    "sky.characters: unknown character '{name}' ignored (known: {})",
                    catalogue::known_characters()
                )),
            }
        }
        if characters.is_empty() {
            if !config.characters.is_empty() {
                warnings.push(
                    "sky.characters: no known characters listed; using all of them".to_string(),
                );
            }
            characters = catalogue::character_ids();
        }
        let specials = match config.specials.as_deref() {
            None => DEFAULT_SPECIALS,
            Some(text) => text
                .trim()
                .parse::<u32>()
                .ok()
                .filter(|count| *count <= 1000)
                .unwrap_or_else(|| {
                    warnings.push(format!(
                        "sky.specials = \"{text}\" is not a whole number from 0 to 1000; using {DEFAULT_SPECIALS}"
                    ));
                    DEFAULT_SPECIALS
                }),
        };
        let every = match config.every.as_deref() {
            None => Every::Calm,
            Some(text) => Every::parse(text).unwrap_or_else(|| {
                warnings.push(format!(
                    "sky.every = \"{text}\" is not calm, normal, busy or a number of seconds (1 or more); using calm"
                ));
                Every::Calm
            }),
        };
        let banner = config
            .banner
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(str::to_string);
        (
            Self {
                enabled,
                characters,
                every,
                specials,
                banner,
                preview: None,
                moon_hidden: false,
            },
            warnings,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sky(flyers: &[&str], every: Option<&str>, banner: Option<&str>) -> SkyConfig {
        SkyConfig {
            flyers: flyers.iter().map(|name| name.to_string()).collect(),
            every: every.map(str::to_string),
            banner: banner.map(str::to_string),
            ..SkyConfig::default()
        }
    }

    #[test]
    fn every_accepts_presets_and_seconds() {
        assert_eq!(Every::parse("calm"), Some(Every::Calm));
        assert_eq!(Every::parse(" Normal "), Some(Every::Normal));
        assert_eq!(Every::parse("busy"), Some(Every::Busy));
        assert_eq!(Every::parse("90"), Some(Every::Seconds(90.0)));
        assert_eq!(Every::parse("2.5"), Some(Every::Seconds(2.5)));
        for bad in ["0", "-5", "fast", "", "inf", "NaN"] {
            assert_eq!(Every::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn gaps_follow_the_presets() {
        assert_eq!(Every::Calm.gap_frames(), 3600..=5400);
        assert_eq!(Every::Normal.gap_frames(), 1200..=2400);
        assert_eq!(Every::Busy.gap_frames(), 600..=1200);
        assert_eq!(Every::Seconds(60.0).gap_frames(), 1350..=2250);
    }

    #[test]
    fn defaults_fly_every_aircraft_calmly() {
        let (settings, warnings) = FlyerSettings::from_config(&SkyConfig::default());
        assert_eq!(settings, FlyerSettings::default());
        assert!(warnings.is_empty());
        assert!(!settings.enabled.contains(&WITCH));
    }

    #[test]
    fn config_typos_warn_and_fall_back() {
        let (settings, warnings) = FlyerSettings::from_config(&sky(
            &["plane", " plane ", "jumbo", "witch"],
            Some("sometimes"),
            Some("  Selamat pagi  "),
        ));
        assert_eq!(settings.enabled, vec!["plane"]);
        assert_eq!(settings.every, Every::Calm);
        assert_eq!(settings.banner.as_deref(), Some("Selamat pagi"));
        assert_eq!(warnings.len(), 3, "{warnings:?}");
        assert!(warnings[0].starts_with("sky.flyers: unknown flyer 'jumbo' ignored (known: "));
        assert!(warnings[1].contains("'witch' only flies on Halloween nights"));
        assert!(warnings[2].starts_with("sky.every = \"sometimes\""));

        let (settings, warnings) = FlyerSettings::from_config(&sky(&["jumbo"], None, Some(" ")));
        assert_eq!(settings.enabled, catalogue::aircraft_ids());
        assert_eq!(settings.banner, None);
        assert!(
            warnings
                .iter()
                .any(|w| w.contains("no known flyers listed"))
        );
    }

    fn sky_with(characters: &[&str], specials: Option<&str>) -> SkyConfig {
        SkyConfig {
            characters: characters.iter().map(|name| name.to_string()).collect(),
            specials: specials.map(str::to_string),
            ..SkyConfig::default()
        }
    }

    #[test]
    fn characters_and_specials_default_to_all_and_twelve() {
        let (settings, warnings) = FlyerSettings::from_config(&SkyConfig::default());
        assert_eq!(settings.characters, catalogue::character_ids());
        assert_eq!(settings.specials, 12);
        assert!(warnings.is_empty());
    }

    #[test]
    fn character_typos_warn_and_fall_back() {
        let (settings, warnings) = FlyerSettings::from_config(&sky_with(
            &["ufo", " ufo ", "griffin", "witch", "kite", "plane"],
            Some("7"),
        ));
        assert_eq!(settings.characters, vec!["ufo"]);
        assert_eq!(settings.specials, 7);
        assert_eq!(warnings.len(), 4, "{warnings:?}");
        assert!(
            warnings[0].starts_with("sky.characters: unknown character 'griffin' ignored (known: ")
        );
        assert!(warnings[1].contains("'witch' only flies on Halloween nights"));
        assert!(warnings[2].contains("'kite' is not a flyer"));
        assert!(warnings[3].starts_with("sky.characters: unknown character 'plane'"));

        let (settings, warnings) = FlyerSettings::from_config(&sky_with(&["griffin"], Some("0")));
        assert_eq!(settings.characters, catalogue::character_ids());
        assert_eq!(settings.specials, 0);
        assert!(
            warnings
                .iter()
                .any(|w| w.contains("no known characters listed"))
        );
    }

    #[test]
    fn bad_specials_warn_and_use_twelve() {
        for bad in ["-1", "1001", "2.5", "often", ""] {
            let (settings, warnings) = FlyerSettings::from_config(&sky_with(&[], Some(bad)));
            assert_eq!(settings.specials, 12, "{bad}");
            assert_eq!(
                warnings,
                vec![format!(
                    "sky.specials = \"{bad}\" is not a whole number from 0 to 1000; using 12"
                )]
            );
        }
        let (settings, _) = FlyerSettings::from_config(&sky_with(&[], Some(" 1000 ")));
        assert_eq!(settings.specials, 1000);
    }

    #[test]
    fn character_ids_are_unknown_in_the_aircraft_list() {
        let (settings, warnings) = FlyerSettings::from_config(&sky(&["ufo"], None, None));
        assert_eq!(settings.enabled, catalogue::aircraft_ids());
        assert!(warnings[0].starts_with("sky.flyers: unknown flyer 'ufo'"));
    }
}
