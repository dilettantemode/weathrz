use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, TerminalSize};
use crate::render::TerminalRenderer;
use crate::weather::types::FogIntensity;
use crossterm::style::Color;

use rand::{Rng, RngExt};
use std::collections::VecDeque;
use std::io;

struct FogWisp {
    x: f32,
    y: f32,
    speed_x: f32,
    character: char,
    tier: usize,
    lifetime: u32,
    max_lifetime: u32,
}

impl FogWisp {
    fn new(terminal_width: u16, terminal_height: u16, rng: &mut (impl Rng + ?Sized)) -> Self {
        let ground_level = crate::scene::world::WorldScene::ground_y(terminal_height);
        let fog_zone_top = ground_level.saturating_sub(15);

        let x = rng.random::<f32>() * terminal_width as f32;
        let y = fog_zone_top as f32 + (rng.random::<f32>() * 15.0);

        let chars = ['.', ',', '-', '~'];
        let char_idx = (rng.random::<u32>() as usize) % chars.len();

        let tier = (rng.random::<u32>() as usize) % 3;

        Self {
            x,
            y,
            speed_x: (rng.random::<f32>() - 0.5) * 0.15,
            character: chars[char_idx],
            tier,
            lifetime: 0,
            max_lifetime: 100 + (rng.random::<u32>() % 200),
        }
    }

    fn update(&mut self) {
        self.x += self.speed_x;
        self.lifetime += 1;
    }

    fn is_alive(&self, terminal_width: u16) -> bool {
        self.lifetime < self.max_lifetime
            && self.x >= -5.0
            && self.x < (terminal_width as f32 + 5.0)
    }
}

pub struct FogSystem {
    glyphs: crate::glyphs::Glyphs,
    wisps: VecDeque<FogWisp>,
    terminal_width: u16,
    terminal_height: u16,
    intensity: FogIntensity,
    spawn_timer: u32,
}

impl AnimationSystem for FogSystem {
    fn id(&self) -> &'static str {
        "fog"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Foreground
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.conditions.is_foggy
    }

    fn on_resize(&mut self, size: TerminalSize) {
        self.terminal_width = size.width;
        self.terminal_height = size.height;
        self.wisps
            .retain(|w| w.is_alive(size.width) && w.y >= 0.0 && w.y < size.height as f32);
    }

    fn on_fog_intensity(&mut self, intensity: FogIntensity) {
        self.set_intensity(intensity);
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.update(ctx.size.width, ctx.size.height, rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        self.glyphs = ctx.state.glyphs;
        FogSystem::render(self, renderer, &ctx.colors.fog)
    }
}

impl FogSystem {
    pub fn new(terminal_width: u16, terminal_height: u16, intensity: FogIntensity) -> Self {
        let wisps_capacity = match intensity {
            FogIntensity::Light => (terminal_width as f32 * 0.3) as usize,
            FogIntensity::Medium => (terminal_width as f32 * 0.6) as usize,
            FogIntensity::Heavy => terminal_width as usize,
        };

        Self {
            glyphs: crate::glyphs::Glyphs::Ascii,
            wisps: VecDeque::with_capacity(wisps_capacity),
            terminal_width,
            terminal_height,
            intensity,
            spawn_timer: 0,
        }
    }

    pub fn set_intensity(&mut self, intensity: FogIntensity) {
        self.intensity = intensity;
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        terminal_height: u16,
        rng: &mut (impl Rng + ?Sized),
    ) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;

        for wisp in &mut self.wisps {
            wisp.update();
        }

        self.wisps.retain(|w| w.is_alive(terminal_width));

        let (target_multiplier, spawn_delay) = match self.intensity {
            FogIntensity::Light => (0.3, 4),
            FogIntensity::Medium => (0.6, 2),
            FogIntensity::Heavy => (1.0, 1),
        };
        let target_count = (terminal_width as f32 * target_multiplier) as usize;

        self.spawn_timer += 1;
        if self.spawn_timer >= spawn_delay && self.wisps.len() < target_count {
            self.spawn_timer = 0;
            for _ in 0..2 {
                if self.wisps.len() < target_count {
                    self.wisps
                        .push_back(FogWisp::new(terminal_width, terminal_height, rng));
                }
            }
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, colors: &[Color; 3]) -> io::Result<()> {
        for wisp in &self.wisps {
            let x = wisp.x as i16;
            let y = wisp.y as i16;

            if x >= 0 && x < self.terminal_width as i16 && y >= 0 && y < self.terminal_height as i16
            {
                let ch =
                    crate::glyphs::swap(self.glyphs, crate::glyphs::Element::Fog, wisp.character);
                renderer.render_char(x as u16, y as u16, ch, colors[wisp.tier])?;
            }
        }
        Ok(())
    }
}
