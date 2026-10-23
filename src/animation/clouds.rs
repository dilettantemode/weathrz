use crate::animation::{
    AnimationSystem, FrameCommands, FrameContext, RenderLayer, TerminalSize, Wind,
};
use crate::render::TerminalRenderer;
use crate::weather::WeatherCondition;
use crossterm::style::Color;

use rand::{Rng, RngExt};
use std::io;
use std::sync::OnceLock;

const CLOUD_FILL: f32 = 0.18;
const CLOUD_SHAPE_SRCS: [&str; 4] = [
    include_str!("assets/cloud_0.txt"),
    include_str!("assets/cloud_1.txt"),
    include_str!("assets/cloud_2.txt"),
    include_str!("assets/cloud_3.txt"),
];

static CLOUD_SHAPES: OnceLock<Vec<Vec<String>>> = OnceLock::new();

const CALM_DRIFT: f32 = 0.01;
const DRIFT_PER_KMH: f32 = 0.005;
const CROSSWIND: f32 = 0.2;

pub(crate) fn cloud_drift(speed_kmh: f32, direction_deg: f32) -> f32 {
    let east_west = -direction_deg.to_radians().sin();
    if east_west.abs() < CROSSWIND {
        return CALM_DRIFT;
    }
    let drift = speed_kmh * DRIFT_PER_KMH * east_west;
    if drift.abs() >= CALM_DRIFT {
        drift
    } else if east_west < 0.0 {
        -CALM_DRIFT
    } else {
        CALM_DRIFT
    }
}

fn cloud_shapes() -> &'static Vec<Vec<String>> {
    CLOUD_SHAPES.get_or_init(|| {
        CLOUD_SHAPE_SRCS
            .iter()
            .map(|src| src.lines().map(|l| l.to_string()).collect())
            .collect()
    })
}

struct Cloud {
    x: f32,
    y: f32,
    parallax: f32,
    shape: Vec<String>,
    color: Color,
}

pub struct CloudSystem {
    clouds: Vec<Cloud>,
    terminal_width: u16,
    terminal_height: u16,
    drift: f32,
}

impl CloudSystem {
    pub fn set_cloud_color(&mut self, color: Color) {
        for cloud in &mut self.clouds {
            cloud.color = color;
        }
    }

    pub fn set_wind(&mut self, speed_kmh: f32, direction_deg: f32) {
        self.drift = cloud_drift(speed_kmh, direction_deg);
    }
}

impl CloudSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        let mut rng = rand::rng();

        // Add few initial clouds
        let count = std::cmp::max(1, terminal_width / 30) as usize;
        let segment = terminal_width as f32 / count as f32;

        let mut clouds = Vec::with_capacity(count);

        for i in 0..count {
            let x_min = (i as f32 * segment) as u16;
            let x_max = ((i as f32 + 1.0) * segment) as u16;
            let x = rng.random_range(x_min..=x_max) as f32;
            clouds.push(Self::create_random_cloud(
                x,
                terminal_height,
                Color::Reset,
                &mut rng,
            ));
        }

        Self {
            clouds,
            terminal_width,
            terminal_height,
            drift: CALM_DRIFT,
        }
    }

    fn create_random_cloud(
        x: f32,
        height: u16,
        color: Color,
        rng: &mut (impl Rng + ?Sized),
    ) -> Cloud {
        let shapes = cloud_shapes();

        let shape_idx = rng.random_range(0..shapes.len());
        let shape = shapes[shape_idx].clone();

        let y_range = (height / 3).max(1);
        let y = rng.random_range(0..y_range) as f32;
        let parallax = 0.8 + rng.random::<f32>() * 0.4;

        Cloud {
            x,
            y,
            parallax,
            shape,
            color,
        }
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        terminal_height: u16,
        is_clear: bool,
        cloud_color: Color,
        rng: &mut (impl Rng + ?Sized),
    ) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;

        for cloud in &mut self.clouds {
            cloud.x += self.drift * cloud.parallax;
        }

        let width_f = terminal_width as f32;
        self.clouds.retain(|cloud| {
            let cloud_width = cloud.shape.iter().map(|line| line.len()).max().unwrap_or(0) as f32;
            if self.drift >= 0.0 {
                cloud.x < width_f
            } else {
                cloud.x + cloud_width > 0.0
            }
        });

        let max_clouds = if is_clear {
            (terminal_width / 30) as usize
        } else {
            (terminal_width / 20) as usize
        };

        let spawn_chance = if is_clear { 0.002 } else { 0.005 };

        if self.clouds.len() < max_clouds && rng.random::<f32>() < spawn_chance {
            let mut cloud = Self::create_random_cloud(0.0, terminal_height, cloud_color, rng);
            let cloud_width = cloud.shape.iter().map(|line| line.len()).max().unwrap_or(0) as f32;

            let spawn_from_left = self.drift >= 0.0;
            let min_gap = (terminal_width as f32 / 8.0).max(15.0);
            let too_close = if spawn_from_left {
                self.clouds.iter().any(|c| c.x < min_gap)
            } else {
                self.clouds.iter().any(|c| c.x > (width_f - min_gap))
            };

            if !too_close {
                cloud.x = if spawn_from_left {
                    -cloud_width
                } else {
                    width_f
                };
                self.clouds.push(cloud);
            }
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for cloud in &self.clouds {
            for (i, line) in cloud.shape.iter().enumerate() {
                let y = cloud.y as i16 + i as i16;
                let x = cloud.x as i16;

                if y < 0 || y >= self.terminal_height as i16 {
                    continue;
                }

                let clip = ((-x).max(0)) as usize;
                let visible = &line[clip.min(line.len())..];

                if !visible.is_empty() {
                    renderer.render_line_colored(
                        x.max(0) as u16,
                        y as u16,
                        visible,
                        cloud.color,
                    )?;
                    let drawn =
                        |at: Option<usize>| at.map(|column| x.max(0) as u16 + column as u16);
                    let first = drawn(visible.find(|ch: char| ch != ' '));
                    let last = drawn(visible.rfind(|ch: char| ch != ' '));
                    if let (Some(first), Some(last)) = (first, last) {
                        for column in first..=last {
                            renderer.tint(column, y as u16, cloud.color, CLOUD_FILL);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn cloud_tint(condition: Option<WeatherCondition>, colors: &[Color; 3]) -> (bool, Color) {
    match condition {
        Some(WeatherCondition::Clear) => (true, colors[0]),
        Some(WeatherCondition::PartlyCloudy) => (false, colors[1]),
        _ => (false, colors[2]),
    }
}

impl AnimationSystem for CloudSystem {
    fn id(&self) -> &'static str {
        "clouds"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Background
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        let is_clear = ctx
            .state
            .current_weather
            .as_ref()
            .is_some_and(|w| matches!(w.condition, crate::weather::WeatherCondition::Clear));

        ctx.conditions.is_cloudy || is_clear
    }

    fn on_resize(&mut self, size: TerminalSize) {
        self.terminal_width = size.width;
        self.terminal_height = size.height;
    }

    fn on_wind(&mut self, wind: Wind) {
        self.set_wind(wind.speed_kmh, wind.direction_deg);
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        let tiers = &ctx.colors.clouds;
        let (is_clear, cloud_color) = cloud_tint(
            ctx.state
                .current_weather
                .as_ref()
                .map(|weather| weather.condition),
            tiers,
        );
        self.set_cloud_color(if is_clear { tiers[0] } else { tiers[2] });
        self.update(ctx.size.width, ctx.size.height, is_clear, cloud_color, rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        _ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        CloudSystem::render(self, renderer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn clouds_drift_with_the_east_west_wind() {
        assert_eq!(cloud_drift(0.0, 0.0), CALM_DRIFT);
        assert!((cloud_drift(10.0, 90.0) + 0.05).abs() < 1e-4);
        assert!((cloud_drift(30.0, 270.0) - 0.15).abs() < 1e-4);
        assert_eq!(cloud_drift(20.0, 0.0), CALM_DRIFT);
        assert!((cloud_drift(1.0, 90.0) + CALM_DRIFT).abs() < 1e-6);
        assert!(cloud_drift(40.0, 270.0) > cloud_drift(20.0, 270.0));
        for direction in [359.0, 1.0, 179.0, 181.0, 5.0, 355.0] {
            assert_eq!(cloud_drift(30.0, direction), CALM_DRIFT, "{direction}");
        }
        assert!(cloud_drift(30.0, 20.0) < 0.0);
        assert!(cloud_drift(30.0, 340.0) > CALM_DRIFT);
    }

    #[test]
    fn a_stronger_wind_moves_the_clouds_faster() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut moved = |kmh: f32| {
            let mut system = CloudSystem::new(110, 32);
            system.set_wind(kmh, 270.0);
            let before: Vec<f32> = system.clouds.iter().map(|cloud| cloud.x).collect();
            system.update(110, 32, true, Color::Reset, &mut rng);
            system
                .clouds
                .iter()
                .zip(&before)
                .map(|(cloud, x)| cloud.x - x)
                .fold(f32::INFINITY, f32::min)
        };
        let gentle = moved(10.0);
        let strong = moved(30.0);
        assert!(gentle > 0.0, "{gentle}");
        assert!(strong > 2.0 * gentle, "{gentle} {strong}");
        assert!(strong < 0.2, "{strong}");
    }

    #[test]
    fn cloud_tint_follows_condition() {
        let tiers = [
            Color::AnsiValue(0),
            Color::AnsiValue(1),
            Color::AnsiValue(2),
        ];
        assert_eq!(
            cloud_tint(Some(WeatherCondition::Clear), &tiers),
            (true, tiers[0])
        );
        assert_eq!(
            cloud_tint(Some(WeatherCondition::PartlyCloudy), &tiers),
            (false, tiers[1])
        );
        assert_eq!(
            cloud_tint(Some(WeatherCondition::Overcast), &tiers),
            (false, tiers[2])
        );
        assert_eq!(cloud_tint(None, &tiers), (false, tiers[2]));
    }
}
