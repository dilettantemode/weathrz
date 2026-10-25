pub const HIDEABLE_SYSTEMS: [&str; 10] = [
    "stars",
    "moon",
    "sun",
    "clouds",
    "fireflies",
    "birds",
    "airplanes",
    "kite",
    "chimney_smoke",
    "leaves",
];

pub const HIDEABLE_DECORATIONS: [&str; 6] =
    ["tree", "pine_tree", "fence", "mailbox", "pond", "flowers"];

const ALWAYS_VISIBLE: [&str; 5] = ["rain", "snow", "fog", "thunderstorm", "house"];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HiddenDecorations {
    pub tree: bool,
    pub pine_tree: bool,
    pub fence: bool,
    pub mailbox: bool,
    pub pond: bool,
    pub flowers: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HideSet {
    systems: Vec<&'static str>,
    pub decorations: HiddenDecorations,
}

impl HideSet {
    pub fn parse(names: &[String]) -> (Self, Vec<String>) {
        let mut set = Self::default();
        let mut warnings = Vec::new();
        for raw in names {
            let name = raw.trim();
            if let Some(id) = HIDEABLE_SYSTEMS.iter().copied().find(|id| *id == name) {
                if !set.systems.contains(&id) {
                    set.systems.push(id);
                }
                continue;
            }
            match name {
                "tree" => set.decorations.tree = true,
                "pine_tree" => set.decorations.pine_tree = true,
                "fence" => set.decorations.fence = true,
                "mailbox" => set.decorations.mailbox = true,
                "pond" => set.decorations.pond = true,
                "flowers" => set.decorations.flowers = true,
                _ if ALWAYS_VISIBLE.contains(&name) => warnings.push(format!(
                    "hide: '{name}' is weather data or the scene's anchor and stays visible"
                )),
                _ => warnings.push(format!(
                    "hide: unknown element '{name}' ignored (hideable: {}, {})",
                    HIDEABLE_SYSTEMS.join(", "),
                    HIDEABLE_DECORATIONS.join(", ")
                )),
            }
        }
        (set, warnings)
    }

    pub fn hides_system(&self, id: &str) -> bool {
        self.systems.contains(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn hides_ambient_systems_and_decorations() {
        let (set, warnings) = HideSet::parse(&names(&[
            "birds",
            " airplanes ",
            "mailbox",
            "pine_tree",
            "birds",
        ]));
        assert!(warnings.is_empty(), "{warnings:?}");
        assert!(set.hides_system("birds"));
        assert!(set.hides_system("airplanes"));
        assert!(!set.hides_system("stars"));
        assert_eq!(
            set.decorations,
            HiddenDecorations {
                mailbox: true,
                pine_tree: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn the_pond_and_the_flower_beds_hide_on_their_own() {
        let (set, warnings) = HideSet::parse(&names(&["pond", "flowers"]));
        assert!(warnings.is_empty());
        assert!(set.decorations.pond && set.decorations.flowers);
        assert!(!set.decorations.tree);
    }

    #[test]
    fn the_kite_hides_on_its_own() {
        let (set, warnings) = HideSet::parse(&names(&["kite"]));
        assert!(warnings.is_empty(), "{warnings:?}");
        assert!(set.hides_system("kite"));
        assert!(!set.hides_system("airplanes"));
    }

    #[test]
    fn weather_and_house_cannot_be_hidden() {
        let (set, warnings) = HideSet::parse(&names(&["rain", "fog", "house", "unicorn"]));
        assert_eq!(set, HideSet::default());
        assert_eq!(warnings.len(), 4, "{warnings:?}");
        assert!(warnings[0].contains("'rain'"));
        assert!(warnings[3].contains("unknown element 'unicorn'"));
    }
}
