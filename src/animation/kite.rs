use crate::animation::flyers::catalogue::{KITE, Sprite, parse_frames};
use crate::animation::flyers::mask;
use crate::animation::sky::SkyBand;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, Wind};
use crate::events::Look;
use crate::render::TerminalRenderer;
use rand::{Rng, RngExt};
use std::io;
use std::ops::RangeInclusive;

pub const CREDIT: &str = "K4, Dr Andrew H.J. Tate (adapted)";

const FLYER_ART: &str = include_str!("assets/kite/flyer.txt");
const FLYER_MASK: &str = include_str!("assets/kite/flyer.mask.txt");
const KITE_ART: &str = include_str!("assets/kite/kite.txt");
const KITE_MASK: &str = include_str!("assets/kite/kite.mask.txt");
const FESTIVAL_KITE_MASK: &str = "rr\nff";
const BRIGHT_KITE_MASK: &str = "mm\nyy";

const FPS: u32 = 30;
const MIN_WIND_KMH: f32 = 10.0;
const FESTIVAL_MIN_WIND_KMH: f32 = 5.0;
const TOP_WIND_KMH: f32 = 30.0;
const STRONG_SWAY_KMH: f32 = 25.0;
const FIRST_COOLDOWN: RangeInclusive<u32> = (60 * FPS)..=(180 * FPS);
const COOLDOWN: RangeInclusive<u32> = (20 * 60 * FPS)..=(30 * 60 * FPS);
const PREVIEW_COOLDOWN: RangeInclusive<u32> = 0..=FPS;
const SPELL: RangeInclusive<u32> = (3 * 60 * FPS)..=(5 * 60 * FPS);
const WALK_COLUMNS: i32 = 5;
const TICKS_PER_STEP: u32 = 12;
const CLIMB_TICKS: u32 = 5 * FPS;
const REEL_TICKS: u32 = 3 * FPS;
const LEG_TICKS: u32 = 9;
const HAND_FROM_RIGHT: i32 = 5;
const HAND_ROW: i32 = 1;
const STRING_SLOPE: f32 = 0.6;
const SWAY_PERIOD_TICKS: u32 = 3 * FPS;
const STRING_MASK: char = 's';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stage {
    Waiting,
    WalkOn,
    Climb,
    Fly,
    Reel,
    WalkOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct KiteNow {
    pub width: u16,
    pub horizon_y: u16,
    pub band: SkyBand,
    pub is_day: bool,
    pub clear: bool,
    pub offline: bool,
    pub festival: bool,
    pub flag_kite: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Piece {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub mask: char,
}

struct Art {
    frames: Vec<Sprite>,
    masks: Vec<Sprite>,
}

impl Art {
    fn parse(art: &str, mask: &str) -> Self {
        Self {
            frames: parse_frames(art),
            masks: parse_frames(mask),
        }
    }

    fn cells(&self, frame: usize, x: i32, y: i32) -> Vec<Piece> {
        let mut cells = Vec::new();
        let rows = self.frames[frame].rows.iter().zip(&self.masks[frame].rows);
        for (dy, (row, letters)) in rows.enumerate() {
            for (dx, (ch, letter)) in row.chars().zip(letters.chars()).enumerate() {
                if ch != ' ' {
                    cells.push(Piece {
                        x: x + dx as i32,
                        y: y + dy as i32,
                        ch,
                        mask: letter,
                    });
                }
            }
        }
        cells
    }
}

fn kite_now(ctx: &FrameContext<'_>) -> KiteNow {
    KiteNow {
        width: ctx.size.width,
        horizon_y: ctx.horizon_y,
        band: SkyBand::below_hud(ctx.skyline_y),
        is_day: ctx.conditions.sun.is_day,
        clear: !ctx.conditions.is_raining
            && !ctx.conditions.is_thunderstorm
            && !ctx.conditions.is_snowing
            && !ctx.conditions.is_foggy,
        offline: ctx.state.is_offline,
        festival: ctx.events.has(Look::Kites),
        flag_kite: ctx
            .events
            .country_of(Look::Kites)
            .is_some_and(|country| country.eq_ignore_ascii_case("id")),
    }
}

fn kite_bottom(band: SkyBand, wind_kmh: f32) -> i32 {
    let t = ((wind_kmh - MIN_WIND_KMH) / (TOP_WIND_KMH - MIN_WIND_KMH)).clamp(0.0, 1.0);
    band.bottom as i32 - 1 - (t * (band.height() as i32 - 2) as f32).round() as i32
}

fn amplitude(wind_kmh: f32) -> i32 {
    if wind_kmh >= STRONG_SWAY_KMH { 2 } else { 1 }
}

fn run(rows: i32, sway: i32) -> i32 {
    ((STRING_SLOPE * rows as f32).round() as i32 - 1 - sway).clamp(0, (rows - 1).max(0))
}

fn string_rows(now: KiteNow, wind_kmh: f32) -> Option<i32> {
    if now.band.height() < 2 {
        return None;
    }
    let hand_x = now.width as i32 - HAND_FROM_RIGHT;
    let lowest = now.band.bottom as i32 - 1;
    (kite_bottom(now.band, wind_kmh)..=lowest)
        .map(|bottom| now.horizon_y as i32 - 1 - bottom)
        .find(|&rows| rows >= 1 && hand_x - 3 - run(rows, -amplitude(wind_kmh)) >= 0)
}

fn scale(full: i32, ticks: u32, total: u32) -> i32 {
    (full as f32 * ticks.min(total) as f32 / total as f32).round() as i32
}

pub struct KiteSystem {
    preview: bool,
    wind_kmh: Option<f32>,
    stage: Stage,
    cooldown: Option<u32>,
    stage_ticks: u32,
    spell_ticks: u32,
    spell_left: u32,
    offset: i32,
    flyer: Art,
    kite: Art,
    festival_kite: Art,
    bright_kite: Art,
}

impl KiteSystem {
    pub fn new(preview: bool) -> Self {
        Self {
            preview,
            wind_kmh: None,
            stage: Stage::Waiting,
            cooldown: None,
            stage_ticks: 0,
            spell_ticks: 0,
            spell_left: 0,
            offset: WALK_COLUMNS,
            flyer: Art::parse(FLYER_ART, FLYER_MASK),
            kite: Art::parse(KITE_ART, KITE_MASK),
            festival_kite: Art::parse(KITE_ART, FESTIVAL_KITE_MASK),
            bright_kite: Art::parse(KITE_ART, BRIGHT_KITE_MASK),
        }
    }

    fn wind(&self, now: KiteNow) -> f32 {
        let wind = if now.offline {
            0.0
        } else {
            self.wind_kmh.unwrap_or(0.0)
        };
        if self.preview {
            wind.max(MIN_WIND_KMH)
        } else {
            wind
        }
    }

    fn wanted(&self, now: KiteNow) -> bool {
        let least = if now.festival {
            FESTIVAL_MIN_WIND_KMH
        } else {
            MIN_WIND_KMH
        };
        self.preview || (now.is_day && self.wind(now) >= least)
    }

    fn endless(&self, now: KiteNow) -> bool {
        self.preview || now.festival
    }

    fn possible(&self, now: KiteNow) -> bool {
        now.clear && string_rows(now, self.wind(now)).is_some()
    }

    fn enter(&mut self, stage: Stage) {
        self.stage = stage;
        self.stage_ticks = 0;
    }

    fn rest(&mut self, endless: bool, rng: &mut (impl Rng + ?Sized)) {
        self.enter(Stage::Waiting);
        self.offset = WALK_COLUMNS;
        let range = if endless { PREVIEW_COOLDOWN } else { COOLDOWN };
        self.cooldown = Some(rng.random_range(range));
    }

    pub(crate) fn advance(&mut self, now: KiteNow, rng: &mut (impl Rng + ?Sized)) {
        if self.stage == Stage::Waiting {
            self.wait(now, rng);
            return;
        }
        if !self.possible(now) {
            self.rest(self.endless(now), rng);
            return;
        }
        self.spell_ticks = (self.spell_ticks + 1) % SWAY_PERIOD_TICKS;
        self.stage_ticks = self.stage_ticks.saturating_add(1);
        match self.stage {
            Stage::Waiting => {}
            Stage::WalkOn if !self.wanted(now) => self.enter(Stage::WalkOff),
            Stage::WalkOn => {
                if self.stage_ticks % TICKS_PER_STEP == 0 {
                    self.offset -= 1;
                    if self.offset == 0 {
                        self.enter(Stage::Climb);
                    }
                }
            }
            Stage::Climb if !self.wanted(now) => {
                let climbed = self.stage_ticks.min(CLIMB_TICKS);
                self.enter(Stage::Reel);
                self.stage_ticks = REEL_TICKS - climbed * REEL_TICKS / CLIMB_TICKS;
            }
            Stage::Climb => {
                if self.stage_ticks >= CLIMB_TICKS {
                    self.enter(Stage::Fly);
                }
            }
            Stage::Fly => {
                let endless = self.endless(now);
                if !endless {
                    self.spell_left = self.spell_left.saturating_sub(1);
                }
                if (!endless && self.spell_left == 0) || !self.wanted(now) {
                    self.enter(Stage::Reel);
                }
            }
            Stage::Reel => {
                if self.stage_ticks >= REEL_TICKS {
                    self.enter(Stage::WalkOff);
                }
            }
            Stage::WalkOff => {
                if self.stage_ticks % TICKS_PER_STEP == 0 {
                    self.offset += 1;
                    if self.offset >= WALK_COLUMNS {
                        self.rest(self.endless(now), rng);
                    }
                }
            }
        }
    }

    fn wait(&mut self, now: KiteNow, rng: &mut (impl Rng + ?Sized)) {
        if self.cooldown.is_none() {
            if !self.preview && !now.is_day {
                return;
            }
            let range = if self.endless(now) {
                PREVIEW_COOLDOWN
            } else {
                FIRST_COOLDOWN
            };
            self.cooldown = Some(rng.random_range(range));
        }
        if now.festival
            && let Some(cooldown) = self.cooldown.as_mut()
        {
            *cooldown = (*cooldown).min(*PREVIEW_COOLDOWN.end());
        }
        if let Some(cooldown) = self.cooldown.as_mut().filter(|cooldown| **cooldown > 0) {
            *cooldown -= 1;
            return;
        }
        if self.wanted(now) && self.possible(now) {
            self.spell_left = if self.endless(now) {
                0
            } else {
                rng.random_range(SPELL)
            };
            self.spell_ticks = 0;
            self.offset = WALK_COLUMNS;
            self.enter(Stage::WalkOn);
        }
    }

    fn sway(&self, wind_kmh: f32) -> i32 {
        let phase = std::f32::consts::TAU * self.spell_ticks as f32 / SWAY_PERIOD_TICKS as f32;
        (amplitude(wind_kmh) as f32 * phase.sin()).round() as i32
    }

    fn sky_cells(&self, hand: (i32, i32), rows: i32, sway: i32, now: KiteNow) -> Vec<Piece> {
        let (hand_x, hand_y) = hand;
        let across = run(rows, sway);
        let kite_x = hand_x - 3 - across;
        let kite_bottom = hand_y - 1 - rows;
        let art = match (now.festival, now.flag_kite) {
            (true, true) => &self.festival_kite,
            (true, false) => &self.bright_kite,
            (false, _) => &self.kite,
        };
        let mut cells = art.cells(0, kite_x, kite_bottom - 1);
        let mut previous = kite_x + 1;
        for i in 0..rows {
            let step = if rows > 1 {
                (i as f32 * across as f32 / (rows - 1) as f32).round() as i32
            } else {
                0
            };
            let x = kite_x + 2 + step;
            let ch = if x > previous { '\\' } else { '|' };
            cells.push(Piece {
                x,
                y: kite_bottom + 1 + i,
                ch,
                mask: STRING_MASK,
            });
            previous = x;
        }
        cells
    }

    pub(crate) fn cells(&self, now: KiteNow) -> Vec<Piece> {
        if self.stage == Stage::Waiting {
            return Vec::new();
        }
        let wind = self.wind(now);
        let Some(full) = string_rows(now, wind) else {
            return Vec::new();
        };
        let hand_x = now.width as i32 - HAND_FROM_RIGHT + self.offset;
        let hand_y = now.horizon_y as i32;
        let walking = matches!(self.stage, Stage::WalkOn | Stage::WalkOff);
        let legs = if walking {
            (self.stage_ticks / LEG_TICKS % 2) as usize
        } else {
            0
        };
        let mut cells = self.flyer.cells(legs, hand_x, hand_y - HAND_ROW);
        let rows = match self.stage {
            Stage::Climb => Some(scale(full, self.stage_ticks, CLIMB_TICKS)),
            Stage::Fly => Some(full),
            Stage::Reel => Some(full - scale(full, self.stage_ticks, REEL_TICKS)),
            _ => None,
        };
        if let Some(rows) = rows {
            let sway = if self.stage == Stage::Fly {
                self.sway(wind)
            } else {
                0
            };
            cells.extend(self.sky_cells((hand_x, hand_y), rows, sway, now));
        }
        cells.retain(|cell| (0..now.width as i32).contains(&cell.x) && cell.y >= 0);
        cells
    }
}

impl AnimationSystem for KiteSystem {
    fn id(&self) -> &'static str {
        KITE
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, _ctx: &FrameContext<'_>) -> bool {
        true
    }

    fn on_wind(&mut self, wind: Wind) {
        self.wind_kmh = Some(wind.speed_kmh);
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.advance(kite_now(ctx), rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        for cell in self.cells(kite_now(ctx)) {
            let color =
                mask::colour(cell.mask, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            renderer.render_char(cell.x as u16, cell.y as u16, cell.ch, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn now(width: u16) -> KiteNow {
        KiteNow {
            width,
            horizon_y: 25,
            band: SkyBand::below_hud(15),
            is_day: true,
            clear: true,
            offline: false,
            festival: false,
            flag_kite: false,
        }
    }

    fn windy(kmh: f32, preview: bool) -> KiteSystem {
        let mut kite = KiteSystem::new(preview);
        kite.on_wind(Wind {
            speed_kmh: kmh,
            direction_deg: 0.0,
        });
        kite
    }

    fn run_until(kite: &mut KiteSystem, now: KiteNow, stage: Stage, rng: &mut StdRng) -> u32 {
        for tick in 0..(40 * 60 * FPS) {
            if kite.stage == stage {
                return tick;
            }
            kite.advance(now, rng);
        }
        panic!("never reached {stage:?}");
    }

    #[test]
    fn the_kite_and_flyer_masks_match_their_art() {
        for (art, mask) in [
            (FLYER_ART, FLYER_MASK),
            (KITE_ART, KITE_MASK),
            (KITE_ART, FESTIVAL_KITE_MASK),
            (KITE_ART, BRIGHT_KITE_MASK),
        ] {
            let frames = parse_frames(art);
            let masks = parse_frames(mask);
            assert_eq!(frames.len(), masks.len());
            for (frame, letters) in frames.iter().zip(&masks) {
                assert_eq!(frame.rows.len(), letters.rows.len());
                for (row, mask_row) in frame.rows.iter().zip(&letters.rows) {
                    let width = row.len().max(mask_row.len());
                    let cells = format!("{row:<width$}");
                    let marks = format!("{mask_row:<width$}");
                    for (ch, letter) in cells.chars().zip(marks.chars()) {
                        assert_eq!(ch == ' ', letter == ' ', "{row:?} / {mask_row:?}");
                        assert!(
                            letter == ' ' || mask::LETTERS.contains(&letter),
                            "{letter:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn a_preview_can_fly_for_years() {
        let mut rng = StdRng::seed_from_u64(8);
        let mut kite = KiteSystem::new(true);
        run_until(&mut kite, now(110), Stage::Fly, &mut rng);
        kite.spell_ticks = u32::MAX - 2;
        kite.stage_ticks = u32::MAX - 2;
        for _ in 0..10 {
            kite.advance(now(110), &mut rng);
        }
        assert_eq!(kite.stage, Stage::Fly);
        assert!(!kite.cells(now(110)).is_empty());
    }

    #[test]
    fn dusk_during_the_walk_on_sends_the_flyer_back() {
        let mut rng = StdRng::seed_from_u64(9);
        let mut kite = windy(20.0, false);
        let day = now(110);
        run_until(&mut kite, day, Stage::WalkOn, &mut rng);
        for _ in 0..TICKS_PER_STEP * 2 {
            kite.advance(day, &mut rng);
        }
        kite.advance(
            KiteNow {
                is_day: false,
                ..day
            },
            &mut rng,
        );
        assert_eq!(kite.stage, Stage::WalkOff);
    }

    #[test]
    fn dusk_during_the_climb_reels_in_from_the_same_height() {
        let mut rng = StdRng::seed_from_u64(10);
        let mut kite = windy(20.0, false);
        let day = now(110);
        let dusk = KiteNow {
            is_day: false,
            ..day
        };
        run_until(&mut kite, day, Stage::Climb, &mut rng);
        for _ in 0..CLIMB_TICKS / 2 {
            kite.advance(day, &mut rng);
        }
        let string = |kite: &KiteSystem, at| {
            kite.cells(at)
                .iter()
                .filter(|cell| cell.mask == STRING_MASK)
                .count()
        };
        let before = string(&kite, day);
        assert!(before > 2, "{before}");
        kite.advance(dusk, &mut rng);
        assert_eq!(kite.stage, Stage::Reel);
        let after = string(&kite, dusk);
        assert!(before.abs_diff(after) <= 1, "{before} -> {after}");
    }

    #[test]
    fn the_kite_climbs_with_the_wind() {
        let band = SkyBand::below_hud(15);
        assert_eq!(kite_bottom(band, 10.0), 14);
        assert_eq!(kite_bottom(band, 20.0), 9);
        assert_eq!(kite_bottom(band, 30.0), 4);
        assert_eq!(kite_bottom(band, 60.0), 4);
        assert_eq!(string_rows(now(110), 20.0), Some(15));
    }

    #[test]
    fn no_band_or_no_room_means_no_kite() {
        assert_eq!(
            string_rows(
                KiteNow {
                    band: SkyBand::below_hud(4),
                    ..now(110)
                },
                20.0
            ),
            None
        );
        assert_eq!(string_rows(now(6), 10.0), None);
        assert_eq!(string_rows(now(20), 30.0), Some(19));
    }

    #[test]
    fn the_kite_never_crosses_the_left_edge() {
        let kite = KiteSystem::new(false);
        for width in 10..=140u16 {
            for wind in [10.0, 18.0, 26.0, 40.0] {
                let Some(rows) = string_rows(now(width), wind) else {
                    continue;
                };
                let swing = amplitude(wind);
                for sway in -swing..=swing {
                    for cell in
                        kite.sky_cells((width as i32 - HAND_FROM_RIGHT, 25), rows, sway, now(width))
                    {
                        assert!(cell.x >= 0, "{width} {wind} {sway}");
                    }
                }
            }
        }
    }

    #[test]
    fn the_string_runs_from_the_kite_to_the_hand() {
        let kite = KiteSystem::new(false);
        for sway in -2..=2 {
            let cells = kite.sky_cells((105, 25), 15, sway, now(120));
            let string: Vec<&Piece> = cells.iter().filter(|cell| cell.mask == 's').collect();
            assert_eq!(string.len(), 15);
            assert_eq!((string[14].x, string[14].y), (104, 24));
            assert_eq!(string[0].ch, '\\');
            for pair in string.windows(2) {
                assert_eq!(pair[1].y, pair[0].y + 1);
                assert!((0..=1).contains(&(pair[1].x - pair[0].x)), "{sway}");
                let expected = if pair[1].x > pair[0].x { '\\' } else { '|' };
                assert_eq!(pair[1].ch, expected);
            }
            let kite_cells: Vec<&Piece> = cells.iter().filter(|cell| cell.mask != 's').collect();
            assert_eq!(kite_cells.len(), 4);
            assert_eq!(kite_cells.iter().map(|cell| cell.y).max(), Some(9));
            assert_eq!(
                kite_cells.iter().map(|cell| cell.ch).collect::<String>(),
                "/\\\\/"
            );
        }
    }

    #[test]
    fn a_spell_walks_on_climbs_flies_reels_in_and_walks_off() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut kite = windy(20.0, false);
        let day = now(110);
        let first = run_until(&mut kite, day, Stage::WalkOn, &mut rng);
        assert!((60 * FPS..=180 * FPS + 1).contains(&first), "{first}");
        let walk = WALK_COLUMNS as u32 * TICKS_PER_STEP;
        assert_eq!(run_until(&mut kite, day, Stage::Climb, &mut rng), walk);
        assert_eq!(
            kite.cells(day)
                .iter()
                .find(|cell| cell.ch == '+')
                .map(|cell| (cell.x, cell.y)),
            Some((105, 25))
        );
        assert_eq!(run_until(&mut kite, day, Stage::Fly, &mut rng), CLIMB_TICKS);
        let flying = run_until(&mut kite, day, Stage::Reel, &mut rng);
        assert!(SPELL.contains(&flying), "{flying}");
        assert_eq!(
            run_until(&mut kite, day, Stage::WalkOff, &mut rng),
            REEL_TICKS
        );
        assert_eq!(run_until(&mut kite, day, Stage::Waiting, &mut rng), walk);
        let rest = run_until(&mut kite, day, Stage::WalkOn, &mut rng);
        assert!(
            (20 * 60 * FPS..=30 * 60 * FPS + 1).contains(&rest),
            "{rest}"
        );
    }

    #[test]
    fn the_kite_reels_in_when_the_wind_drops_or_at_dusk() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut kite = windy(20.0, false);
        run_until(&mut kite, now(110), Stage::Fly, &mut rng);
        kite.on_wind(Wind {
            speed_kmh: 8.0,
            direction_deg: 0.0,
        });
        kite.advance(now(110), &mut rng);
        assert_eq!(kite.stage, Stage::Reel);

        let mut kite = windy(20.0, false);
        run_until(&mut kite, now(110), Stage::Fly, &mut rng);
        kite.advance(
            KiteNow {
                is_day: false,
                ..now(110)
            },
            &mut rng,
        );
        assert_eq!(kite.stage, Stage::Reel);
    }

    #[test]
    fn rain_ends_the_spell_at_once() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut kite = windy(20.0, false);
        run_until(&mut kite, now(110), Stage::Fly, &mut rng);
        let wet = KiteNow {
            clear: false,
            ..now(110)
        };
        kite.advance(wet, &mut rng);
        assert_eq!(kite.stage, Stage::Waiting);
        assert!(kite.cells(wet).is_empty());
        assert!(COOLDOWN.contains(&kite.cooldown.unwrap()));
    }

    #[test]
    fn independence_day_keeps_a_red_and_white_kite_up_all_day_in_a_light_wind() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut kite = windy(6.0, false);
        let festival = KiteNow {
            festival: true,
            flag_kite: true,
            ..now(110)
        };
        let waited = run_until(&mut kite, festival, Stage::Fly, &mut rng);
        assert!(waited < 15 * FPS, "took {waited} ticks");
        for _ in 0..(2 * 60 * 60 * FPS) {
            kite.advance(festival, &mut rng);
            assert_eq!(kite.stage, Stage::Fly);
        }
        let letters: Vec<char> = kite
            .cells(festival)
            .iter()
            .filter(|cell| matches!(cell.ch, '/' | '\\') && cell.mask != STRING_MASK)
            .map(|cell| cell.mask)
            .collect();
        assert!(
            letters.contains(&'r') && letters.contains(&'f'),
            "{letters:?}"
        );
        let uttarayan = KiteNow {
            flag_kite: false,
            ..festival
        };
        let bright: Vec<char> = kite
            .cells(uttarayan)
            .iter()
            .filter(|cell| matches!(cell.ch, '/' | '\\') && cell.mask != STRING_MASK)
            .map(|cell| cell.mask)
            .collect();
        assert!(
            bright.contains(&'m') && bright.contains(&'y') && !bright.contains(&'r'),
            "{bright:?}"
        );
        let dusk = KiteNow {
            is_day: false,
            ..festival
        };
        run_until(&mut kite, dusk, Stage::Waiting, &mut rng);
        let mut ordinary = windy(6.0, false);
        for _ in 0..(10 * 60 * FPS) {
            ordinary.advance(now(110), &mut rng);
            assert_eq!(ordinary.stage, Stage::Waiting);
        }
    }

    #[test]
    fn offline_wind_counts_as_calm() {
        let mut rng = StdRng::seed_from_u64(4);
        let mut kite = windy(40.0, false);
        let offline = KiteNow {
            offline: true,
            ..now(110)
        };
        for _ in 0..(10 * 60 * FPS) {
            kite.advance(offline, &mut rng);
            assert_eq!(kite.stage, Stage::Waiting);
        }
        let mut unknown = KiteSystem::new(false);
        for _ in 0..(10 * 60 * FPS) {
            unknown.advance(now(110), &mut rng);
            assert_eq!(unknown.stage, Stage::Waiting);
        }
    }

    #[test]
    fn night_ticks_draw_nothing() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut kite = windy(20.0, false);
        let night = KiteNow {
            is_day: false,
            ..now(110)
        };
        for _ in 0..1000 {
            kite.advance(night, &mut rng);
        }
        assert_eq!(kite.cooldown, None);
        assert_eq!(kite.stage, Stage::Waiting);
    }

    #[test]
    fn the_preview_comes_within_a_second_and_stays() {
        let mut rng = StdRng::seed_from_u64(6);
        let mut kite = KiteSystem::new(true);
        let night = KiteNow {
            is_day: false,
            ..now(110)
        };
        assert!(run_until(&mut kite, night, Stage::WalkOn, &mut rng) <= FPS + 1);
        for _ in 0..(10 * 60 * FPS) {
            kite.advance(night, &mut rng);
        }
        assert_eq!(kite.stage, Stage::Fly);
        kite.advance(
            KiteNow {
                clear: false,
                ..night
            },
            &mut rng,
        );
        assert_eq!(kite.stage, Stage::Waiting);
        assert!(run_until(&mut kite, night, Stage::WalkOn, &mut rng) <= FPS + 1);
    }

    #[test]
    fn a_resize_that_leaves_no_room_ends_the_spell() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut kite = windy(20.0, false);
        run_until(&mut kite, now(110), Stage::Fly, &mut rng);
        let tiny = KiteNow {
            width: 20,
            horizon_y: 0,
            band: SkyBand::below_hud(0),
            ..now(110)
        };
        kite.advance(tiny, &mut rng);
        assert_eq!(kite.stage, Stage::Waiting);
        assert!(kite.cells(tiny).is_empty());
    }
}
