use super::bag::ShuffleBag;
use super::catalogue::{self, Colouring, Model, SLEIGH, WITCH, When};
use super::mask;
use super::settings::FlyerSettings;
use super::specials::Specials;
use crate::animation::moon::{moon_anchor, moon_spot, phase_step};
use crate::animation::sky::SkyBand;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, Wind};
use crate::events::{self, Look};
use crate::render::TerminalRenderer;
use crate::theme::colors::ElementColors;
use crossterm::style::Color;
use rand::{Rng, RngExt};
use std::io;
use std::ops::RangeInclusive;

const FIRST_FLIGHT_FRAMES: RangeInclusive<u32> = 300..=900;
const PREVIEW_GAP_FRAMES: RangeInclusive<u32> = 0..=30;
const MAX_CALM_WIND_KMH: f32 = 20.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SkyNow<'a> {
    pub width: u16,
    pub height: u16,
    pub band: SkyBand,
    pub roofline: u16,
    pub is_day: bool,
    pub halloween: bool,
    pub christmas_eve: bool,
    pub city: Option<&'a str>,
    pub real_moon: bool,
    pub moon_at: Option<u16>,
}

pub(crate) const FRAME_TICKS: u32 = 9;

const GAS_BAGS: [&str; 3] = ["blimp", "balloon", "airship"];
const ENVELOPE: (u8, u8, u8) = (196, 204, 222);
const ENVELOPE_INK: Color = Color::Rgb {
    r: 232,
    g: 236,
    b: 248,
};
const ENVELOPE_STRIPE: Color = Color::Rgb {
    r: 150,
    g: 52,
    b: 74,
};
const DAY_ENVELOPE: f32 = 0.4;
const NIGHT_ENVELOPE: f32 = 0.25;
const DARK_SKY: f32 = 40.0;
const ACCENT_SPREAD: u8 = 60;
const ACCENT_SHADE: f32 = 0.45;

fn accent_ink(colour: Color) -> Option<Color> {
    let (r, g, b) = crate::render::rgb(colour)?;
    let spread = r.max(g).max(b) - r.min(g).min(b);
    (spread >= ACCENT_SPREAD).then(|| crate::render::mix((r, g, b), (0, 0, 0), 1.0 - ACCENT_SHADE))
}

const HIGH_FLYERS: [&str; 6] = [
    "plane",
    "tiny-plane",
    "mini-plane",
    "trijet",
    "airliner",
    "fighter",
];

struct Flight {
    model: usize,
    frames: Vec<Vec<String>>,
    solids: Vec<Vec<Vec<bool>>>,
    masks: Vec<Vec<String>>,
    x: f32,
    top: u16,
    step: f32,
    tick: u32,
}

impl Flight {
    fn frame(&self) -> usize {
        (self.tick / FRAME_TICKS) as usize % self.frames.len().max(1)
    }

    fn rows(&self) -> &[String] {
        &self.frames[self.frame()]
    }

    fn mask(&self) -> Option<&[String]> {
        self.masks.get(self.frame()).map(Vec::as_slice)
    }

    fn width(&self) -> f32 {
        self.frames
            .iter()
            .flatten()
            .map(|row| row.chars().count())
            .max()
            .unwrap_or(0) as f32
    }
}

const BIG_FLYER_ROWS: u16 = 8;
const BIG_FLYER_BAND: u16 = 14;
const BIG_FLYER_WIDTH: u16 = 140;
const ABOVE_THE_CROWNS: u16 =
    crate::scene::world::trees::TALLEST_ROWS - crate::scene::world::house::House::HEIGHT + 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cell {
    pub x: u16,
    pub y: u16,
    pub ch: char,
    pub mask: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pool {
    Aircraft,
    Characters,
}

pub fn allowed(when: When, is_day: bool, wind_kmh: f32, moon_up: bool) -> bool {
    match when {
        When::Any => true,
        When::CalmDay => is_day && wind_kmh < MAX_CALM_WIND_KMH,
        When::Day => is_day,
        When::Night => !is_day,
        When::MoonNight => !is_day && moon_up,
        When::HalloweenNight | When::ChristmasEveNight => false,
    }
}

pub(crate) fn moon_row(sky: SkyNow<'_>, height: u16) -> Option<u16> {
    let moon_y = if sky.real_moon {
        sky.moon_at?
    } else {
        moon_anchor(sky.width, sky.height).1
    };
    let top = moon_y + 1;
    (top >= sky.band.top && top + height <= sky.roofline).then_some(top)
}

pub fn banner_text(halloween: bool, configured: Option<&str>, city: Option<&str>) -> String {
    let text = if halloween {
        "HAPPY HALLOWEEN".to_string()
    } else {
        configured
            .or(city)
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .unwrap_or("WEATHR")
            .to_uppercase()
    };
    let text: String = text.chars().take(24).collect();
    format!("* * * * {text} * * * *")
}

pub fn airplane_color(ch: char, tiers: &[Color; 5]) -> Color {
    match ch {
        'o' | 'O' => tiers[1],
        '=' => tiers[3],
        _ => tiers[4],
    }
}

pub fn witch_color(ch: char, tiers: &[Color; 3]) -> Color {
    match ch {
        '<' | '>' => tiers[2],
        '=' => tiers[1],
        _ => tiers[0],
    }
}

pub(crate) fn cell_colour(colouring: Colouring, cell: Cell, colors: &ElementColors) -> Color {
    match colouring {
        Colouring::Airplane => airplane_color(cell.ch, &colors.airplane),
        Colouring::Witch => witch_color(cell.ch, &colors.witch),
        Colouring::Mask => cell
            .mask
            .and_then(|letter| mask::colour(letter, &colors.palette))
            .unwrap_or(colors.palette[0]),
    }
}

fn sky_now<'a>(ctx: &FrameContext<'a>, moon_phase: f64) -> SkyNow<'a> {
    let moon_at = moon_spot(
        ctx.state.moon_sky,
        moon_phase,
        ctx.state.location.latitude < 0.0,
        (ctx.size.width, ctx.size.height, ctx.horizon_y),
    )
    .map(|spot| spot.y);
    SkyNow {
        real_moon: ctx.state.moon_sky.is_some(),
        moon_at,
        width: ctx.size.width,
        height: ctx.size.height,
        band: SkyBand::below_hud(ctx.skyline_y.saturating_sub(ABOVE_THE_CROWNS)),
        roofline: ctx.skyline_y,
        is_day: ctx.conditions.sun.is_day,
        halloween: ctx.events.has(Look::Halloween),
        christmas_eve: ctx.events.has(Look::Christmas)
            && events::is_christmas_eve_night(ctx.events.now()),
        city: if ctx.state.hide_location {
            None
        } else {
            ctx.state.city_name.as_deref()
        },
    }
}

pub struct FlyerSystem {
    models: Vec<Model>,
    settings: FlyerSettings,
    bag: ShuffleBag,
    characters: ShuffleBag,
    specials: Specials,
    flight: Option<Flight>,
    frames_until_flight: Option<u32>,
    wind_kmh: f32,
    moon_phase: f64,
}

impl FlyerSystem {
    pub fn new(settings: FlyerSettings) -> Self {
        let specials = Specials::new(settings.specials);
        Self {
            models: catalogue::models(),
            settings,
            bag: ShuffleBag::new(),
            characters: ShuffleBag::new(),
            specials,
            flight: None,
            frames_until_flight: None,
            wind_kmh: 0.0,
            moon_phase: 0.5,
        }
    }

    fn index(&self, id: &str) -> Option<usize> {
        self.models.iter().position(|model| model.def.id == id)
    }

    fn gap(&self, rng: &mut (impl Rng + ?Sized)) -> u32 {
        if self.settings.preview.is_some() {
            rng.random_range(PREVIEW_GAP_FRAMES)
        } else {
            rng.random_range(self.settings.every.gap_frames())
        }
    }

    fn first_wait(&self, rng: &mut (impl Rng + ?Sized)) -> u32 {
        if self.settings.preview.is_some() {
            rng.random_range(PREVIEW_GAP_FRAMES)
        } else {
            rng.random_range(FIRST_FLIGHT_FRAMES)
        }
    }

    pub(crate) fn advance(&mut self, sky: SkyNow<'_>, rng: &mut (impl Rng + ?Sized)) {
        if let Some(flight) = self.flight.as_mut() {
            flight.x += flight.step;
            flight.tick += 1;
            let width = flight.width();
            let gone = if flight.step > 0.0 {
                flight.x >= sky.width as f32
            } else {
                flight.x + width <= 0.0
            };
            if gone {
                self.flight = None;
                self.frames_until_flight = Some(self.gap(rng));
            }
            return;
        }
        if self.frames_until_flight.is_none() {
            self.frames_until_flight = Some(self.first_wait(rng));
        }
        if let Some(wait) = self.frames_until_flight.as_mut() {
            if *wait > 0 {
                *wait -= 1;
                return;
            }
        }
        match self.pick(sky, rng) {
            Some(index) => self.launch(index, sky, rng),
            None => self.frames_until_flight = Some(self.gap(rng)),
        }
    }

    fn pick(&mut self, sky: SkyNow<'_>, rng: &mut (impl Rng + ?Sized)) -> Option<usize> {
        if sky.band.height() == 0 {
            return None;
        }
        let fits = |model: &Model| model.height() > 0;
        if let Some(id) = self.settings.preview {
            return self.index(id).filter(|&index| fits(&self.models[index]));
        }
        if sky.halloween && !sky.is_day {
            return self.index(WITCH).filter(|&index| fits(&self.models[index]));
        }
        if sky.christmas_eve && !sky.is_day {
            return self
                .index(SLEIGH)
                .filter(|&index| fits(&self.models[index]));
        }
        let special = if self.specials.due(rng) {
            self.draw_from(Pool::Characters, sky, rng)
        } else {
            None
        };
        if let Some(index) = special {
            self.specials.character_flew(rng);
            return Some(index);
        }
        let index = self.draw_from(Pool::Aircraft, sky, rng)?;
        self.specials.aircraft_flew();
        Some(index)
    }

    fn draw_from(
        &mut self,
        pool: Pool,
        sky: SkyNow<'_>,
        rng: &mut (impl Rng + ?Sized),
    ) -> Option<usize> {
        let models = &self.models;
        let wind = self.wind_kmh;
        let moon_up = !self.settings.moon_hidden
            && if sky.real_moon {
                sky.moon_at.is_some()
            } else {
                phase_step(self.moon_phase) != 0
            };
        let eligible = |id: &str| {
            models.iter().any(|model| {
                model.def.id == id
                    && model.height() > 0
                    && (model.height() < BIG_FLYER_ROWS
                        || (sky.band.height() >= BIG_FLYER_BAND && sky.width >= BIG_FLYER_WIDTH))
                    && allowed(model.def.when, sky.is_day, wind, moon_up)
                    && (model.def.when != When::MoonNight
                        || moon_row(sky, model.height()).is_some())
            })
        };
        let id = match pool {
            Pool::Aircraft => self.bag.next(&self.settings.enabled, rng, eligible),
            Pool::Characters => self
                .characters
                .next(&self.settings.characters, rng, eligible),
        }?;
        self.index(id)
    }

    fn launch(&mut self, index: usize, sky: SkyNow<'_>, rng: &mut (impl Rng + ?Sized)) {
        let model = &self.models[index];
        let rightward = match (model.right.is_empty(), model.left.is_empty()) {
            (false, false) => rng.random_bool(0.5),
            (false, true) => true,
            _ => false,
        };
        let variants = if rightward { &model.right } else { &model.left };
        let variant = &variants[rng.random_range(0..variants.len())];
        let speed: f32 = rng.random_range(model.def.pace.columns_per_frame());
        let height = variant.height();
        let top = match moon_row(sky, height).filter(|_| model.def.when == When::MoonNight) {
            Some(row) => row,
            None if height <= sky.band.height() => {
                let room = sky.band.height() - height;
                sky.band.top + rng.random_range(0..=room).min(rng.random_range(0..=room))
            }
            None => sky.band.top,
        };
        let text = banner_text(sky.halloween, self.settings.banner.as_deref(), sky.city);
        let frames: Vec<Vec<String>> = variant
            .frames
            .iter()
            .map(|frame| {
                frame
                    .rows
                    .iter()
                    .map(|row| row.replace("{text}", &text))
                    .collect()
            })
            .collect();
        let masks: Vec<Vec<String>> = variant.masks.iter().map(|mask| mask.rows.clone()).collect();
        let solids = frames
            .iter()
            .map(|rows| crate::animation::solid::solid_cells(rows))
            .collect();
        let mut flight = Flight {
            model: index,
            frames,
            solids,
            masks,
            x: 0.0,
            top,
            step: if rightward { speed } else { -speed },
            tick: 0,
        };
        flight.x = if rightward {
            -flight.width()
        } else {
            sky.width as f32
        };
        self.flight = Some(flight);
    }

    pub(crate) fn blanks(&self, sky: SkyNow<'_>) -> Vec<(u16, u16)> {
        let Some(flight) = &self.flight else {
            return Vec::new();
        };
        let Some(solid) = flight.solids.get(flight.frame()) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for (dy, (row, inside)) in flight.rows().iter().zip(solid).enumerate() {
            let chars: Vec<char> = row.chars().collect();
            for (dx, solid) in inside.iter().enumerate() {
                let x = flight.x.floor() as i32 + dx as i32;
                if *solid
                    && chars.get(dx).is_none_or(|ch| *ch == ' ')
                    && (0..i32::from(sky.width)).contains(&x)
                {
                    out.push((x as u16, flight.top + dy as u16));
                }
            }
        }
        out
    }

    pub(crate) fn cells(&self, sky: SkyNow<'_>) -> Vec<Cell> {
        let Some(flight) = &self.flight else {
            return Vec::new();
        };
        let mask = flight.mask();
        let mut cells = Vec::new();
        for (dy, row) in flight.rows().iter().enumerate() {
            let y = flight.top + dy as u16;
            let letters: Vec<char> = mask
                .and_then(|rows| rows.get(dy))
                .map(|row| row.chars().collect())
                .unwrap_or_default();
            for (dx, ch) in row.chars().enumerate() {
                if ch == ' ' {
                    continue;
                }
                let x = flight.x.floor() as i32 + dx as i32;
                if x < 0 || x >= sky.width as i32 {
                    continue;
                }
                cells.push(Cell {
                    x: x as u16,
                    y,
                    ch,
                    mask: letters.get(dx).copied(),
                });
            }
        }
        cells
    }
}

impl AnimationSystem for FlyerSystem {
    fn id(&self) -> &'static str {
        "airplanes"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Background
    }

    fn drawn_before(&self) -> Option<&'static str> {
        let flight = self.flight.as_ref()?;
        HIGH_FLYERS
            .contains(&self.models[flight.model].def.id)
            .then_some("clouds")
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        !ctx.conditions.is_raining
            && !ctx.conditions.is_thunderstorm
            && !ctx.conditions.is_snowing
            && !ctx.conditions.is_foggy
    }

    fn on_wind(&mut self, wind: Wind) {
        self.wind_kmh = wind.speed_kmh;
    }

    fn on_moon_phase(&mut self, phase: f64) {
        self.moon_phase = phase;
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.advance(sky_now(ctx, self.moon_phase), rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        let Some(flight) = &self.flight else {
            return Ok(());
        };
        let colouring = self.models[flight.model].def.colouring;
        let sky = sky_now(ctx, self.moon_phase);
        let blanks = self.blanks(sky);
        for &(x, y) in &blanks {
            renderer.clear_to_sky(x, y);
        }
        let cells = self.cells(sky);
        if !blanks.is_empty() {
            let mut ends: std::collections::BTreeMap<u16, (u16, u16)> = Default::default();
            for cell in &cells {
                let span = ends.entry(cell.y).or_insert((cell.x, cell.x));
                *span = (span.0.min(cell.x), span.1.max(cell.x));
            }
            for (y, (left, right)) in ends {
                for x in left.saturating_sub(1)..=right.saturating_add(1) {
                    renderer.clear_beside(x, y);
                }
            }
        }
        for cell in cells {
            let colour = cell_colour(colouring, cell, ctx.colors);
            if blanks.is_empty() {
                renderer.render_char_over_disc_in(
                    cell.x,
                    cell.y,
                    cell.ch,
                    colour,
                    accent_ink(colour),
                )?;
            } else {
                renderer.clear_to_sky(cell.x, cell.y);
                renderer.render_char(cell.x, cell.y, cell.ch, colour)?;
            }
        }
        if self.drawn_before().is_some() {
            for &(x, y) in &blanks {
                renderer.protect(x, y);
            }
            for cell in self.cells(sky) {
                renderer.protect(cell.x, cell.y);
            }
        }
        if GAS_BAGS.contains(&self.models[flight.model].def.id) {
            self.paint_envelope(renderer, ctx.colors.airplane, sky.width)?;
        }
        Ok(())
    }
}

impl FlyerSystem {
    fn paint_envelope(
        &self,
        renderer: &mut TerminalRenderer,
        tiers: [Color; 5],
        width: u16,
    ) -> io::Result<()> {
        let Some(flight) = &self.flight else {
            return Ok(());
        };
        let Some(solid) = flight.solids.get(flight.frame()) else {
            return Ok(());
        };
        let bag = super::envelope::envelope(flight.rows(), solid);
        for (dy, (row, span)) in flight.rows().iter().zip(&bag.spans).enumerate() {
            let y = flight.top + dy as u16;
            let (Some((first, last)), Some(sky)) =
                (span, renderer.sky_row(y).and_then(crate::render::rgb))
            else {
                continue;
            };
            let night = 0.3 * f32::from(sky.0) + 0.59 * f32::from(sky.1) + 0.11 * f32::from(sky.2)
                < DARK_SKY;
            let background = if bag.stripe == Some(dy) {
                ENVELOPE_STRIPE
            } else {
                crate::render::mix(
                    sky,
                    ENVELOPE,
                    if night { NIGHT_ENVELOPE } else { DAY_ENVELOPE },
                )
            };
            let chars: Vec<char> = row.chars().collect();
            for column in *first..=*last {
                let x = flight.x.floor() as i32 + column as i32;
                if !(0..i32::from(width)).contains(&x) {
                    continue;
                }
                let ch = chars.get(column).copied().unwrap_or(' ');
                let ink = match ch {
                    'o' | 'O' => tiers[1],
                    '=' => tiers[3],
                    _ => ENVELOPE_INK,
                };
                renderer.render_char_on(x as u16, y, ch, ink, background)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::flyers::catalogue::{Fill, FlyerDef, Kind, Pace, Sprite, Variant};
    use crate::animation::flyers::settings::Every;
    use crate::animation::halloween::test_support::Fixture;
    use crate::animation::moon::moon_anchor;
    use crate::weather::WeatherCondition;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn sky(is_day: bool, halloween: bool) -> SkyNow<'static> {
        SkyNow {
            width: 110,
            height: 32,
            band: SkyBand::below_hud(15),
            roofline: (15).max(crate::animation::sky::SKY_TOP),
            is_day,
            halloween,
            christmas_eve: false,
            city: None,
            real_moon: false,
            moon_at: None,
        }
    }

    fn frames_until_launch(system: &mut FlyerSystem, sky: SkyNow<'_>, rng: &mut StdRng) -> usize {
        for frame in 0..20_000 {
            if system.flight.is_some() {
                return frame;
            }
            system.advance(sky, rng);
        }
        panic!("nothing flew");
    }

    fn launched_id(system: &FlyerSystem) -> &'static str {
        system.models[system.flight.as_ref().expect("a flight").model]
            .def
            .id
    }

    #[test]
    fn airplane_parts_map_to_tiers() {
        let tiers = [
            Color::AnsiValue(0),
            Color::AnsiValue(1),
            Color::AnsiValue(2),
            Color::AnsiValue(3),
            Color::AnsiValue(4),
        ];
        assert_eq!(
            airplane_color('o', &tiers),
            tiers[1],
            "windows in the accent"
        );
        assert_eq!(airplane_color('O', &tiers), tiers[1]);
        assert_eq!(airplane_color('=', &tiers), tiers[3], "dim exhaust");
        for body in ['_', '\\', '"', '~', '|', '/'] {
            assert_eq!(
                airplane_color(body, &tiers),
                tiers[4],
                "one body ink: {body}"
            );
        }
    }

    #[test]
    fn witch_parts_map_to_tiers() {
        let tiers = [
            Color::AnsiValue(0),
            Color::AnsiValue(1),
            Color::AnsiValue(2),
        ];
        assert_eq!(witch_color('{', &tiers), tiers[0]);
        assert_eq!(witch_color('=', &tiers), tiers[1]);
        assert_eq!(witch_color('<', &tiers), tiers[2]);
        assert_eq!(witch_color('>', &tiers), tiers[2]);
    }

    #[test]
    fn when_rules() {
        assert!(allowed(When::Any, false, 50.0, false));
        assert!(allowed(When::CalmDay, true, 10.0, false));
        assert!(!allowed(When::CalmDay, true, 25.0, false));
        assert!(!allowed(When::CalmDay, false, 5.0, false));
        assert!(!allowed(When::HalloweenNight, false, 0.0, false));
    }

    #[test]
    fn christmas_eve_night_sends_the_sleigh() {
        let mut rng = StdRng::seed_from_u64(24);
        let eve = SkyNow {
            christmas_eve: true,
            ..sky(false, false)
        };
        let mut system = FlyerSystem::new(FlyerSettings {
            every: Every::Busy,
            ..FlyerSettings::default()
        });
        for _ in 0..3 {
            frames_until_launch(&mut system, eve, &mut rng);
            assert_eq!(launched_id(&system), catalogue::SLEIGH);
            land(&mut system, eve, &mut rng);
        }
        let morning = SkyNow {
            christmas_eve: true,
            ..sky(true, false)
        };
        frames_until_launch(&mut system, morning, &mut rng);
        assert_ne!(launched_id(&system), catalogue::SLEIGH);
    }

    #[test]
    fn halloween_nights_send_the_witch_and_leave_the_cycle_alone() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut system = FlyerSystem::new(FlyerSettings::default());
        frames_until_launch(&mut system, sky(false, true), &mut rng);
        assert_eq!(launched_id(&system), WITCH);
        assert!(system.bag.is_empty());

        let mut system = FlyerSystem::new(FlyerSettings::default());
        frames_until_launch(&mut system, sky(true, true), &mut rng);
        assert_ne!(launched_id(&system), WITCH);

        let mut system = FlyerSystem::new(FlyerSettings::default());
        frames_until_launch(&mut system, sky(false, false), &mut rng);
        assert_ne!(launched_id(&system), WITCH);
    }

    #[test]
    fn preview_flies_the_chosen_flyer_quickly() {
        let mut rng = StdRng::seed_from_u64(2);
        let settings = FlyerSettings {
            preview: Some(WITCH),
            ..FlyerSettings::default()
        };
        let mut system = FlyerSystem::new(settings);
        let frames = frames_until_launch(&mut system, sky(true, false), &mut rng);
        assert!(frames <= 31, "{frames}");
        assert_eq!(launched_id(&system), WITCH);
    }

    #[test]
    fn first_flight_waits_ten_to_thirty_seconds() {
        for seed in 0..10 {
            let mut rng = StdRng::seed_from_u64(seed);
            let mut system = FlyerSystem::new(FlyerSettings::default());
            let frames = frames_until_launch(&mut system, sky(true, false), &mut rng);
            assert!((300..=901).contains(&frames), "seed {seed}: {frames}");
        }
    }

    #[test]
    fn calm_gap_after_a_flight_is_two_to_three_minutes() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut system = FlyerSystem::new(FlyerSettings::default());
        let now = sky(true, false);
        frames_until_launch(&mut system, now, &mut rng);
        let mut frames = 0;
        while system.flight.is_some() {
            system.advance(now, &mut rng);
            frames += 1;
            assert!(frames < 2000, "the flight never left");
        }
        let gap = frames_until_launch(&mut system, now, &mut rng);
        assert!((3600..=5401).contains(&gap), "{gap}");
    }

    #[test]
    fn flights_enter_from_off_screen_and_stay_in_the_band() {
        for seed in 0..20 {
            let mut rng = StdRng::seed_from_u64(seed);
            let mut system = FlyerSystem::new(FlyerSettings {
                every: crate::animation::flyers::settings::Every::Busy,
                ..FlyerSettings::default()
            });
            let now = sky(true, false);
            frames_until_launch(&mut system, now, &mut rng);
            let flight = system.flight.as_ref().unwrap();
            let width = flight.width();
            if flight.step > 0.0 {
                assert_eq!(flight.x, -width);
            } else {
                assert_eq!(flight.x, 110.0);
            }
            assert!(system.models[flight.model].height() <= now.band.height());
            for _ in 0..3000 {
                for cell in system.cells(now) {
                    assert!(cell.x < 110, "column {}", cell.x);
                    assert!((3..15).contains(&cell.y), "row {}", cell.y);
                }
                system.advance(now, &mut rng);
            }
        }
    }

    #[test]
    fn tall_flyers_still_fly_on_short_panes() {
        let mut rng = StdRng::seed_from_u64(5);
        let short = SkyNow {
            band: SkyBand::below_hud(7),
            roofline: (7).max(crate::animation::sky::SKY_TOP),
            ..sky(true, false)
        };
        let mut system = FlyerSystem::new(FlyerSettings::default());
        frames_until_launch(&mut system, short, &mut rng);
        let flight = system.flight.as_ref().unwrap();
        assert!(system.models[flight.model].height() > short.band.height());
        assert_eq!(flight.top, 3);

        let no_sky = SkyNow {
            band: SkyBand::below_hud(3),
            roofline: (3).max(crate::animation::sky::SKY_TOP),
            ..sky(true, false)
        };
        let mut system = FlyerSystem::new(FlyerSettings::default());
        for _ in 0..5000 {
            system.advance(no_sky, &mut rng);
        }
        assert!(system.flight.is_none());
    }

    #[test]
    fn flyers_stay_grounded_in_wet_weather() {
        let system = FlyerSystem::new(FlyerSettings::default());
        for condition in [
            WeatherCondition::Rain,
            WeatherCondition::Thunderstorm,
            WeatherCondition::Snow,
            WeatherCondition::Fog,
        ] {
            assert!(
                !system.is_active(&Fixture::new(condition, true).ctx()),
                "{condition:?}"
            );
        }
        assert!(system.is_active(&Fixture::new(WeatherCondition::Clear, true).ctx()));
    }

    #[test]
    fn wind_feeds_the_calm_rule() {
        let mut system = FlyerSystem::new(FlyerSettings::default());
        system.on_wind(Wind {
            speed_kmh: 30.0,
            direction_deg: 90.0,
        });
        assert_eq!(system.wind_kmh, 30.0);
    }

    #[test]
    fn banner_text_follows_halloween_config_and_city() {
        assert_eq!(
            banner_text(true, Some("ignored"), Some("Jakarta")),
            "* * * * HAPPY HALLOWEEN * * * *"
        );
        assert_eq!(
            banner_text(false, Some("Selamat pagi"), Some("Jakarta")),
            "* * * * SELAMAT PAGI * * * *"
        );
        assert_eq!(
            banner_text(false, None, Some("Jakarta")),
            "* * * * JAKARTA * * * *"
        );
        assert_eq!(banner_text(false, None, None), "* * * * WEATHR * * * *");
        assert_eq!(
            banner_text(false, Some("abcdefghijklmnopqrstuvwxyz"), None),
            "* * * * ABCDEFGHIJKLMNOPQRSTUVWX * * * *"
        );
    }

    #[test]
    fn a_hidden_location_keeps_the_city_off_the_banner() {
        let mut fixture = Fixture::new(WeatherCondition::Clear, true);
        fixture.state.city_name = Some("Jakarta".to_string());
        assert_eq!(sky_now(&fixture.ctx(), 0.5).city, Some("Jakarta"));
        fixture.state.hide_location = true;
        assert_eq!(sky_now(&fixture.ctx(), 0.5).city, None);
    }

    #[test]
    fn calm_day_drifters_only_fly_by_day_in_light_wind() {
        let settings = FlyerSettings {
            enabled: vec!["balloon"],
            every: crate::animation::flyers::settings::Every::Busy,
            ..FlyerSettings::default()
        };
        let mut rng = StdRng::seed_from_u64(4);
        let mut system = FlyerSystem::new(settings.clone());
        for _ in 0..5000 {
            system.advance(sky(false, false), &mut rng);
        }
        assert!(system.flight.is_none());
        let mut system = FlyerSystem::new(settings);
        system.wind_kmh = 10.0;
        frames_until_launch(&mut system, sky(true, false), &mut rng);
        assert_eq!(launched_id(&system), "balloon");
    }

    static BLINK: FlyerDef = FlyerDef {
        id: "blink",
        credit: "test",
        right: None,
        left: None,
        fill: Fill::None,
        pace: Pace::Slow,
        when: When::Any,
        colouring: Colouring::Mask,
        kind: Kind::Character,
        mask: None,
    };

    fn sprite(row: &str) -> Sprite {
        Sprite {
            rows: vec![row.to_string()],
        }
    }

    #[test]
    fn art_frames_advance_every_nine_ticks_and_wrap() {
        let mut system = FlyerSystem::new(FlyerSettings {
            preview: Some("blink"),
            ..FlyerSettings::default()
        });
        system.models.push(Model {
            def: &BLINK,
            right: vec![Variant {
                frames: vec![sprite("a"), sprite("b")],
                masks: vec![sprite("r"), sprite("g")],
            }],
            left: Vec::new(),
        });
        let mut rng = StdRng::seed_from_u64(6);
        let now = sky(true, false);
        frames_until_launch(&mut system, now, &mut rng);
        let mut seen = String::new();
        for _ in 0..36 {
            seen.push_str(&system.flight.as_ref().unwrap().rows()[0]);
            system.advance(now, &mut rng);
        }
        assert_eq!(seen, format!("{0}{1}{0}{1}", "a".repeat(9), "b".repeat(9)));
        let flight = system.flight.as_ref().unwrap();
        assert_eq!(
            system.cells(now),
            vec![Cell {
                x: flight.x.floor() as u16,
                y: flight.top,
                ch: 'a',
                mask: Some('r'),
            }]
        );
        for _ in 0..9 {
            system.advance(now, &mut rng);
        }
        let flight = system.flight.as_ref().unwrap();
        assert_eq!(
            system.cells(now),
            vec![Cell {
                x: flight.x.floor() as u16,
                y: flight.top,
                ch: 'b',
                mask: Some('g'),
            }]
        );
    }

    #[test]
    fn the_moon_biker_stays_home_when_the_moon_row_is_off_the_sky() {
        let cramped = SkyNow {
            width: 80,
            height: 24,
            band: SkyBand::below_hud(7),
            roofline: (7).max(crate::animation::sky::SKY_TOP),
            is_day: false,
            halloween: false,
            christmas_eve: false,
            city: None,
            real_moon: false,
            moon_at: None,
        };
        assert_eq!(moon_row(cramped, 3), None);
        let mut system = FlyerSystem::new(FlyerSettings {
            every: Every::Busy,
            specials: 1,
            characters: vec!["moon-biker"],
            ..FlyerSettings::default()
        });
        let mut rng = StdRng::seed_from_u64(21);
        for _ in 0..3 {
            frames_until_launch(&mut system, cramped, &mut rng);
            assert_eq!(kind_of(&system), Kind::Aircraft);
            land(&mut system, cramped, &mut rng);
        }
    }

    #[test]
    fn masked_cells_take_their_palette_colour() {
        let mut colors = crate::theme::catalogue::default_colors().day;
        colors.palette[3] = Color::AnsiValue(203);
        let cell = Cell {
            x: 0,
            y: 0,
            ch: 'S',
            mask: Some('r'),
        };
        assert_eq!(
            cell_colour(Colouring::Mask, cell, &colors),
            Color::AnsiValue(203)
        );
        assert_eq!(
            cell_colour(
                Colouring::Mask,
                Cell {
                    mask: Some('?'),
                    ..cell
                },
                &colors
            ),
            colors.palette[0]
        );
        assert_eq!(
            cell_colour(Colouring::Mask, Cell { mask: None, ..cell }, &colors),
            colors.palette[0]
        );
        assert_eq!(
            cell_colour(Colouring::Airplane, cell, &colors),
            colors.airplane[4]
        );
        assert_eq!(
            cell_colour(Colouring::Witch, Cell { ch: '=', ..cell }, &colors),
            colors.witch[1]
        );
    }

    fn land(system: &mut FlyerSystem, sky: SkyNow<'_>, rng: &mut StdRng) {
        for _ in 0..20_000 {
            if system.flight.is_none() {
                return;
            }
            system.advance(sky, rng);
        }
        panic!("the flight never left");
    }

    fn kind_of(system: &FlyerSystem) -> Kind {
        system.models[system.flight.as_ref().expect("a flight").model]
            .def
            .kind
    }

    #[test]
    fn day_and_night_rules() {
        assert!(allowed(When::Day, true, 0.0, false));
        assert!(!allowed(When::Day, false, 0.0, false));
        assert!(allowed(When::Night, false, 0.0, false));
        assert!(!allowed(When::Night, true, 0.0, false));
    }

    #[test]
    fn specials_of_one_send_a_character_every_flight() {
        let mut rng = StdRng::seed_from_u64(12);
        let mut system = FlyerSystem::new(FlyerSettings {
            every: Every::Busy,
            specials: 1,
            ..FlyerSettings::default()
        });
        let day = sky(true, false);
        for _ in 0..3 {
            frames_until_launch(&mut system, day, &mut rng);
            assert_eq!(kind_of(&system), Kind::Character);
            land(&mut system, day, &mut rng);
        }
    }

    #[test]
    fn about_one_flight_in_twelve_is_a_character() {
        let day = sky(true, false);
        for seed in 100..105 {
            let mut rng = StdRng::seed_from_u64(seed);
            let mut system = FlyerSystem::new(FlyerSettings {
                every: Every::Busy,
                ..FlyerSettings::default()
            });
            let mut characters = Vec::new();
            for flight in 1..=50 {
                frames_until_launch(&mut system, day, &mut rng);
                if kind_of(&system) == Kind::Character {
                    characters.push(flight);
                }
                land(&mut system, day, &mut rng);
            }
            assert!(characters.len() >= 3, "{seed}: {characters:?}");
            assert!((9..=15).contains(&characters[0]), "{seed}: {characters:?}");
            for pair in characters.windows(2) {
                assert!(
                    (9..=15).contains(&(pair[1] - pair[0])),
                    "{seed}: {characters:?}"
                );
            }
        }
    }

    #[test]
    fn a_special_waits_for_an_eligible_character() {
        let mut rng = StdRng::seed_from_u64(13);
        let mut system = FlyerSystem::new(FlyerSettings {
            every: Every::Busy,
            specials: 1,
            characters: vec!["ufo"],
            ..FlyerSettings::default()
        });
        let day = sky(true, false);
        for _ in 0..3 {
            frames_until_launch(&mut system, day, &mut rng);
            assert_eq!(kind_of(&system), Kind::Aircraft);
            land(&mut system, day, &mut rng);
        }
        frames_until_launch(&mut system, sky(false, false), &mut rng);
        assert_eq!(launched_id(&system), "ufo");
    }

    #[test]
    fn zero_specials_fly_aircraft_only() {
        let mut rng = StdRng::seed_from_u64(14);
        let mut system = FlyerSystem::new(FlyerSettings {
            every: Every::Busy,
            specials: 0,
            ..FlyerSettings::default()
        });
        let night = sky(false, false);
        for _ in 0..30 {
            frames_until_launch(&mut system, night, &mut rng);
            assert_eq!(kind_of(&system), Kind::Aircraft);
            land(&mut system, night, &mut rng);
        }
    }

    #[test]
    fn halloween_nights_hold_the_countdown() {
        let mut rng = StdRng::seed_from_u64(15);
        let mut system = FlyerSystem::new(FlyerSettings {
            every: Every::Busy,
            specials: 1,
            ..FlyerSettings::default()
        });
        let halloween = sky(false, true);
        for _ in 0..3 {
            frames_until_launch(&mut system, halloween, &mut rng);
            assert_eq!(launched_id(&system), WITCH);
            land(&mut system, halloween, &mut rng);
        }
        assert!(system.characters.is_empty());
        frames_until_launch(&mut system, sky(false, false), &mut rng);
        assert_eq!(kind_of(&system), Kind::Character);
    }

    #[test]
    fn moonlit_nights_need_the_moon() {
        assert!(allowed(When::MoonNight, false, 0.0, true));
        assert!(!allowed(When::MoonNight, false, 0.0, false));
        assert!(!allowed(When::MoonNight, true, 0.0, true));
    }

    #[test]
    fn moon_biker_rides_the_moon_row() {
        for (width, height) in [(110u16, 32u16), (140, 40), (90, 30)] {
            let now = SkyNow {
                width,
                height,
                band: SkyBand::below_hud(height - 17),
                roofline: (height - 17).max(crate::animation::sky::SKY_TOP),
                is_day: false,
                halloween: false,
                christmas_eve: false,
                city: None,
                real_moon: false,
                moon_at: None,
            };
            let mut system = FlyerSystem::new(FlyerSettings {
                preview: Some("moon-biker"),
                ..FlyerSettings::default()
            });
            let mut rng = StdRng::seed_from_u64(u64::from(width));
            frames_until_launch(&mut system, now, &mut rng);
            let (_, moon_y) = moon_anchor(width, height);
            assert_eq!(
                system.flight.as_ref().unwrap().top + 1,
                moon_y + 2,
                "{width}x{height}"
            );
        }
    }

    #[test]
    fn moon_biker_needs_night_and_a_visible_moon_that_is_not_new() {
        let settings = FlyerSettings {
            every: Every::Busy,
            specials: 1,
            characters: vec!["moon-biker"],
            ..FlyerSettings::default()
        };
        let night = sky(false, false);
        let mut rng = StdRng::seed_from_u64(16);
        let mut system = FlyerSystem::new(settings.clone());
        assert_eq!(system.moon_phase, 0.5);
        frames_until_launch(&mut system, night, &mut rng);
        assert_eq!(launched_id(&system), "moon-biker");

        let mut new_moon = FlyerSystem::new(settings.clone());
        new_moon.on_moon_phase(0.0);
        let mut hidden = FlyerSystem::new(FlyerSettings {
            moon_hidden: true,
            ..settings.clone()
        });
        let mut daytime = FlyerSystem::new(settings);
        for (system, now) in [
            (&mut new_moon, night),
            (&mut hidden, night),
            (&mut daytime, sky(true, false)),
        ] {
            for _ in 0..3 {
                frames_until_launch(system, now, &mut rng);
                assert_eq!(kind_of(system), Kind::Aircraft);
                land(system, now, &mut rng);
            }
        }
    }

    #[test]
    fn the_moon_biker_follows_the_real_moon() {
        let settings = FlyerSettings {
            every: Every::Busy,
            specials: 1,
            characters: vec!["moon-biker"],
            ..FlyerSettings::default()
        };
        let mut rng = StdRng::seed_from_u64(31);
        let moon_down = SkyNow {
            real_moon: true,
            moon_at: None,
            ..sky(false, false)
        };
        let mut system = FlyerSystem::new(settings.clone());
        for _ in 0..3 {
            frames_until_launch(&mut system, moon_down, &mut rng);
            assert_eq!(kind_of(&system), Kind::Aircraft);
            land(&mut system, moon_down, &mut rng);
        }
        let moon_up = SkyNow {
            real_moon: true,
            moon_at: Some(5),
            ..sky(false, false)
        };
        let mut system = FlyerSystem::new(FlyerSettings {
            preview: Some("moon-biker"),
            ..settings
        });
        frames_until_launch(&mut system, moon_up, &mut rng);
        assert_eq!(system.flight.as_ref().unwrap().top, 6);
    }
}
