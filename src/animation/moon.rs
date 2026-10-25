use crate::animation::sunny::arc_spot;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, TerminalSize};
use crate::astro::MoonSky;
use crate::events::Look;
use crate::glyphs::Glyphs;
use crate::render::TerminalRenderer;
use rand::Rng;
use std::io;

const HARVEST: (u8, u8, u8) = (250, 214, 140);
const HARVEST_SHARE: f32 = 0.55;
const FULL: &str = include_str!("assets/moon/full.txt");
const MIN_FRACTION: f64 = 0.02;
const BODY: char = '~';
const RIM: f64 = 0.97;
const QUADRANTS: [char; 16] = [
    ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
];
const QUARTERS: [(f64, f64); 4] = [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)];
const SIXTHS: [(f64, f64); 6] = [
    (-0.25, -1.0 / 3.0),
    (0.25, -1.0 / 3.0),
    (-0.25, 0.0),
    (0.25, 0.0),
    (-0.25, 1.0 / 3.0),
    (0.25, 1.0 / 3.0),
];

pub fn sextant(bits: usize) -> char {
    match bits {
        0 => ' ',
        21 => '▌',
        42 => '▐',
        63 => '█',
        _ => {
            let skipped = usize::from(bits > 21) + usize::from(bits > 42);
            char::from_u32(0x1FB00 + (bits - 1 - skipped) as u32).unwrap_or('█')
        }
    }
}

pub fn moon_anchor(width: u16, height: u16) -> (u16, u16) {
    (
        (width / 4 * 3).min(width.saturating_sub(15)),
        (height / 4).max(2),
    )
}

pub fn phase_step(phase: f64) -> usize {
    (phase * 8.0).round() as usize % 8
}

pub fn fraction(elongation_deg: f64) -> f64 {
    (1.0 - elongation_deg.to_radians().cos()) / 2.0
}

pub fn textbook_limb(phase: f64, southern: bool) -> f64 {
    let lit_right = (phase.rem_euclid(1.0) < 0.5) != southern;
    if lit_right { 270.0 } else { 90.0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Disc {
    pub elongation_deg: f64,
    pub limb_deg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoonCell {
    pub dx: u16,
    pub dy: u16,
    pub ch: char,
}

struct Shape {
    cells: Vec<(usize, usize, char)>,
    centre: (f64, f64),
    radius: (f64, f64),
}

fn shape() -> Shape {
    let cells: Vec<(usize, usize, char)> = FULL
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| *ch != ' ')
                .map(move |(x, ch)| (x, y, ch))
        })
        .collect();
    let xs = || cells.iter().map(|(x, _, _)| *x as f64);
    let ys = || cells.iter().map(|(_, y, _)| *y as f64);
    let (left, right) = (xs().fold(f64::MAX, f64::min), xs().fold(f64::MIN, f64::max));
    let (top, bottom) = (ys().fold(f64::MAX, f64::min), ys().fold(f64::MIN, f64::max));
    Shape {
        centre: ((left + right) / 2.0, (top + bottom) / 2.0),
        radius: ((right - left) / 2.0 + 0.5, (bottom - top) / 2.0 + 0.5),
        cells,
    }
}

pub fn art_width() -> u16 {
    FULL.lines()
        .map(|row| row.trim_end().len())
        .max()
        .unwrap_or(0) as u16
}

impl Disc {
    fn sun(self) -> (f64, f64, f64) {
        let phase_angle = (180.0 - self.elongation_deg.rem_euclid(360.0)).to_radians();
        let limb = self.limb_deg.to_radians();
        let (lx, ly) = (-limb.sin(), limb.cos());
        let side = phase_angle.sin().abs();
        (side * lx, side * ly, phase_angle.cos())
    }

    fn lit(self, u: f64, v: f64) -> bool {
        let (sx, sy, sz) = self.sun();
        let reach = (u * u + v * v).sqrt();
        let (u, v) = if reach > RIM {
            (u * RIM / reach, v * RIM / reach)
        } else {
            (u, v)
        };
        let w = (1.0 - u * u - v * v).max(0.0).sqrt();
        u * sx + v * sy + w * sz > 0.0
    }

    fn edge(self, shape: &Shape) -> (char, bool) {
        let (sx, sy, _) = self.sun();
        let (dx, dy) = (-sx, -sy);
        let (across, up) = (dx * shape.radius.0, dy * shape.radius.1 * 2.0);
        let thin = fraction(self.elongation_deg);
        if across.abs() >= 2.0 * up.abs() {
            let dark_left = dx < 0.0;
            let ch = if thin < 0.4 {
                if dark_left { ')' } else { '(' }
            } else if thin > 0.6 {
                if dark_left { '(' } else { ')' }
            } else {
                ':'
            };
            (ch, false)
        } else if up.abs() >= 2.0 * across.abs() {
            let dark_above = dy > 0.0;
            (if (thin < 0.5) == dark_above { '_' } else { '-' }, true)
        } else if (dx > 0.0) == (dy > 0.0) {
            ('\\', false)
        } else {
            ('/', false)
        }
    }

    pub fn ascii_cells(self) -> Vec<MoonCell> {
        let shape = shape();
        let (cx, cy) = shape.centre;
        let (rx, ry) = shape.radius;
        let lit = |x: usize, y: usize| self.lit((x as f64 - cx) / rx, (cy - y as f64) / ry);
        let inside = |x: usize, y: usize| {
            shape
                .cells
                .iter()
                .any(|(cell_x, cell_y, _)| (*cell_x, *cell_y) == (x, y))
        };
        let (edge, horizontal) = self.edge(&shape);
        shape
            .cells
            .iter()
            .map(|&(x, y, ch)| {
                let shown = if !lit(x, y) {
                    ' '
                } else if ch == BODY {
                    let neighbours = if horizontal {
                        [(x, y.wrapping_sub(1)), (x, y + 1)]
                    } else {
                        [(x.wrapping_sub(1), y), (x + 1, y)]
                    };
                    let at_terminator = neighbours
                        .iter()
                        .any(|&(nx, ny)| inside(nx, ny) && !lit(nx, ny));
                    if at_terminator { edge } else { ' ' }
                } else {
                    ch
                };
                MoonCell {
                    dx: x as u16,
                    dy: y as u16,
                    ch: shown,
                }
            })
            .collect()
    }

    pub fn block_cells(self) -> Vec<MoonCell> {
        self.sampled_cells(&QUARTERS, |bits| QUADRANTS[bits])
    }

    pub fn sextant_cells(self) -> Vec<MoonCell> {
        self.sampled_cells(&SIXTHS, sextant)
    }

    fn sampled_cells(self, samples: &[(f64, f64)], glyph: impl Fn(usize) -> char) -> Vec<MoonCell> {
        let shape = shape();
        let (cx, cy) = shape.centre;
        let (rx, ry) = shape.radius;
        let mut out = Vec::new();
        let rows = (cy + ry).ceil() as usize + 1;
        let columns = (cx + rx).ceil() as usize + 1;
        for y in 0..rows {
            for x in 0..columns {
                let mut bits = 0;
                let mut covered = false;
                for (bit, &(ox, oy)) in samples.iter().enumerate() {
                    let u = (x as f64 + ox - cx) / rx;
                    let v = (cy - (y as f64 + oy)) / ry;
                    if u * u + v * v <= 1.0 {
                        covered = true;
                        if self.lit(u, v) {
                            bits |= 1 << bit;
                        }
                    }
                }
                if covered {
                    out.push(MoonCell {
                        dx: x as u16,
                        dy: y as u16,
                        ch: glyph(bits),
                    });
                }
            }
        }
        out
    }
}

fn screen_limb(sky: MoonSky) -> f64 {
    let north_of_you = sky.azimuth_deg.to_radians().cos() > 0.0;
    if north_of_you {
        (360.0 - sky.limb_deg).rem_euclid(360.0)
    } else {
        sky.limb_deg
    }
}

pub struct MoonSpot {
    pub x: u16,
    pub y: u16,
    pub disc: Disc,
}

pub fn moon_spot(
    sky: Option<MoonSky>,
    phase: f64,
    southern: bool,
    (width, height, horizon_y): (u16, u16, u16),
) -> Option<MoonSpot> {
    let top = if height > 20 { 3 } else { 2 };
    match sky {
        Some(sky) => {
            if sky.altitude_deg <= 0.0 || fraction(sky.elongation_deg) < MIN_FRACTION {
                return None;
            }
            let (x, y) = arc_spot(
                sky.progress.unwrap_or(0.5),
                sky.altitude_deg,
                sky.peak_deg,
                (width, horizon_y, top),
                (art_width(), FULL.lines().count() as u16),
            );
            Some(MoonSpot {
                x,
                y,
                disc: Disc {
                    elongation_deg: sky.elongation_deg,
                    limb_deg: screen_limb(sky),
                },
            })
        }
        None => {
            let elongation_deg = phase.rem_euclid(1.0) * 360.0;
            if fraction(elongation_deg) < MIN_FRACTION {
                return None;
            }
            let (x, y) = moon_anchor(width, height);
            Some(MoonSpot {
                x,
                y,
                disc: Disc {
                    elongation_deg,
                    limb_deg: textbook_limb(phase, southern),
                },
            })
        }
    }
}

pub struct MoonSystem {
    phase: f64,
}

impl MoonSystem {
    pub fn new(_terminal_width: u16, _terminal_height: u16, phase: Option<f64>) -> Self {
        Self {
            phase: phase.unwrap_or(0.5),
        }
    }

    pub fn set_phase(&mut self, phase: f64) {
        self.phase = phase;
    }

    fn spot(&self, ctx: &FrameContext<'_>) -> Option<MoonSpot> {
        moon_spot(
            ctx.state.moon_sky,
            self.phase,
            ctx.state.location.latitude < 0.0,
            (ctx.size.width, ctx.size.height, ctx.horizon_y),
        )
    }
}

impl AnimationSystem for MoonSystem {
    fn id(&self) -> &'static str {
        "moon"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Background
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        !ctx.conditions.sun.is_day
    }

    fn on_resize(&mut self, _size: TerminalSize) {}

    fn on_moon_phase(&mut self, phase: f64) {
        self.set_phase(phase);
    }

    fn update(
        &mut self,
        _ctx: &FrameContext<'_>,
        _rng: &mut dyn Rng,
        _commands: &mut FrameCommands,
    ) {
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        let Some(spot) = self.spot(ctx) else {
            return Ok(());
        };
        let cells = match ctx.state.glyphs {
            Glyphs::Ascii => spot.disc.ascii_cells(),
            Glyphs::Unicode => spot.disc.block_cells(),
            Glyphs::Fine => spot.disc.sextant_cells(),
        };
        let veiled = ctx.conditions.veils_the_sky();
        let harvest = ctx.events.has(Look::HarvestMoon);
        let moon = if harvest {
            crate::render::rgb(ctx.colors.moon).map_or(ctx.colors.moon, |moon| {
                crate::render::mix(moon, HARVEST, HARVEST_SHARE)
            })
        } else {
            ctx.colors.moon
        };
        for cell in cells {
            let (x, y) = (
                spot.x.saturating_add(cell.dx),
                spot.y.saturating_add(cell.dy),
            );
            let color = if veiled {
                renderer.veiled(y, moon)
            } else {
                moon
            };
            renderer.render_char(x, y, cell.ch, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drawn(cells: &[MoonCell]) -> String {
        let rows = cells.iter().map(|cell| cell.dy).max().unwrap_or(0) as usize + 1;
        let columns = cells.iter().map(|cell| cell.dx).max().unwrap_or(0) as usize + 1;
        let mut grid = vec![vec![' '; columns]; rows];
        for cell in cells {
            grid[cell.dy as usize][cell.dx as usize] = cell.ch;
        }
        grid.iter()
            .map(|row| row.iter().collect::<String>().trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn shown(cells: &[MoonCell]) -> usize {
        cells.iter().filter(|cell| cell.ch != ' ').count()
    }

    fn disc(elongation_deg: f64, limb_deg: f64) -> Disc {
        Disc {
            elongation_deg,
            limb_deg,
        }
    }

    #[test]
    fn the_full_moon_is_the_whole_drawing() {
        let full = disc(180.0, 0.0).ascii_cells();
        let expected: Vec<String> = FULL
            .lines()
            .map(|row| row.trim_end().replace('~', " "))
            .collect();
        assert_eq!(drawn(&full), expected.join("\n"));
        assert_eq!(shown(&disc(0.0, 0.0).ascii_cells()), 0);
    }

    #[test]
    fn the_lit_part_grows_to_full_and_shrinks_again() {
        let counts: Vec<usize> = (0..=8)
            .map(|step| shown(&disc(f64::from(step) * 45.0, 270.0).ascii_cells()))
            .collect();
        assert!(
            counts[..5].windows(2).all(|pair| pair[0] <= pair[1]),
            "{counts:?}"
        );
        assert!(
            counts[4..].windows(2).all(|pair| pair[0] >= pair[1]),
            "{counts:?}"
        );
        assert!(counts[1] > 0);
    }

    fn sides(text: &str) -> (usize, usize) {
        let mut sides = (0, 0);
        for row in text.lines() {
            for (x, ch) in row.chars().enumerate() {
                if ch != ' ' {
                    if x < 9 {
                        sides.0 += 1;
                    } else {
                        sides.1 += 1;
                    }
                }
            }
        }
        sides
    }

    #[test]
    fn a_textbook_crescent_is_lit_on_the_right_and_its_mirror_on_the_left() {
        let right = drawn(&disc(60.0, 270.0).ascii_cells());
        let left = drawn(&disc(60.0, 90.0).ascii_cells());
        let (l, r) = sides(&right);
        assert!(r > 2 * l, "{right}");
        let (l, r) = sides(&left);
        assert!(l > 2 * r, "{left}");
        assert!(right.contains(')') && left.contains('('), "{right}\n{left}");
    }

    #[test]
    fn near_the_equator_the_crescent_lies_on_its_back() {
        let boat = drawn(&disc(50.0, 200.0).ascii_cells());
        let rows: Vec<&str> = boat.lines().collect();
        let lit_in = |row: &str| row.chars().filter(|ch| *ch != ' ').count();
        assert!(lit_in(rows.last().unwrap()) > lit_in(rows[0]), "{boat}");
        assert!(boat.contains('_') || boat.contains('-'), "{boat}");
    }

    #[test]
    fn a_tilted_terminator_is_one_mark_per_row_not_hatching() {
        for limb in [30.0, 120.0, 210.0, 300.0] {
            let gibbous = drawn(&disc(140.0, limb).ascii_cells());
            assert!(
                !gibbous.contains("\\\\") && !gibbous.contains("//"),
                "{limb}\n{gibbous}"
            );
        }
    }

    #[test]
    fn the_half_moon_keeps_a_soft_edge() {
        let half = drawn(&disc(90.0, 270.0).ascii_cells());
        assert!(half.contains(':'), "{half}");
        assert!(!half.contains('|'), "{half}");
    }

    #[test]
    fn sextants_cover_all_sixty_four_patterns() {
        assert_eq!(sextant(0), ' ');
        assert_eq!(sextant(1), '\u{1fb00}');
        assert_eq!(sextant(20), '\u{1fb13}');
        assert_eq!(sextant(21), '▌');
        assert_eq!(sextant(22), '\u{1fb14}');
        assert_eq!(sextant(42), '▐');
        assert_eq!(sextant(62), '\u{1fb3b}');
        assert_eq!(sextant(63), '█');
        let all: std::collections::HashSet<char> = (0..64).map(sextant).collect();
        assert_eq!(all.len(), 64);
    }

    #[test]
    fn the_fine_moon_is_lit_on_the_same_side_and_rounder() {
        let crescent = disc(60.0, 270.0);
        let fine = crescent.sextant_cells();
        let lit_x: Vec<u16> = fine
            .iter()
            .filter(|cell| cell.ch != ' ')
            .map(|cell| cell.dx)
            .collect();
        assert!(lit_x.iter().all(|x| *x >= 8), "{lit_x:?}");
        let full = disc(180.0, 0.0);
        let rows = |cells: &[MoonCell]| cells.iter().map(|cell| cell.dy).max();
        assert_eq!(rows(&full.sextant_cells()), rows(&full.block_cells()));
        let partial = |cells: &[MoonCell]| {
            cells
                .iter()
                .filter(|cell| !matches!(cell.ch, ' ' | '█'))
                .count()
        };
        assert!(
            full.sextant_cells()
                .iter()
                .any(|cell| ('\u{1fb00}'..='\u{1fb3b}').contains(&cell.ch)),
            "the rim uses sextants"
        );
        assert!(partial(&full.sextant_cells()) > 0);
    }

    #[test]
    fn blocks_fill_the_disc_and_follow_the_same_light() {
        let full = disc(180.0, 0.0).block_cells();
        assert!(full.iter().filter(|cell| cell.ch == '█').count() > 20);
        let crescent = disc(60.0, 270.0).block_cells();
        let lit_x: Vec<u16> = crescent
            .iter()
            .filter(|cell| cell.ch != ' ')
            .map(|cell| cell.dx)
            .collect();
        assert!(lit_x.iter().all(|x| *x >= 8), "{lit_x:?}");
        assert!(
            crescent.iter().any(|cell| cell.ch == ' '),
            "the dark side hides the stars"
        );
    }

    #[test]
    fn the_moon_shows_only_above_the_horizon_and_follows_its_path() {
        let sky = |altitude_deg: f64, progress: f64| MoonSky {
            altitude_deg,
            azimuth_deg: 180.0,
            limb_deg: 200.0,
            elongation_deg: 120.0,
            progress: Some(progress),
            peak_deg: 80.0,
        };
        let size = (110, 32, 25);
        assert!(moon_spot(Some(sky(-3.0, 0.5)), 0.3, true, size).is_none());
        let rising = moon_spot(Some(sky(5.0, 0.05)), 0.3, true, size).unwrap();
        let high = moon_spot(Some(sky(78.0, 0.5)), 0.3, true, size).unwrap();
        let setting = moon_spot(Some(sky(5.0, 0.95)), 0.3, true, size).unwrap();
        assert!(rising.x < high.x && high.x < setting.x);
        assert!(high.y < rising.y);
        assert_eq!(high.disc.limb_deg, 200.0);
        let north = MoonSky {
            azimuth_deg: 295.0,
            limb_deg: 132.0,
            ..sky(60.0, 0.6)
        };
        let mirrored = moon_spot(Some(north), 0.3, true, size).unwrap();
        assert_eq!(
            mirrored.disc.limb_deg, 228.0,
            "a moon north of you faces the screen's sunset"
        );
        let new_moon = MoonSky {
            elongation_deg: 5.0,
            ..sky(40.0, 0.5)
        };
        assert!(moon_spot(Some(new_moon), 0.0, true, size).is_none());
    }

    #[test]
    fn without_a_sky_the_moon_keeps_its_old_place_and_textbook_side() {
        let spot = moon_spot(None, 0.25, false, (110, 32, 25)).unwrap();
        assert_eq!((spot.x, spot.y), moon_anchor(110, 32));
        assert_eq!(spot.disc.limb_deg, 270.0);
        assert_eq!(
            moon_spot(None, 0.25, true, (110, 32, 25))
                .unwrap()
                .disc
                .limb_deg,
            90.0
        );
        assert!(moon_spot(None, 0.0, false, (110, 32, 25)).is_none());
    }
}
