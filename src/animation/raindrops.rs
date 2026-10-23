use crate::animation::{
    AnimationSystem, FrameCommands, FrameContext, RenderLayer, TerminalSize, Wind,
};
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use crate::scene::world::house::House;
use crate::weather::types::RainIntensity;
use crossterm::style::Color;

use rand::{Rng, RngExt};
use std::collections::VecDeque;
use std::io;

const MAX_SPLASHES: usize = 100;
const SHEET_CHANCE: f32 = 0.004;
const SHEET_DROPS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    Far,
    Near,
}

struct Raindrop {
    x: f32,
    y: f32,
    speed_y: f32,
    speed_x: f32,
    character: char,
    land_y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Surface {
    Roof,
    Grass,
    Puddle,
}

#[derive(Clone, Copy)]
struct Splash {
    x: u16,
    y: u16,
    timer: u8,
    surface: Surface,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Sheet {
    x: f32,
    width: f32,
    life: u16,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Ground {
    horizon: u16,
    roof_left: u16,
    roof: Vec<Option<u16>>,
}

impl Ground {
    fn from(horizon: u16, door: Option<DoorAnchor>) -> Self {
        let Some(door) = door else {
            return Self {
                horizon,
                ..Self::default()
            };
        };
        let house_y = door.ground_y.saturating_sub(House::HEIGHT);
        Self {
            horizon,
            roof_left: door.x.saturating_sub(House::DOOR_X_OFFSET),
            roof: House::roof_rows()
                .into_iter()
                .map(|row| row.map(|row| house_y + row))
                .collect(),
        }
    }

    fn roof_at(&self, x: f32) -> Option<u16> {
        let column = x.floor() as i32 - i32::from(self.roof_left);
        usize::try_from(column)
            .ok()
            .and_then(|column| self.roof.get(column).copied().flatten())
    }
}

pub struct RaindropSystem {
    depth: Depth,
    glyphs: crate::glyphs::Glyphs,
    drops: Vec<Raindrop>,
    splashes: VecDeque<Splash>,
    terminal_width: u16,
    terminal_height: u16,
    intensity: RainIntensity,
    wind_x: f32,
    sheet: Option<Sheet>,
    ground: Ground,
}

impl RaindropSystem {
    pub fn new(
        depth: Depth,
        terminal_width: u16,
        terminal_height: u16,
        intensity: RainIntensity,
    ) -> Self {
        let mut system = Self {
            depth,
            glyphs: crate::glyphs::Glyphs::Ascii,
            drops: Vec::new(),
            splashes: VecDeque::with_capacity(MAX_SPLASHES),
            terminal_width,
            terminal_height,
            intensity,
            wind_x: 0.0,
            sheet: None,
            ground: Ground {
                horizon: terminal_height.saturating_sub(1),
                ..Ground::default()
            },
        };
        let wind_dir = if rand::random::<bool>() { 1.0 } else { -1.0 };
        system.set_intensity_with_dir(intensity, wind_dir);
        system
    }

    pub fn set_intensity(&mut self, intensity: RainIntensity) {
        let current_dir = if self.wind_x >= 0.0 { 1.0 } else { -1.0 };
        self.set_intensity_with_dir(intensity, current_dir);
    }

    pub fn set_intensity_with_dir(&mut self, intensity: RainIntensity, direction_multiplier: f32) {
        self.intensity = intensity;
        let base_wind = match intensity {
            RainIntensity::Drizzle => 0.05,
            RainIntensity::Light => 0.1,
            RainIntensity::Heavy => 0.15,
            RainIntensity::Storm => 0.8,
        };
        self.wind_x = base_wind * direction_multiplier;
    }

    pub fn set_wind(&mut self, speed_kmh: f32, direction_deg: f32) {
        let speed_factor = speed_kmh / 40.0;
        let direction_rad = direction_deg.to_radians();
        let x_component = -direction_rad.sin();
        self.wind_x = speed_factor * x_component;
    }

    fn target_count(&self) -> usize {
        let width = self.terminal_width as f32;
        let all = match self.intensity {
            RainIntensity::Drizzle => width / 4.0,
            RainIntensity::Light => width / 2.0,
            RainIntensity::Heavy => width,
            RainIntensity::Storm => width * 1.5,
        };
        let sheet = if self.sheet.is_some() { 1.5 } else { 1.0 };
        (all / 2.0 * sheet) as usize
    }

    fn spawn_drop(&mut self, rng: &mut (impl Rng + ?Sized), around: Option<(f32, f32)>) {
        let x = match around {
            Some((left, width)) => left + rng.random::<f32>() * width,
            None => {
                let span = (self.terminal_width as u32).saturating_mul(2).max(1);
                (rng.random::<u32>() % span) as f32 - (self.terminal_width as f32 * 0.5)
            }
        };
        let near = self.depth == Depth::Near;
        let (speed_y, chars): (f32, &[char]) = match self.intensity {
            RainIntensity::Drizzle => (if near { 0.4 } else { 0.2 }, &['.', ',']),
            RainIntensity::Light => (if near { 0.7 } else { 0.4 }, &['|', ':', '.']),
            RainIntensity::Heavy => (if near { 0.9 } else { 0.6 }, &['|', ':']),
            RainIntensity::Storm => (
                if near { 1.8 } else { 1.2 },
                if self.wind_x > 0.0 { &['\\'] } else { &['/'] },
            ),
        };
        let land_y = match self.depth {
            Depth::Near => {
                let soil = self
                    .terminal_height
                    .saturating_sub(self.ground.horizon)
                    .max(1);
                f32::from(self.ground.horizon + (rng.random::<u16>() % soil))
            }
            Depth::Far => f32::from(self.ground.horizon.saturating_sub(1)),
        };
        self.drops.push(Raindrop {
            x,
            y: 0.0,
            speed_y: speed_y + (rng.random::<f32>() * 0.2),
            speed_x: self.wind_x + (rng.random::<f32>() * 0.1 - 0.05),
            character: chars[(rng.random::<u32>() as usize) % chars.len()],
            land_y,
        });
    }

    fn move_sheet(&mut self, rng: &mut (impl Rng + ?Sized)) {
        let heavy = matches!(self.intensity, RainIntensity::Heavy | RainIntensity::Storm);
        if self.depth == Depth::Far || !heavy {
            self.sheet = None;
            return;
        }
        match self.sheet.as_mut() {
            Some(sheet) => {
                sheet.x += self.wind_x * 2.0 + 0.3_f32.copysign(self.wind_x);
                sheet.life = sheet.life.saturating_sub(1);
                if sheet.life == 0 {
                    self.sheet = None;
                }
            }
            None if rng.random::<f32>() < SHEET_CHANCE => {
                self.sheet = Some(Sheet {
                    x: rng.random::<f32>() * f32::from(self.terminal_width),
                    width: 8.0 + rng.random::<f32>() * 8.0,
                    life: 90 + rng.random::<u16>() % 60,
                });
            }
            None => {}
        }
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        terminal_height: u16,
        rng: &mut (impl Rng + ?Sized),
    ) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;
        self.move_sheet(rng);

        if self.drops.len() < self.target_count() {
            let spawn_rate = match self.intensity {
                RainIntensity::Drizzle => 1,
                RainIntensity::Light => 1,
                _ => 3,
            };
            for _ in 0..spawn_rate {
                self.spawn_drop(rng, None);
            }
        }
        if let Some(sheet) = self.sheet {
            for _ in 0..SHEET_DROPS {
                self.spawn_drop(rng, Some((sheet.x, sheet.width)));
            }
        }

        let splash_chance = match self.intensity {
            RainIntensity::Drizzle => 0.1,
            RainIntensity::Light => 0.3,
            _ => 0.6,
        };
        let depth = self.depth;
        let ground = &self.ground;
        let splashes = &mut self.splashes;
        self.drops.retain_mut(|drop| {
            drop.y += drop.speed_y;
            drop.x += drop.speed_x;
            let landing = match depth {
                Depth::Far => ground
                    .roof_at(drop.x)
                    .map_or((drop.land_y, Surface::Grass), |roof| {
                        (f32::from(roof), Surface::Roof)
                    }),
                Depth::Near => (drop.land_y, Surface::Puddle),
            };
            if drop.y >= landing.0 {
                let chance = if landing.1 == Surface::Roof {
                    0.5
                } else {
                    splash_chance
                };
                if drop.x >= 0.0 && rng.random::<f32>() < chance {
                    splashes.push_back(Splash {
                        x: drop.x as u16,
                        y: landing.0 as u16,
                        timer: 0,
                        surface: landing.1,
                    });
                }
                return false;
            }
            drop.x >= -10.0 && drop.x <= (terminal_width as f32 + 10.0)
        });

        while self.splashes.len() > MAX_SPLASHES {
            self.splashes.pop_front();
        }
        self.splashes.retain_mut(|splash| {
            splash.timer += 1;
            usize::from(splash.timer) < splash_frames(splash.surface).len()
        });
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        near: &[Color; 4],
        far: Color,
        splash_color: Color,
    ) -> io::Result<()> {
        let drop_color = match self.depth {
            Depth::Near => near[rain_tier(self.intensity)],
            Depth::Far => far,
        };
        for drop in &self.drops {
            let x = drop.x as i16;
            let y = drop.y as i16;
            if x >= 0 && x < self.terminal_width as i16 && y >= 0 && y < self.terminal_height as i16
            {
                let ch = if matches!(self.intensity, RainIntensity::Storm | RainIntensity::Heavy) {
                    if drop.speed_x > 0.5 {
                        '\\'
                    } else if drop.speed_x < -0.5 {
                        '/'
                    } else {
                        drop.character
                    }
                } else {
                    drop.character
                };
                let ch = crate::glyphs::swap(self.glyphs, crate::glyphs::Element::Rain, ch);
                renderer.render_char(x as u16, y as u16, ch, drop_color)?;
            }
        }
        for splash in &self.splashes {
            if splash.x < self.terminal_width && splash.y < self.terminal_height {
                let frames = splash_frames(splash.surface);
                let ch = frames[usize::from(splash.timer).min(frames.len() - 1)];
                let color = match splash.surface {
                    Surface::Puddle => splash_color,
                    Surface::Roof | Surface::Grass => far,
                };
                renderer.render_char(splash.x, splash.y, ch, color)?;
            }
        }
        Ok(())
    }
}

fn splash_frames(surface: Surface) -> &'static [char] {
    match surface {
        Surface::Puddle => &['.', 'o', 'O'],
        Surface::Roof => &['\'', '.'],
        Surface::Grass => &[',', '.'],
    }
}

fn rain_tier(intensity: RainIntensity) -> usize {
    match intensity {
        RainIntensity::Drizzle => 0,
        RainIntensity::Light => 1,
        RainIntensity::Heavy => 2,
        RainIntensity::Storm => 3,
    }
}

impl AnimationSystem for RaindropSystem {
    fn id(&self) -> &'static str {
        match self.depth {
            Depth::Near => "rain",
            Depth::Far => "rain_far",
        }
    }

    fn layer(&self) -> RenderLayer {
        match self.depth {
            Depth::Near => RenderLayer::Foreground,
            Depth::Far => RenderLayer::Background,
        }
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.conditions.is_raining || ctx.conditions.is_thunderstorm
    }

    fn on_resize(&mut self, size: TerminalSize) {
        self.terminal_width = size.width;
        self.terminal_height = size.height;
        self.drops.retain(|d| {
            d.x >= -10.0
                && d.x <= (size.width as f32 + 10.0)
                && d.y >= 0.0
                && d.y < size.height as f32
        });
        self.splashes
            .retain(|s| s.x < size.width && s.y < size.height);
    }

    fn on_wind(&mut self, wind: Wind) {
        self.set_wind(wind.speed_kmh, wind.direction_deg);
    }

    fn on_rain_intensity(&mut self, intensity: RainIntensity) {
        self.set_intensity(intensity);
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        if self.ground.horizon != ctx.horizon_y || self.ground.roof.is_empty() != ctx.door.is_none()
        {
            self.ground = Ground::from(ctx.horizon_y, ctx.door);
        }
        self.update(ctx.size.width, ctx.size.height, rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        self.glyphs = ctx.state.glyphs;
        RaindropSystem::render(
            self,
            renderer,
            &ctx.colors.rain_near,
            ctx.colors.rain_far,
            ctx.colors.rain_splash,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn door() -> DoorAnchor {
        DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 25,
        }
    }

    fn rain(depth: Depth, intensity: RainIntensity) -> RaindropSystem {
        let mut system = RaindropSystem::new(depth, 110, 32, intensity);
        system.ground = Ground::from(25, Some(door()));
        system.set_wind(0.0, 0.0);
        system
    }

    fn run(system: &mut RaindropSystem, frames: usize) -> Vec<Splash> {
        let mut rng = StdRng::seed_from_u64(3);
        let mut seen = Vec::new();
        for _ in 0..frames {
            system.update(110, 32, &mut rng);
            seen.extend(system.splashes.iter().filter(|splash| splash.timer == 1));
        }
        seen
    }

    #[test]
    fn rain_intensity_selects_a_near_tier() {
        assert_eq!(rain_tier(RainIntensity::Drizzle), 0);
        assert_eq!(rain_tier(RainIntensity::Light), 1);
        assert_eq!(rain_tier(RainIntensity::Heavy), 2);
        assert_eq!(rain_tier(RainIntensity::Storm), 3);
    }

    #[test]
    fn far_rain_splashes_on_the_roof_and_the_grass_line() {
        let mut far = rain(Depth::Far, RainIntensity::Heavy);
        let splashes = run(&mut far, 600);
        let roof: Vec<&Splash> = splashes
            .iter()
            .filter(|splash| splash.surface == Surface::Roof)
            .collect();
        assert!(!roof.is_empty());
        for splash in roof {
            assert_eq!(far.ground.roof_at(f32::from(splash.x)), Some(splash.y));
            assert!(splash.y < 25 - 4, "on the roof, above the walls");
        }
        assert!(
            splashes
                .iter()
                .filter(|splash| splash.surface == Surface::Grass)
                .all(|splash| splash.y == 24)
        );
    }

    #[test]
    fn near_rain_lands_across_the_soil_in_front() {
        let mut near = rain(Depth::Near, RainIntensity::Heavy);
        let splashes = run(&mut near, 600);
        assert!(
            splashes
                .iter()
                .all(|splash| splash.surface == Surface::Puddle)
        );
        assert!(splashes.iter().all(|splash| (25..32).contains(&splash.y)));
        let rows: std::collections::HashSet<u16> = splashes.iter().map(|splash| splash.y).collect();
        assert!(rows.len() > 3, "{rows:?}");
    }

    #[test]
    fn heavy_rain_sends_sheets_and_light_rain_does_not() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut heavy = rain(Depth::Near, RainIntensity::Heavy);
        let mut light = rain(Depth::Near, RainIntensity::Light);
        let mut far = rain(Depth::Far, RainIntensity::Storm);
        let mut sheets = 0;
        for _ in 0..3000 {
            heavy.update(110, 32, &mut rng);
            light.update(110, 32, &mut rng);
            far.update(110, 32, &mut rng);
            sheets += usize::from(heavy.sheet.is_some_and(|sheet| sheet.life == 1));
            assert!(light.sheet.is_none() && far.sheet.is_none());
        }
        assert!(sheets >= 1);
    }

    #[test]
    fn far_rain_sits_behind_the_scene_and_near_rain_in_front() {
        let far = rain(Depth::Far, RainIntensity::Light);
        let near = rain(Depth::Near, RainIntensity::Light);
        assert_eq!(far.layer(), RenderLayer::Background);
        assert_eq!(near.layer(), RenderLayer::Foreground);
        assert_ne!(far.id(), near.id());
    }
}
