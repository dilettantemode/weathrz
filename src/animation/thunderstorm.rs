use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, TerminalSize};
use crate::render::TerminalRenderer;
use crossterm::style::Color;

use rand::{Rng, RngExt};
use std::collections::VecDeque;
use std::io;

const MAX_BOLTS: usize = 10;
const FORM_FRAMES: usize = 3;
const RESTRIKE_CHANCE: f32 = 0.6;
const FORK_CHANCE: f32 = 0.3;
const FORK_GAP: i16 = 3;
const MAX_BRANCHES: usize = 2;
const BRANCH_CLEAR: i16 = 3;
const BRANCH_FADE: f32 = 0.45;
const GLOW: f32 = 0.18;
const INSIDE_THE_DECK: u16 = 2;
const DISC_GAP: i32 = 4;
const START_TRIES: usize = 20;
const BOLT_WHITE: (u8, u8, u8) = (250, 235, 170);
const BOLT_LIFT: f32 = 0.6;

pub(crate) fn bolt_colour(theme: Color) -> Color {
    crate::render::rgb(theme).map_or(theme, |bolt| {
        crate::render::mix(bolt, BOLT_WHITE, BOLT_LIFT)
    })
}

fn slant(corner: &mut i16, y: i16, direction: i16) -> (u16, u16, char) {
    if direction > 0 {
        let cell = (*corner as u16, y as u16, '\\');
        *corner += 1;
        cell
    } else {
        *corner -= 1;
        (*corner as u16, y as u16, '/')
    }
}

#[derive(Clone, Copy, PartialEq)]
enum LightningState {
    Forming,
    Strike,
    Flash,
    Restrike,
    Fading,
    Idle,
}

#[derive(Clone, Copy)]
struct Segment {
    x: u16,
    y: u16,
    ch: char,
    branch: bool,
}

impl Segment {
    fn of((x, y, ch): (u16, u16, char), branch: bool) -> Self {
        Self { x, y, ch, branch }
    }
}

struct LightningBolt {
    segments: Vec<Segment>,
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
    moon_phase: f64,
    disc: Option<(u16, u16)>,
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
            moon_phase: 0.5,
            disc: None,
        }
    }

    fn clear_of_the_disc(&self, x: u16) -> bool {
        self.disc.is_none_or(|(left, right)| {
            i32::from(x) + DISC_GAP < i32::from(left) || i32::from(x) >= i32::from(right) + DISC_GAP
        })
    }

    fn generate_bolt(&mut self, rng: &mut (impl Rng + ?Sized)) -> bool {
        if self.terminal_width < 12 || self.terminal_height < 8 {
            return false;
        }
        let top = crate::animation::clouds::deck_floor(self.terminal_height)
            .saturating_sub(INSIDE_THE_DECK)
            .max(crate::animation::sky::SKY_TOP) as i16;
        let y_end = self.bottom.min(self.terminal_height.saturating_sub(5)) as i16;
        if y_end <= top + 2 {
            return false;
        }
        let (low, high) = (
            (self.terminal_width * 15 / 100).max(5),
            self.terminal_width * 85 / 100,
        );
        let Some(start_x) = (0..START_TRIES)
            .map(|_| low + rng.random::<u16>() % high.saturating_sub(low).max(1))
            .find(|x| self.clear_of_the_disc(*x))
        else {
            return false;
        };
        let mut lean: i16 = match self.disc {
            Some((left, _)) if start_x < left => -1,
            Some(_) => 1,
            None if rng.random_bool(0.5) => 1,
            None => -1,
        };

        let max_x = self.terminal_width.saturating_sub(3) as i16;
        let mut segments = Vec::new();
        let mut corner = start_x as i16;
        let mut run_left = rng.random_range(2..=3);
        let (mut branches, mut since_fork) = (0, FORK_GAP);
        for y in top..y_end {
            let kick = run_left == 0;
            let mut dx = if kick { -lean } else { lean };
            if !(2..=max_x).contains(&(corner + dx)) {
                lean = -lean;
                dx = -dx;
            }
            segments.push(Segment::of(slant(&mut corner, y, dx), false));
            if kick {
                run_left = rng.random_range(2..=3);
                continue;
            }
            run_left -= 1;
            since_fork += 1;
            let length = rng.random_range(2..=3);
            if branches < MAX_BRANCHES
                && since_fork >= FORK_GAP
                && run_left + 1 >= length
                && y + length + BRANCH_CLEAR <= y_end
                && rng.random::<f32>() < FORK_CHANCE
            {
                let mut fork = corner;
                for step in 1..=length {
                    if (2..=max_x).contains(&(fork - lean)) {
                        segments.push(Segment::of(slant(&mut fork, y + step, -lean), true));
                    }
                }
                branches += 1;
                since_fork = 0;
                run_left = run_left.max(length);
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

                    self.state = if self.generate_bolt(rng) {
                        LightningState::Forming
                    } else {
                        LightningState::Strike
                    };
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

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        colors: &[Color; 2],
        glyphs: crate::glyphs::Glyphs,
    ) -> io::Result<()> {
        let color = bolt_colour(colors[1]);
        for bolt in &self.bolts {
            for segment in &bolt.segments[..bolt.shown] {
                let ch = crate::glyphs::swap(glyphs, crate::glyphs::Element::Rain, segment.ch);
                let sky = renderer.sky_row(segment.y).and_then(crate::render::rgb);
                match (segment.branch, crate::render::rgb(color), sky) {
                    (true, Some(bolt), Some(sky)) => renderer.render_char(
                        segment.x,
                        segment.y,
                        ch,
                        crate::render::mix(bolt, sky, BRANCH_FADE),
                    )?,
                    (false, Some(bolt), Some(sky)) => renderer.render_char_on(
                        segment.x,
                        segment.y,
                        ch,
                        color,
                        crate::render::mix(sky, bolt, GLOW),
                    )?,
                    _ => renderer.render_char(segment.x, segment.y, ch, color)?,
                }
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
        RenderLayer::Background
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

    fn drawn_before(&self) -> Option<&'static str> {
        Some("clouds")
    }

    fn on_moon_phase(&mut self, phase: f64) {
        self.moon_phase = phase;
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, commands: &mut FrameCommands) {
        self.disc = (!ctx.conditions.sun.is_day)
            .then(|| {
                crate::animation::moon::moon_spot(
                    ctx.state.moon_sky,
                    self.moon_phase,
                    ctx.state.location.latitude < 0.0,
                    (ctx.size.width, ctx.size.height, ctx.horizon_y),
                )
            })
            .flatten()
            .map(|spot| (spot.x, spot.x + crate::animation::moon::art_width()));
        self.update(
            ctx.size.width,
            ctx.size.height,
            ctx.horizon_y.saturating_sub(1),
            rng,
        );
        commands.flash_screen |= self.flash_active;
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        ThunderstormSystem::render(self, renderer, &ctx.colors.lightning, ctx.state.glyphs)
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
                assert!(bolt.segments.iter().all(|segment| segment.y <= 14));
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
    fn a_bolt_zigzags_with_one_lean_and_branches_away() {
        for seed in 0..60 {
            let mut storm = ThunderstormSystem::new(120, 40);
            storm.bottom = 24;
            storm.disc = Some((60, 71));
            let mut rng = StdRng::seed_from_u64(seed);
            if !storm.generate_bolt(&mut rng) {
                continue;
            }
            let bolt = storm.bolts.back().unwrap();
            let trunk: Vec<Segment> = bolt
                .segments
                .iter()
                .copied()
                .filter(|s| !s.branch)
                .collect();
            let dx: Vec<i16> = trunk
                .iter()
                .map(|s| if s.ch == '\\' { 1 } else { -1 })
                .collect();
            let longest = dx
                .chunk_by(|a, b| a == b)
                .map(<[i16]>::len)
                .max()
                .unwrap_or(0);
            assert!(longest <= 4, "{seed}: {dx:?}");
            assert!(storm.clear_of_the_disc(trunk[0].x), "{seed}");
            assert!(trunk.windows(2).all(|pair| pair[1].y == pair[0].y + 1));
            let end = trunk.last().unwrap().y;
            for branch in bolt.segments.iter().filter(|s| s.branch) {
                assert!(branch.y + BRANCH_CLEAR as u16 <= end + 1, "{seed}");
                let beside = trunk.iter().find(|s| s.y == branch.y).unwrap();
                assert!(branch.ch != beside.ch, "{seed}: the branch runs parallel");
            }
        }
    }

    #[test]
    fn with_no_room_above_the_roof_the_sky_still_flashes() {
        let mut storm = ThunderstormSystem::new(80, 20);
        storm.next_strike_in = 0;
        let mut rng = StdRng::seed_from_u64(4);
        let mut flashes = 0;
        for _ in 0..10 {
            storm.update(80, 20, 3, &mut rng);
            assert!(storm.bolts.is_empty());
            flashes += usize::from(storm.flash_active);
        }
        assert!(flashes >= 1, "sheet lightning");
    }
}
