use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::Look;
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use crate::scene::world::house::House;
use rand::Rng;
use std::io;

pub const BUNTING: &str = "holiday_bunting";

const PENNANT: char = 'v';
const COLOURS: [char; 2] = ['r', 'f'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Pennant {
    pub x: u16,
    pub y: u16,
    pub letter: char,
}

pub(crate) fn pennants(door: DoorAnchor) -> Vec<Pennant> {
    let house_x = door.x.saturating_sub(House::DOOR_X_OFFSET);
    let house_y = door.ground_y.saturating_sub(House::HEIGHT);
    House::light_spots(house_x, house_y)
        .into_iter()
        .enumerate()
        .map(|(index, (x, y))| Pennant {
            x,
            y,
            letter: COLOURS[index % COLOURS.len()],
        })
        .collect()
}

pub struct HolidayBunting;

impl HolidayBunting {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HolidayBunting {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for HolidayBunting {
    fn id(&self) -> &'static str {
        BUNTING
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.events.has(Look::Bunting) && ctx.door.is_some()
    }

    fn update(
        &mut self,
        _ctx: &FrameContext<'_>,
        _rng: &mut dyn Rng,
        _commands: &mut FrameCommands,
    ) {
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        let Some(door) = ctx.door else {
            return Ok(());
        };
        for pennant in pennants(door) {
            let color =
                mask::colour(pennant.letter, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            renderer.render_char(pennant.x, pennant.y, PENNANT, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pennants_alternate_red_and_white_along_the_eave_and_veranda() {
        let door = DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 25,
        };
        let pennants = pennants(door);
        assert!(pennants.len() > 10);
        assert!(
            pennants
                .windows(2)
                .all(|pair| pair[0].letter != pair[1].letter)
        );
        assert!(pennants.iter().all(|pennant| pennant.y < 25));
    }
}
