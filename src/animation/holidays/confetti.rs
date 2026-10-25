use crate::animation::flyers::mask;
use crate::animation::sky::SKY_TOP;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, Wind};
use crate::events::Look;
use crate::render::TerminalRenderer;
use rand::{Rng, RngExt};
use std::io;

pub const CONFETTI: &str = "carnival_confetti";

const COLOURS: [char; 7] = ['m', 'y', 'g', 'c', 'o', 'r', 'b'];
const BITS: [char; 6] = [',', '\'', '`', '.', '*', '\''];
const COLUMNS_PER_BIT: u16 = 3;
const FALL: std::ops::RangeInclusive<f32> = 0.08..=0.2;
const FLUTTER: f32 = 0.35;
const FLUTTER_SPEED: f32 = 0.15;
const DRIFT_PER_KMH: f32 = 0.004;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Bit {
    x: f32,
    y: f32,
    fall: f32,
    phase: f32,
    ch: char,
    letter: char,
}

pub struct CarnivalConfetti {
    bits: Vec<Bit>,
    drift: f32,
}

impl CarnivalConfetti {
    pub fn new() -> Self {
        Self {
            bits: Vec::new(),
            drift: 0.0,
        }
    }

    fn spawn(width: u16, top: f32, rng: &mut (impl Rng + ?Sized)) -> Bit {
        Bit {
            x: rng.random_range(0.0..f32::from(width.max(1))),
            y: top,
            fall: rng.random_range(FALL),
            phase: rng.random_range(0.0..std::f32::consts::TAU),
            ch: BITS[rng.random_range(0..BITS.len())],
            letter: COLOURS[rng.random_range(0..COLOURS.len())],
        }
    }

    pub(crate) fn step(&mut self, width: u16, ground_y: u16, rng: &mut (impl Rng + ?Sized)) {
        let wanted = usize::from(width / COLUMNS_PER_BIT);
        let top = f32::from(SKY_TOP);
        let floor = f32::from(ground_y);
        while self.bits.len() < wanted {
            let mut bit = Self::spawn(width, top, rng);
            bit.y = rng.random_range(top..floor.max(top + 1.0));
            self.bits.push(bit);
        }
        self.bits.truncate(wanted);
        for bit in &mut self.bits {
            bit.phase += FLUTTER_SPEED;
            bit.y += bit.fall;
            bit.x += self.drift + FLUTTER * bit.phase.sin() * FLUTTER_SPEED;
            if bit.y >= floor || bit.x < -1.0 || bit.x > f32::from(width) + 1.0 {
                *bit = Self::spawn(width, top, rng);
            }
        }
    }
}

impl Default for CarnivalConfetti {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for CarnivalConfetti {
    fn id(&self) -> &'static str {
        CONFETTI
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Foreground
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.events.has(Look::Carnival) && !ctx.conditions.veils_the_sky()
    }

    fn on_wind(&mut self, wind: Wind) {
        self.drift = -wind.direction_deg.to_radians().sin() * wind.speed_kmh * DRIFT_PER_KMH;
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.step(ctx.size.width, ctx.horizon_y, rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        for bit in &self.bits {
            if bit.x < 0.0 || bit.y < 0.0 {
                continue;
            }
            let color =
                mask::colour(bit.letter, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            renderer.render_char_in_front(bit.x as u16, bit.y as u16, bit.ch, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn confetti_falls_between_the_hud_and_the_ground_in_many_colours() {
        let mut rng = StdRng::seed_from_u64(9);
        let mut confetti = CarnivalConfetti::new();
        for _ in 0..400 {
            confetti.step(120, 25, &mut rng);
            assert_eq!(confetti.bits.len(), 40);
            assert!(
                confetti
                    .bits
                    .iter()
                    .all(|bit| bit.y >= f32::from(SKY_TOP) && bit.y < 25.0)
            );
        }
        let colours: std::collections::HashSet<char> =
            confetti.bits.iter().map(|bit| bit.letter).collect();
        assert!(colours.len() >= 5, "{colours:?}");
    }
}
