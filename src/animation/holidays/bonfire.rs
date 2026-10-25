use super::{Cell, Placed, cells, place};
use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::{ActiveEvents, Look};
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use crate::scene::world::house::House;
use chrono::{Duration, Timelike};
use rand::Rng;
use std::io;

pub const BONFIRE: &str = "holiday_bonfire";
pub const CREDIT: &str = "bonfire drawn for this fork";

const FRAMES: [(&str, &str); 3] = [
    (
        "    (\n   ) )\n  ( ( (\n  )\\|/(\n ==/|\\==",
        "    y\n   o y\n  r o y\n  rnnnr\n nnnnnnn",
    ),
    (
        "     )\n   ( (\n  ) ) )\n  (\\|/)\n ==/|\\==",
        "     y\n   o y\n  r y o\n  rnnnr\n nnnnnnn",
    ),
    (
        "    )\n    ( )\n  ( ) (\n  )\\|/(\n ==/|\\==",
        "    y\n    o y\n  r y o\n  rnnnr\n nnnnnnn",
    ),
];
const FLICKER_FRAMES: u64 = 5;
const SPARK_FRAMES: u64 = 4;
const SPARK_RISE: u64 = 4;
const SPARKS: [(i32, u64); 3] = [(3, 0), (5, 7), (4, 13)];
const FROM_HOUSE: i32 = 44;
const BASE_BELOW_GROUND: i32 = 3;
const EVE_HOURS: i64 = 6;
const DUSK_HOUR: u32 = 17;

pub(crate) fn burning(events: &ActiveEvents) -> bool {
    let Some(since) = events.since_start_of(Look::Bonfire) else {
        return false;
    };
    let start = events.now() - since;
    if start.hour() == 18 {
        since < Duration::hours(EVE_HOURS)
    } else {
        events.now().hour() >= DUSK_HOUR
    }
}

pub(crate) fn placed(door: DoorAnchor, width: u16, height: u16, frame: u64) -> Vec<Placed> {
    let house_x = i32::from(door.x) - i32::from(House::DOOR_X_OFFSET);
    let left = house_x + FROM_HOUSE;
    let (art, art_mask) = FRAMES[(frame / FLICKER_FRAMES) as usize % FRAMES.len()];
    let rows = art.lines().count() as i32;
    let top = i32::from(door.ground_y) + BASE_BELOW_GROUND - rows + 1;
    let drawn = cells(art, art_mask);
    let mut out = Vec::new();
    for dy in 0..rows {
        let row: Vec<&Cell> = drawn.iter().filter(|cell| cell.dy == dy).collect();
        let (Some(first), Some(last)) = (
            row.iter().map(|cell| cell.dx).min(),
            row.iter().map(|cell| cell.dx).max(),
        ) else {
            continue;
        };
        for dx in first..=last {
            let (ch, letter) = row
                .iter()
                .find(|cell| cell.dx == dx)
                .map_or((' ', 'f'), |cell| (cell.ch, cell.letter));
            out.extend(place(left + dx, top + dy, ch, letter, width, height));
        }
    }
    for (column, offset) in SPARKS {
        let step = (frame / SPARK_FRAMES + offset) % (SPARK_RISE + 2);
        if step < SPARK_RISE {
            let letter = if step < 2 { 'y' } else { 'o' };
            out.extend(place(
                left + column,
                top - 1 - step as i32,
                '.',
                letter,
                width,
                height,
            ));
        }
    }
    out
}

pub struct HolidayBonfire {
    frame: u64,
}

impl HolidayBonfire {
    pub fn new() -> Self {
        Self { frame: 0 }
    }
}

impl Default for HolidayBonfire {
    fn default() -> Self {
        Self::new()
    }
}

fn wet(ctx: &FrameContext<'_>) -> bool {
    ctx.conditions.is_raining || ctx.conditions.is_thunderstorm || ctx.conditions.is_snowing
}

impl AnimationSystem for HolidayBonfire {
    fn id(&self) -> &'static str {
        BONFIRE
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.door.is_some() && !ctx.conditions.sun.is_day && !wet(ctx) && burning(ctx.events)
    }

    fn update(
        &mut self,
        _ctx: &FrameContext<'_>,
        _rng: &mut dyn Rng,
        _commands: &mut FrameCommands,
    ) {
        self.frame = self.frame.wrapping_add(1);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        let Some(door) = ctx.door else {
            return Ok(());
        };
        for cell in placed(door, ctx.size.width, ctx.size.height, self.frame) {
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
    use crate::animation::holidays::assert_mask_fits;
    use crate::events::{self, EventSettings};
    use chrono::NaiveDateTime;

    fn at(text: &str, country: &'static str) -> ActiveEvents {
        events::resolve(
            NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap(),
            &EventSettings::in_country(country),
        )
    }

    #[test]
    fn every_frame_has_a_mask_that_copies_it() {
        for (index, (art, art_mask)) in FRAMES.iter().enumerate() {
            assert_mask_fits(&format!("bonfire {index}"), art, art_mask);
            assert_eq!(art.lines().count(), FRAMES[0].0.lines().count());
        }
    }

    #[test]
    fn eves_burn_until_midnight_and_bonfire_night_burns_from_dusk() {
        assert!(burning(&at("2027-09-27T21:00", "ET")));
        assert!(!burning(&at("2027-09-28T21:00", "ET")));
        assert!(burning(&at("2027-03-22T21:00", "IN")));
        assert!(!burning(&at("2027-03-23T21:00", "IN")));
        assert!(burning(&at("2026-11-05T19:00", "GB")));
        assert!(!burning(&at("2026-11-05T12:00", "GB")));
        assert!(!burning(&at("2026-11-06T19:00", "GB")));
    }

    #[test]
    fn the_fire_stands_in_the_front_yard_and_its_sparks_rise() {
        let door = DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 25,
        };
        let cells = placed(door, 140, 32, 0);
        let base = cells.iter().map(|cell| cell.y).max().unwrap();
        assert_eq!(base, 25 + BASE_BELOW_GROUND as u16);
        assert!(cells.iter().any(|cell| cell.ch == '.'));
        assert!(
            cells.iter().any(|cell| cell.ch == ' '),
            "gaps hide the soil behind"
        );
        assert_ne!(
            placed(door, 140, 32, 0),
            placed(door, 140, 32, FLICKER_FRAMES)
        );
    }
}
