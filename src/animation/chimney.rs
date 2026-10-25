use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, TerminalSize};
use crate::render::TerminalRenderer;
use crossterm::style::Color;

use rand::{Rng, RngExt};
use std::io;

const MAX_PARTICLES: usize = 200;
const MIN_PARTICLE_MAX_AGE: u32 = 45;
const PARTICLE_MAX_AGE_VARIANCE: u32 = 16;
const PARTICLE_VERTICAL_SPEED: f32 = 0.1;
const PARTICLE_DRIFT_SCALE: f32 = 0.08;
const PARTICLE_SPAWN_JITTER_X: f32 = 1.6;
const DEFAULT_SPAWN_RATE: u32 = 12;

struct SmokeParticle {
    x: f32,
    y: f32,
    age: u32,
    max_age: u32,
    drift: f32,
}

impl SmokeParticle {
    fn new(chimney_x: u16, chimney_y: u16, rng: &mut (impl Rng + ?Sized)) -> Self {
        let drift = (rng.random::<f32>() - 0.5) * PARTICLE_DRIFT_SCALE;
        let max_age = MIN_PARTICLE_MAX_AGE + (rng.random::<u32>() % PARTICLE_MAX_AGE_VARIANCE);

        Self {
            x: chimney_x as f32 + (rng.random::<f32>() - 0.5) * PARTICLE_SPAWN_JITTER_X,
            y: chimney_y as f32,
            age: 0,
            max_age,
            drift,
        }
    }

    fn update(&mut self) {
        self.age += 1;
        self.y -= PARTICLE_VERTICAL_SPEED;
        self.x += self.drift;
    }

    fn is_alive(&self) -> bool {
        self.age < self.max_age
    }

    fn color(&self, colors: &[Color; 3]) -> Color {
        colors[smoke_tier(self.age as f32 / self.max_age as f32)]
    }
}

pub struct ChimneySmoke {
    particles: Vec<SmokeParticle>,
    spawn_counter: u32,
    spawn_rate: u32,
}

impl ChimneySmoke {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(MAX_PARTICLES),
            spawn_counter: 0,
            spawn_rate: DEFAULT_SPAWN_RATE,
        }
    }

    pub fn update(&mut self, chimney_x: u16, chimney_y: u16, rng: &mut (impl Rng + ?Sized)) {
        for particle in &mut self.particles {
            particle.update();
        }

        self.particles.retain(|p| p.is_alive() && p.y >= 0.0);

        self.spawn_counter += 1;
        if self.spawn_counter >= self.spawn_rate && self.particles.len() < MAX_PARTICLES {
            self.spawn_counter = 0;
            self.particles
                .push(SmokeParticle::new(chimney_x, chimney_y, rng));
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, colors: &[Color; 3]) -> io::Result<()> {
        for particle in &self.particles {
            let x = particle.x as i16;
            let y = particle.y as i16;

            if x >= 0 && y >= 0 {
                let display_char = match particle.age {
                    0..=6 => 'o',
                    7..=14 => '.',
                    15..=25 => '~',
                    _ => '·',
                };

                renderer.render_char_on_open_sky(
                    x as u16,
                    y as u16,
                    display_char,
                    particle.color(colors),
                )?;
            }
        }
        Ok(())
    }
}

impl Default for ChimneySmoke {
    fn default() -> Self {
        Self::new()
    }
}

fn smoke_tier(life_ratio: f32) -> usize {
    if life_ratio < 0.3 {
        0
    } else if life_ratio < 0.6 {
        1
    } else {
        2
    }
}

impl AnimationSystem for ChimneySmoke {
    fn id(&self) -> &'static str {
        "chimney_smoke"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        !ctx.conditions.is_raining && !ctx.conditions.is_thunderstorm && ctx.chimney.is_some()
    }

    fn on_resize(&mut self, _size: TerminalSize) {}

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        let Some(chimney) = ctx.chimney else {
            return;
        };

        self.update(chimney.x, chimney.y, rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        if ctx.chimney.is_none() {
            return Ok(());
        }

        ChimneySmoke::render(self, renderer, &ctx.colors.smoke)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_tiers_follow_life_ratio() {
        assert_eq!(smoke_tier(0.0), 0);
        assert_eq!(smoke_tier(0.29), 0);
        assert_eq!(smoke_tier(0.3), 1);
        assert_eq!(smoke_tier(0.59), 1);
        assert_eq!(smoke_tier(0.6), 2);
        assert_eq!(smoke_tier(1.0), 2);
    }
}
