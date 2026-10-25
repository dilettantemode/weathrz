#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Yard {
    pub wind_kmh: f32,
    pub toward_east: f32,
    pub frame: u64,
    pub wet: bool,
    pub road: bool,
    pub seed: u64,
}

const STILL_KMH: f32 = 8.0;
const GALE_KMH: f32 = 40.0;
const GUST_PERIOD: i64 = 24;
const UPRIGHT_EVERY: i64 = 5;
const RIPPLE_FRAMES: u64 = 20;
const RING_FRAMES: u64 = 12;
const BLOOMS: [char; 4] = ['@', '*', 'o', '&'];

pub const POND: [&str; 3] = [" .-~~~~~~~-.", "(  ~   ~    )", " `-._____.-'"];
pub const POND_FROM_HOUSE: i32 = -27;
pub const POND_BELOW_GROUND: i32 = 2;
pub const BEDS_FROM_HOUSE: [(i32, usize); 2] = [(1, 3), (26, 4)];
pub const BED_BELOW_GROUND: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Rim,
    Water,
    Lily,
    Ring,
    Bloom(usize),
    Stem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bit {
    pub dx: i32,
    pub dy: i32,
    pub ch: char,
    pub part: Part,
}

pub fn seed_for(latitude: f64, longitude: f64) -> u64 {
    let mix = (latitude * 1000.0).round() as i64 * 31 + (longitude * 1000.0).round() as i64;
    (mix as u64)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .rotate_left(17)
}

fn gust_width(wind_kmh: f32) -> i64 {
    let widest = GUST_PERIOD * 2 / 3;
    if wind_kmh < STILL_KMH {
        0
    } else if wind_kmh >= GALE_KMH {
        widest
    } else {
        let share = (wind_kmh - STILL_KMH) / (GALE_KMH - STILL_KMH);
        4 + (share * (widest - 4) as f32).round() as i64
    }
}

pub fn grass(base: char, x: u16, yard: &Yard) -> char {
    if base != '^' && base != ',' {
        return base;
    }
    let width = gust_width(yard.wind_kmh);
    if width == 0 {
        return base;
    }
    let step = (yard.frame / 3) as i64;
    let travel = if yard.toward_east < 0.0 { -step } else { step };
    let phase = (i64::from(x) - travel).rem_euclid(GUST_PERIOD);
    let inside = phase < width && phase % UPRIGHT_EVERY != UPRIGHT_EVERY - 1;
    match (inside, yard.toward_east < 0.0) {
        (false, _) => base,
        (true, false) => '/',
        (true, true) => '\\',
    }
}

pub fn pond(yard: &Yard) -> Vec<Bit> {
    let shift = (yard.frame / RIPPLE_FRAMES) as usize;
    let lily = yard.seed % 3 != 0;
    let ring = yard.wet.then(|| {
        (yard.frame / RING_FRAMES)
            .wrapping_mul(7)
            .wrapping_add(yard.seed)
            % 9
    });
    let mut out = Vec::new();
    for (dy, row) in POND.iter().enumerate() {
        for (dx, ch) in row.chars().enumerate() {
            if ch == ' ' && dy != 1 {
                continue;
            }
            let inside = dy == 1 && dx > 0 && dx < row.len() - 1;
            let bit = |ch: char, part: Part| Bit {
                dx: dx as i32,
                dy: dy as i32,
                ch,
                part,
            };
            if !inside {
                out.push(bit(ch, Part::Rim));
                continue;
            }
            let water = dx - 1;
            let ch = if lily && water == 7 {
                out.push(bit('@', Part::Lily));
                continue;
            } else if ring.is_some_and(|ring| ring as usize + 1 == water) {
                out.push(bit('o', Part::Ring));
                continue;
            } else if (water + shift) % 4 == 1 {
                '~'
            } else {
                ' '
            };
            out.push(bit(ch, Part::Water));
        }
    }
    out
}

pub fn bed(units: usize, seed: u64) -> Vec<Bit> {
    let mut out = Vec::new();
    for unit in 0..units {
        let pick = (seed.rotate_left(unit as u32 * 7) as usize).wrapping_add(unit);
        let left = unit as i32 * 4;
        out.push(Bit {
            dx: left + 1,
            dy: 0,
            ch: BLOOMS[pick % BLOOMS.len()],
            part: Part::Bloom(pick / BLOOMS.len()),
        });
        for (dx, ch) in [(0, '\\'), (1, '|'), (2, '/')] {
            out.push(Bit {
                dx: left + dx,
                dy: 1,
                ch,
                part: Part::Stem,
            });
        }
        let gaps: &[(i32, i32)] = if unit + 1 < units {
            &[(0, 0), (2, 0), (3, 0), (3, 1)]
        } else {
            &[(0, 0), (2, 0)]
        };
        for &(dx, dy) in gaps {
            out.push(Bit {
                dx: left + dx,
                dy,
                ch: ' ',
                part: Part::Stem,
            });
        }
    }
    out
}

pub fn road_row(dy: u16) -> Option<char> {
    match dy {
        1..=3 => Some(' '),
        4 => Some('='),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn yard(wind_kmh: f32, toward_east: f32, frame: u64) -> Yard {
        Yard {
            wind_kmh,
            toward_east,
            frame,
            ..Yard::default()
        }
    }

    #[test]
    fn grass_stands_in_calm_air_and_leans_downwind_in_gusts() {
        let calm = yard(3.0, 1.0, 0);
        assert!((0..60).all(|x| grass('^', x, &calm) == '^'));
        let breeze = yard(20.0, 1.0, 0);
        let leaning: Vec<char> = (0..48).map(|x| grass('^', x, &breeze)).collect();
        assert!(
            leaning.contains(&'/') && leaning.contains(&'^'),
            "{leaning:?}"
        );
        assert!(!leaning.contains(&'\\'));
        let westward = yard(20.0, -1.0, 0);
        assert!((0..48).any(|x| grass('^', x, &westward) == '\\'));
        let gale = yard(60.0, 1.0, 0);
        let bent: String = (0..120).map(|x| grass('^', x, &gale)).collect();
        assert!(bent.matches('/').count() <= 72, "{bent}");
        assert!(!bent.contains("/////"), "{bent}");
        assert_eq!(grass('*', 3, &gale), '*');
    }

    #[test]
    fn the_gust_travels() {
        let at = |frame| -> Vec<char> {
            (0..48)
                .map(|x| grass('^', x, &yard(20.0, 1.0, frame)))
                .collect()
        };
        assert_ne!(at(0), at(30));
    }

    #[test]
    fn the_pond_ripples_and_rain_rings_it() {
        let still = pond(&yard(0.0, 0.0, 0));
        let later = pond(&yard(0.0, 0.0, RIPPLE_FRAMES));
        assert_ne!(still, later);
        assert!(still.iter().any(|bit| bit.part == Part::Rim));
        assert!(!still.iter().any(|bit| bit.part == Part::Ring));
        let wet = pond(&Yard {
            wet: true,
            ..Yard::default()
        });
        assert!(wet.iter().any(|bit| bit.part == Part::Ring));
        let widths: Vec<usize> = POND.iter().map(|row| row.len()).collect();
        assert!(widths.iter().all(|w| *w <= 13), "{widths:?}");
    }

    #[test]
    fn beds_vary_with_the_place_but_stay_put() {
        let jakarta = bed(3, seed_for(-6.2088, 106.8456));
        assert_eq!(jakarta, bed(3, seed_for(-6.2088, 106.8456)));
        assert_eq!(
            jakarta
                .iter()
                .filter(|bit| matches!(bit.part, Part::Bloom(_)))
                .count(),
            3
        );
        assert_eq!(jakarta.iter().filter(|bit| bit.ch == '|').count(), 3);
        let blooms = |bits: &[Bit]| -> String {
            bits.iter()
                .filter(|bit| matches!(bit.part, Part::Bloom(_)))
                .map(|bit| bit.ch)
                .collect()
        };
        let places = [(51.5, -0.13), (35.7, 139.7), (-23.5, -46.6), (28.6, 77.2)];
        assert!(
            places
                .iter()
                .any(|(lat, lon)| blooms(&bed(3, seed_for(*lat, *lon))) != blooms(&jakarta))
        );
    }

    #[test]
    fn the_road_clears_three_rows_and_draws_a_kerb() {
        assert_eq!(road_row(0), None);
        assert_eq!(road_row(1), Some(' '));
        assert_eq!(road_row(3), Some(' '));
        assert_eq!(road_row(4), Some('='));
        assert_eq!(road_row(5), None);
    }
}
