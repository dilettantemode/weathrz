use super::characters::CHARACTERS;
use std::ops::Range;

pub const WITCH: &str = "witch";
pub const SLEIGH: &str = "sleigh";
pub const KITE: &str = "kite";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pace {
    Slow,
    Normal,
    Fast,
}

impl Pace {
    pub fn columns_per_frame(self) -> Range<f32> {
        match self {
            Pace::Slow => 0.1..0.2,
            Pace::Normal => 0.3..0.5,
            Pace::Fast => 0.8..1.2,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Pace::Slow => "slow",
            Pace::Normal => "normal",
            Pace::Fast => "fast",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum When {
    Any,
    CalmDay,
    Day,
    Night,
    MoonNight,
    HalloweenNight,
    ChristmasEveNight,
}

impl When {
    pub fn label(self) -> &'static str {
        match self {
            When::Any => "any time",
            When::CalmDay => "calm days",
            When::Day => "by day",
            When::Night => "at night",
            When::MoonNight => "moonlit nights",
            When::HalloweenNight => "Halloween nights",
            When::ChristmasEveNight => "Christmas Eve night",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colouring {
    Airplane,
    Witch,
    Mask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Aircraft,
    Witch,
    Character,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fill {
    None,
    Same,
    Mirror,
}

#[derive(Debug)]
pub struct FlyerDef {
    pub id: &'static str,
    pub credit: &'static str,
    pub right: Option<&'static str>,
    pub left: Option<&'static str>,
    pub fill: Fill,
    pub pace: Pace,
    pub when: When,
    pub colouring: Colouring,
    pub kind: Kind,
    pub mask: Option<&'static str>,
}

pub const CATALOGUE: &[FlyerDef] = &[
    FlyerDef {
        id: "plane",
        credit: "P1 + P9, jgs",
        right: Some(include_str!("../assets/flyers/plane-right.txt")),
        left: Some(include_str!("../assets/flyers/plane-left.txt")),
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "prop",
        credit: "P10, dew",
        right: Some(include_str!("../assets/flyers/prop-right.txt")),
        left: None,
        fill: Fill::Mirror,
        pace: Pace::Normal,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "tiny-plane",
        credit: "P8 tiny airplane, jgs",
        right: Some(include_str!("../assets/flyers/tiny-plane-right.txt")),
        left: None,
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Day,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "mini-plane",
        credit: "P11, wkm",
        right: Some(include_str!("../assets/flyers/mini-plane-right.txt")),
        left: None,
        fill: Fill::Mirror,
        pace: Pace::Normal,
        when: When::Day,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "trijet",
        credit: "P12 three-engine jet, kh",
        right: None,
        left: Some(include_str!("../assets/flyers/trijet-left.txt")),
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "banner",
        credit: "P13 banner plane, unknown",
        right: None,
        left: Some(include_str!("../assets/flyers/banner-left.txt")),
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Day,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "biplane",
        credit: "P14, Paulo Santos",
        right: None,
        left: Some(include_str!("../assets/flyers/biplane-left.txt")),
        fill: Fill::Mirror,
        pace: Pace::Normal,
        when: When::Day,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "airliner",
        credit: "P16, Roland",
        right: Some(include_str!("../assets/flyers/airliner-right.txt")),
        left: None,
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "fighter",
        credit: "P17 fighter jet, Billcow168",
        right: Some(include_str!("../assets/flyers/fighter-right.txt")),
        left: None,
        fill: Fill::Mirror,
        pace: Pace::Fast,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "heli",
        credit: "P18, Rob Logan",
        right: Some(include_str!("../assets/flyers/heli-right.txt")),
        left: None,
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "heli-59",
        credit: "P19, pr59",
        right: Some(include_str!("../assets/flyers/heli-59-right.txt")),
        left: None,
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "heli-tiny",
        credit: "P22 six tiny helicopters, sl",
        right: None,
        left: Some(include_str!("../assets/flyers/heli-tiny-left.txt")),
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::Any,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "blimp",
        credit: "P4 dirigible, jgs",
        right: None,
        left: Some(include_str!("../assets/flyers/blimp-left.txt")),
        fill: Fill::Mirror,
        pace: Pace::Slow,
        when: When::CalmDay,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "balloon",
        credit: "P23 hot-air balloon, fsc",
        right: Some(include_str!("../assets/flyers/balloon.txt")),
        left: None,
        fill: Fill::Same,
        pace: Pace::Slow,
        when: When::CalmDay,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: "airship",
        credit: "P24 rigid airship, unknown",
        right: None,
        left: Some(include_str!("../assets/flyers/airship-left.txt")),
        fill: Fill::Mirror,
        pace: Pace::Slow,
        when: When::CalmDay,
        colouring: Colouring::Airplane,
        kind: Kind::Aircraft,
        mask: None,
    },
    FlyerDef {
        id: WITCH,
        credit: "W1 small witches, jgs",
        right: Some(include_str!("../assets/flyers/witch-right.txt")),
        left: Some(include_str!("../assets/flyers/witch-left.txt")),
        fill: Fill::None,
        pace: Pace::Normal,
        when: When::HalloweenNight,
        colouring: Colouring::Witch,
        kind: Kind::Witch,
        mask: None,
    },
    FlyerDef {
        id: SLEIGH,
        credit: "sleigh and reindeer, jgs",
        right: None,
        left: Some(include_str!("../assets/flyers/sleigh-left.txt")),
        fill: Fill::Mirror,
        pace: Pace::Normal,
        when: When::ChristmasEveNight,
        colouring: Colouring::Mask,
        kind: Kind::Witch,
        mask: Some(include_str!("../assets/flyers/sleigh-left.mask.txt")),
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sprite {
    pub rows: Vec<String>,
}

impl Sprite {
    pub fn width(&self) -> u16 {
        self.rows
            .iter()
            .map(|row| row.chars().count())
            .max()
            .unwrap_or(0) as u16
    }

    pub fn height(&self) -> u16 {
        self.rows.len() as u16
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub frames: Vec<Sprite>,
    pub masks: Vec<Sprite>,
}

impl Variant {
    pub fn still(sprite: Sprite) -> Self {
        Self {
            frames: vec![sprite],
            masks: Vec::new(),
        }
    }

    pub fn width(&self) -> u16 {
        self.frames.iter().map(Sprite::width).max().unwrap_or(0)
    }

    pub fn height(&self) -> u16 {
        self.frames.iter().map(Sprite::height).max().unwrap_or(0)
    }

    pub(crate) fn mirrored(&self) -> Option<Self> {
        let width = self.width() as usize;
        Some(Self {
            frames: self
                .frames
                .iter()
                .map(|frame| super::mirror::mirror_sprite_at(frame, width))
                .collect::<Option<Vec<_>>>()?,
            masks: self
                .masks
                .iter()
                .map(|mask| super::mask::mirror_mask(mask, width))
                .collect(),
        })
    }
}

pub struct Model {
    pub def: &'static FlyerDef,
    pub right: Vec<Variant>,
    pub left: Vec<Variant>,
}

impl Model {
    pub fn height(&self) -> u16 {
        self.right
            .iter()
            .chain(&self.left)
            .map(Variant::height)
            .max()
            .unwrap_or(0)
    }

    pub fn width(&self) -> u16 {
        self.right
            .iter()
            .chain(&self.left)
            .map(Variant::width)
            .max()
            .unwrap_or(0)
    }

    pub fn directions(&self) -> &'static str {
        match (self.right.is_empty(), self.left.is_empty()) {
            (false, false) => "both ways",
            (false, true) => "right",
            (true, false) => "left",
            (true, true) => "none",
        }
    }
}

pub fn parse_asset(text: &str) -> Vec<Sprite> {
    let body: Vec<&str> = text.lines().filter(|line| !line.starts_with('#')).collect();
    body.split(|line| *line == "%")
        .map(normalise)
        .filter(|sprite| !sprite.rows.is_empty())
        .collect()
}

fn normalise(lines: &[&str]) -> Sprite {
    let mut rows: Vec<&str> = lines.iter().map(|line| line.trim_end()).collect();
    while rows.last().is_some_and(|row| row.is_empty()) {
        rows.pop();
    }
    while rows.first().is_some_and(|row| row.is_empty()) {
        rows.remove(0);
    }
    let indent = rows
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.len() - row.trim_start().len())
        .min()
        .unwrap_or(0);
    Sprite {
        rows: rows
            .iter()
            .map(|row| row.get(indent..).unwrap_or("").to_string())
            .collect(),
    }
}

pub fn parse_frames(text: &str) -> Vec<Sprite> {
    let body: Vec<&str> = text
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(str::trim_end)
        .collect();
    let blocks: Vec<&[&str]> = body.split(|line| *line == "---").collect();
    let height = blocks.iter().map(|block| block.len()).max().unwrap_or(0);
    let blank_everywhere = |row: usize| {
        blocks
            .iter()
            .all(|block| block.get(row).is_none_or(|line| line.is_empty()))
    };
    let first = (0..height)
        .find(|&row| !blank_everywhere(row))
        .unwrap_or(height);
    let last = (first..height)
        .rev()
        .find(|&row| !blank_everywhere(row))
        .map_or(first, |row| row + 1);
    let indent = blocks
        .iter()
        .flat_map(|block| block.iter())
        .filter(|line| !line.is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    blocks
        .iter()
        .map(|block| Sprite {
            rows: (first..last)
                .map(|row| {
                    block
                        .get(row)
                        .and_then(|line| line.get(indent..))
                        .unwrap_or("")
                        .to_string()
                })
                .collect(),
        })
        .collect()
}

fn drawn(art: Option<&'static str>, mask: Option<&'static str>) -> Vec<Variant> {
    match (art, mask) {
        (Some(art), Some(mask)) => vec![Variant {
            frames: parse_frames(art),
            masks: parse_frames(mask),
        }],
        (Some(art), None) => parse_asset(art).into_iter().map(Variant::still).collect(),
        (None, _) => Vec::new(),
    }
}

fn mirrored(variants: &[Variant]) -> Vec<Variant> {
    variants
        .iter()
        .map(Variant::mirrored)
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default()
}

pub fn models() -> Vec<Model> {
    definitions()
        .map(|def| {
            let mut right = drawn(def.right, def.mask);
            let mut left = drawn(def.left, def.mask);
            if def.fill == Fill::Same {
                if left.is_empty() {
                    left = right.clone();
                }
                if right.is_empty() {
                    right = left.clone();
                }
            }
            if def.fill == Fill::Mirror {
                if left.is_empty() {
                    left = mirrored(&right);
                }
                if right.is_empty() {
                    right = mirrored(&left);
                }
            }
            Model { def, right, left }
        })
        .collect()
}

pub fn definitions() -> impl Iterator<Item = &'static FlyerDef> {
    CATALOGUE.iter().chain(CHARACTERS)
}

pub fn character_ids() -> Vec<&'static str> {
    CHARACTERS.iter().map(|def| def.id).collect()
}

pub fn known_characters() -> String {
    character_ids().join(", ")
}

pub fn find_character(id: &str) -> Option<&'static FlyerDef> {
    CHARACTERS.iter().find(|def| def.id == id)
}

pub fn find_any(id: &str) -> Option<&'static FlyerDef> {
    definitions().find(|def| def.id == id)
}

pub fn find(id: &str) -> Option<&'static FlyerDef> {
    CATALOGUE.iter().find(|def| def.id == id)
}

pub fn aircraft_ids() -> Vec<&'static str> {
    CATALOGUE
        .iter()
        .filter(|def| def.kind == Kind::Aircraft)
        .map(|def| def.id)
        .collect()
}

pub fn known_aircraft() -> String {
    aircraft_ids().join(", ")
}

fn listing_line(model: &Model, width: usize) -> String {
    format!(
        "{:<width$}  {:>2}x{:<3}  {:<9}  {:<6}  {:<16}  {}",
        model.def.id,
        model.height(),
        model.width(),
        model.directions(),
        model.def.pace.label(),
        model.def.when.label(),
        model.def.credit,
    )
}

pub fn format_listing() -> String {
    let models = models();
    let width = models
        .iter()
        .map(|model| model.def.id.len())
        .max()
        .unwrap_or(0);
    let section = |characters: bool| {
        models
            .iter()
            .filter(|model| (model.def.kind == Kind::Character) == characters)
            .map(|model| listing_line(model, width))
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "Aircraft\n{}\n\nCharacters\n{}\n\nKite\n{:<width$}  by day in wind of 10 km/h or more, 3-5 min spells  {}",
        section(false),
        section(true),
        KITE,
        crate::animation::kite::CREDIT,
    )
}

#[cfg(test)]
mod tests {
    use super::super::characters::CHARACTERS;
    use super::*;

    #[test]
    fn parse_asset_strips_headers_indent_and_splits_variants() {
        let sprites = parse_asset("# credit\n   ab\n    c  \n%\n\n x\n");
        assert_eq!(
            sprites,
            vec![
                Sprite {
                    rows: vec!["ab".to_string(), " c".to_string()]
                },
                Sprite {
                    rows: vec!["x".to_string()]
                },
            ]
        );
    }

    #[test]
    fn every_flyer_parses_and_fits_the_sky() {
        for model in models() {
            let id = model.def.id;
            let cap = if model.def.kind == Kind::Character {
                9
            } else {
                8
            };
            assert_ne!(model.directions(), "none", "{id}");
            assert!(model.height() <= cap, "{id} is {} rows", model.height());
            for variant in model.right.iter().chain(&model.left) {
                let rows: Vec<&String> = variant
                    .frames
                    .iter()
                    .flat_map(|frame| &frame.rows)
                    .collect();
                assert!(rows.iter().all(|row| *row == row.trim_end()), "{id}");
                assert!(
                    rows.iter()
                        .any(|row| !row.is_empty() && !row.starts_with(' ')),
                    "{id} keeps a common indent"
                );
            }
        }
    }

    #[test]
    fn every_asset_credits_its_artist() {
        for def in CATALOGUE {
            for text in [def.right, def.left].into_iter().flatten() {
                assert!(text.starts_with("# "), "{}", def.id);
            }
        }
    }

    #[test]
    fn the_witch_flies_both_ways_on_halloween_nights() {
        let witch = models()
            .into_iter()
            .find(|model| model.def.id == WITCH)
            .unwrap();
        assert_eq!((witch.height(), witch.width()), (7, 12));
        assert_eq!(witch.directions(), "both ways");
        assert_eq!(witch.def.when, When::HalloweenNight);
        assert!(!aircraft_ids().contains(&WITCH));
        assert!(aircraft_ids().contains(&"plane"));
        assert_eq!(find("plane").map(|def| def.id), Some("plane"));
        assert!(find("kite").is_none());
    }

    #[test]
    fn listing_has_aircraft_and_character_sections() {
        let listing = format_listing();
        let lines: Vec<&str> = listing.lines().collect();
        assert_eq!(lines[0], "Aircraft");
        let characters = lines
            .iter()
            .position(|line| *line == "Characters")
            .expect("a Characters heading");
        assert_eq!(characters, CATALOGUE.len() + 2);
        assert_eq!(lines[characters - 1], "");
        assert!(
            lines[1..characters]
                .iter()
                .any(|line| line.starts_with("witch")
                    && line.contains("Halloween nights")
                    && line.contains("both ways"))
        );
        let kite = lines
            .iter()
            .position(|line| *line == "Kite")
            .expect("a Kite heading");
        let below = &lines[characters + 1..kite - 1];
        for def in CHARACTERS {
            assert!(
                below.iter().any(|line| line.starts_with(def.id)),
                "{}",
                def.id
            );
        }
        assert!(
            below
                .iter()
                .any(|line| line.starts_with("ufo") && line.contains("at night"))
        );
        assert_eq!(lines.len(), kite + 2);
        assert!(lines[kite + 1].starts_with("kite "));
        assert!(lines[kite + 1].contains("10 km/h") && lines[kite + 1].contains("Tate"));
    }

    #[test]
    fn no_artist_signature_is_left_in_the_art() {
        let signatures = [
            "jgs",
            "ldb",
            "ejm",
            "ejm98",
            "hjw",
            "tre",
            "dew",
            "wkm",
            "kh",
            "fsc",
            "sl",
            "wh",
            "pr59",
            "bp",
            "gpyy",
            "bf",
            "roland",
            "billcow168",
            "llizard",
        ];
        for def in definitions() {
            for text in [def.right, def.left].into_iter().flatten() {
                for line in text.lines().filter(|line| !line.starts_with('#')) {
                    for word in line.split(|ch: char| !ch.is_ascii_alphanumeric()) {
                        assert!(
                            !signatures.contains(&word.to_lowercase().as_str()),
                            "{}: '{word}' in {line:?}",
                            def.id
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_flyer_keeps_its_hours() {
        let hours: Vec<(&str, &str)> = models()
            .iter()
            .map(|model| (model.def.id, model.def.when.label()))
            .collect();
        assert_eq!(
            hours,
            [
                ("plane", "any time"),
                ("prop", "any time"),
                ("tiny-plane", "by day"),
                ("mini-plane", "by day"),
                ("trijet", "any time"),
                ("banner", "by day"),
                ("biplane", "by day"),
                ("airliner", "any time"),
                ("fighter", "any time"),
                ("heli", "any time"),
                ("heli-59", "any time"),
                ("heli-tiny", "any time"),
                ("blimp", "calm days"),
                ("balloon", "calm days"),
                ("airship", "calm days"),
                ("witch", "Halloween nights"),
                ("sleigh", "Christmas Eve night"),
                ("caped-hero", "any time"),
                ("ufo", "at night"),
                ("moon-biker", "moonlit nights"),
                ("broom-rider", "by day"),
                ("fairy", "at night"),
                ("pterosaur", "by day"),
                ("flying-car", "any time"),
                ("dragon", "by day"),
                ("long-dragon", "at night"),
            ]
        );
    }

    #[test]
    fn fifteen_aircraft_and_the_witch() {
        assert_eq!(aircraft_ids().len(), 15);
        assert_eq!(CATALOGUE.len(), 17);
        assert!(find("heli-long").is_none() && find("chinook").is_none());
        for model in models() {
            if model.def.fill == Fill::Mirror || model.def.fill == Fill::Same {
                assert_eq!(model.directions(), "both ways", "{}", model.def.id);
            }
        }
        let tiny = models()
            .into_iter()
            .find(|m| m.def.id == "heli-tiny")
            .unwrap();
        assert_eq!(tiny.left.len(), 6);
    }

    #[test]
    fn parse_frames_keeps_inner_blank_rows_and_one_shared_indent() {
        let frames = parse_frames("# credit\n   ab\n\n---\n\n    c\n");
        assert_eq!(
            frames,
            vec![
                Sprite {
                    rows: vec!["ab".to_string(), String::new()]
                },
                Sprite {
                    rows: vec![String::new(), " c".to_string()]
                },
            ]
        );
        let trimmed = parse_frames("\n x\n\n---\n\n y\n\n");
        assert_eq!(
            trimmed,
            vec![
                Sprite {
                    rows: vec!["x".to_string()]
                },
                Sprite {
                    rows: vec!["y".to_string()]
                },
            ]
        );
    }

    #[test]
    fn frames_mirror_at_the_piece_width_so_they_stay_aligned() {
        let sprite = |row: &str| Sprite {
            rows: vec![row.to_string()],
        };
        let right = Variant {
            frames: vec![sprite("/="), sprite("/==")],
            masks: vec![sprite("ab"), sprite("abc")],
        };
        let left = right.mirrored().unwrap();
        assert_eq!(left.frames, vec![sprite(" =\\"), sprite("==\\")]);
        assert_eq!(left.masks, vec![sprite(" ba"), sprite("cba")]);
        assert_eq!(left.mirrored(), Some(right));
        assert_eq!(Variant::still(sprite("PAN")).mirrored(), None);
    }

    fn source_frames(text: &str) -> Vec<usize> {
        text.lines()
            .filter(|line| !line.starts_with('#'))
            .collect::<Vec<_>>()
            .split(|line| line.trim_end() == "---")
            .map(<[&str]>::len)
            .collect()
    }

    #[test]
    fn every_character_has_a_matching_mask() {
        for def in CHARACTERS
            .iter()
            .chain(CATALOGUE.iter().filter(|def| def.mask.is_some()))
        {
            let id = def.id;
            assert!(
                def.right.is_none() || def.left.is_none(),
                "{id} draws one side"
            );
            let art = def.right.or(def.left).expect("a drawn side");
            let mask = def.mask.expect("a mask");
            assert!(art.starts_with("# "), "{id}");
            assert!(mask.starts_with("# mask for "), "{id}");
            let rows = source_frames(art);
            assert!(
                rows.windows(2).all(|pair| pair[0] == pair[1]),
                "{id} {rows:?}"
            );
            assert_eq!(rows, source_frames(mask), "{id}");
            let frames = parse_frames(art);
            let masks = parse_frames(mask);
            assert_eq!(frames.len(), masks.len(), "{id}");
            for (frame, mask) in frames.iter().zip(&masks) {
                assert_eq!(frame.rows.len(), mask.rows.len(), "{id}");
                for (row, letters) in frame.rows.iter().zip(&mask.rows) {
                    let width = row.len().max(letters.len());
                    let art_cells = format!("{row:<width$}");
                    let mask_cells = format!("{letters:<width$}");
                    for (ch, letter) in art_cells.chars().zip(mask_cells.chars()) {
                        assert_eq!(ch == ' ', letter == ' ', "{id}: {row:?} / {letters:?}");
                        assert!(
                            letter == ' '
                                || crate::animation::flyers::mask::LETTERS.contains(&letter),
                            "{id}: {letter:?}"
                        );
                    }
                }
                for signature in ["BP", "ejm98", "jgs", "ldb", "Tate"] {
                    assert!(
                        frame.rows.iter().all(|row| !row.contains(signature)),
                        "{id} keeps {signature}"
                    );
                }
            }
        }
    }

    #[test]
    fn characters_fly_their_pinned_directions() {
        let directions: Vec<(&str, &str)> = models()
            .iter()
            .filter(|model| model.def.kind == Kind::Character)
            .map(|model| (model.def.id, model.directions()))
            .collect();
        assert_eq!(
            directions,
            [
                ("caped-hero", "right"),
                ("ufo", "both ways"),
                ("moon-biker", "both ways"),
                ("broom-rider", "both ways"),
                ("fairy", "both ways"),
                ("pterosaur", "both ways"),
                ("flying-car", "right"),
                ("dragon", "right"),
                ("long-dragon", "both ways"),
            ]
        );
    }

    fn model(id: &str) -> Model {
        models()
            .into_iter()
            .find(|model| model.def.id == id)
            .unwrap()
    }

    #[test]
    fn drawn_poses_stay_aligned() {
        let pterosaur = model("pterosaur");
        let root = |variant: &Variant, rightward: bool| -> Vec<Option<usize>> {
            variant
                .frames
                .iter()
                .map(|frame| {
                    frame.rows.iter().find_map(|row| {
                        if rightward {
                            row.find('=')
                        } else {
                            row.rfind('=')
                        }
                    })
                })
                .collect()
        };
        assert_eq!(root(&pterosaur.right[0], true), vec![Some(18), Some(18)]);
        assert_eq!(root(&pterosaur.left[0], false), vec![Some(3), Some(3)]);

        let eyes: Vec<Option<(usize, usize)>> = model("dragon").right[0]
            .frames
            .iter()
            .map(|frame| {
                frame
                    .rows
                    .iter()
                    .enumerate()
                    .find_map(|(y, row)| row.find('@').map(|x| (x, y)))
            })
            .collect();
        assert_eq!(eyes, vec![Some((31, 3)), Some((31, 3))]);
        let dragon = model("dragon");
        let poses = &dragon.right[0].frames;
        assert_eq!(poses[0].rows[3..], poses[1].rows[3..]);
        for pose in poses {
            assert!(pose.rows[4].starts_with("<~~--("), "{:?}", pose.rows[4]);
            assert_eq!(pose.rows[4].matches('E').count(), 2);
        }

        let fairy = model("fairy");
        assert_eq!((fairy.height(), fairy.width()), (4, 6));
        assert_eq!(fairy.right[0].frames.len(), 2);
        for frame in &fairy.right[0].frames {
            assert_eq!(frame.rows[0].find('o'), Some(3));
            assert_eq!(frame.rows[3], "  /\\");
        }

        let long = model("long-dragon");
        assert_eq!((long.height(), long.width()), (3, 59));
        let frames = &long.left[0].frames;
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].rows[0], frames[1].rows[0]);
        assert_eq!(frames[0].rows[1].find("<(o"), Some(0));
        assert_eq!(frames[1].rows[1].find("<(o"), Some(0));
        assert_ne!(frames[0].rows[2], frames[1].rows[2]);
        for frame in frames {
            let legs: Vec<usize> = frame.rows[2]
                .char_indices()
                .filter(|&(x, ch)| x >= 6 && ch != ' ')
                .map(|(x, _)| x)
                .collect();
            assert_eq!(legs, [9, 10, 38, 39], "{:?}", frame.rows[2]);
        }
    }

    const BORROWED_NAMES: [u64; 13] = [
        0x3dff_a475_37ea_2280,
        0xef8b_a5d7_2158_d88d,
        0x53d8_41de_33c2_3cf1,
        0xc3c9_1318_f10b_5bfa,
        0x7e15_d172_ab95_5621,
        0xcab5_4718_f478_f8fb,
        0xc6f3_f7f0_ad9d_8cff,
        0xd718_9724_bf89_2b76,
        0x957f_6949_5266_c6da,
        0x902b_832f_e1ce_7416,
        0x69e8_b1d4_633f_f0df,
        0xf69f_287a_3cc2_c2f3,
        0x96ff_baa8_f070_7763,
    ];

    fn fnv1a(text: &str) -> u64 {
        text.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        })
    }

    fn words_and_pairs(text: &str) -> Vec<String> {
        let lower = text.to_lowercase();
        let words: Vec<&str> = lower
            .split(|ch: char| !(ch.is_alphanumeric() || ch == '.' || ch == '-'))
            .map(|word| word.trim_matches(|ch| ch == '.' || ch == '-'))
            .filter(|word| !word.is_empty())
            .collect();
        words
            .iter()
            .map(|word| word.to_string())
            .chain(
                words
                    .windows(2)
                    .map(|pair| format!("{} {}", pair[0], pair[1])),
            )
            .collect()
    }

    #[test]
    fn the_word_check_catches_a_listed_name() {
        assert!(words_and_pairs("A piece of FAN ART.").contains(&"fan art".to_string()));
        assert!(BORROWED_NAMES.contains(&fnv1a("fan art")));
    }

    #[test]
    fn the_sky_carries_no_borrowed_names() {
        let mut texts = vec![("the listing".to_string(), format_listing())];
        for def in definitions() {
            for (part, text) in [("art", def.right.or(def.left)), ("mask", def.mask)] {
                texts.push((
                    format!("the {} {part}", def.id),
                    text.unwrap_or_default().into(),
                ));
            }
            texts.push((format!("the {} credit", def.id), def.credit.to_string()));
        }
        for (place, text) in texts {
            for word in words_and_pairs(&text) {
                assert!(
                    !BORROWED_NAMES.contains(&fnv1a(&word)),
                    "a borrowed name in {place}"
                );
            }
        }
    }

    #[test]
    fn nine_characters() {
        assert_eq!(CHARACTERS.len(), 9);
        assert_eq!(character_ids().len(), 9);
    }

    #[test]
    fn character_lookups_stay_apart_from_aircraft() {
        assert_eq!(find_character("ufo").map(|def| def.id), Some("ufo"));
        assert!(find_character("plane").is_none());
        assert!(find("ufo").is_none());
        assert_eq!(find_any("ufo").map(|def| def.id), Some("ufo"));
        assert_eq!(find_any("plane").map(|def| def.id), Some("plane"));
        assert_eq!(
            character_ids(),
            CHARACTERS.iter().map(|def| def.id).collect::<Vec<_>>()
        );
        assert!(!aircraft_ids().contains(&"ufo"));
    }
}
