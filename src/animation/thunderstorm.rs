use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, TerminalSize};
use crate::render::TerminalRenderer;
use crossterm::style::Color;

use rand::{Rng, RngExt};
use std::collections::VecDeque;
use std::io;

const MAX_BOLTS: usize = 10;
const FORM_FRAMES: usize = 3;
const RESTRIKE_CHANCE: f32 = 0.6;

#[derive(Clone, Copy, PartialEq)]
enum LightningState {
    Forming,
    Strike,
    Flash,
    Restrike,
    Fading,
    Idle,
}

struct LightningBolt {
    segments: Vec<(u16, u16, char)>,
    shown: usize,
    age: u8,
    max_age: u8,
}

pub struct ThunderstormSystem {
    bolts: VecDeque<LightningBolt>,
    state: LightningState,
    timer: u16,
    terminal_width: u16,
    terminal_height: u16,
    bottom: u16,
    flash_active: bool,
    next_strike_in: u16,
}

impl ThunderstormSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            bolts: VecDeque::with_capacity(MAX_BOLTS),
            state: LightningState::Idle,
            timer: 0,
            terminal_width,
            terminal_height,
            bottom: terminal_height.saturating_sub(5),
            flash_active: false,
            next_strike_in: 60 + (rand::random::<u16>() % 120), // Random start delay
        }
    }

    fn generate_bolt(&mut self, rng: &mut (impl Rng + ?Sized)) -> bool {
        if self.terminal_width < 12 || self.terminal_height < 8 {
            return false;
        }

        let usable_width = self.terminal_width.saturating_sub(10);
        if usable_width == 0 {
            return false;
        }

        let start_x = (rng.random::<u16>() % usable_width) + 5;
        let mut segments = Vec::new();
        let mut x = start_x as i16;
        let mut y = 2; // Start below top bar

        segments.push((x as u16, y as u16, '+')); // Start point

        let y_end = self.bottom.min(self.terminal_height.saturating_sub(5)) as i16;
        if y_end <= y + 2 {
            return false;
        }
        let max_x = self.terminal_width.saturating_sub(3) as i16;

        while y < y_end {
            let direction = (rng.random::<i8>() % 3) - 1; // -1, 0, 1
            x += direction as i16;
            y += 1;

            // Constrain x
            if x < 2 {
                x = 2;
            }
            if x > max_x {
                x = max_x;
            }

            let char = match direction {
                -1 => '/',
                1 => '\\',
                _ => '|',
            };

            segments.push((x as u16, y as u16, char));

            // Occasionally branch
            if rng.random::<f32>() < 0.2 {
                let branch_dir = -direction;
                let mut bx = x + branch_dir as i16;
                let mut by = y + 1;
                for _ in 0..3 {
                    if by < y_end {
                        segments.push((
                            bx as u16,
                            by as u16,
                            if branch_dir < 0 { '/' } else { '\\' },
                        ));
                        bx += branch_dir as i16;
                        by += 1;
                    }
                }
            }
        }

        self.bolts.push_back(LightningBolt {
            segments,
            shown: 0,
            age: 0,
            max_age: 10,
        });

        while self.bolts.len() > MAX_BOLTS {
            self.bolts.pop_front();
        }

        true
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        terminal_height: u16,
        bottom: u16,
        rng: &mut (impl Rng + ?Sized),
    ) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;
        self.bottom = bottom;

        if self.terminal_width < 12 || self.terminal_height < 8 {
            self.bolts.clear();
            self.flash_active = false;
            self.state = LightningState::Idle;
            self.timer = 0;
            self.next_strike_in = 60 + (rng.random::<u16>() % 120);
            return;
        }

        match self.state {
            LightningState::Idle => {
                self.flash_active = false;
                if self.timer >= self.next_strike_in {
                    self.timer = 0;

                    if self.generate_bolt(rng) {
                        self.state = LightningState::Forming;
                    } else {
                        self.next_strike_in = 30 + (rng.random::<u16>() % 200);
                    }
                } else {
                    self.timer += 1;
                }
            }
            LightningState::Forming => {
                self.timer += 1;
                let reach = usize::from(self.timer).min(FORM_FRAMES);
                for bolt in &mut self.bolts {
                    bolt.shown = bolt.segments.len() * reach / FORM_FRAMES;
                }
                if reach == FORM_FRAMES {
                    self.state = LightningState::Strike;
                    self.timer = 0;
                }
            }
            LightningState::Strike => {
                self.flash_active = true;
                self.state = LightningState::Flash;
                self.timer = 0;
            }
            LightningState::Flash => {
                self.flash_active = false;
                if self.timer > 2 {
                    self.state = if rng.random::<f32>() < RESTRIKE_CHANCE {
                        LightningState::Restrike
                    } else {
                        LightningState::Fading
                    };
                    self.timer = 0;
                } else {
                    self.timer += 1;
                }
            }
            LightningState::Restrike => {
                self.flash_active = true;
                self.state = LightningState::Fading;
                self.timer = 0;
            }
            LightningState::Fading => {
                self.flash_active = false;
                self.bolts.retain_mut(|bolt| {
                    bolt.age += 1;
                    bolt.age < bolt.max_age
                });

                if self.bolts.is_empty() {
                    self.state = LightningState::Idle;
                    self.timer = 0;
                    self.next_strike_in = 30 + (rng.random::<u16>() % 200);
                }
            }
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, colors: &[Color; 2]) -> io::Result<()> {
        let color = if self.flash_active {
            colors[0]
        } else {
            colors[1]
        };

        for bolt in &self.bolts {
            for segment in &bolt.segments[..bolt.shown] {
                renderer.render_char(segment.0, segment.1, segment.2, color)?;
            }
        }
        Ok(())
    }
}

impl AnimationSystem for ThunderstormSystem {
    fn id(&self) -> &'static str {
        "thunderstorm"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Foreground
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.conditions.is_thunderstorm
    }

    fn on_resize(&mut self, size: TerminalSize) {
        self.terminal_width = size.width;
        self.terminal_height = size.height;

        if self.terminal_width < 12 || self.terminal_height < 8 {
            self.bolts.clear();
            self.flash_active = false;
            self.state = LightningState::Idle;
            self.timer = 0;
        }
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, commands: &mut FrameCommands) {
        self.update(
            ctx.size.width,
            ctx.size.height,
            ctx.skyline_y.saturating_sub(1),
            rng,
        );
        commands.flash_screen |= self.flash_active;
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        ThunderstormSystem::render(self, renderer, &ctx.colors.lightning)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn a_bolt_grows_down_strikes_and_stays_above_the_roofline() {
        let mut storm = ThunderstormSystem::new(110, 32);
        storm.next_strike_in = 0;
        let mut rng = StdRng::seed_from_u64(4);
        let mut reveals = Vec::new();
        let mut flashes = 0;
        for _ in 0..40 {
            storm.update(110, 32, 14, &mut rng);
            if let Some(bolt) = storm.bolts.front() {
                reveals.push((bolt.shown, bolt.segments.len()));
                assert!(bolt.segments.iter().all(|(_, y, _)| *y <= 14));
            }
            flashes += usize::from(storm.flash_active);
        }
        let (first_shown, length) = reveals
            .iter()
            .copied()
            .find(|(shown, _)| *shown > 0)
            .expect("the bolt shows");
        assert!(first_shown < length, "{reveals:?}");
        assert!(reveals.iter().any(|(shown, length)| shown == length));
        assert!(
            reveals.windows(2).all(|pair| pair[0].0 <= pair[1].0),
            "the bolt only grows"
        );
        assert!(flashes >= 1);
    }

    #[test]
    fn no_room_under_the_hud_means_no_bolt() {
        let mut storm = ThunderstormSystem::new(110, 32);
        storm.next_strike_in = 0;
        let mut rng = StdRng::seed_from_u64(4);
        for _ in 0..10 {
            storm.update(110, 32, 3, &mut rng);
        }
        assert!(storm.bolts.is_empty());
    }
}
