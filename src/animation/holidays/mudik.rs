use super::{Placed, cells, place};
use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::Look;
use crate::render::TerminalRenderer;
use rand::{Rng, RngExt};
use std::io;
use std::ops::RangeInclusive;

pub const MUDIK_CARS: &str = "mudik_cars";
pub const CREDIT: &str = "mudik car drawn for this fork";

const CAR: &str = " _[##]_\n/_|__|_\\_\n'-O----O-'";
const CAR_MASK: &str = " XnNNnX\nXXXXXXXXX\nXXsXXXXsXX";
const CAR_WIDTH: f32 = 10.0;
const BODY_COLOURS: [char; 6] = ['r', 'b', 'c', 'g', 'm', 'y'];
const SPEED: f32 = 0.25;
const GAP_FRAMES: RangeInclusive<u32> = 60..=240;
const FOLLOWING_DISTANCE: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Car {
    pub x: f32,
    pub colour: char,
}

pub struct MudikCars {
    cars: Vec<Car>,
    frames_until_next: u32,
}

impl Default for MudikCars {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn lane_top(ground_y: u16, height: u16) -> Option<u16> {
    let top = ground_y + 1;
    (top + 3 <= height).then_some(top)
}

impl MudikCars {
    pub fn new() -> Self {
        Self {
            cars: Vec::new(),
            frames_until_next: 0,
        }
    }

    pub(crate) fn advance(&mut self, traffic: bool, width: u16, rng: &mut (impl Rng + ?Sized)) {
        for car in &mut self.cars {
            car.x += SPEED;
        }
        self.cars.retain(|car| car.x < f32::from(width));
        if !traffic {
            return;
        }
        if self.frames_until_next > 0 {
            self.frames_until_next -= 1;
            return;
        }
        let entry = -CAR_WIDTH;
        let room = self
            .cars
            .iter()
            .all(|car| car.x - entry >= CAR_WIDTH + FOLLOWING_DISTANCE);
        if room {
            self.cars.push(Car {
                x: entry,
                colour: BODY_COLOURS[rng.random_range(0..BODY_COLOURS.len())],
            });
            self.frames_until_next = rng.random_range(GAP_FRAMES);
        }
    }

    pub(crate) fn placed(&self, lane_top: u16, width: u16, height: u16) -> Vec<Placed> {
        let shape = cells(CAR, CAR_MASK);
        let mut out = Vec::new();
        for car in &self.cars {
            let left = car.x.round() as i32;
            for cell in &shape {
                let letter = if cell.letter == 'X' {
                    car.colour
                } else {
                    cell.letter
                };
                out.extend(place(
                    left + cell.dx,
                    i32::from(lane_top) + cell.dy,
                    cell.ch,
                    letter,
                    width,
                    height,
                ));
            }
        }
        out
    }
}

impl AnimationSystem for MudikCars {
    fn id(&self) -> &'static str {
        MUDIK_CARS
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        lane_top(ctx.horizon_y, ctx.size.height).is_some()
            && (ctx.events.has(Look::Mudik) || !self.cars.is_empty())
    }

    fn update(&mut self, ctx: &FrameContext<'_>, rng: &mut dyn Rng, _commands: &mut FrameCommands) {
        self.advance(ctx.events.has(Look::Mudik), ctx.size.width, rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        let Some(top) = lane_top(ctx.horizon_y, ctx.size.height) else {
            return Ok(());
        };
        for cell in self.placed(top, ctx.size.width, ctx.size.height) {
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
    fn the_car_mask_copies_the_art() {
        assert_mask_fits("car", CAR, &CAR_MASK.replace('X', "r"));
        assert_eq!(
            CAR.lines().map(|row| row.len()).max(),
            Some(CAR_WIDTH as usize)
        );
    }

    #[test]
    fn traffic_drives_out_of_town_without_bumping() {
        let mut rng = StdRng::seed_from_u64(9);
        let mut cars = MudikCars::new();
        let mut most = 0;
        for _ in 0..4000 {
            cars.advance(true, 110, &mut rng);
            most = most.max(cars.cars.len());
            for pair in cars.cars.windows(2) {
                assert!(
                    pair[0].x - pair[1].x >= CAR_WIDTH + FOLLOWING_DISTANCE,
                    "{pair:?}"
                );
            }
        }
        assert!(most >= 2, "{most}");
        for _ in 0..600 {
            cars.advance(false, 110, &mut rng);
        }
        assert!(cars.cars.is_empty());
    }

    #[test]
    fn a_car_waits_until_the_one_ahead_has_moved_off() {
        let mut rng = StdRng::seed_from_u64(11);
        let mut cars = MudikCars {
            cars: vec![Car {
                x: -CAR_WIDTH + 2.0,
                colour: 'r',
            }],
            frames_until_next: 0,
        };
        cars.advance(true, 110, &mut rng);
        assert_eq!(cars.cars.len(), 1);
        while cars.cars[0].x < FOLLOWING_DISTANCE - SPEED {
            cars.advance(true, 110, &mut rng);
            assert_eq!(cars.cars.len(), 1, "{:?}", cars.cars);
        }
        cars.advance(true, 110, &mut rng);
        assert_eq!(cars.cars.len(), 2);
    }

    #[test]
    fn cars_ride_the_soil_rows_in_their_own_colour() {
        let cars = MudikCars {
            cars: vec![Car {
                x: 20.0,
                colour: 'b',
            }],
            frames_until_next: 0,
        };
        let cells = cars.placed(26, 110, 32);
        assert!(cells.iter().all(|cell| (26..=28).contains(&cell.y)));
        assert!(cells.iter().all(|cell| (20..30).contains(&cell.x)));
        assert!(cells.iter().any(|cell| cell.letter == 'b'));
        assert!(cells.iter().all(|cell| cell.letter != 'X'));
        assert_eq!(
            cells.iter().filter(|cell| cell.ch == 'O').count(),
            2,
            "two wheels"
        );
    }

    #[test]
    fn the_road_needs_three_soil_rows_and_the_mudik_week() {
        assert_eq!(lane_top(25, 32), Some(26));
        assert_eq!(lane_top(25, 29), Some(26));
        assert_eq!(lane_top(25, 28), None);
        let mut fixture = Fixture::new(WeatherCondition::Rain, true);
        let cars = MudikCars::new();
        assert!(!cars.is_active(&fixture.ctx()));
        fixture.events = events::resolve(
            NaiveDateTime::parse_from_str("2027-03-05T10:00", "%Y-%m-%dT%H:%M").unwrap(),
            &EventSettings::in_country("ID"),
        );
        assert!(cars.is_active(&fixture.ctx()));
    }

    #[test]
    fn narrow_panes_never_panic() {
        let mut rng = StdRng::seed_from_u64(10);
        let mut cars = MudikCars::new();
        for width in [0, 3, 12] {
            for _ in 0..400 {
                cars.advance(true, width, &mut rng);
                let _ = cars.placed(5, width, 8);
            }
        }
    }
}
