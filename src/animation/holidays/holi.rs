use super::{Placed, place};
use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::Look;
use crate::render::TerminalRenderer;
use chrono::{Duration, NaiveDate, Timelike};
use rand::{Rng, RngExt};
use std::io;

pub const HOLI: &str = "holi_colours";

const COLOURS: [char; 7] = ['m', 'y', 'g', 'c', 'o', 'r', 'b'];
const PUFF_FRAMES: u32 = 45;
const MAX_PUFFS: usize = 8;
const GAP_FRAMES: std::ops::RangeInclusive<u32> = 8..=30;
const MAX_STAINS: usize = 160;
const STAINS_PER_PUFF: usize = 3;
const ABOVE_GROUND: i32 = 4;
const BELOW_GROUND: i32 = 5;
const EVE_HOUR: u32 = 18;
const EVE_HOURS: i64 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Puff {
    pub x: i32,
    pub y: i32,
    pub age: u32,
    pub letter: char,
}

impl Puff {
    fn radius(self) -> i32 {
        (self.age * 3 / PUFF_FRAMES) as i32
    }

    pub(crate) fn cells(self, width: u16, height: u16) -> Vec<Placed> {
        let radius = self.radius();
        let mut out = Vec::new();
        for dy in -radius..=radius {
            for dx in -2 * radius..=2 * radius {
                let reach = dx.abs() + 2 * dy.abs();
                if reach > 2 * radius {
                    continue;
                }
                let ch = match radius {
                    0 => '*',
                    1 if reach == 0 => '@',
                    1 => '*',
                    _ if (dx + dy + self.age as i32).rem_euclid(3) != 0 => continue,
                    _ => '.',
                };
                out.extend(place(
                    self.x + dx,
                    self.y + dy,
                    ch,
                    self.letter,
                    width,
                    height,
                ));
            }
        }
        out
    }
}

pub struct HoliColours {
    puffs: Vec<Puff>,
    stains: Vec<Placed>,
    wait: u32,
    day: Option<NaiveDate>,
}

impl HoliColours {
    pub fn new() -> Self {
        Self {
            puffs: Vec::new(),
            stains: Vec::new(),
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
            self.puffs.clear();
            self.stains.clear();
        }
        for puff in &mut self.puffs {
            puff.age += 1;
        }
        let ground = i32::from(horizon_y);
        let (done, live): (Vec<Puff>, Vec<Puff>) = self
            .puffs
            .drain(..)
            .partition(|puff| puff.age >= PUFF_FRAMES);
        self.puffs = live;
        for puff in done {
            for _ in 0..STAINS_PER_PUFF {
                let x = puff.x + rng.random_range(-4..=4);
                let y = ground + rng.random_range(1..=BELOW_GROUND);
                let ch = if rng.random_bool(0.5) { ',' } else { '.' };
                if let Some(stain) = place(x, y, ch, puff.letter, width, height) {
                    if self.stains.len() >= MAX_STAINS {
                        self.stains.remove(0);
                    }
                    self.stains.push(stain);
                }
            }
        }
        if self.wait > 0 {
            self.wait -= 1;
            return;
        }
        if self.puffs.len() < MAX_PUFFS && width > 4 {
            self.puffs.push(Puff {
                x: rng.random_range(2..i32::from(width) - 2),
                y: ground + rng.random_range(-ABOVE_GROUND..=BELOW_GROUND - 2),
                age: 0,
                letter: COLOURS[rng.random_range(0..COLOURS.len())],
            });
        }
        self.wait = rng.random_range(GAP_FRAMES);
    }

    pub(crate) fn cells(&self, width: u16, height: u16) -> Vec<Placed> {
        let mut out: Vec<Placed> = self
            .stains
            .iter()
            .copied()
            .filter(|stain| stain.x < width && stain.y < height)
            .collect();
        for puff in &self.puffs {
            out.extend(puff.cells(width, height));
        }
        out
    }
}

impl Default for HoliColours {
    fn default() -> Self {
        Self::new()
    }
}

fn on_the_day(ctx: &FrameContext<'_>) -> bool {
    ctx.events.since_start_of(Look::Holi).is_some_and(|since| {
        let start = ctx.events.now() - since;
        start.hour() != EVE_HOUR || since >= Duration::hours(EVE_HOURS)
    })
}

fn wet(ctx: &FrameContext<'_>) -> bool {
    ctx.conditions.is_raining || ctx.conditions.is_thunderstorm || ctx.conditions.is_snowing
}

impl AnimationSystem for HoliColours {
    fn id(&self) -> &'static str {
        HOLI
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.events.has(Look::Holi) && ctx.conditions.sun.is_day && !wet(ctx) && on_the_day(ctx)
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
            renderer.render_char(cell.x, cell.y, cell.ch, color)?;
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
    fn puffs_grow_then_leave_stains_in_the_yard() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut holi = HoliColours::new();
        for _ in 0..(30 * 30) {
            holi.step(day("2027-03-23"), 120, 32, 25, &mut rng);
            assert!(holi.puffs.len() <= MAX_PUFFS);
        }
        assert!(!holi.stains.is_empty());
        assert!(holi.stains.iter().all(|stain| stain.y > 25 && stain.y < 32));
        let letters: std::collections::HashSet<char> =
            holi.cells(120, 32).iter().map(|cell| cell.letter).collect();
        assert!(letters.len() >= 3, "{letters:?}");
    }

    #[test]
    fn stains_wash_away_the_next_day() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut holi = HoliColours::new();
        for _ in 0..(30 * 30) {
            holi.step(day("2027-03-23"), 120, 32, 25, &mut rng);
        }
        holi.step(day("2028-03-11"), 120, 32, 25, &mut rng);
        assert!(holi.stains.is_empty());
    }

    #[test]
    fn a_puff_stays_inside_the_pane() {
        let puff = Puff {
            x: 1,
            y: 30,
            age: PUFF_FRAMES - 1,
            letter: 'm',
        };
        assert!(
            puff.cells(10, 32)
                .iter()
                .all(|cell| cell.x < 10 && cell.y < 32)
        );
        assert_eq!(
            Puff { age: 0, ..puff }.cells(10, 32).len(),
            1,
            "a new puff is one grain"
        );
    }
}
