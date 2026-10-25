use crate::theme::colors::{ELEMENTS, ElementColors, SLOT_NAMES, ThemeColors};
use crate::theme::value::{dim, parse_concrete};
use crossterm::style::Color;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const BASE: &str = include_str!("builtin/base.toml");

pub const BUILTIN: [(&str, &str); 7] = [
    ("default", include_str!("builtin/default.toml")),
    ("tokyo-night", include_str!("builtin/tokyo-night.toml")),
    ("terminal", include_str!("builtin/terminal.toml")),
    (
        "catppuccin-mocha",
        include_str!("builtin/catppuccin-mocha.toml"),
    ),
    (
        "catppuccin-latte",
        include_str!("builtin/catppuccin-latte.toml"),
    ),
    ("gruvbox", include_str!("builtin/gruvbox.toml")),
    ("kanagawa", include_str!("builtin/kanagawa.toml")),
];

#[derive(Debug, Clone, PartialEq)]
enum ElementValue {
    Color(Color),
    Slot(&'static str),
    DimSlot(&'static str),
}

#[derive(Debug, Default, Clone)]
struct Half {
    slots: HashMap<&'static str, Color>,
    elements: HashMap<&'static str, Vec<ElementValue>>,
}

impl Half {
    fn overlay(&mut self, upper: &Half) {
        self.slots.extend(upper.slots.iter().map(|(k, v)| (*k, *v)));
        self.elements
            .extend(upper.elements.iter().map(|(k, v)| (*k, v.clone())));
    }
}

#[derive(Debug, Default)]
struct Layer {
    name: Option<String>,
    day: Half,
    night: Half,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedTheme {
    pub id: String,
    pub display_name: String,
    pub colors: ThemeColors,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeSource {
    Builtin,
    BuiltinWithOverride(PathBuf),
    BuiltinWithIgnoredOverride(PathBuf),
    User(PathBuf),
    Ignored(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeListing {
    pub id: String,
    pub source: ThemeSource,
}

pub fn list_themes(user_dir: Option<&Path>) -> Vec<ThemeListing> {
    let mut listing: Vec<ThemeListing> = BUILTIN
        .iter()
        .map(|(id, _)| ThemeListing {
            id: id.to_string(),
            source: match user_theme_path(user_dir, id) {
                Some(path) if parses_as_toml(&path) => ThemeSource::BuiltinWithOverride(path),
                Some(path) => ThemeSource::BuiltinWithIgnoredOverride(path),
                None => ThemeSource::Builtin,
            },
        })
        .collect();

    let (mut user, mut ignored): (Vec<ThemeListing>, Vec<ThemeListing>) = user_dir
        .and_then(|dir| fs::read_dir(dir).ok())
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                return None;
            }
            let id = path.file_stem()?.to_str()?.to_string();
            if !is_valid_id(&id) {
                return Some(ThemeListing {
                    id,
                    source: ThemeSource::Ignored(path),
                });
            }
            builtin_source(&id).is_none().then_some(ThemeListing {
                id,
                source: ThemeSource::User(path),
            })
        })
        .partition(|theme| matches!(theme.source, ThemeSource::User(_)));
    user.sort_by(|a, b| a.id.cmp(&b.id));
    ignored.sort_by(|a, b| a.id.cmp(&b.id));
    listing.extend(user);
    listing.extend(ignored);
    listing
}

fn parses_as_toml(path: &Path) -> bool {
    fs::read_to_string(path).is_ok_and(|src| toml::from_str::<toml::Table>(&src).is_ok())
}

fn display_path(path: &Path, home: Option<&Path>) -> String {
    match home.and_then(|home| path.strip_prefix(home).ok()) {
        Some(rest) => format!("~/{}", rest.display()),
        None => path.display().to_string(),
    }
}

pub fn format_listing(listing: &[ThemeListing]) -> String {
    let home = dirs::home_dir();
    let shown = |path: &Path| display_path(path, home.as_deref());
    listing
        .iter()
        .map(|theme| {
            let source = match &theme.source {
                ThemeSource::Builtin => "built-in".to_string(),
                ThemeSource::BuiltinWithOverride(path) => {
                    format!("built-in, user override ({})", shown(path))
                }
                ThemeSource::BuiltinWithIgnoredOverride(path) => format!(
                    "built-in (override ignored: invalid TOML, see warning: {})",
                    shown(path)
                ),
                ThemeSource::User(path) => format!("user ({})", shown(path)),
                ThemeSource::Ignored(path) => format!(
                    "ignored: invalid id (use a-z, 0-9 and '-') ({})",
                    shown(path)
                ),
            };
            format!("{:<18}{}", theme.id, source)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn is_valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn builtin_source(id: &str) -> Option<&'static str> {
    BUILTIN
        .iter()
        .find(|(name, _)| *name == id)
        .map(|(_, src)| *src)
}

fn user_theme_path(user_dir: Option<&Path>, id: &str) -> Option<PathBuf> {
    let path = user_dir?.join(format!("{id}.toml"));
    path.is_file().then_some(path)
}

pub fn resolve(requested: &str, user_dir: Option<&Path>) -> ResolvedTheme {
    let mut warnings = Vec::new();
    let mut id = requested.trim();
    if !is_valid_id(id) {
        warnings.push(format!(
            "theme id '{requested}' is invalid (use lowercase letters, digits and '-'); using 'default'"
        ));
        id = "default";
    }
    if builtin_source(id).is_none() && user_theme_path(user_dir, id).is_none() {
        warnings.push(format!("theme '{id}' not found; using 'default'"));
        id = "default";
    }

    let mut layers = Vec::new();
    layers.extend(parse_layer(BASE, "built-in base", &mut warnings));
    if let Some(src) = builtin_source(id) {
        layers.extend(parse_layer(src, &format!("built-in {id}"), &mut warnings));
    }
    if let Some(path) = user_theme_path(user_dir, id) {
        let origin = path.display().to_string();
        match fs::read_to_string(&path) {
            Ok(src) => layers.extend(parse_layer(&src, &origin, &mut warnings)),
            Err(e) => warnings.push(format!("{origin}: cannot read ({e}); ignoring file")),
        }
    }

    let mut day = Half::default();
    let mut night = Half::default();
    let mut name = None;
    for layer in layers {
        day.overlay(&layer.day);
        night.overlay(&layer.night);
        name = layer.name.or(name);
    }
    fill_missing_slots(&mut day, &mut warnings);
    let mut night_full = day.clone();
    night_full.overlay(&night);

    ResolvedTheme {
        id: id.to_string(),
        display_name: name.unwrap_or_else(|| id.to_string()),
        colors: ThemeColors {
            day: build(&day),
            night: build(&night_full),
        },
        warnings,
    }
}

fn fill_missing_slots(day: &mut Half, warnings: &mut Vec<String>) {
    let missing: Vec<&'static str> = SLOT_NAMES
        .iter()
        .copied()
        .filter(|slot| !day.slots.contains_key(slot))
        .collect();
    if missing.is_empty() {
        return;
    }
    let default_src = builtin_source("default").expect("default theme is built in");
    let fallback = parse_layer(default_src, "built-in default", &mut Vec::new())
        .expect("built-in default theme parses");
    for &slot in &missing {
        if let Some(color) = fallback.day.slots.get(slot) {
            day.slots.insert(slot, *color);
        }
    }
    warnings.push(format!(
        "theme does not set colour slots {}; using the default theme's values",
        missing.join(", ")
    ));
}

fn build(half: &Half) -> ElementColors {
    ElementColors::from_lookup(&|name: &str| {
        half.elements[name]
            .iter()
            .map(|value| match value {
                ElementValue::Color(color) => *color,
                ElementValue::Slot(slot) => half.slots[slot],
                ElementValue::DimSlot(slot) => dim(half.slots[slot]),
            })
            .collect()
    })
    .with_palette(SLOT_NAMES.map(|slot| half.slots[slot]))
}

fn parse_layer(src: &str, origin: &str, warnings: &mut Vec<String>) -> Option<Layer> {
    let table: toml::Table = match toml::from_str(src) {
        Ok(table) => table,
        Err(e) => {
            warnings.push(format!("{origin}: invalid TOML, ignoring file: {e}"));
            return None;
        }
    };
    let mut layer = Layer::default();
    for (key, value) in &table {
        match (key.as_str(), value) {
            ("name", toml::Value::String(name)) => layer.name = Some(name.clone()),
            ("day", toml::Value::Table(half)) => {
                layer.day = parse_half(half, origin, "day", warnings)
            }
            ("night", toml::Value::Table(half)) => {
                layer.night = parse_half(half, origin, "night", warnings)
            }
            _ => warnings.push(format!("{origin}: unknown key '{key}' ignored")),
        }
    }
    Some(layer)
}

fn parse_half(table: &toml::Table, origin: &str, half: &str, warnings: &mut Vec<String>) -> Half {
    let mut out = Half::default();
    for (key, value) in table {
        if key.as_str() == "elements" {
            match value {
                toml::Value::Table(elements) => {
                    parse_elements(elements, origin, half, &mut out, warnings)
                }
                _ => warnings.push(format!("{origin}: '{half}.elements' must be a table")),
            }
            continue;
        }
        let Some(slot) = SLOT_NAMES
            .iter()
            .copied()
            .find(|slot| *slot == key.as_str())
        else {
            warnings.push(format!(
                "{origin}: unknown colour slot '{half}.{key}' ignored"
            ));
            continue;
        };
        match value.as_str().and_then(parse_concrete) {
            Some(color) => {
                out.slots.insert(slot, color);
            }
            None => warnings.push(format!(
                "{origin}: '{half}.{key}' = {value} is not a colour (use \"#rrggbb\", \"ansi:<name>\" or \"default\")"
            )),
        }
    }
    out
}

fn parse_elements(
    table: &toml::Table,
    origin: &str,
    half: &str,
    out: &mut Half,
    warnings: &mut Vec<String>,
) {
    for (key, value) in table {
        let Some((name, tiers)) = ELEMENTS
            .iter()
            .copied()
            .find(|(name, _)| *name == key.as_str())
        else {
            warnings.push(format!(
                "{origin}: unknown element '{half}.elements.{key}' ignored"
            ));
            continue;
        };
        let (items, is_list): (Vec<&toml::Value>, bool) = match value {
            toml::Value::Array(items) => (items.iter().collect(), true),
            single => (vec![single], false),
        };
        if items.is_empty() {
            warnings.push(format!(
                "{origin}: '{half}.elements.{key}' is an empty list; ignored"
            ));
            continue;
        }
        let parsed: Option<Vec<ElementValue>> = items
            .iter()
            .map(|item| item.as_str().and_then(parse_element_value))
            .collect();
        let Some(mut values) = parsed else {
            warnings.push(format!(
                "{origin}: '{half}.elements.{key}' = {value} is not a colour, slot name or dim:<colour>; ignored"
            ));
            continue;
        };
        if !is_list {
            values = vec![values[0].clone(); tiers];
        } else if values.len() != tiers {
            warnings.push(format!(
                "{origin}: '{half}.elements.{key}' needs {tiers} colours, got {}; adjusted",
                values.len()
            ));
            if let Some(last) = values.last().cloned() {
                values.resize(tiers, last);
            }
        }
        out.elements.insert(name, values);
    }
}

fn parse_element_value(raw: &str) -> Option<ElementValue> {
    match raw.trim().strip_prefix("dim:") {
        Some(inner) => match parse_plain_value(inner)? {
            ElementValue::Color(color) => Some(ElementValue::Color(dim(color))),
            ElementValue::Slot(slot) => Some(ElementValue::DimSlot(slot)),
            ElementValue::DimSlot(_) => None,
        },
        None => parse_plain_value(raw),
    }
}

fn parse_plain_value(raw: &str) -> Option<ElementValue> {
    if let Some(color) = parse_concrete(raw) {
        return Some(ElementValue::Color(color));
    }
    let name = raw.trim();
    SLOT_NAMES
        .iter()
        .copied()
        .find(|slot| *slot == name)
        .map(ElementValue::Slot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::colors::{ElementColors, SLOT_NAMES, ThemeColors};
    use crossterm::style::Color;
    use std::fs;
    use std::path::PathBuf;

    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb { r, g, b }
    }

    fn temp_themes_dir(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("weathr-theme-test-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn upstream_day() -> ElementColors {
        ElementColors {
            roof: Color::DarkRed,
            wood: rgb(210, 180, 140),
            door: rgb(139, 69, 19),
            window: Color::Cyan,
            trim: Color::DarkGrey,
            path: Color::Reset,
            grass: Color::Green,
            grass_alt: Color::DarkGreen,
            soil: rgb(101, 67, 33),
            flowers: [Color::Magenta, Color::Red, Color::Cyan, Color::Yellow],
            tree: Color::DarkGreen,
            fence: Color::White,
            mailbox: Color::Blue,
            sun: Color::Yellow,
            moon: Color::White,
            shooting_star: Color::White,
            birds: Color::Yellow,
            stars: [Color::White, Color::DarkGrey],
            clouds: [Color::White, Color::Grey, Color::DarkGrey],
            rain_near: [Color::Cyan, Color::White, Color::Cyan, Color::White],
            rain_far: Color::DarkGrey,
            rain_splash: Color::White,
            snow: [Color::White, Color::DarkGrey],
            fog: [Color::Grey, Color::DarkGrey, rgb(120, 120, 120)],
            lightning: [Color::White, Color::Yellow],
            fireflies: [
                Color::Yellow,
                rgb(200, 255, 100),
                rgb(150, 200, 80),
                Color::DarkGrey,
            ],
            airplane: [
                Color::Cyan,
                Color::Blue,
                Color::DarkGrey,
                Color::Grey,
                Color::White,
            ],
            smoke: [Color::White, Color::Grey, Color::DarkGrey],
            leaves: [
                rgb(255, 165, 0),
                rgb(218, 165, 32),
                rgb(184, 134, 11),
                rgb(205, 92, 92),
                rgb(160, 82, 45),
                rgb(139, 69, 19),
            ],
            pumpkin: [rgb(255, 165, 0), Color::Yellow, Color::Green],
            bats: Color::Grey,
            hud_greeting: rgb(255, 165, 0),
            witch: [Color::Magenta, rgb(210, 180, 140), rgb(255, 165, 0)],
            hud_humidity: Color::Cyan,
            aqi: [
                Color::Green,
                Color::Yellow,
                rgb(255, 165, 0),
                Color::Red,
                Color::Magenta,
                rgb(139, 69, 19),
            ],
            hud: Color::Cyan,
            hud_condition: Color::Cyan,
            hud_temp: Color::Yellow,
            hud_wind: Color::Cyan,
            hud_precip: Color::Blue,
            hud_location: Color::Grey,
            hud_offline: Color::Red,
            attribution: Color::DarkGrey,
            palette: [
                Color::White,
                Color::Grey,
                Color::DarkGrey,
                Color::Red,
                rgb(255, 165, 0),
                Color::Yellow,
                Color::Green,
                Color::DarkGreen,
                Color::Cyan,
                Color::Blue,
                Color::DarkBlue,
                Color::Magenta,
                rgb(139, 69, 19),
                rgb(101, 67, 33),
                rgb(210, 180, 140),
            ],
        }
    }

    fn upstream_night() -> ElementColors {
        ElementColors {
            roof: Color::DarkMagenta,
            wood: rgb(100, 70, 50),
            door: rgb(80, 40, 10),
            window: Color::Yellow,
            grass: Color::DarkGreen,
            grass_alt: rgb(0, 50, 0),
            soil: rgb(60, 40, 20),
            flowers: [
                Color::DarkMagenta,
                Color::DarkRed,
                Color::Blue,
                Color::DarkYellow,
            ],
            tree: rgb(0, 50, 0),
            fence: Color::Grey,
            mailbox: Color::DarkBlue,
            ..upstream_day()
        }
    }

    #[test]
    fn default_resolves_without_warnings() {
        let theme = resolve("default", None);
        assert_eq!(theme.id, "default");
        assert_eq!(theme.display_name, "Default");
        assert!(theme.warnings.is_empty(), "{:?}", theme.warnings);
    }

    #[test]
    fn default_day_matches_upstream_literals() {
        assert_eq!(resolve("default", None).colors.day, upstream_day());
    }

    #[test]
    fn default_night_matches_upstream_literals() {
        assert_eq!(resolve("default", None).colors.night, upstream_night());
    }

    #[test]
    fn unknown_theme_falls_back_to_default() {
        let theme = resolve("no-such-theme", None);
        assert_eq!(theme.id, "default");
        assert_eq!(theme.colors, resolve("default", None).colors);
        assert!(
            theme
                .warnings
                .iter()
                .any(|w| w.contains("'no-such-theme' not found")),
            "{:?}",
            theme.warnings
        );
    }

    #[test]
    fn invalid_ids_never_touch_the_filesystem() {
        let root = temp_themes_dir("invalid-id");
        let themes = root.join("themes");
        fs::create_dir_all(themes.join("evil")).unwrap();
        let evil = "[day.elements]\nroof = \"#010203\"\n";
        fs::write(root.join("evil.toml"), evil).unwrap();
        fs::write(themes.join("evil.toml"), evil).unwrap();
        for raw in ["../evil", "evil/../evil", "Tokyo", "", "a b", "évil"] {
            let theme = resolve(raw, Some(&themes));
            assert_eq!(theme.id, "default", "{raw:?}");
            assert_eq!(theme.colors.day.roof, Color::DarkRed, "{raw:?}");
            assert!(!theme.warnings.is_empty(), "{raw:?}");
        }
    }

    #[test]
    fn night_flowers_are_dimmed_and_rain_brightens_with_intensity() {
        let colors = resolve("tokyo-night", None).colors;
        let slot = |half: &ElementColors, name: &str| {
            half.palette[SLOT_NAMES.iter().position(|slot| *slot == name).unwrap()]
        };
        let night = &colors.night;
        assert_eq!(
            night.flowers,
            ["magenta", "red", "blue", "orange"]
                .map(|name| crate::theme::value::dim(slot(night, name)))
        );
        let day = &colors.day;
        assert_eq!(
            day.rain_near,
            ["muted", "blue", "cyan", "fg"].map(|name| slot(day, name))
        );
    }

    #[test]
    fn dim_values_take_slots_and_concrete_colours() {
        let dir = temp_themes_dir("dim");
        fs::write(
            dir.join("dusky.toml"),
            "name = \"Dusky\"\n[day.elements]\nroof = \"dim:#c86432\"\nmailbox = \"dim:red\"\ndoor = \"dim:nope\"\nfence = \"dim:dim:#c86432\"\nwood = \"dim:dim:red\"\n",
        )
        .unwrap();
        let theme = resolve("dusky", Some(&dir));
        assert_eq!(theme.colors.day.roof, rgb(110, 55, 28));
        let red =
            theme.colors.day.palette[SLOT_NAMES.iter().position(|slot| *slot == "red").unwrap()];
        assert_eq!(theme.colors.day.mailbox, crate::theme::value::dim(red));
        assert!(
            theme
                .warnings
                .iter()
                .any(|warning| warning.contains("'day.elements.door' = \"dim:nope\"")),
            "{:?}",
            theme.warnings
        );
        for key in ["fence", "wood"] {
            assert!(
                theme
                    .warnings
                    .iter()
                    .any(|warning| warning.contains(&format!("'day.elements.{key}'"))),
                "{key} {:?}",
                theme.warnings
            );
        }
    }

    #[test]
    fn user_file_layers_over_builtin() {
        let dir = temp_themes_dir("override");
        fs::write(
            dir.join("default.toml"),
            "[day.elements]\nroof = \"#010203\"\n",
        )
        .unwrap();
        let theme = resolve("default", Some(&dir));
        assert!(theme.warnings.is_empty(), "{:?}", theme.warnings);
        assert_eq!(theme.colors.day.roof, rgb(1, 2, 3));
        assert_eq!(theme.colors.day.door, upstream_day().door);
        assert_eq!(theme.colors.night.roof, upstream_night().roof);
    }

    #[test]
    fn new_user_theme_fills_missing_slots_from_default() {
        let dir = temp_themes_dir("new-theme");
        fs::write(
            dir.join("mine.toml"),
            "name = \"Mine\"\n[day]\nred = \"#ff0000\"\n",
        )
        .unwrap();
        let theme = resolve("mine", Some(&dir));
        assert_eq!(theme.id, "mine");
        assert_eq!(theme.display_name, "Mine");
        assert_eq!(theme.colors.day.roof, rgb(255, 0, 0));
        assert_eq!(theme.colors.day.fence, Color::White);
        assert!(
            theme
                .warnings
                .iter()
                .any(|w| w.contains("does not set colour slots") && w.contains("fg")),
            "{:?}",
            theme.warnings
        );
    }

    #[test]
    fn night_inherits_day_slots_and_applies_night_overrides() {
        let dir = temp_themes_dir("night");
        let mut src = String::from("[day]\n");
        for slot in SLOT_NAMES {
            let value = match slot {
                "yellow" => "#222222",
                "green_dark" => "#333333",
                _ => "#111111",
            };
            src.push_str(&format!("{slot} = \"{value}\"\n"));
        }
        src.push_str("[night]\ngreen_dark = \"#444444\"\n");
        fs::write(dir.join("dusk.toml"), src).unwrap();
        let theme = resolve("dusk", Some(&dir));
        assert!(theme.warnings.is_empty(), "{:?}", theme.warnings);
        assert_eq!(theme.colors.day.window, rgb(0x11, 0x11, 0x11));
        assert_eq!(theme.colors.night.window, rgb(0x22, 0x22, 0x22));
        assert_eq!(theme.colors.day.tree, rgb(0x33, 0x33, 0x33));
        assert_eq!(theme.colors.night.tree, rgb(0x44, 0x44, 0x44));
    }

    #[test]
    fn invalid_toml_in_user_file_is_skipped() {
        let dir = temp_themes_dir("bad-toml");
        fs::write(dir.join("default.toml"), "[day.elements\nroof = ").unwrap();
        let theme = resolve("default", Some(&dir));
        assert_eq!(theme.colors, resolve("default", None).colors);
        assert!(
            theme.warnings.iter().any(|w| w.contains("invalid TOML")),
            "{:?}",
            theme.warnings
        );
    }

    #[test]
    fn bad_values_keep_the_lower_layer() {
        let dir = temp_themes_dir("bad-values");
        fs::write(
            dir.join("default.toml"),
            "[day]\nfg = \"white\"\nsparkle = \"#ffffff\"\n[day.elements]\nroof = \"#zzzzzz\"\nchimney = \"#ffffff\"\nwindow = \"nope\"\n",
        )
        .unwrap();
        let theme = resolve("default", Some(&dir));
        assert_eq!(theme.colors, resolve("default", None).colors);
        for needle in [
            "'day.fg'",
            "'day.sparkle'",
            "'day.elements.roof'",
            "'day.elements.chimney'",
            "'day.elements.window'",
        ] {
            assert!(
                theme.warnings.iter().any(|w| w.contains(needle)),
                "missing warning for {needle}: {:?}",
                theme.warnings
            );
        }
    }

    #[test]
    fn tier_lists_are_padded_or_truncated_and_single_values_broadcast() {
        let dir = temp_themes_dir("tiers");
        fs::write(
            dir.join("default.toml"),
            "[day.elements]\nstars = [\"#010101\"]\nsmoke = [\"#010101\", \"#020202\", \"#030303\", \"#040404\"]\nfog = \"#050505\"\n",
        )
        .unwrap();
        let theme = resolve("default", Some(&dir));
        assert_eq!(theme.colors.day.stars, [rgb(1, 1, 1), rgb(1, 1, 1)]);
        assert_eq!(
            theme.colors.day.smoke,
            [rgb(1, 1, 1), rgb(2, 2, 2), rgb(3, 3, 3)]
        );
        assert_eq!(theme.colors.day.fog, [rgb(5, 5, 5); 3]);
        assert_eq!(theme.warnings.len(), 2, "{:?}", theme.warnings);
    }

    #[test]
    fn every_builtin_resolves_cleanly() {
        for (id, _) in BUILTIN {
            let theme = resolve(id, None);
            assert_eq!(theme.id, id);
            assert!(theme.warnings.is_empty(), "{id}: {:?}", theme.warnings);
        }
    }

    #[test]
    fn builtin_themes_are_distinct() {
        let all: Vec<ThemeColors> = BUILTIN
            .iter()
            .map(|(id, _)| resolve(id, None).colors)
            .collect();
        for (i, a) in all.iter().enumerate() {
            for b in &all[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }

    #[test]
    fn terminal_theme_uses_only_terminal_colours() {
        let colors = resolve("terminal", None).colors;
        assert!(!format!("{colors:?}").contains("Rgb"), "{colors:?}");
    }

    #[test]
    fn tokyo_night_uses_the_official_palette() {
        let colors = resolve("tokyo-night", None).colors;
        assert_eq!(colors.day.roof, rgb(0xf7, 0x76, 0x8e));
        assert_eq!(colors.day.window, rgb(0x7d, 0xcf, 0xff));
        assert_eq!(colors.night.roof, rgb(0xbb, 0x9a, 0xf7));
        assert_eq!(colors.night.tree, rgb(0x48, 0x5a, 0x3e));
    }

    #[test]
    fn listing_shows_builtins_overrides_and_user_themes() {
        let dir = temp_themes_dir("listing");
        fs::write(
            dir.join("tokyo-night.toml"),
            "[day.elements]\nroof = \"#010203\"\n",
        )
        .unwrap();
        fs::write(dir.join("mine.toml"), "name = \"Mine\"\n").unwrap();
        fs::write(dir.join("Bad Name.toml"), "").unwrap();
        fs::write(dir.join("notes.txt"), "").unwrap();
        let listing = list_themes(Some(&dir));
        let ids: Vec<&str> = listing.iter().map(|t| t.id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "default",
                "tokyo-night",
                "terminal",
                "catppuccin-mocha",
                "catppuccin-latte",
                "gruvbox",
                "kanagawa",
                "mine",
                "Bad Name"
            ]
        );
        assert_eq!(listing[0].source, ThemeSource::Builtin);
        assert_eq!(
            listing[1].source,
            ThemeSource::BuiltinWithOverride(dir.join("tokyo-night.toml"))
        );
        assert_eq!(listing[7].source, ThemeSource::User(dir.join("mine.toml")));
        assert_eq!(
            listing[8].source,
            ThemeSource::Ignored(dir.join("Bad Name.toml"))
        );
        let text = format_listing(&listing);
        assert_eq!(text.lines().count(), 9);
        assert_eq!(text.lines().next(), Some("default           built-in"));
        assert!(
            text.lines()
                .last()
                .unwrap()
                .contains("ignored: invalid id (use a-z, 0-9 and '-')"),
            "{text}"
        );
    }

    #[test]
    fn listing_marks_an_unparsable_override_as_ignored() {
        let dir = temp_themes_dir("broken-override");
        fs::write(dir.join("gruvbox.toml"), "[day.elements\n").unwrap();
        let listing = list_themes(Some(&dir));
        let gruvbox = listing.iter().find(|t| t.id == "gruvbox").unwrap();
        assert_eq!(
            gruvbox.source,
            ThemeSource::BuiltinWithIgnoredOverride(dir.join("gruvbox.toml"))
        );
        let text = format_listing(std::slice::from_ref(gruvbox));
        assert!(
            text.contains("built-in (override ignored: invalid TOML, see warning:"),
            "{text}"
        );
    }

    #[test]
    fn display_path_shortens_the_home_directory() {
        let home = Path::new("/home/someone");
        assert_eq!(
            display_path(&home.join(".config/weathr/themes/x.toml"), Some(home)),
            "~/.config/weathr/themes/x.toml"
        );
        assert_eq!(
            display_path(Path::new("/etc/x.toml"), Some(home)),
            "/etc/x.toml"
        );
        assert_eq!(display_path(Path::new("/etc/x.toml"), None), "/etc/x.toml");
    }

    #[test]
    fn listing_without_a_user_dir_has_only_builtins() {
        assert_eq!(list_themes(None).len(), BUILTIN.len());
    }

    #[test]
    fn halloween_elements_follow_each_palette() {
        let default = resolve("default", None).colors;
        assert_eq!(
            default.day.pumpkin,
            [rgb(0xff, 0xa5, 0x00), Color::Yellow, Color::Green]
        );
        assert_eq!(default.night.bats, Color::Grey);
        assert_eq!(default.day.hud_greeting, rgb(0xff, 0xa5, 0x00));
        let tokyo = resolve("tokyo-night", None).colors;
        assert_eq!(tokyo.night.pumpkin[0], rgb(0xff, 0x9e, 0x64));
        assert_eq!(tokyo.night.hud_greeting, rgb(0xff, 0x9e, 0x64));
        for (id, _) in BUILTIN {
            assert!(resolve(id, None).warnings.is_empty(), "{id}");
        }
    }

    #[test]
    fn witch_element_follows_each_palette() {
        let default = resolve("default", None).colors;
        assert_eq!(
            default.night.witch,
            [Color::Magenta, rgb(0xd2, 0xb4, 0x8c), rgb(0xff, 0xa5, 0x00)]
        );
        for (id, _) in BUILTIN {
            assert!(resolve(id, None).warnings.is_empty(), "{id}");
        }
    }

    #[test]
    fn aqi_bands_have_six_distinct_colours_in_every_theme() {
        for (id, _) in BUILTIN {
            let theme = resolve(id, None);
            assert!(theme.warnings.is_empty(), "{id}: {:?}", theme.warnings);
            for (half, colors) in [("day", theme.colors.day), ("night", theme.colors.night)] {
                for i in 0..6 {
                    for j in i + 1..6 {
                        assert_ne!(
                            colors.aqi[i], colors.aqi[j],
                            "{id} {half}: tiers {i} and {j}"
                        );
                    }
                }
            }
        }
        let default = resolve("default", None).colors;
        assert_eq!(default.day.hud_humidity, Color::Cyan);
    }

    #[test]
    fn the_palette_follows_the_slot_order_day_and_night() {
        let slot = |name: &str| SLOT_NAMES.iter().position(|slot| *slot == name).unwrap();
        let theme = resolve("tokyo-night", None);
        assert_eq!(
            theme.colors.day.palette[slot("green")],
            rgb(0x9e, 0xce, 0x6a)
        );
        assert_eq!(
            theme.colors.night.palette[slot("green")],
            rgb(0x70, 0x8f, 0x52)
        );
        assert_eq!(theme.colors.day.palette[slot("tan")], rgb(0xd5, 0xb8, 0x99));
        assert_eq!(
            theme.colors.night.palette[slot("red")],
            rgb(0xf7, 0x76, 0x8e)
        );
        for (id, _) in BUILTIN {
            let theme = resolve(id, None);
            assert!(
                !theme.colors.day.palette.contains(&Color::Reset),
                "{id} day"
            );
            assert!(
                !theme.colors.night.palette.contains(&Color::Reset),
                "{id} night"
            );
        }
    }
}
