use super::{Cell, Placed, cells};
use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, Wind};
use crate::events::{ActiveEvents, Look};
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use crate::scene::world::house::House;
use crate::scene::world::trees;
use chrono::Duration;
use rand::Rng;
use std::io;

pub const HANGINGS: &str = "holiday_hangings";
pub const CREDIT: &str = "lampion, ketupat and crescent drawn for this fork";

const LAMPION: &str = " .-.\n(|||)\n '-'\n  :";
const LAMPION_MASK: &str = " yyy\nrrrrr\n yyy\n  y";
const KETUPAT: &str = " /\\\n/\\/\\\n\\/\\/\n \\/";
const KETUPAT_MASK: &str = " gy\ngygy\nygyg\n yg";
const CRESCENT: &str = " .-\n( *\n '-";
const CRESCENT_MASK: &str = " yy\ny f\n yy";
const SWING_PHASE_FRAMES: u64 = 20;
const EVE_HOURS: i64 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Hanging {
    Lampion,
    Ketupat,
    Crescent,
}

impl Hanging {
    fn swings(self) -> bool {
        self != Hanging::Crescent
    }

    fn art(self) -> (&'static str, &'static str) {
        match self {
            Hanging::Lampion => (LAMPION, LAMPION_MASK),
            Hanging::Ketupat => (KETUPAT, KETUPAT_MASK),
            Hanging::Crescent => (CRESCENT, CRESCENT_MASK),
        }
    }
}

fn past_the_eve(events: &ActiveEvents, look: Look) -> bool {
    events
        .since_start_of(look)
        .is_some_and(|since| since >= Duration::hours(EVE_HOURS))
}

pub(crate) fn hangings_for(events: &ActiveEvents) -> Vec<(Hanging, i32)> {
    let mut out = Vec::new();
    if events.has(Look::LunarNewYear) || events.has(Look::Lanterns) {
        out.extend([(Hanging::Lampion, 11), (Hanging::Lampion, 20)]);
    } else if past_the_eve(events, Look::Ketupat) {
        out.extend([(Hanging::Ketupat, 11), (Hanging::Ketupat, 21)]);
    }
    if events.has(Look::Ramadan) || events.has(Look::Eid) {
        out.push((Hanging::Crescent, 7));
    }
    out
}

pub(crate) fn placed(
    hangings: &[(Hanging, i32)],
    door: DoorAnchor,
    width: u16,
    wind_kmh: f32,
    frame: u64,
) -> Vec<Placed> {
    let house_x = i32::from(door.x) - i32::from(House::DOOR_X_OFFSET);
    let house_y = door.ground_y.saturating_sub(House::HEIGHT);
    let top = i32::from(house_y + House::EAVE_ROW + 1);
    let mut out = Vec::new();
    for (index, &(hanging, column)) in hangings.iter().enumerate() {
        let (art, art_mask) = hanging.art();
        let swing =
            trees::sway(wind_kmh, frame + index as u64 * SWING_PHASE_FRAMES).clamp(-1, 1) as i32;
        for Cell { dx, dy, ch, letter } in cells(art, art_mask) {
            let shift = if dy == 0 || !hanging.swings() {
                0
            } else {
                swing
            };
            let x = house_x + column + dx + shift;
            let y = top + dy;
            if (0..i32::from(width)).contains(&x) && y >= 0 {
                out.push(Placed {
                    x: x as u16,
                    y: y as u16,
                    ch,
                    letter,
                    halves: None,
                });
            }
        }
    }
    out
}

pub struct HolidayHangings {
    frame: u64,
    wind_kmh: f32,
}

impl HolidayHangings {
    pub fn new() -> Self {
        Self {
            frame: 0,
            wind_kmh: 0.0,
        }
    }
}

impl Default for HolidayHangings {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for HolidayHangings {
    fn id(&self) -> &'static str {
        HANGINGS
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.door.is_some() && !hangings_for(ctx.events).is_empty()
    }

    fn on_wind(&mut self, wind: Wind) {
        self.wind_kmh = wind.speed_kmh;
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
        let hangings = hangings_for(ctx.events);
        for cell in placed(&hangings, door, ctx.size.width, self.wind_kmh, self.frame) {
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

    fn at(text: &str) -> ActiveEvents {
        events::resolve(
            NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap(),
            &EventSettings::in_country("ID"),
        )
    }

    fn door() -> DoorAnchor {
        DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 25,
        }
    }

    fn kinds(text: &str) -> Vec<Hanging> {
        hangings_for(&at(text))
            .into_iter()
            .map(|(hanging, _)| hanging)
            .collect()
    }

    #[test]
    fn masks_copy_the_art() {
        assert_mask_fits("lampion", LAMPION, LAMPION_MASK);
        assert_mask_fits("ketupat", KETUPAT, KETUPAT_MASK);
        assert_mask_fits("crescent", CRESCENT, CRESCENT_MASK);
    }

    #[test]
    fn each_holiday_hangs_its_own_ornaments() {
        use Hanging::*;
        assert_eq!(kinds("2027-02-06T10:00"), [Lampion, Lampion]);
        assert_eq!(kinds("2027-02-12T10:00"), [Lampion, Lampion, Crescent]);
        assert_eq!(kinds("2027-02-25T10:00"), [Crescent]);
        assert_eq!(kinds("2027-03-09T21:00"), []);
        assert_eq!(kinds("2027-03-10T08:00"), [Ketupat, Ketupat]);
        assert_eq!(kinds("2027-03-15T20:00"), [Ketupat, Ketupat]);
        assert_eq!(kinds("2027-03-16T08:00"), []);
        assert_eq!(kinds("2027-05-16T20:00"), []);
        assert_eq!(kinds("2027-05-17T08:00"), [Ketupat, Ketupat]);
        assert_eq!(kinds("2026-10-20T10:00"), []);
    }

    #[test]
    fn ornaments_hang_from_the_eave_beside_the_door() {
        let cells = placed(&hangings_for(&at("2027-02-12T10:00")), door(), 110, 0.0, 0);
        let eave_y = 25 - House::HEIGHT + House::EAVE_ROW;
        assert_eq!(cells.iter().map(|cell| cell.y).min(), Some(eave_y + 1));
        assert!(cells.iter().all(|cell| cell.y < 25));
        let house_x = 39 - House::DOOR_X_OFFSET;
        let lampion_body: Vec<u16> = cells
            .iter()
            .filter(|cell| cell.ch == '(' && cell.letter == 'r')
            .map(|cell| cell.x - house_x)
            .collect();
        assert_eq!(lampion_body, [11, 20]);
        assert!(
            cells
                .iter()
                .any(|cell| cell.ch == '*' && cell.x - house_x == 9)
        );
    }

    #[test]
    fn ornaments_swing_below_the_hook_in_the_wind() {
        let hangings = hangings_for(&at("2027-02-06T10:00"));
        let still = placed(&hangings, door(), 110, 0.0, 0);
        let moved = (0..90).any(|frame| {
            let windy = placed(&hangings, door(), 110, 30.0, frame);
            windy.iter().zip(&still).any(|(a, b)| a.dx_from(b) != 0)
                && windy
                    .iter()
                    .zip(&still)
                    .filter(|(a, _)| a.y == still[0].y)
                    .all(|(a, b)| a.x == b.x)
        });
        assert!(moved);
        for frame in 0..90 {
            let windy = placed(&hangings, door(), 110, 60.0, frame);
            assert!(
                windy
                    .iter()
                    .zip(&still)
                    .all(|(a, b)| a.dx_from(b).abs() <= 1)
            );
        }
    }

    #[test]
    fn the_crescent_holds_still_and_never_touches_a_lampion() {
        let hangings = hangings_for(&at("2027-02-12T21:00"));
        let crescent = [(Hanging::Crescent, 7)];
        let still = placed(&crescent, door(), 110, 0.0, 0);
        for frame in 0..120 {
            let windy = placed(&hangings, door(), 110, 60.0, frame);
            let mut seen = std::collections::HashSet::new();
            assert!(
                windy.iter().all(|cell| seen.insert((cell.x, cell.y))),
                "overlap at frame {frame}"
            );
            assert_eq!(placed(&crescent, door(), 110, 60.0, frame), still);
        }
    }

    impl Placed {
        fn dx_from(&self, other: &Placed) -> i32 {
            i32::from(self.x) - i32::from(other.x)
        }
    }

    #[test]
    fn on_short_panes_ornaments_still_hang_under_the_eave() {
        let short = DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 8,
        };
        let cells = placed(&hangings_for(&at("2027-02-12T10:00")), short, 110, 0.0, 0);
        assert_eq!(
            cells.iter().map(|cell| cell.y).min(),
            Some(House::EAVE_ROW + 1)
        );
    }

    #[test]
    fn narrow_panes_clip_instead_of_panicking() {
        let hangings = hangings_for(&at("2027-02-12T10:00"));
        let near_edge = DoorAnchor {
            x: 2,
            width: 4,
            ground_y: 3,
        };
        let cells = placed(&hangings, near_edge, 10, 40.0, 7);
        assert!(cells.iter().all(|cell| cell.x < 10));
    }
}
