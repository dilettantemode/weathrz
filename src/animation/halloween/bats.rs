use crate::animation::sky::SkyBand;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::Look;
use crate::render::TerminalRenderer;
use crate::weather::RainIntensity;
use rand::{Rng, RngExt};
use std::io;
use std::ops::RangeInclusive;

const BAT_UP: &str = include_str!("../assets/halloween/bat_up.txt");
const BAT_DOWN: &str = include_str!("../assets/halloween/bat_down.txt");
const SMALL_UP: [&str; 1] = ["^v^"];
const SMALL_DOWN: [&str; 1] = ["-v-"];
const SMALL_WIDTH: u16 = 3;
const BIG_HEIGHT: u16 = 4;
const FLAP_FRAMES: u32 = 5;
const FLUTTER_FRAMES: u32 = 10;
const FIRST_FLIGHT_FRAMES: RangeInclusive<u32> = 90..=240;
const NEXT_FLIGHT_FRAMES: RangeInclusive<u32> = 600..=1200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BatKind {
    Big,
    Small,
}

#[derive(Debug, Clone)]
struct Bat {
    kind: BatKind,
    x: f32,
    row: u16,
    flutter: i32,
}

fn art_rows(asset: &'static str) -> Vec<&'static str> {
    asset
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect()
}

fn wings_up(tick: u32) -> bool {
    (tick / FLAP_FRAMES) % 2 == 0
}

fn draw_row(bat: &Bat, band: SkyBand, height: u16) -> u16 {
    let lowest = band.bottom.saturating_sub(height).max(band.top) as i32;
    (bat.row as i32 + bat.flutter).clamp(band.top as i32, lowest) as u16
}

pub struct HalloweenBats {
    big_up: Vec<&'static str>,
    big_down: Vec<&'static str>,
    big_width: u16,
    bats: Vec<Bat>,
    step: f32,
    tick: u32,
    frames_until_flight: Option<u32>,
    rain: RainIntensity,
}

impl HalloweenBats {
    pub fn new() -> Self {
        let big_up = art_rows(BAT_UP);
        let big_down = art_rows(BAT_DOWN);
        let big_width = big_up
            .iter()
            .chain(big_down.iter())
            .map(|line| line.chars().count() as u16)
            .max()
            .unwrap_or(0);
        Self {
            big_up,
            big_down,
            big_width,
            bats: Vec::new(),
            step: 0.0,
            tick: 0,
            frames_until_flight: None,
            rain: RainIntensity::Light,
        }
    }

    fn sprite(&self, kind: BatKind, up: bool) -> &[&'static str] {
        match (kind, up) {
            (BatKind::Big, true) => self.big_up.as_slice(),
            (BatKind::Big, false) => self.big_down.as_slice(),
            (BatKind::Small, true) => &SMALL_UP[..],
            (BatKind::Small, false) => &SMALL_DOWN[..],
        }
    }

    fn width_of(&self, kind: BatKind) -> u16 {
        match kind {
            BatKind::Big => self.big_width,
            BatKind::Small => SMALL_WIDTH,
        }
    }

    fn advance(&mut self, width: u16, band: SkyBand, rng: &mut (impl Rng + ?Sized)) {
        self.tick = self.tick.wrapping_add(1);
        if self.bats.is_empty() {
            let wait = self
                .frames_until_flight
                .get_or_insert_with(|| rng.random_range(FIRST_FLIGHT_FRAMES));
            if *wait > 0 {
                *wait -= 1;
                return;
            }
            if !self.spawn_flight(width, band, rng) {
                self.frames_until_flight = Some(rng.random_range(NEXT_FLIGHT_FRAMES));
            }
            return;
        }

        let flutter = self.tick % FLUTTER_FRAMES == 0;
        for bat in &mut self.bats {
            bat.x += self.step;
            if flutter {
                bat.flutter = rng.random_range(-1..=1);
            }
        }

        let rightward = self.step > 0.0;
        let big_width = self.big_width as f32;
        let gone = self.bats.iter().all(|bat| {
            let bat_width = match bat.kind {
                BatKind::Big => big_width,
                BatKind::Small => SMALL_WIDTH as f32,
            };
            if rightward {
                bat.x >= width as f32
            } else {
                bat.x + bat_width <= 0.0
            }
        });
        if gone {
            self.bats.clear();
            self.frames_until_flight = Some(rng.random_range(NEXT_FLIGHT_FRAMES));
        }
    }

    fn spawn_flight(&mut self, width: u16, band: SkyBand, rng: &mut (impl Rng + ?Sized)) -> bool {
        let height = band.height();
        if height == 0 {
            return false;
        }
        let rightward = rng.random_bool(0.5);
        let speed: f32 = rng.random_range(0.5..0.8);
        self.step = if rightward { speed } else { -speed };

        let lead = if height >= BIG_HEIGHT {
            BatKind::Big
        } else {
            BatKind::Small
        };
        let lead_height = if lead == BatKind::Big { BIG_HEIGHT } else { 1 };
        let lead_width = self.width_of(lead) as f32;
        let mut x = if rightward { -lead_width } else { width as f32 };
        self.bats.push(Bat {
            kind: lead,
            x,
            row: band.top + rng.random_range(0..=height - lead_height),
            flutter: 0,
        });

        let mut ahead_width = lead_width;
        let followers: u8 = rng.random_range(0..=2);
        for _ in 0..followers {
            let gap = rng.random_range(3..=8) as f32;
            x = if rightward {
                x - gap - SMALL_WIDTH as f32
            } else {
                x + ahead_width + gap
            };
            ahead_width = SMALL_WIDTH as f32;
            self.bats.push(Bat {
                kind: BatKind::Small,
                x,
                row: band.top + rng.random_range(0..height),
                flutter: 0,
            });
        }
        true
    }

    pub(crate) fn cells(&self, width: u16, band: SkyBand) -> Vec<(u16, u16, char)> {
        let up = wings_up(self.tick);
        let mut cells = Vec::new();
        for bat in &self.bats {
            let rows = self.sprite(bat.kind, up);
            let top = draw_row(bat, band, rows.len() as u16);
            for (dy, line) in rows.iter().enumerate() {
                let y = top + dy as u16;
                if y >= band.bottom {
                    continue;
                }
                for (dx, ch) in line.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    let x = bat.x.floor() as i32 + dx as i32;
                    if x < 0 || x >= width as i32 {
                        continue;
                    }
                    cells.push((x as u16, y, ch));
                }
            }
        }
        cells
    }
}

impl Default for HalloweenBats {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for HalloweenBats {
    fn id(&self) -> &'static str {
        "halloween_bats"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Background
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.events.has(Look::Halloween)
            && !ctx.conditions.sun.is_day
            && !ctx.conditions.is_thunderstorm
            && !ctx.conditions.is_foggy
            && !ctx.conditions.is_snowing
            && !matches!(self.rain, RainIntensity::Heavy | RainIntensity::Storm)
    }

    fn on_rain_intensity(&mut self, intensity: RainIntensity) {
        self.rain = intensity;
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.advance(ctx.size.width, SkyBand::below_hud(ctx.skyline_y), rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        for (x, y, ch) in self.cells(ctx.size.width, SkyBand::below_hud(ctx.skyline_y)) {
            renderer.render_char(x, y, ch, ctx.colors.bats)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::halloween::test_support::Fixture;
    use crate::events::ActiveEvents;
    use crate::weather::WeatherCondition;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    const CALM: [WeatherCondition; 7] = [
        WeatherCondition::Clear,
        WeatherCondition::PartlyCloudy,
        WeatherCondition::Cloudy,
        WeatherCondition::Overcast,
        WeatherCondition::Drizzle,
        WeatherCondition::Rain,
        WeatherCondition::RainShowers,
    ];

    fn run(
        bats: &mut HalloweenBats,
        skyline_y: u16,
        rng: &mut StdRng,
        frames: usize,
        mut each: impl FnMut(usize, Vec<(u16, u16, char)>),
    ) {
        let band = SkyBand::below_hud(skyline_y);
        for frame in 0..frames {
            bats.advance(110, band, rng);
            each(frame, bats.cells(110, band));
        }
    }

    #[test]
    fn bats_fly_only_on_halloween_nights_in_calm_weather() {
        for &condition in WeatherCondition::ALL.iter() {
            for is_day in [true, false] {
                let fixture = Fixture::new(condition, is_day);
                let mut bats = HalloweenBats::new();
                bats.on_rain_intensity(condition.rain_intensity());
                let expected = !is_day && CALM.contains(&condition);
                assert_eq!(
                    bats.is_active(&fixture.ctx()),
                    expected,
                    "{condition:?} day={is_day}"
                );
            }
        }
        let mut fixture = Fixture::new(WeatherCondition::Clear, false);
        fixture.events = ActiveEvents::default();
        assert!(!HalloweenBats::new().is_active(&fixture.ctx()));
    }

    #[test]
    fn frames_share_the_body_column() {
        let bats = HalloweenBats::new();
        assert_eq!(bats.big_up.len(), 4);
        assert_eq!(bats.big_down.len(), 4);
        assert_eq!(bats.big_up[0].find(",_,"), Some(8));
        assert_eq!(bats.big_down[0].find(",_,"), Some(8));
        assert_eq!(bats.big_width, 19);
    }

    #[test]
    fn wings_flap_every_five_frames() {
        let pattern: Vec<bool> = (0..12).map(wings_up).collect();
        assert_eq!(
            pattern,
            [
                true, true, true, true, true, false, false, false, false, false, true, true
            ]
        );
    }

    #[test]
    fn bats_stay_inside_the_sky_band() {
        let mut bats = HalloweenBats::new();
        let mut rng = StdRng::seed_from_u64(1);
        let mut drawn = 0;
        run(&mut bats, 15, &mut rng, 3000, |_, cells| {
            for &(x, y, _) in &cells {
                assert!((3..15).contains(&y), "row {y}");
                assert!(x < 110, "column {x}");
            }
            drawn += cells.len();
        });
        assert!(drawn > 0, "no bat flew in 100 s");
    }

    #[test]
    fn band_height_picks_the_bat_sizes() {
        for (skyline_y, expect_any, expect_big) in
            [(3, false, false), (5, true, false), (9, true, true)]
        {
            let mut bats = HalloweenBats::new();
            let mut rng = StdRng::seed_from_u64(2);
            let (mut any, mut big) = (false, false);
            run(&mut bats, skyline_y, &mut rng, 3000, |_, cells| {
                any |= !cells.is_empty();
                big |= cells.iter().any(|&(_, _, ch)| ch == ',' || ch == '_');
                if skyline_y == 5 {
                    assert!(cells.iter().all(|&(_, _, ch)| "^v-".contains(ch)));
                }
            });
            assert_eq!((any, big), (expect_any, expect_big), "skyline {skyline_y}");
        }
    }

    #[test]
    fn first_flight_starts_within_three_to_eight_seconds() {
        for seed in 0..20 {
            let mut bats = HalloweenBats::new();
            let mut rng = StdRng::seed_from_u64(seed);
            let mut first = None;
            run(&mut bats, 15, &mut rng, 400, |frame, cells| {
                if first.is_none() && !cells.is_empty() {
                    first = Some(frame);
                }
            });
            let first = first.expect("a bat appears");
            assert!((90..=260).contains(&first), "seed {seed}: frame {first}");
        }
    }

    #[test]
    fn next_flight_waits_twenty_to_forty_seconds() {
        let mut bats = HalloweenBats::new();
        let mut rng = StdRng::seed_from_u64(3);
        let band = SkyBand::below_hud(15);
        let mut flying = Vec::new();
        for _ in 0..4000 {
            bats.advance(110, band, &mut rng);
            flying.push(!bats.bats.is_empty());
        }
        let start = flying.iter().position(|&f| f).expect("first flight");
        let end = start
            + flying[start..]
                .iter()
                .position(|&f| !f)
                .expect("first flight ends");
        let next = end
            + flying[end..]
                .iter()
                .position(|&f| f)
                .expect("second flight starts");
        assert!((600..=1201).contains(&(next - end)), "gap {}", next - end);
    }

    #[test]
    fn shrinking_the_pane_mid_flight_keeps_bats_inside() {
        let mut bats = HalloweenBats::new();
        let mut rng = StdRng::seed_from_u64(4);
        let band = SkyBand::below_hud(15);
        let mut frames = 0;
        while bats.cells(110, band).is_empty() {
            bats.advance(110, band, &mut rng);
            frames += 1;
            assert!(frames < 400, "no flight started");
        }
        for _ in 0..40 {
            bats.advance(110, band, &mut rng);
        }
        let small = SkyBand::below_hud(5);
        for _ in 0..300 {
            for (x, y, _) in bats.cells(40, small) {
                assert!(x < 40, "column {x}");
                assert!((3..5).contains(&y), "row {y}");
            }
            bats.advance(40, small, &mut rng);
        }
    }
}
