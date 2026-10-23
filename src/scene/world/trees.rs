pub const LEAVES: char = 'l';
pub const TALLEST_ROWS: u16 = 12;
pub const WOOD: char = 'w';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Climate {
    Tropical,
    Temperate,
    Cold,
    Dry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TreesSetting {
    #[default]
    Auto,
    Fixed(Climate),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeArt {
    pub id: &'static str,
    pub credit: &'static str,
    pub art: &'static str,
    pub mask: &'static str,
    pub sway_rows: usize,
}

pub const ROUND: TreeArt = TreeArt {
    id: "round",
    credit: "round shade tree, jgs",
    art: include_str!("assets/trees/round.txt"),
    mask: include_str!("assets/trees/round.mask.txt"),
    sway_rows: 7,
};

pub const FRUIT: TreeArt = TreeArt {
    id: "fruit",
    credit: "fruit tree, ejm97",
    art: include_str!("assets/trees/fruit.txt"),
    mask: include_str!("assets/trees/fruit.mask.txt"),
    sway_rows: 6,
};

pub const FIR: TreeArt = TreeArt {
    id: "fir",
    credit: "fir, R. Nykvist",
    art: include_str!("assets/trees/fir.txt"),
    mask: include_str!("assets/trees/fir.mask.txt"),
    sway_rows: 9,
};

pub const SNOWY_FIR: TreeArt = TreeArt {
    id: "snowy_fir",
    credit: "snowy fir, jgs",
    art: include_str!("assets/trees/snowy_fir.txt"),
    mask: include_str!("assets/trees/snowy_fir.mask.txt"),
    sway_rows: 9,
};

pub const BARE: TreeArt = TreeArt {
    id: "bare",
    credit: "bare winter tree, jgs",
    art: include_str!("assets/trees/bare.txt"),
    mask: include_str!("assets/trees/bare.mask.txt"),
    sway_rows: 6,
};

pub const SAGUARO: TreeArt = TreeArt {
    id: "saguaro",
    credit: "saguaro, mga",
    art: include_str!("assets/trees/saguaro.txt"),
    mask: include_str!("assets/trees/saguaro.mask.txt"),
    sway_rows: 0,
};

pub const TALL_SAGUARO: TreeArt = TreeArt {
    id: "tall_saguaro",
    credit: "saguaro, kra",
    art: include_str!("assets/trees/tall_saguaro.txt"),
    mask: include_str!("assets/trees/tall_saguaro.mask.txt"),
    sway_rows: 0,
};

pub const ALL: [&TreeArt; 7] = [
    &ROUND,
    &FRUIT,
    &FIR,
    &SNOWY_FIR,
    &BARE,
    &SAGUARO,
    &TALL_SAGUARO,
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Garden {
    pub main: &'static TreeArt,
    pub second: &'static TreeArt,
    pub sway: i16,
    pub ornaments: Option<u64>,
    pub yard: crate::scene::world::yard::Yard,
}

const TROPICS_DEG: f64 = 23.5;
const COLD_DEG: f64 = 55.0;
const CALM_KMH: f32 = 8.0;
const BREEZE_KMH: f32 = 20.0;
const GALE_KMH: f32 = 35.0;
const FPS: f32 = 30.0;

pub fn default_garden() -> Garden {
    Garden {
        main: &ROUND,
        second: &FRUIT,
        sway: 0,
        ornaments: None,
        yard: crate::scene::world::yard::Yard::default(),
    }
}

pub fn climate_for(latitude: f64) -> Climate {
    let distance = latitude.abs();
    if distance < TROPICS_DEG {
        Climate::Tropical
    } else if distance < COLD_DEG {
        Climate::Temperate
    } else {
        Climate::Cold
    }
}

pub fn parse_setting(value: &str) -> Result<TreesSetting, String> {
    match value.trim() {
        "" | "auto" => Ok(TreesSetting::Auto),
        "tropical" => Ok(TreesSetting::Fixed(Climate::Tropical)),
        "temperate" => Ok(TreesSetting::Fixed(Climate::Temperate)),
        "cold" => Ok(TreesSetting::Fixed(Climate::Cold)),
        "dry" => Ok(TreesSetting::Fixed(Climate::Dry)),
        other => Err(format!(
            "trees = \"{other}\" is not valid (use \"auto\", \"tropical\", \"temperate\", \"cold\" or \"dry\"); using \"auto\""
        )),
    }
}

pub fn trees_for(
    setting: TreesSetting,
    latitude: f64,
    freezing: bool,
) -> (&'static TreeArt, &'static TreeArt) {
    let climate = match setting {
        TreesSetting::Auto => climate_for(latitude),
        TreesSetting::Fixed(climate) => climate,
    };
    match climate {
        Climate::Temperate if freezing => (&BARE, &FRUIT),
        Climate::Tropical | Climate::Temperate => (&ROUND, &FRUIT),
        Climate::Cold => (&FIR, &SNOWY_FIR),
        Climate::Dry => (&SAGUARO, &TALL_SAGUARO),
    }
}

pub fn sway(wind_kmh: f32, frame: u64) -> i16 {
    let (amplitude, period_seconds) = if wind_kmh < CALM_KMH {
        return 0;
    } else if wind_kmh < BREEZE_KMH {
        (1.0, 3.0)
    } else if wind_kmh < GALE_KMH {
        (1.0, 1.5)
    } else {
        (2.0, 1.0)
    };
    let seconds = (frame % (FPS as u64 * 60)) as f32 / FPS;
    let phase = std::f32::consts::TAU * seconds / period_seconds;
    (amplitude * phase.sin()).round() as i16
}

pub fn row_offset(row: usize, sway_rows: usize, sway: i16) -> i16 {
    if row >= sway_rows {
        0
    } else if row < sway_rows / 2 {
        sway
    } else {
        sway / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JAKARTA: (f64, f64) = (-6.2088, 106.8456);
    const BERLIN: (f64, f64) = (52.52, 13.41);

    #[test]
    fn the_climate_follows_the_latitude() {
        assert_eq!(climate_for(JAKARTA.0), Climate::Tropical);
        assert_eq!(climate_for(BERLIN.0), Climate::Temperate);
        assert_eq!(climate_for(-40.0), Climate::Temperate);
        assert_eq!(climate_for(64.1), Climate::Cold);
        assert_eq!(climate_for(-23.4), Climate::Tropical);
    }

    #[test]
    fn the_setting_picks_a_climate_or_auto() {
        assert_eq!(parse_setting(""), Ok(TreesSetting::Auto));
        assert_eq!(parse_setting("auto"), Ok(TreesSetting::Auto));
        assert_eq!(
            parse_setting(" dry "),
            Ok(TreesSetting::Fixed(Climate::Dry))
        );
        assert_eq!(
            parse_setting("tropical"),
            Ok(TreesSetting::Fixed(Climate::Tropical))
        );
        assert_eq!(
            parse_setting("cold"),
            Ok(TreesSetting::Fixed(Climate::Cold))
        );
        let err = parse_setting("jungle").unwrap_err();
        assert!(err.contains("trees = \"jungle\""), "{err}");
    }

    #[test]
    fn the_tropics_get_the_round_shade_tree_and_the_fruit_tree() {
        let (main, second) = trees_for(TreesSetting::Auto, JAKARTA.0, false);
        assert_eq!((main.id, second.id), ("round", "fruit"));
        assert!(ALL.iter().all(|tree| !tree.id.contains("palm")));
    }

    #[test]
    fn temperate_trees_lose_their_leaves_below_freezing() {
        let warm = trees_for(TreesSetting::Auto, BERLIN.0, false);
        assert_eq!((warm.0.id, warm.1.id), ("round", "fruit"));
        let cold = trees_for(TreesSetting::Auto, BERLIN.0, true);
        assert_eq!(cold.0.id, "bare");
    }

    #[test]
    fn a_fixed_climate_wins_over_the_latitude() {
        let dry = trees_for(TreesSetting::Fixed(Climate::Dry), JAKARTA.0, false);
        assert_eq!((dry.0.id, dry.1.id), ("saguaro", "tall_saguaro"));
        let cold = trees_for(TreesSetting::Fixed(Climate::Cold), JAKARTA.0, false);
        assert_eq!((cold.0.id, cold.1.id), ("fir", "snowy_fir"));
    }

    #[test]
    fn every_tree_fits_beside_the_house_and_its_mask_matches() {
        for tree in ALL {
            let rows: Vec<&str> = tree.art.lines().collect();
            let masks: Vec<&str> = tree.mask.lines().collect();
            assert!(rows.len() <= usize::from(TALLEST_ROWS), "{}", tree.id);
            assert!(rows.iter().all(|row| row.len() <= 21), "{}", tree.id);
            assert!(tree.sway_rows <= rows.len(), "{}", tree.id);
            assert_eq!(rows.len(), masks.len(), "{}", tree.id);
            for (row, letters) in rows.iter().zip(&masks) {
                let width = row.len().max(letters.len());
                let cells = format!("{row:<width$}");
                let marks = format!("{letters:<width$}");
                for (ch, letter) in cells.chars().zip(marks.chars()) {
                    assert_eq!(
                        ch == ' ',
                        letter == ' ',
                        "{}: {row:?} / {letters:?}",
                        tree.id
                    );
                    assert!(
                        letter == ' '
                            || letter == LEAVES
                            || letter == WOOD
                            || crate::animation::flyers::mask::LETTERS.contains(&letter),
                        "{}: {letter:?}",
                        tree.id
                    );
                }
            }
        }
    }

    #[test]
    fn the_tallest_tree_sets_the_tallest_rows() {
        let tallest = ALL.iter().map(|tree| tree.art.lines().count()).max();
        assert_eq!(tallest, Some(usize::from(TALLEST_ROWS)));
    }

    #[test]
    fn calm_air_keeps_the_trees_still() {
        assert!((0..300).all(|frame| sway(5.0, frame) == 0));
    }

    #[test]
    fn the_sway_grows_and_quickens_with_the_wind() {
        let swing = |kmh: f32| (0..300).map(|frame| sway(kmh, frame)).collect::<Vec<_>>();
        let breeze = swing(12.0);
        assert!(breeze.contains(&1) && breeze.contains(&-1), "{breeze:?}");
        assert!(breeze.iter().all(|offset| offset.abs() <= 1));
        let gale = swing(50.0);
        assert!(gale.contains(&2) && gale.contains(&-2), "{gale:?}");
        let turns = |offsets: &[i16]| offsets.windows(2).filter(|pair| pair[0] != pair[1]).count();
        assert!(turns(&swing(28.0)) > turns(&breeze));
    }

    #[test]
    fn each_wind_band_starts_where_the_readme_says() {
        let swing = |kmh: f32| (0..300).map(|frame| sway(kmh, frame)).collect::<Vec<_>>();
        let turns = |offsets: &[i16]| offsets.windows(2).filter(|pair| pair[0] != pair[1]).count();
        assert!(swing(7.9).iter().all(|offset| *offset == 0));
        assert!(swing(8.0).contains(&1));
        assert!(turns(&swing(20.0)) > turns(&swing(19.9)));
        assert!(swing(34.9).iter().all(|offset| offset.abs() <= 1));
        assert!(swing(35.0).contains(&2));
    }

    #[test]
    fn the_crown_bends_more_at_the_top() {
        assert_eq!(row_offset(0, 6, 1), 1);
        assert_eq!(row_offset(5, 6, 1), 0);
        assert_eq!(row_offset(5, 6, 2), 1);
        assert_eq!(row_offset(6, 6, 2), 0);
        assert_eq!(row_offset(0, 0, 2), 0);
    }
}
