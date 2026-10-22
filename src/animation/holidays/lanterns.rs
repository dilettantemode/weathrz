use super::{Placed, cells, place};
use crate::animation::clouds::cloud_drift;
use crate::animation::flyers::mask;
use crate::animation::sky::SkyBand;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, Wind};
use crate::events::Look;
use crate::render::TerminalRenderer;
use rand::{Rng, RngExt};
use std::io;

pub const LANTERNS: &str = "waisak_lanterns";
pub const CREDIT: &str = "sky lanterns drawn for this fork";

const LANTERN: &str = ".-.\n|_|";
const LANTERN_MASK: &str = "yyy\nooo";
const MOST: usize = 6;
const LAUNCH_CHANCE: f64 = 1.0 / 90.0;
const RISE_PER_FRAME: std::ops::RangeInclusive<f32> = 0.02..=0.04;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Lantern {
    pub x: f32,
    pub y: f32,
    pub rise: f32,
}

pub struct WaisakLanterns {
    lanterns: Vec<Lantern>,
    drift: f32,
}

impl Default for WaisakLanterns {
    fn default() -> Self {
        Self::new()
    }
}

impl WaisakLanterns {
    pub fn new() -> Self {
        Self {
            lanterns: Vec::new(),
            drift: 0.0,
        }
    }

    pub(crate) fn advance(
        &mut self,
        launching: bool,
        width: u16,
        band: SkyBand,
        launch_y: u16,
        rng: &mut (impl Rng + ?Sized),
    ) {
        let drift = self.drift;
        for lantern in &mut self.lanterns {
            lantern.y -= lantern.rise;
            lantern.x += drift;
        }
        let top = f32::from(band.top);
        let right = f32::from(width);
        self.lanterns
            .retain(|lantern| lantern.y >= top && lantern.x > -3.0 && lantern.x < right);
        if launching
            && width > 6
            && launch_y > band.top
            && self.lanterns.len() < MOST
            && rng.random_bool(LAUNCH_CHANCE)
        {
            self.lanterns.push(Lantern {
                x: rng.random_range(2.0..f32::from(width) - 4.0),
                y: f32::from(launch_y),
                rise: rng.random_range(RISE_PER_FRAME),
            });
        }
    }

    pub(crate) fn placed(&self, width: u16, height: u16) -> Vec<Placed> {
        let shape = cells(LANTERN, LANTERN_MASK);
        self.lanterns
            .iter()
            .flat_map(|lantern| {
                let (x, y) = (lantern.x.round() as i32, lantern.y.round() as i32);
                shape
                    .iter()
                    .filter_map(move |cell| {
                        place(
                            x + cell.dx,
                            y + cell.dy,
                            cell.ch,
                            cell.letter,
                            width,
                            height,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn clear_night(ctx: &FrameContext<'_>) -> bool {
    !ctx.conditions.sun.is_day
        && !ctx.conditions.is_raining
        && !ctx.conditions.is_thunderstorm
        && !ctx.conditions.is_snowing
        && !ctx.conditions.is_foggy
}

impl AnimationSystem for WaisakLanterns {
    fn id(&self) -> &'static str {
        LANTERNS
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::Background
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        (ctx.events.has(Look::Waisak) && clear_night(ctx)) || !self.lanterns.is_empty()
    }

    fn on_wind(&mut self, wind: Wind) {
        self.drift = cloud_drift(wind.speed_kmh, wind.direction_deg);
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        let band = SkyBand::below_hud(ctx.skyline_y);
        self.advance(
            ctx.events.has(Look::Waisak) && clear_night(ctx),
            ctx.size.width,
            band,
            band.bottom.saturating_sub(2),
            rng,
        );
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        for cell in self.placed(ctx.size.width, ctx.size.height) {
            let color =
                mask::colour(cell.letter, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            renderer.render_char(cell.x, cell.y, cell.ch, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::halloween::test_support::Fixture;
    use crate::animation::holidays::assert_mask_fits;
    use crate::events::{self, EventSettings};
    use crate::weather::WeatherCondition;
    use chrono::NaiveDateTime;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn the_lantern_mask_copies_the_art() {
        assert_mask_fits("lantern", LANTERN, LANTERN_MASK);
    }

    #[test]
    fn lanterns_rise_over_the_roof_and_leave_at_the_top() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut lanterns = WaisakLanterns::new();
        let band = SkyBand::below_hud(15);
        let mut highest = f32::MAX;
        for _ in 0..3000 {
            lanterns.advance(true, 110, band, 23, &mut rng);
            assert!(lanterns.lanterns.len() <= MOST);
            for lantern in &lanterns.lanterns {
                assert!(lantern.y <= 23.0 && lantern.y >= 3.0, "{lantern:?}");
                highest = highest.min(lantern.y);
            }
        }
        assert!(highest < 5.0, "{highest}");
        for _ in 0..1500 {
            lanterns.advance(false, 110, band, 23, &mut rng);
        }
        assert!(lanterns.lanterns.is_empty());
    }

    #[test]
    fn the_wind_carries_them_sideways() {
        let mut rng = StdRng::seed_from_u64(6);
        let mut lanterns = WaisakLanterns::new();
        lanterns.on_wind(Wind {
            speed_kmh: 30.0,
            direction_deg: 270.0,
        });
        lanterns.lanterns.push(Lantern {
            x: 50.0,
            y: 20.0,
            rise: 0.02,
        });
        for _ in 0..100 {
            lanterns.advance(false, 110, SkyBand::below_hud(15), 23, &mut rng);
        }
        assert!(lanterns.lanterns[0].x > 60.0, "{:?}", lanterns.lanterns);
    }

    #[test]
    fn only_a_clear_waisak_night_launches_them() {
        let waisak = events::resolve(
            NaiveDateTime::parse_from_str("2027-05-20T21:00", "%Y-%m-%dT%H:%M").unwrap(),
            &EventSettings::in_country("ID"),
        );
        let lanterns = WaisakLanterns::new();
        let mut night = Fixture::new(WeatherCondition::Clear, false);
        night.events = waisak.clone();
        assert!(lanterns.is_active(&night.ctx()));
        let mut rain = Fixture::new(WeatherCondition::Rain, false);
        rain.events = waisak.clone();
        assert!(!lanterns.is_active(&rain.ctx()));
        let mut day = Fixture::new(WeatherCondition::Clear, true);
        day.events = waisak;
        assert!(!lanterns.is_active(&day.ctx()));
        assert!(!lanterns.is_active(&Fixture::new(WeatherCondition::Clear, false).ctx()));
    }

    #[test]
    fn tiny_panes_never_launch_or_panic() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut lanterns = WaisakLanterns::new();
        for (width, skyline, launch) in [(4, 15, 23), (110, 3, 2), (0, 0, 0)] {
            for _ in 0..500 {
                lanterns.advance(true, width, SkyBand::below_hud(skyline), launch, &mut rng);
                let _ = lanterns.placed(width, 8);
            }
        }
    }
}
