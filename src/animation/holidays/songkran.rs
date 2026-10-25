use super::{Placed, place};
use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::Look;
use crate::render::TerminalRenderer;
use chrono::NaiveDate;
use rand::{Rng, RngExt};
use std::io;

pub const SONGKRAN: &str = "songkran_water";

const WATER: [char; 2] = ['c', 'b'];
const MAX_THROWS: usize = 10;
const GAP_FRAMES: std::ops::RangeInclusive<u32> = 3..=10;
const LIFT: std::ops::RangeInclusive<f32> = 0.45..=0.65;
const REACH: std::ops::RangeInclusive<f32> = 0.6..=1.0;
const GRAVITY: f32 = 0.02;
const TRAIL: u32 = 6;
const TRAIL_STEP: u32 = 2;
const WET_FRAMES: u32 = 90;
const MAX_WET: usize = 60;
const SPLASH: i32 = 2;
const BELOW_GROUND: i32 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Throw {
    x: f32,
    y: f32,
    toward: f32,
    reach: f32,
    lift: f32,
    age: u32,
    letter: char,
}

impl Throw {
    fn at(self, frame: u32) -> (i32, i32) {
        let t = frame as f32;
        (
            (self.x + self.toward * self.reach * t).round() as i32,
            (self.y - self.lift * t + GRAVITY * t * t).round() as i32,
        )
    }

    fn landed(self) -> bool {
        self.age > 0 && self.at(self.age).1 > self.y as i32
    }

    fn cells(self, width: u16, height: u16) -> Vec<Placed> {
        (0..TRAIL)
            .filter_map(|step| {
                let frame = self.age.checked_sub(step * TRAIL_STEP)?;
                let (x, y) = self.at(frame);
                let ch = if step == 0 { 'o' } else { '.' };
                place(x, y, ch, self.letter, width, height)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Wet {
    spot: Placed,
    age: u32,
}

pub struct SongkranWater {
    throws: Vec<Throw>,
    wet: Vec<Wet>,
    wait: u32,
    day: Option<NaiveDate>,
}

impl SongkranWater {
    pub fn new() -> Self {
        Self {
            throws: Vec::new(),
            wet: Vec::new(),
            wait: 0,
            day: None,
        }
    }

    pub(crate) fn step(
        &mut self,
        day: NaiveDate,
        width: u16,
        height: u16,
        horizon_y: u16,
        rng: &mut (impl Rng + ?Sized),
    ) {
        if self.day != Some(day) {
            self.day = Some(day);
            self.throws.clear();
            self.wet.clear();
        }
        for throw in &mut self.throws {
            throw.age += 1;
        }
        for wet in &mut self.wet {
            wet.age += 1;
        }
        self.wet.retain(|wet| wet.age < WET_FRAMES);
        let (landed, flying): (Vec<Throw>, Vec<Throw>) =
            self.throws.drain(..).partition(|throw| throw.landed());
        self.throws = flying;
        let ground = i32::from(horizon_y);
        for throw in landed {
            let (x, _) = throw.at(throw.age);
            for dx in -SPLASH..=SPLASH {
                let y = ground + rng.random_range(0..=BELOW_GROUND);
                let ch = if dx == 0 { '~' } else { ',' };
                if let Some(spot) = place(x + dx, y, ch, throw.letter, width, height) {
                    if self.wet.len() >= MAX_WET {
                        self.wet.remove(0);
                    }
                    self.wet.push(Wet { spot, age: 0 });
                }
            }
        }
        if self.wait > 0 {
            self.wait -= 1;
            return;
        }
        if self.throws.len() < MAX_THROWS && width > 8 {
            self.throws.push(Throw {
                x: rng.random_range(4..i32::from(width) - 4) as f32,
                y: (ground - 1) as f32,
                toward: if rng.random_bool(0.5) { 1.0 } else { -1.0 },
                reach: rng.random_range(REACH),
                lift: rng.random_range(LIFT),
                age: 0,
                letter: WATER[rng.random_range(0..WATER.len())],
            });
        }
        self.wait = rng.random_range(GAP_FRAMES);
    }

    pub(crate) fn cells(&self, width: u16, height: u16) -> Vec<Placed> {
        let mut out: Vec<Placed> = self
            .wet
            .iter()
            .map(|wet| wet.spot)
            .filter(|spot| spot.x < width && spot.y < height)
            .collect();
        for throw in &self.throws {
            out.extend(throw.cells(width, height));
        }
        out
    }
}

impl Default for SongkranWater {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for SongkranWater {
    fn id(&self) -> &'static str {
        SONGKRAN
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.events.has(Look::Songkran)
            && ctx.conditions.sun.is_day
            && !ctx.conditions.veils_the_sky()
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.step(
            ctx.events.now().date(),
            ctx.size.width,
            ctx.size.height,
            ctx.horizon_y,
            rng,
        );
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        for cell in self.cells(ctx.size.width, ctx.size.height) {
            let color =
                mask::colour(cell.letter, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            renderer.render_char_in_front(cell.x, cell.y, cell.ch, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn day(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn water_arcs_up_and_lands_in_the_yard() {
        let throw = Throw {
            x: 50.0,
            y: 24.0,
            toward: 1.0,
            reach: 0.6,
            lift: 0.5,
            age: 0,
            letter: 'c',
        };
        let peak = (0..40).map(|frame| throw.at(frame).1).min().unwrap();
        assert!(peak <= 21, "it rises a few rows: {peak}");
        let landing = (1..80)
            .find(|frame| {
                Throw {
                    age: *frame,
                    ..throw
                }
                .landed()
            })
            .unwrap();
        assert!(
            throw.at(landing).0 > 55,
            "it travels downwind of the thrower"
        );
    }

    #[test]
    fn splashes_dry_and_a_new_day_starts_dry() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut water = SongkranWater::new();
        for _ in 0..600 {
            water.step(day("2027-04-13"), 120, 32, 25, &mut rng);
            assert!(water.throws.len() <= MAX_THROWS);
            assert!(water.wet.iter().all(|wet| wet.age < WET_FRAMES));
        }
        assert!(!water.wet.is_empty());
        assert!(water.wet.iter().all(|wet| wet.spot.y >= 25));
        assert!(water.cells(120, 32).iter().all(|cell| cell.y >= 3));
        water.step(day("2027-04-14"), 120, 32, 25, &mut rng);
        assert!(water.wet.is_empty() && water.throws.len() <= 1);
    }
}
