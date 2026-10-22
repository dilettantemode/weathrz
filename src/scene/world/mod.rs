mod decorations;
mod ground;
pub(crate) mod house;
mod style;
pub mod trees;
pub mod yard;

use crate::hide::HiddenDecorations;
use crate::render::TerminalRenderer;
use crate::scene::{ChimneyPosition, DoorAnchor, Scene, SceneContext, SceneLayout};
use decorations::{DecorationLayout, Decorations};
use ground::Ground;
use house::House;
use std::io;
use style::WorldSceneStyle;

pub struct WorldScene {
    house: House,
    ground: Ground,
    decorations: Decorations,
    width: u16,
    height: u16,
}

impl WorldScene {
    const GROUND_HEIGHT: u16 = 7;

    pub fn new(width: u16, height: u16, hidden: HiddenDecorations) -> Self {
        Self {
            house: House,
            ground: Ground,
            decorations: Decorations::new(hidden),
            width,
            height,
        }
    }
}

impl Scene for WorldScene {
    fn id(&self) -> &'static str {
        "world"
    }

    fn update_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    fn layout(&self) -> SceneLayout {
        let ground_y = self.height.saturating_sub(Self::GROUND_HEIGHT);
        let house_x = (self.width / 2).saturating_sub(House::WIDTH / 2);
        let house_y = ground_y.saturating_sub(House::HEIGHT);
        let chimney_x = house_x + House::CHIMNEY_X_OFFSET;

        SceneLayout {
            ground_y,
            skyline_y: house_y,
            chimney_pos: Some(ChimneyPosition {
                x: chimney_x,
                y: house_y,
            }),
            door: Some(DoorAnchor {
                x: house_x + House::DOOR_X_OFFSET,
                width: House::DOOR_WIDTH,
                ground_y,
            }),
            width: self.width,
            height: self.height,
        }
    }

    fn render(&self, renderer: &mut TerminalRenderer, ctx: &SceneContext<'_>) -> io::Result<()> {
        let layout = self.layout();
        let house_x = (self.width / 2).saturating_sub(self.house.width() / 2);
        let house_y = layout.ground_y.saturating_sub(self.house.height());
        let style = WorldSceneStyle::resolve(ctx);

        self.ground.render(
            renderer,
            (self.width, Self::GROUND_HEIGHT, layout.ground_y),
            &ctx.garden.yard,
            &style,
        )?;
        self.house.render(renderer, house_x, house_y, &style)?;
        self.decorations.render(
            renderer,
            &DecorationLayout {
                horizon_y: layout.ground_y,
                house_x,
                house_width: self.house.width(),
                width: self.width,
                garden: ctx.garden,
            },
            &style,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_exposes_the_door_and_the_roof_line() {
        let layout = WorldScene::new(110, 32, HiddenDecorations::default()).layout();
        assert_eq!(layout.ground_y, 25);
        assert_eq!(layout.skyline_y, 15);
        assert_eq!(
            layout.door,
            Some(DoorAnchor {
                x: 39,
                width: 4,
                ground_y: 25
            })
        );
    }
}
