use crossterm::style::Color;

pub const SLOT_NAMES: [&str; 15] = [
    "fg",
    "muted",
    "subtle",
    "red",
    "orange",
    "yellow",
    "green",
    "green_dark",
    "cyan",
    "blue",
    "blue_dark",
    "magenta",
    "brown",
    "brown_dark",
    "tan",
];

pub const ELEMENTS: [(&str, usize); 43] = [
    ("roof", 1),
    ("wood", 1),
    ("door", 1),
    ("window", 1),
    ("trim", 1),
    ("path", 1),
    ("grass", 1),
    ("grass_alt", 1),
    ("soil", 1),
    ("flowers", 4),
    ("tree", 1),
    ("fence", 1),
    ("mailbox", 1),
    ("sun", 1),
    ("moon", 1),
    ("shooting_star", 1),
    ("birds", 1),
    ("stars", 2),
    ("clouds", 3),
    ("rain_near", 4),
    ("rain_far", 1),
    ("rain_splash", 1),
    ("snow", 2),
    ("fog", 3),
    ("lightning", 2),
    ("fireflies", 4),
    ("airplane", 5),
    ("smoke", 3),
    ("leaves", 6),
    ("pumpkin", 3),
    ("bats", 1),
    ("hud_greeting", 1),
    ("witch", 3),
    ("hud_humidity", 1),
    ("aqi", 6),
    ("hud", 1),
    ("hud_condition", 1),
    ("hud_temp", 1),
    ("hud_wind", 1),
    ("hud_precip", 1),
    ("hud_location", 1),
    ("hud_offline", 1),
    ("attribution", 1),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementColors {
    pub roof: Color,
    pub wood: Color,
    pub door: Color,
    pub window: Color,
    pub trim: Color,
    pub path: Color,
    pub grass: Color,
    pub grass_alt: Color,
    pub soil: Color,
    pub flowers: [Color; 4],
    pub tree: Color,
    pub fence: Color,
    pub mailbox: Color,
    pub sun: Color,
    pub moon: Color,
    pub shooting_star: Color,
    pub birds: Color,
    pub stars: [Color; 2],
    pub clouds: [Color; 3],
    pub rain_near: [Color; 4],
    pub rain_far: Color,
    pub rain_splash: Color,
    pub snow: [Color; 2],
    pub fog: [Color; 3],
    pub lightning: [Color; 2],
    pub fireflies: [Color; 4],
    pub airplane: [Color; 5],
    pub smoke: [Color; 3],
    pub leaves: [Color; 6],
    pub pumpkin: [Color; 3],
    pub bats: Color,
    pub hud_greeting: Color,
    pub witch: [Color; 3],
    pub hud_humidity: Color,
    pub aqi: [Color; 6],
    pub hud: Color,
    pub hud_condition: Color,
    pub hud_temp: Color,
    pub hud_wind: Color,
    pub hud_precip: Color,
    pub hud_location: Color,
    pub hud_offline: Color,
    pub attribution: Color,
    pub palette: [Color; 15],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColors {
    pub day: ElementColors,
    pub night: ElementColors,
}

impl ThemeColors {
    pub fn for_time(&self, is_day: bool) -> &ElementColors {
        if is_day { &self.day } else { &self.night }
    }
}

fn tiers<const N: usize>(values: Vec<Color>) -> [Color; N] {
    std::array::from_fn(|i| values[i])
}

impl ElementColors {
    pub fn from_lookup(lookup: &dyn Fn(&str) -> Vec<Color>) -> Self {
        let one = |name: &str| lookup(name)[0];
        Self {
            roof: one("roof"),
            wood: one("wood"),
            door: one("door"),
            window: one("window"),
            trim: one("trim"),
            path: one("path"),
            grass: one("grass"),
            grass_alt: one("grass_alt"),
            soil: one("soil"),
            flowers: tiers(lookup("flowers")),
            tree: one("tree"),
            fence: one("fence"),
            mailbox: one("mailbox"),
            sun: one("sun"),
            moon: one("moon"),
            shooting_star: one("shooting_star"),
            birds: one("birds"),
            stars: tiers(lookup("stars")),
            clouds: tiers(lookup("clouds")),
            rain_near: tiers(lookup("rain_near")),
            rain_far: one("rain_far"),
            rain_splash: one("rain_splash"),
            snow: tiers(lookup("snow")),
            fog: tiers(lookup("fog")),
            lightning: tiers(lookup("lightning")),
            fireflies: tiers(lookup("fireflies")),
            airplane: tiers(lookup("airplane")),
            smoke: tiers(lookup("smoke")),
            leaves: tiers(lookup("leaves")),
            pumpkin: tiers(lookup("pumpkin")),
            bats: one("bats"),
            hud_greeting: one("hud_greeting"),
            witch: tiers(lookup("witch")),
            hud_humidity: one("hud_humidity"),
            aqi: tiers(lookup("aqi")),
            hud: one("hud"),
            hud_condition: one("hud_condition"),
            hud_temp: one("hud_temp"),
            hud_wind: one("hud_wind"),
            hud_precip: one("hud_precip"),
            hud_location: one("hud_location"),
            hud_offline: one("hud_offline"),
            attribution: one("attribution"),
            palette: [Color::Reset; 15],
        }
    }

    pub fn with_palette(self, palette: [Color; 15]) -> Self {
        Self { palette, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashSet;

    #[test]
    fn slot_and_element_names_are_unique() {
        let slots: HashSet<_> = SLOT_NAMES.iter().collect();
        assert_eq!(slots.len(), SLOT_NAMES.len());
        let elements: HashSet<_> = ELEMENTS.iter().map(|(name, _)| name).collect();
        assert_eq!(elements.len(), ELEMENTS.len());
    }

    #[test]
    fn from_lookup_asks_for_every_element_once_with_its_tier_count() {
        let asked = RefCell::new(Vec::new());
        let colors = ElementColors::from_lookup(&|name: &str| {
            asked.borrow_mut().push(name.to_string());
            let tiers = ELEMENTS
                .iter()
                .find(|(element, _)| *element == name)
                .map(|(_, tiers)| *tiers)
                .expect("struct field missing from ELEMENTS");
            (0..tiers).map(|i| Color::AnsiValue(i as u8)).collect()
        });
        let mut asked = asked.into_inner();
        asked.sort();
        let mut expected: Vec<String> = ELEMENTS.iter().map(|(name, _)| name.to_string()).collect();
        expected.sort();
        assert_eq!(asked, expected);
        assert_eq!(colors.roof, Color::AnsiValue(0));
        assert_eq!(colors.leaves[5], Color::AnsiValue(5));
    }

    #[test]
    fn for_time_picks_the_half() {
        let day = ElementColors::from_lookup(&|name: &str| {
            let tiers = ELEMENTS.iter().find(|(e, _)| *e == name).unwrap().1;
            vec![Color::White; tiers]
        });
        let night = ElementColors {
            roof: Color::Black,
            ..day
        };
        let theme = ThemeColors { day, night };
        assert_eq!(theme.for_time(true).roof, Color::White);
        assert_eq!(theme.for_time(false).roof, Color::Black);
    }
}
