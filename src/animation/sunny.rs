use super::Animation;
use crate::animation::{
    AnimationController, AnimationSystem, FrameCommands, FrameContext, RenderLayer,
};
use crate::astro::SunSky;
use crate::render::TerminalRenderer;
use crate::scene::world::house::House;
use crossterm::style::Color;
use rand::Rng;

use std::io;
use std::time::{Duration, Instant};

const FRAME_DELAY: Duration = Duration::from_millis(500);

const SUN_FRAMES: [&str; 2] = [
    include_str!("assets/sun_0.txt"),
    include_str!("assets/sun_1.txt"),
];

pub struct SunnyAnimation {
    frames: Vec<Vec<String>>,
    color: Color,
}

impl SunnyAnimation {
    pub fn new() -> Self {
        let frames = SUN_FRAMES
            .iter()
            .map(|src| src.lines().map(|l| l.to_string()).collect())
            .collect();
        Self {
            frames,
            color: Color::Reset,
        }
    }
}

impl Animation for SunnyAnimation {
    fn get_frame(&self, frame_number: usize) -> &[String] {
        &self.frames[frame_number % self.frames.len()]
    }

    fn frame_count(&self) -> usize {
        self.frames.len()
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

impl Default for SunnyAnimation {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SunSystem {
    animation: SunnyAnimation,
    controller: AnimationController,
    last_frame_time: Instant,
}

impl SunSystem {
    pub fn new() -> Self {
        Self {
            animation: SunnyAnimation::new(),
            controller: AnimationController::new(),
            last_frame_time: Instant::now(),
        }
    }
}

impl Default for SunSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for SunSystem {
    fn id(&self) -> &'static str {
        "sun"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Background
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        !ctx.conditions.is_raining && !ctx.conditions.is_thunderstorm && !ctx.conditions.is_snowing
    }

    fn update(
        &mut self,
        _ctx: &FrameContext<'_>,
        _rng: &mut dyn Rng,
        _commands: &mut FrameCommands,
    ) {
        if self.last_frame_time.elapsed() >= FRAME_DELAY {
            self.controller.next_frame(&self.animation);
            self.last_frame_time = Instant::now();
        }
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        if !ctx.state.should_show_sun()
            || ctx.conditions.is_raining
            || ctx.conditions.is_thunderstorm
            || ctx.conditions.is_snowing
        {
            return Ok(());
        }

        let top = if ctx.size.height > 20 { 3 } else { 2 };
        let lines = self.controller.frame(&self.animation);
        let art_width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
        let (x, y) = sun_spot(
            ctx.state.sun_sky,
            ctx.size.width,
            ctx.horizon_y,
            top,
            art_width,
        );
        let veiled = ctx.conditions.veils_the_sky();
        for (dy, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            let cell_y = y + dy as u16;
            let color = if veiled {
                renderer.veiled(cell_y, ctx.colors.sun)
            } else {
                ctx.colors.sun
            };
            for dx in 0..chars.len() {
                let cell_x = x + dx as u16;
                if let Some(ch) = chars.get(dx).filter(|ch| **ch != ' ') {
                    renderer.render_char(cell_x, cell_y, *ch, color)?;
                }
            }
        }
        Ok(())
    }
}

const SIDE_MARGIN: u16 = 1;
const SUN_ROWS: u16 = 9;
const ROOF_GAP: u16 = 2;
const HIGH_DEG: f64 = 45.0;
const CLEAR_DEG: f64 = 20.0;
const HIGH_SHARE: f64 = 0.75;
const CLEAR_SHARE: f64 = 0.35;

pub(crate) fn sun_spot(
    sky: Option<SunSky>,
    width: u16,
    horizon_y: u16,
    top: u16,
    art_width: u16,
) -> (u16, u16) {
    let centred = (width.saturating_sub(art_width) / 2, top);
    let Some((sky, progress)) = sky.and_then(|sky| sky.day_progress.map(|p| (sky, p))) else {
        return centred;
    };
    arc_spot(
        progress,
        sky.elevation_deg,
        sky.noon_elevation_deg,
        (width, horizon_y, top),
        (art_width, SUN_ROWS),
    )
}

pub(crate) fn arc_spot(
    progress: f64,
    elevation_deg: f64,
    peak_deg: f64,
    (width, horizon_y, top): (u16, u16, u16),
    (art_width, art_height): (u16, u16),
) -> (u16, u16) {
    let span = width.saturating_sub(art_width + 2 * SIDE_MARGIN);
    let x = if width >= art_width + 2 * SIDE_MARGIN {
        SIDE_MARGIN + (progress.clamp(0.0, 1.0) * f64::from(span)).round() as u16
    } else {
        0
    };
    (
        x,
        disc_top(elevation_deg, peak_deg, (horizon_y, top), art_height),
    )
}

fn disc_top(
    elevation_deg: f64,
    peak_deg: f64,
    (horizon_y, top): (u16, u16),
    art_height: u16,
) -> u16 {
    let roof = horizon_y.saturating_sub(House::HEIGHT);
    let clear = roof.saturating_sub(ROOF_GAP + art_height).max(top);
    let low = horizon_y.saturating_sub(art_height / 2).max(clear);
    let peak = if peak_deg > 0.0 { peak_deg } else { HIGH_DEG };
    let (high_at, clear_at) = (
        (peak * HIGH_SHARE).min(HIGH_DEG),
        (peak * CLEAR_SHARE).min(CLEAR_DEG),
    );
    let between = |from: u16, to: u16, share: f64| {
        (f64::from(from) + (f64::from(to) - f64::from(from)) * share.clamp(0.0, 1.0)).round() as u16
    };
    if elevation_deg >= high_at {
        top
    } else if elevation_deg >= clear_at {
        between(
            clear,
            top,
            (elevation_deg - clear_at) / (high_at - clear_at),
        )
    } else {
        between(low, clear, elevation_deg.max(0.0) / clear_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::TerminalSize;
    use crate::app_state::AppState;
    use crate::astro::SunSky;
    use crate::config::LocationDisplay;
    use crate::weather::types::CelestialEvents;
    use crate::weather::{
        WeatherCondition, WeatherConditions, WeatherData, WeatherLocation, WeatherUnits,
    };

    fn sky(progress: f64, elevation: f64) -> Option<SunSky> {
        Some(SunSky {
            elevation_deg: elevation,
            azimuth_deg: 0.0,
            noon_elevation_deg: 80.0,
            day_progress: Some(progress),
        })
    }

    #[test]
    fn the_sun_rises_left_peaks_in_the_middle_and_sets_right() {
        assert_eq!(sun_spot(sky(0.0, -0.8), 110, 25, 3, 21), (1, 21));
        assert_eq!(sun_spot(sky(0.1, 10.0), 110, 25, 3, 21), (10, 13));
        assert_eq!(sun_spot(sky(0.25, 40.0), 110, 25, 3, 21), (23, 3));
        assert_eq!(sun_spot(sky(0.5, 80.0), 110, 25, 3, 21), (45, 3));
        assert_eq!(sun_spot(sky(1.0, -0.8), 110, 25, 3, 21), (88, 21));
        assert_eq!(sun_spot(sky(1.3, -10.0), 110, 25, 3, 21), (88, 21));
        let (_, high) = sun_spot(sky(0.25, 25.0), 110, 25, 3, 21);
        assert!(high + 9 + 2 <= 15, "clear of the roof at 25°");
    }

    #[test]
    fn without_a_sky_the_sun_keeps_its_old_spot() {
        assert_eq!(sun_spot(None, 110, 25, 3, 21), (44, 3));
        let never_rises = SunSky {
            day_progress: None,
            ..sky(0.5, 80.0).unwrap()
        };
        assert_eq!(sun_spot(Some(never_rises), 110, 25, 3, 21), (44, 3));
    }

    #[test]
    fn tiny_panes_keep_the_sun_on_screen() {
        assert_eq!(sun_spot(sky(1.0, 0.0), 15, 6, 2, 21), (0, 2));
    }

    #[test]
    fn the_sun_is_drawn_where_the_sky_puts_it() {
        let sun = CelestialEvents::from_bool(true);
        let location = WeatherLocation {
            latitude: 0.0,
            longitude: 0.0,
            elevation: None,
        };
        let units = WeatherUnits::metric();
        let mut state = AppState::new(location, None, LocationDisplay::Coordinates, false, units);
        state.current_weather = Some(WeatherData {
            condition: WeatherCondition::Clear,
            temperature: 20.0,
            precipitation: 0.0,
            wind_speed: 5.0,
            wind_direction: 0.0,
            sun,
            moon_phase: None,
            humidity: None,
            timestamp: String::new(),
            attribution: String::new(),
            feels_like: None,
            uv_index: None,
            outlook: Vec::new(),
        });
        state.sun_sky = sky(0.0, -0.8);
        let conditions = WeatherConditions {
            sun,
            ..WeatherConditions::default()
        };
        let colors = crate::theme::catalogue::default_colors().day;
        let events = crate::events::ActiveEvents::default();
        let ctx = FrameContext {
            size: TerminalSize {
                width: 80,
                height: 24,
            },
            horizon_y: 18,
            conditions: &conditions,
            state: &state,
            show_leaves: false,
            chimney: None,
            skyline_y: 0,
            door: None,
            events: &events,
            colors: &colors,
        };
        let mut renderer = TerminalRenderer::headless(80, 24);
        SunSystem::new().render(&mut renderer, &ctx).unwrap();
        let text = renderer.text();
        let rows: Vec<&str> = text.lines().collect();
        let widest = rows
            .iter()
            .position(|row| row.contains("~ -- :"))
            .expect("the sun");
        assert_eq!(
            widest,
            14 + 4,
            "rising, its middle on the ground line: {text}"
        );
        assert_eq!(rows[widest].find('~'), Some(1), "{text}");
    }
}
