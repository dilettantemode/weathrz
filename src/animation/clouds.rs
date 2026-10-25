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
const DARK_SKY: f32 = 40.0;
const CLOUD_GAP: f32 = 2.0;
const DECK_SHARE: f32 = 0.3;
const NEIGHBOUR_ROWS: f32 = 2.0;
const NEIGHBOUR_TRIES: usize = 6;
const TOP_UP_TRIES: usize = 4;
const DECK_TRIES: usize = 400;
const DECK_COVER: f32 = 0.8;
const STACK_GAP: f32 = 1.0;
const NEAR_SHARE: f64 = 0.5;
const DECK_STEP: f32 = 3.0;
const DECK_GAP: usize = 8;
const CLOUD_INK: (u8, u8, u8) = (230, 236, 248);
const DAY_CLOUD: (f32, (u8, u8, u8), f32) = (0.07, CLOUD_INK, 0.62);
const GREY_CLOUD: (f32, (u8, u8, u8), f32) = (0.08, (120, 124, 140), 0.55);
const DECK_CLOUD: (f32, (u8, u8, u8), f32) = (0.08, (120, 124, 140), 0.35);
const LIT_FROM_BEHIND: f32 = 0.35;
const THIN_CLOUD_GLOW: f32 = 0.6;
const CLOUD_SHAPE_SRCS: [&str; 5] = [
    include_str!("assets/cloud_0.txt"),
    include_str!("assets/cloud_1.txt"),
    include_str!("assets/cloud_2.txt"),
    include_str!("assets/cloud_3.txt"),
    include_str!("assets/cloud_4.txt"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cover {
    Clear,
    Partly,
    Cloudy,
    Deck,
}

impl Cover {
    fn of(condition: Option<WeatherCondition>, veiled: bool) -> Self {
        match condition {
            _ if veiled => Self::Deck,
            Some(WeatherCondition::Clear) => Self::Clear,
            Some(WeatherCondition::Cloudy) => Self::Cloudy,
            _ => Self::Partly,
        }
    }

    fn columns_per_cloud(self) -> u16 {
        match self {
            Self::Clear => 30,
            Self::Partly => 20,
            Self::Cloudy => 14,
            Self::Deck => 6,
        }
    }

    fn spawn_chance(self) -> f32 {
        match self {
            Self::Clear => 0.002,
            Self::Partly => 0.005,
            Self::Cloudy => 0.01,
            Self::Deck => 0.03,
        }
    }
}

struct Cloud {
    x: f32,
    y: f32,
    parallax: f32,
    kind: usize,
    near: bool,
    shape: Vec<String>,
    solid: Vec<Vec<bool>>,
    color: Color,
}

pub struct CloudSystem {
    clouds: Vec<Cloud>,
    terminal_width: u16,
    terminal_height: u16,
    drift: f32,
    cover: Option<Cover>,
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

fn mirrored(shape: &[String]) -> Vec<String> {
    let wide = shape
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0);
    shape
        .iter()
        .map(|row| {
            let mut chars: Vec<char> = row.chars().collect();
            chars.resize(wide, ' ');
            let flipped: String = chars
                .iter()
                .rev()
                .map(|ch| match ch {
                    '(' => ')',
                    ')' => '(',
                    '/' => '\\',
                    '\\' => '/',
                    other => *other,
                })
                .collect();
            flipped.trim_end().to_string()
        })
        .collect()
}

pub(crate) fn deck_floor(height: u16) -> u16 {
    let top = crate::animation::sky::SKY_TOP;
    let ground = crate::scene::world::WorldScene::ground_y(height);
    top + (f32::from(ground.saturating_sub(top)) * DECK_SHARE) as u16
}

fn cloud_rows(height: u16, rows: u16, cover: Cover) -> (u16, u16) {
    let top = crate::animation::sky::SKY_TOP;
    let ground = crate::scene::world::WorldScene::ground_y(height);
    let floor = if cover == Cover::Deck {
        deck_floor(height)
    } else {
        ground.saturating_sub(crate::scene::world::trees::TALLEST_ROWS + 2)
    };
    (top, floor.saturating_sub(rows).max(top))
}

fn centre_of(cloud: &Cloud) -> f32 {
    cloud.x + width_of(cloud) / 2.0
}

impl CloudSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            clouds: Vec::new(),
            terminal_width,
            terminal_height,
            drift: CALM_DRIFT,
            cover: None,
        }
    }

    fn target(&self, cover: Cover) -> usize {
        usize::from((self.terminal_width / cover.columns_per_cloud()).max(1))
    }

    fn random_cloud(&self, color: Color, fresh: bool, rng: &mut (impl Rng + ?Sized)) -> Cloud {
        let shapes = cloud_shapes();
        let looks = shapes.len() * 2;
        let shown = |look: usize, same: fn(usize, usize) -> bool| {
            self.clouds.iter().any(|cloud| same(cloud.kind, look))
        };
        let new_shapes: Vec<usize> = (0..looks)
            .filter(|look| !shown(*look, |a, b| a / 2 == b / 2))
            .collect();
        let new_looks: Vec<usize> = (0..looks)
            .filter(|look| !shown(*look, |a, b| a == b))
            .collect();
        let pool = if !fresh {
            (0..looks).collect()
        } else if !new_shapes.is_empty() {
            new_shapes
        } else if !new_looks.is_empty() {
            new_looks
        } else {
            (0..looks).collect()
        };
        let kind = pool[rng.random_range(0..pool.len())];
        let shape = if kind % 2 == 1 {
            mirrored(&shapes[kind / 2])
        } else {
            shapes[kind / 2].clone()
        };
        let solid = crate::animation::solid::solid_cells(&shape);
        Cloud {
            x: 0.0,
            y: f32::from(crate::animation::sky::SKY_TOP),
            parallax: 1.0,
            kind,
            near: rng.random_bool(NEAR_SHARE),
            shape,
            solid,
            color,
        }
    }

    fn settle(&self, cloud: &mut Cloud, cover: Cover, rng: &mut (impl Rng + ?Sized)) {
        let (top, lowest) = cloud_rows(self.terminal_height, cloud.shape.len() as u16, cover);
        let centre = centre_of(cloud);
        let neighbour = self
            .clouds
            .iter()
            .min_by(|a, b| {
                (centre_of(a) - centre)
                    .abs()
                    .total_cmp(&(centre_of(b) - centre).abs())
            })
            .map(|cloud| cloud.y);
        for _ in 0..NEIGHBOUR_TRIES {
            cloud.y = f32::from(rng.random_range(top..=lowest));
            if neighbour.is_none_or(|y| (y - cloud.y).abs() >= NEIGHBOUR_ROWS) {
                break;
            }
        }
    }

    fn top_up(&mut self, cover: Cover, color: Color, rng: &mut (impl Rng + ?Sized)) {
        if cover == Cover::Deck {
            self.pack_deck(color, rng);
            return;
        }
        let width = f32::from(self.terminal_width);
        let mut misses = 0;
        while self.clouds.len() < self.target(cover) && misses < TOP_UP_TRIES {
            let mut cloud = self.random_cloud(color, true, rng);
            let mut spans: Vec<(f32, f32)> = self
                .clouds
                .iter()
                .map(|cloud| (cloud.x, cloud.x + width_of(cloud)))
                .collect();
            spans.sort_by(|a, b| a.0.total_cmp(&b.0));
            let (mut left, mut widest) = (0.0_f32, (0.0_f32, 0.0_f32));
            for (start, end) in spans.into_iter().chain([(width, width)]) {
                if start - left > widest.1 {
                    widest = (left, start - left);
                }
                left = left.max(end);
            }
            let room = widest.1 - width_of(&cloud) - 2.0 * CLOUD_GAP;
            if room < 0.0 {
                misses += 1;
                continue;
            }
            cloud.x = widest.0 + CLOUD_GAP + rng.random::<f32>() * room;
            self.settle(&mut cloud, cover, rng);
            self.clouds.push(cloud);
        }
    }

    fn pack_deck(&mut self, color: Color, rng: &mut (impl Rng + ?Sized)) {
        let width = f32::from(self.terminal_width);
        let mut x = -rng.random_range(0.0..=DECK_STEP);
        while x < width {
            let mut cloud = self.random_cloud(color, true, rng);
            cloud.x = x;
            let (top, lowest) =
                cloud_rows(self.terminal_height, cloud.shape.len() as u16, Cover::Deck);
            cloud.y = f32::from(rng.random_range(top..=lowest));
            x += width_of(&cloud) + CLOUD_GAP + rng.random_range(0.0..=DECK_STEP);
            if !self.crowds(&cloud) {
                self.clouds.push(cloud);
            }
        }
        for attempt in 0..DECK_TRIES {
            if self.clouds.len() >= self.target(Cover::Deck) || self.deck_closed() {
                break;
            }
            let mut cloud = self.random_cloud(color, attempt % 2 == 0, rng);
            let (start, length) = self.widest_gap();
            let slack = length as f32 - width_of(&cloud) - 2.0 * CLOUD_GAP;
            let shift = if slack >= 0.0 {
                rng.random::<f32>() * slack - slack / 2.0
            } else {
                rng.random_range(-4.0..=4.0)
            };
            let centre = start as f32 + length as f32 / 2.0 + shift;
            cloud.x =
                (centre - width_of(&cloud) / 2.0).clamp(0.0, (width - width_of(&cloud)).max(0.0));
            let (top, lowest) =
                cloud_rows(self.terminal_height, cloud.shape.len() as u16, Cover::Deck);
            cloud.y = f32::from(rng.random_range(top..=lowest));
            if !self.crowds(&cloud) {
                self.clouds.push(cloud);
            }
        }
    }

    fn covered_columns(&self) -> Vec<bool> {
        let width = usize::from(self.terminal_width);
        let mut covered = vec![false; width];
        for cloud in &self.clouds {
            let start = cloud.x.max(0.0) as usize;
            let end = ((cloud.x + width_of(cloud)).max(0.0) as usize).min(width);
            covered[start.min(end)..end].fill(true);
        }
        covered
    }

    fn widest_gap(&self) -> (usize, usize) {
        let mut widest = (0, 0);
        let mut start = 0;
        for (x, covered) in self.covered_columns().into_iter().chain([true]).enumerate() {
            if covered {
                if x - start > widest.1 {
                    widest = (start, x - start);
                }
                start = x + 1;
            }
        }
        widest
    }

    fn deck_closed(&self) -> bool {
        let covered = self.covered_columns();
        let shown = covered.iter().filter(|cell| **cell).count();
        shown as f32 >= DECK_COVER * covered.len() as f32 && self.widest_gap().1 <= DECK_GAP
    }

    fn crowds(&self, cloud: &Cloud) -> bool {
        let rows = |c: &Cloud| (c.y, c.y + c.shape.len() as f32);
        let (top, bottom) = rows(cloud);
        self.clouds.iter().any(|other| {
            let (other_top, other_bottom) = rows(other);
            let shares_rows = top < other_bottom + STACK_GAP && other_top < bottom + STACK_GAP;
            let near = cloud.x < other.x + width_of(other) + CLOUD_GAP
                && other.x < cloud.x + width_of(cloud) + CLOUD_GAP;
            shares_rows && near
        })
    }

    pub fn update(
        &mut self,
        terminal_width: u16,
        terminal_height: u16,
        cover: Cover,
        cloud_color: Color,
        rng: &mut (impl Rng + ?Sized),
    ) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;
        if self.cover != Some(cover) {
            self.cover = Some(cover);
            self.top_up(cover, cloud_color, rng);
        }

        for cloud in &mut self.clouds {
            cloud.x += self.drift * cloud.parallax;
        }

        let width_f = terminal_width as f32;
        self.clouds.retain(|cloud| {
            if self.drift >= 0.0 {
                cloud.x < width_f
            } else {
                cloud.x + width_of(cloud) > 0.0
            }
        });

        if self.clouds.len() < self.target(cover) && rng.random::<f32>() < cover.spawn_chance() {
            let mut cloud = self.random_cloud(cloud_color, true, rng);
            let cloud_width = width_of(&cloud);

            let spawn_from_left = self.drift >= 0.0;
            cloud.x = if spawn_from_left {
                -cloud_width
            } else {
                width_f
            };
            self.settle(&mut cloud, cover, rng);
            let min_gap = (width_f / 8.0).max(cloud_width + CLOUD_GAP);
            let too_close = match (cover == Cover::Deck, spawn_from_left) {
                (true, _) => self.crowds(&cloud),
                (false, true) => self.clouds.iter().any(|c| c.x < min_gap),
                (false, false) => self.clouds.iter().any(|c| c.x > width_f - min_gap),
            };
            if !too_close {
                self.clouds.push(cloud);
            }
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for cloud in &self.clouds {
            let base = cloud.shape.len().saturating_sub(1);
            for (i, (line, inside)) in cloud.shape.iter().zip(&cloud.solid).enumerate() {
                let y = cloud.y as i32 + i as i32;
                if !(0..i32::from(self.terminal_height)).contains(&y) {
                    continue;
                }
                let chars: Vec<char> = line.chars().collect();
                let span = glyph_span(&chars);
                let filled = |dx: usize, ch: char| {
                    span.is_some_and(|(first, last)| {
                        (first..=last).contains(&dx)
                            && !(i != base
                                && crate::animation::solid::bare_edge(&cloud.shape, i, dx, ch))
                    })
                };
                let lit = renderer
                    .sky_row(y as u16)
                    .map(|sky| lighting(sky, self.cover == Some(Cover::Deck)));
                for (dx, solid) in inside.iter().enumerate() {
                    let x = cloud.x as i32 + dx as i32;
                    if !*solid || !(0..i32::from(self.terminal_width)).contains(&x) {
                        continue;
                    }
                    let (x, y) = (x as u16, y as u16);
                    if !cloud.near && renderer.is_protected(x, y) {
                        continue;
                    }
                    let (outline, fill, toward) = match lit {
                        Some((fill, toward, outline)) => (outline, fill, toward),
                        None => (cloud.color, CLOUD_FILL, cloud.color),
                    };
                    let disc = renderer.disc_under(x, y);
                    match chars.get(dx).filter(|ch| **ch != ' ') {
                        Some(ch) if filled(dx, *ch) => {
                            renderer.render_char(x, y, *ch, outline)?;
                            renderer.tint(x, y, toward, fill);
                        }
                        Some(ch) => renderer.render_char_over_disc(x, y, *ch, outline)?,
                        None => {
                            renderer.clear_to_sky(x, y);
                            renderer.tint(x, y, toward, fill);
                        }
                    }
                    if let Some(disc) = disc {
                        let through = if self.cover == Some(Cover::Deck) {
                            LIT_FROM_BEHIND
                        } else {
                            THIN_CLOUD_GLOW
                        };
                        renderer.lit_from_behind(x, y, disc, through);
                    }
                }
                if let Some((first, last)) = span {
                    let left = cloud.x as i32 + first as i32 - 1;
                    let right = cloud.x as i32 + last as i32 + 1;
                    let mut ring = vec![(left, y), (right, y)];
                    if self.cover != Some(Cover::Deck) {
                        let edge = if i == 0 {
                            Some(y - 1)
                        } else if i == base {
                            Some(y + 1)
                        } else {
                            None
                        };
                        if let Some(edge) = edge {
                            ring.extend((left..=right).map(|x| (x, edge)));
                        }
                    }
                    for (x, y) in ring {
                        if (0..i32::from(self.terminal_width)).contains(&x)
                            && (0..i32::from(self.terminal_height)).contains(&y)
                        {
                            renderer.clear_beside(x as u16, y as u16);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn glyph_span(chars: &[char]) -> Option<(usize, usize)> {
    let first = chars.iter().position(|ch| *ch != ' ')?;
    let last = chars.iter().rposition(|ch| *ch != ' ')?;
    Some((first, last))
}

fn width_of(cloud: &Cloud) -> f32 {
    cloud
        .shape
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as f32
}

fn lighting(sky: Color, deck: bool) -> (f32, Color, Color) {
    let Color::Rgb { r, g, b } = sky else {
        return (CLOUD_FILL, sky, sky);
    };
    let dark = 0.3 * f32::from(r) + 0.59 * f32::from(g) + 0.11 * f32::from(b) < DARK_SKY;
    let (fill, toward, outline) = match (deck, dark) {
        (true, _) => DECK_CLOUD,
        (false, true) => GREY_CLOUD,
        (false, false) => DAY_CLOUD,
    };
    let toward = Color::Rgb {
        r: toward.0,
        g: toward.1,
        b: toward.2,
    };
    (
        fill,
        toward,
        crate::render::mix((r, g, b), CLOUD_INK, outline),
    )
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

        ctx.conditions.is_cloudy
            || is_clear
            || (ctx.conditions.veils_the_sky() && !ctx.conditions.is_foggy)
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
        let cover = Cover::of(
            ctx.state
                .current_weather
                .as_ref()
                .map(|weather| weather.condition),
            ctx.conditions.veils_the_sky(),
        );
        self.update(ctx.size.width, ctx.size.height, cover, cloud_color, rng);
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
    fn a_jet_crosses_in_front_of_far_clouds_and_behind_near_ones() {
        let sky = Color::Rgb {
            r: 40,
            g: 70,
            b: 120,
        };
        for near in [false, true] {
            let mut renderer = TerminalRenderer::headless_truecolor(30, 12);
            renderer.paint_backdrop(&[sky; 12]);
            renderer.render_char(5, 4, '=', Color::White).unwrap();
            renderer.protect(5, 4);
            let shape: Vec<String> = ["    .--.", " .-(    ).", "(___.__)_)"]
                .iter()
                .map(|row| row.to_string())
                .collect();
            let system = CloudSystem {
                clouds: vec![Cloud {
                    x: 0.0,
                    y: 3.0,
                    parallax: 1.0,
                    kind: 0,
                    near,
                    solid: crate::animation::solid::solid_cells(&shape),
                    shape,
                    color: Color::Reset,
                }],
                terminal_width: 30,
                terminal_height: 12,
                drift: CALM_DRIFT,
                cover: Some(Cover::Partly),
            };
            system.render(&mut renderer).unwrap();
            let (ch, _) = renderer.cell_at(5, 4);
            assert_eq!(ch == '=', !near, "near = {near}");
        }
    }

    #[test]
    fn a_mirrored_cloud_turns_its_brackets_round() {
        let shape: Vec<String> = ["    .--.", " .-(    ).", "(___.__)_)"]
            .iter()
            .map(|row| row.to_string())
            .collect();
        assert_eq!(mirrored(&shape), ["  .--.", ".(    )-.", "(_(__.___)"]);
    }

    #[test]
    fn the_weather_sets_the_cover() {
        use WeatherCondition::*;
        assert_eq!(Cover::of(Some(Clear), false), Cover::Clear);
        assert_eq!(Cover::of(Some(PartlyCloudy), false), Cover::Partly);
        assert_eq!(Cover::of(Some(Cloudy), false), Cover::Cloudy);
        assert_eq!(Cover::of(Some(Rain), true), Cover::Deck);
        assert_eq!(Cover::of(None, false), Cover::Partly);
    }

    #[test]
    fn a_deck_fills_the_top_of_the_sky_without_overlaps() {
        for (seed, height) in (1..12).flat_map(|seed| [(seed, 32), (seed, 40)]) {
            let mut rng = StdRng::seed_from_u64(seed);
            let mut system = CloudSystem::new(120, height);
            system.update(120, height, Cover::Deck, Color::Reset, &mut rng);
            assert!(
                system.deck_closed(),
                "{seed} {height} {:?}",
                system.widest_gap()
            );
            let floor = deck_floor(height);
            while let Some(cloud) = system.clouds.pop() {
                assert!(
                    cloud.y as u16 + cloud.shape.len() as u16 <= floor,
                    "{}",
                    cloud.y
                );
                assert!(!system.crowds(&cloud), "{} {}", cloud.x, cloud.y);
            }
        }
    }

    #[test]
    fn a_clear_sky_shows_each_shape_once() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut system = CloudSystem::new(150, 40);
        system.update(150, 40, Cover::Clear, Color::Reset, &mut rng);
        let mut kinds: Vec<usize> = system.clouds.iter().map(|cloud| cloud.kind / 2).collect();
        let shown = kinds.len();
        kinds.dedup();
        kinds.sort_unstable();
        kinds.dedup();
        assert_eq!(kinds.len(), shown);
    }

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
            system.update(110, 32, Cover::Clear, Color::Reset, &mut rng);
            system.set_wind(kmh, 270.0);
            let before: Vec<f32> = system.clouds.iter().map(|cloud| cloud.x).collect();
            system.update(110, 32, Cover::Clear, Color::Reset, &mut rng);
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
