use super::{Cell, Placed, cells, place};
use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::{ActiveEvents, Look};
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use chrono::Duration;
use rand::Rng;
use std::io;

pub const GROUND: &str = "holiday_ground";
pub const CREDIT: &str =
    "candles after jgs's 5-line candle; eggs and obor torches drawn for this fork";

const CANDLE: &str = " )\n(_)\n|`|\n|_|";
const CANDLE_MASK: &str = " y\nfff\nfff\nfff";
const FLAMES: [char; 2] = [')', '('];
const TORCH_FLAMES: [char; 3] = [',', '\'', '*'];
const TORCH_COLOURS: [char; 3] = ['r', 'o', 'y'];
const EGG_COLOURS: [char; 3] = ['m', 'c', 'y'];
const EGG_OFFSETS: [i32; 3] = [-12, 9, 14];
const TORCH_SPACING: i32 = 8;
const FLICKER_FRAMES: u64 = 6;
const EVE_HOURS: i64 = 6;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Pieces {
    pub candles: bool,
    pub eggs: bool,
    pub torches: bool,
}

impl Pieces {
    fn any(self) -> bool {
        self.candles || self.eggs || self.torches
    }
}

fn on_the_eve(events: &ActiveEvents, look: Look) -> bool {
    events
        .since_start_of(look)
        .is_some_and(|since| since < Duration::hours(EVE_HOURS))
}

pub(crate) fn pieces_for(events: &ActiveEvents, is_day: bool, wet: bool) -> Pieces {
    Pieces {
        candles: events.has(Look::Waisak) || events.has(Look::Candles) || events.has(Look::Diwali),
        eggs: events.has(Look::Easter),
        torches: !is_day
            && !wet
            && (on_the_eve(events, Look::Takbiran) || on_the_eve(events, Look::Torches)),
    }
}

fn flicker(frame: u64, index: usize, choices: usize) -> usize {
    ((frame / FLICKER_FRAMES + index as u64 * 5) % choices as u64) as usize
}

pub(crate) fn placed(
    pieces: Pieces,
    door: DoorAnchor,
    width: u16,
    height: u16,
    frame: u64,
) -> Vec<Placed> {
    let ground = i32::from(door.ground_y);
    let door_x = i32::from(door.x);
    let mut out = Vec::new();
    if pieces.candles {
        let candle_cells = cells(CANDLE, CANDLE_MASK);
        for (index, left) in [door_x - 5, door_x + i32::from(door.width) + 2]
            .into_iter()
            .enumerate()
        {
            for Cell { dx, dy, ch, letter } in &candle_cells {
                let ch = if *dy == 0 {
                    FLAMES[flicker(frame, index, FLAMES.len())]
                } else {
                    *ch
                };
                out.extend(place(
                    left + dx,
                    ground - 2 + dy,
                    ch,
                    *letter,
                    width,
                    height,
                ));
            }
        }
    }
    if pieces.eggs {
        for (offset, colour) in EGG_OFFSETS.into_iter().zip(EGG_COLOURS) {
            out.extend(place(door_x + offset, ground, '(', colour, width, height));
            out.extend(place(
                door_x + offset + 1,
                ground,
                ')',
                colour,
                width,
                height,
            ));
        }
    }
    if pieces.torches {
        let path = door_x - 1..=door_x + i32::from(door.width);
        for (index, x) in (TORCH_SPACING / 2..i32::from(width))
            .step_by(TORCH_SPACING as usize)
            .filter(|x| !path.contains(x))
            .enumerate()
        {
            let flame = TORCH_FLAMES[flicker(frame, index, TORCH_FLAMES.len())];
            let colour = TORCH_COLOURS[flicker(frame + 3, index * 2, TORCH_COLOURS.len())];
            out.extend(place(x, ground - 1, flame, colour, width, height));
            out.extend(place(x, ground, '|', 'n', width, height));
        }
    }
    out
}

pub struct HolidayGround {
    frame: u64,
}

impl HolidayGround {
    pub fn new() -> Self {
        Self { frame: 0 }
    }
}

impl Default for HolidayGround {
    fn default() -> Self {
        Self::new()
    }
}

fn wet(ctx: &FrameContext<'_>) -> bool {
    ctx.conditions.is_raining || ctx.conditions.is_thunderstorm || ctx.conditions.is_snowing
}

impl AnimationSystem for HolidayGround {
    fn id(&self) -> &'static str {
        GROUND
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.door.is_some() && pieces_for(ctx.events, ctx.conditions.sun.is_day, wet(ctx)).any()
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
        let pieces = pieces_for(ctx.events, ctx.conditions.sun.is_day, wet(ctx));
        for cell in placed(pieces, door, ctx.size.width, ctx.size.height, self.frame) {
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

    #[test]
    fn the_candle_mask_copies_the_art() {
        assert_mask_fits("candle", CANDLE, CANDLE_MASK);
    }

    #[test]
    fn each_day_sets_out_its_own_pieces() {
        let only = |candles, eggs, torches| Pieces {
            candles,
            eggs,
            torches,
        };
        assert_eq!(
            pieces_for(&at("2027-05-20T21:00"), false, false),
            only(true, false, false)
        );
        assert_eq!(
            pieces_for(&at("2027-03-28T10:00"), true, false),
            only(false, true, false)
        );
        assert_eq!(
            pieces_for(&at("2027-03-09T21:00"), false, false),
            only(false, false, true)
        );
        assert_eq!(
            pieces_for(&at("2027-06-05T21:00"), false, false),
            only(false, false, true)
        );
        assert_eq!(
            pieces_for(&at("2027-03-09T21:00"), true, false),
            Pieces::default()
        );
        assert_eq!(
            pieces_for(&at("2027-03-09T21:00"), false, true),
            Pieces::default()
        );
        assert_eq!(
            pieces_for(&at("2027-03-10T21:00"), false, false),
            Pieces::default()
        );
        assert_eq!(
            pieces_for(&at("2026-10-20T21:00"), false, false),
            Pieces::default()
        );
    }

    #[test]
    fn candles_flank_the_door_and_their_flames_flicker() {
        let pieces = Pieces {
            candles: true,
            ..Pieces::default()
        };
        let first = placed(pieces, door(), 110, 32, 0);
        let flames: Vec<&Placed> = first.iter().filter(|cell| cell.letter == 'y').collect();
        assert_eq!(flames.len(), 2);
        assert!(flames[0].x < 39 && flames[1].x > 39 + 4);
        assert!(first.iter().all(|cell| (23..=26).contains(&cell.y)));
        let changed = (0..60).any(|frame| {
            placed(pieces, door(), 110, 32, frame)
                .iter()
                .zip(&first)
                .any(|(a, b)| a.ch != b.ch)
        });
        assert!(changed);
        for frame in 0..60 {
            let rims: String = placed(pieces, door(), 110, 32, frame)
                .iter()
                .filter(|cell| cell.y == 24)
                .map(|cell| cell.ch)
                .collect();
            assert_eq!(rims, "(_)(_)");
        }
    }

    #[test]
    fn eggs_lie_in_the_grass_in_three_colours() {
        let eggs = placed(
            Pieces {
                eggs: true,
                ..Pieces::default()
            },
            door(),
            110,
            32,
            0,
        );
        assert_eq!(eggs.len(), 6);
        assert!(eggs.iter().all(|cell| cell.y == 25));
        let mut colours: Vec<char> = eggs.iter().map(|cell| cell.letter).collect();
        colours.dedup();
        assert_eq!(colours, ['m', 'c', 'y']);
    }

    #[test]
    fn torches_line_the_ground_but_leave_the_path_clear() {
        let only_torches = Pieces {
            torches: true,
            ..Pieces::default()
        };
        for door_x in 30..46 {
            let door = DoorAnchor {
                x: door_x,
                ..door()
            };
            let sticks: Vec<u16> = placed(only_torches, door, 110, 32, 0)
                .iter()
                .filter(|cell| cell.ch == '|')
                .map(|cell| cell.x)
                .collect();
            assert!(sticks.len() >= 12, "{sticks:?}");
            let path = door_x - 1..=door_x + 4;
            assert!(
                sticks.iter().all(|x| !path.contains(x)),
                "{door_x}: {sticks:?}"
            );
        }
        let torches = placed(only_torches, door(), 110, 32, 0);
        assert!(
            torches
                .iter()
                .filter(|cell| cell.ch != '|')
                .all(|cell| cell.y == 24 && TORCH_FLAMES.contains(&cell.ch))
        );
    }

    #[test]
    fn small_panes_clip_every_piece() {
        let all = Pieces {
            candles: true,
            eggs: true,
            torches: true,
        };
        let tiny = DoorAnchor {
            x: 3,
            width: 4,
            ground_y: 4,
        };
        let cells = placed(all, tiny, 12, 5, 9);
        assert!(cells.iter().all(|cell| cell.x < 12 && cell.y < 5));
    }
}
