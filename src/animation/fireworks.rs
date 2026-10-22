use crate::animation::flyers::mask;
use crate::animation::sky::SkyBand;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::{ActiveEvents, Look};
use crate::render::TerminalRenderer;
use chrono::{Datelike, Timelike};
use rand::{Rng, RngExt};
use std::io;
use std::ops::RangeInclusive;

pub const FIREWORKS: &str = "fireworks";
pub const CREDIT: &str = "bursts after jgs's Sparks and fireworks cluster";

const SHAPES: [&str; 4] = [
    " .''.\n:_\\/_:\n: /\\ :\n '..'",
    " *''*\n*_\\/_*\n* /\\ *\n *..*",
    ".'.:.'.\n-=:o:=-\n'.':'.'",
    "    .\n  \\   /\n-=  o  =-\n  /   \\\n    '",
];
const COLOURS: [char; 7] = ['r', 'o', 'y', 'g', 'c', 'b', 'm'];
const SPARKS: [char; 3] = ['*', '+', '.'];
const RISE_FRAMES: u32 = 20;
const BURST_FRAMES: u32 = 30;
const FADE_FRAMES: u32 = 20;
const SPARKLE_FRAMES: u32 = 10;
const TALLEST_SHAPE: i32 = 5;
const SPARKLE_CHANCE: f64 = 0.15;
const SHOW_GAP: RangeInclusive<u32> = 15..=60;
const FADING_GAP: RangeInclusive<u32> = 90..=240;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Show {
    Off,
    Sparkles,
    Bursts,
    Fading,
}

pub(crate) fn show_for(events: &ActiveEvents) -> Show {
    let now = events.now();
    if events.has(Look::NewYear) {
        return match (now.month(), now.hour()) {
            (12, 20..=22) => Show::Sparkles,
            (12, 23) | (1, 0) => Show::Bursts,
            (1, 1) => Show::Fading,
            _ => Show::Off,
        };
    }
    let hours_in = |look| events.since_start_of(look).map(|since| since.num_hours());
    let evening = (20..=23).contains(&now.hour());
    match events
        .since_lunar_new_year_eve()
        .map(|since| since.num_hours())
    {
        Some(2..=4) => return Show::Sparkles,
        Some(5..=6) => return Show::Fading,
        _ => {}
    }
    if events.has(Look::Diwali) && (19..=23).contains(&now.hour()) {
        return Show::Bursts;
    }
    if evening && matches!(hours_in(Look::Fireworks), Some(0..=29)) {
        return Show::Bursts;
    }
    if matches!(hours_in(Look::Takbiran), Some(1..=5)) {
        return Show::Sparkles;
    }
    Show::Off
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Spark {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub colour: char,
}

#[derive(Debug, Clone, PartialEq)]
struct Burst {
    x: i32,
    top: i32,
    floor: i32,
    shape: usize,
    colour: char,
    age: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Twinkle {
    x: i32,
    y: i32,
    ch: char,
    colour: char,
    age: u32,
}

pub struct FireworksSystem {
    bursts: Vec<Burst>,
    twinkles: Vec<Twinkle>,
    frames_until_burst: u32,
}

impl Default for FireworksSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FireworksSystem {
    pub fn new() -> Self {
        Self {
            bursts: Vec::new(),
            twinkles: Vec::new(),
            frames_until_burst: 0,
        }
    }

    pub(crate) fn advance(
        &mut self,
        show: Show,
        width: u16,
        band: SkyBand,
        rng: &mut (impl Rng + ?Sized),
    ) {
        for burst in &mut self.bursts {
            burst.age += 1;
        }
        self.bursts
            .retain(|burst| burst.age < RISE_FRAMES + BURST_FRAMES + FADE_FRAMES);
        for twinkle in &mut self.twinkles {
            twinkle.age += 1;
        }
        self.twinkles.retain(|twinkle| twinkle.age < SPARKLE_FRAMES);
        if band.height() < 4 || width <= 12 {
            return;
        }
        match show {
            Show::Off => {}
            Show::Sparkles => {
                if rng.random_bool(SPARKLE_CHANCE) {
                    self.twinkles.push(Twinkle {
                        x: rng.random_range(2..i32::from(width) - 2),
                        y: rng.random_range(i32::from(band.top)..i32::from(band.bottom)),
                        ch: SPARKS[rng.random_range(0..SPARKS.len())],
                        colour: COLOURS[rng.random_range(0..COLOURS.len())],
                        age: 0,
                    });
                }
            }
            Show::Bursts | Show::Fading => {
                if self.frames_until_burst > 0 {
                    self.frames_until_burst -= 1;
                    return;
                }
                let gap = if show == Show::Bursts {
                    SHOW_GAP
                } else {
                    FADING_GAP
                };
                self.frames_until_burst = rng.random_range(gap);
                let room = i32::from(band.height()) - TALLEST_SHAPE;
                if room < 1 {
                    return;
                }
                let reach = (i32::from(band.height()) * 2 / 3).min(room).max(1);
                self.bursts.push(Burst {
                    x: rng.random_range(6..i32::from(width) - 6),
                    top: i32::from(band.top) + rng.random_range(0..reach),
                    floor: i32::from(band.bottom) - 1,
                    shape: rng.random_range(0..SHAPES.len()),
                    colour: COLOURS[rng.random_range(0..COLOURS.len())],
                    age: 0,
                });
            }
        }
    }

    pub(crate) fn sparks(&self, width: u16, height: u16) -> Vec<Spark> {
        let mut sparks = Vec::new();
        for twinkle in &self.twinkles {
            sparks.push(Spark {
                x: twinkle.x,
                y: twinkle.y,
                ch: twinkle.ch,
                colour: twinkle.colour,
            });
        }
        for burst in &self.bursts {
            if burst.age < RISE_FRAMES {
                let climbed = (burst.floor - burst.top) * burst.age as i32 / RISE_FRAMES as i32;
                sparks.push(Spark {
                    x: burst.x,
                    y: burst.floor - climbed,
                    ch: '\'',
                    colour: burst.colour,
                });
                continue;
            }
            let fading = burst.age >= RISE_FRAMES + BURST_FRAMES;
            let rows: Vec<&str> = SHAPES[burst.shape].lines().collect();
            let half_width = rows.iter().map(|row| row.len()).max().unwrap_or(0) as i32 / 2;
            for (dy, row) in rows.iter().enumerate() {
                for (dx, ch) in row.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    sparks.push(Spark {
                        x: burst.x - half_width + dx as i32,
                        y: burst.top + dy as i32,
                        ch: if fading { '.' } else { ch },
                        colour: burst.colour,
                    });
                }
            }
        }
        sparks.retain(|spark| {
            (0..i32::from(width)).contains(&spark.x) && (0..i32::from(height)).contains(&spark.y)
        });
        sparks
    }
}

fn show_now(ctx: &FrameContext<'_>) -> Show {
    let clear_night = !ctx.conditions.sun.is_day
        && !ctx.conditions.is_raining
        && !ctx.conditions.is_thunderstorm
        && !ctx.conditions.is_snowing
        && !ctx.conditions.is_foggy;
    if clear_night {
        show_for(ctx.events)
    } else {
        Show::Off
    }
}

impl AnimationSystem for FireworksSystem {
    fn id(&self) -> &'static str {
        FIREWORKS
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Background
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        show_now(ctx) != Show::Off || !self.bursts.is_empty() || !self.twinkles.is_empty()
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.advance(
            show_now(ctx),
            ctx.size.width,
            SkyBand::below_hud(ctx.skyline_y),
            rng,
        );
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        for spark in self.sparks(ctx.size.width, ctx.size.height) {
            let colour =
                mask::colour(spark.colour, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            renderer.render_char(spark.x as u16, spark.y as u16, spark.ch, colour)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{self, EventSettings};
    use chrono::NaiveDateTime;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn at(text: &str) -> ActiveEvents {
        let now = NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap();
        events::resolve(now, &EventSettings::in_country("ID"))
    }

    #[test]
    fn the_show_builds_to_midnight_and_fades() {
        assert_eq!(show_for(&at("2026-12-31T19:30")), Show::Off);
        assert_eq!(show_for(&at("2026-12-31T20:30")), Show::Sparkles);
        assert_eq!(show_for(&at("2026-12-31T23:30")), Show::Bursts);
        assert_eq!(show_for(&at("2027-01-01T00:30")), Show::Bursts);
        assert_eq!(show_for(&at("2027-01-01T01:30")), Show::Fading);
        assert_eq!(show_for(&at("2027-01-01T02:30")), Show::Off);
        assert_eq!(show_for(&at("2026-10-31T23:30")), Show::Off);
    }

    #[test]
    fn imlek_eve_and_takbiran_get_their_own_shows() {
        assert_eq!(show_for(&at("2027-02-05T19:30")), Show::Off);
        assert_eq!(show_for(&at("2027-02-05T20:30")), Show::Sparkles);
        assert_eq!(show_for(&at("2027-02-05T23:30")), Show::Fading);
        assert_eq!(show_for(&at("2027-02-06T00:30")), Show::Fading);
        assert_eq!(show_for(&at("2027-02-06T01:30")), Show::Off);
        assert_eq!(show_for(&at("2027-02-06T21:00")), Show::Off);
        assert_eq!(show_for(&at("2027-03-09T18:30")), Show::Off);
        assert_eq!(show_for(&at("2027-03-09T19:30")), Show::Sparkles);
        assert_eq!(show_for(&at("2027-03-09T23:30")), Show::Sparkles);
        assert_eq!(show_for(&at("2027-03-10T00:30")), Show::Off);
        assert_eq!(show_for(&at("2027-03-10T21:00")), Show::Off);
        assert_eq!(show_for(&at("2027-03-08T21:00")), Show::Off);
    }

    #[test]
    fn bursts_rise_open_fade_and_leave() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut fireworks = FireworksSystem::new();
        let band = SkyBand::below_hud(15);
        let mut opened = false;
        for _ in 0..600 {
            fireworks.advance(Show::Bursts, 110, band, &mut rng);
            let sparks = fireworks.sparks(110, 32);
            opened |= sparks.iter().any(|spark| "_\\/=o*".contains(spark.ch));
            assert!(
                sparks.iter().all(|spark| spark.y < 15 && spark.y >= 3),
                "{sparks:?}"
            );
        }
        assert!(opened);
        for _ in 0..(RISE_FRAMES + BURST_FRAMES + FADE_FRAMES) {
            fireworks.advance(Show::Off, 110, band, &mut rng);
        }
        assert!(fireworks.sparks(110, 32).is_empty());
    }

    #[test]
    fn rain_stops_new_bursts_but_lets_the_ones_in_the_air_finish() {
        use crate::animation::halloween::test_support::Fixture;
        use crate::weather::WeatherCondition;
        let mut rng = StdRng::seed_from_u64(4);
        let mut fireworks = FireworksSystem::new();
        while fireworks.bursts.is_empty() {
            fireworks.advance(Show::Bursts, 110, SkyBand::below_hud(15), &mut rng);
        }
        let mut rain = Fixture::new(WeatherCondition::Rain, false);
        rain.events = at("2026-12-31T23:30");
        let mut commands = FrameCommands::default();
        assert!(fireworks.is_active(&rain.ctx()));
        for _ in 0..(RISE_FRAMES + BURST_FRAMES + FADE_FRAMES) {
            fireworks.update(&rain.ctx(), &mut rng, &mut commands);
        }
        assert!(fireworks.bursts.is_empty());
        assert!(!fireworks.is_active(&rain.ctx()));
        let mut clear = Fixture::new(WeatherCondition::Clear, false);
        clear.events = at("2026-12-31T23:30");
        assert!(fireworks.is_active(&clear.ctx()));
    }

    #[test]
    fn the_evening_only_twinkles() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut fireworks = FireworksSystem::new();
        for _ in 0..300 {
            fireworks.advance(Show::Sparkles, 110, SkyBand::below_hud(15), &mut rng);
        }
        assert!(fireworks.bursts.is_empty());
        assert!(!fireworks.twinkles.is_empty());
        assert!(
            fireworks
                .sparks(110, 32)
                .iter()
                .all(|spark| SPARKS.contains(&spark.ch))
        );
    }

    #[test]
    fn tiny_panes_get_no_fireworks_and_never_panic() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut fireworks = FireworksSystem::new();
        for (width, skyline) in [(10, 15), (12, 15), (13, 15), (110, 4), (20, 6), (0, 0)] {
            for show in [Show::Bursts, Show::Sparkles] {
                for _ in 0..200 {
                    fireworks.advance(show, width, SkyBand::below_hud(skyline), &mut rng);
                    assert!(
                        fireworks
                            .sparks(width, 8)
                            .iter()
                            .all(|spark| (spark.x as u16) < width && spark.y < 8)
                    );
                }
            }
        }
    }
}
