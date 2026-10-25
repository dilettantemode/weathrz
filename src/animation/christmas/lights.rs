use crate::animation::flyers::mask;
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::{CHRISTMAS_LIGHTS, Look};
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use crate::scene::world::house::House;
use rand::Rng;
use std::io;

const BULBS: [char; 5] = ['r', 'y', 'g', 'b', 'm'];
const UNLIT: char = 'u';
const TWINKLE_FRAMES: u64 = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Bulb {
    pub x: u16,
    pub y: u16,
    pub ch: char,
    pub letter: char,
}

pub(crate) fn bulbs(door: DoorAnchor, is_day: bool, frame: u64) -> Vec<Bulb> {
    let house_x = door.x.saturating_sub(House::DOOR_X_OFFSET);
    let house_y = door.ground_y.saturating_sub(House::HEIGHT);
    let step = (frame / TWINKLE_FRAMES) as usize;
    House::light_spots(house_x, house_y)
        .into_iter()
        .enumerate()
        .map(|(index, (x, y))| {
            if is_day {
                Bulb {
                    x,
                    y,
                    ch: '.',
                    letter: UNLIT,
                }
            } else {
                Bulb {
                    x,
                    y,
                    ch: 'o',
                    letter: BULBS[(index + step) % BULBS.len()],
                }
            }
        })
        .collect()
}

pub struct ChristmasLights {
    frame: u64,
}

impl ChristmasLights {
    pub fn new() -> Self {
        Self { frame: 0 }
    }
}

impl Default for ChristmasLights {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for ChristmasLights {
    fn id(&self) -> &'static str {
        CHRISTMAS_LIGHTS
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        (ctx.events.has(Look::ChristmasLights) || ctx.events.has(Look::Diwali))
            && ctx.door.is_some()
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
        for bulb in bulbs(door, ctx.conditions.sun.is_day, self.frame) {
            let color =
                mask::colour(bulb.letter, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            renderer.render_char(bulb.x, bulb.y, bulb.ch, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::halloween::test_support::Fixture;
    use crate::events::{self, EventSettings};
    use crate::weather::WeatherCondition;
    use chrono::NaiveDateTime;

    fn door() -> DoorAnchor {
        DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 25,
        }
    }

    fn christmas(fixture: &mut Fixture) {
        let night = NaiveDateTime::parse_from_str("2026-12-20T21:00", "%Y-%m-%dT%H:%M").unwrap();
        fixture.events = events::resolve(night, &EventSettings::in_country("ID"));
    }

    #[test]
    fn lights_twinkle_at_night_and_rest_by_day() {
        let night = bulbs(door(), false, 0);
        assert!(night.len() >= 30);
        assert!(night.iter().all(|bulb| bulb.ch == 'o'));
        assert!(
            night
                .windows(2)
                .all(|pair| pair[0].letter != pair[1].letter)
        );
        let later = bulbs(door(), false, TWINKLE_FRAMES);
        assert_ne!(night[0].letter, later[0].letter);
        let day = bulbs(door(), true, 0);
        assert!(
            day.iter()
                .all(|bulb| bulb.ch == '.' && bulb.letter == UNLIT)
        );
        assert_eq!(
            night
                .iter()
                .map(|bulb| (bulb.x, bulb.y))
                .collect::<Vec<_>>(),
            day.iter().map(|bulb| (bulb.x, bulb.y)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn the_lights_only_hang_in_the_christmas_window() {
        let mut fixture = Fixture::new(WeatherCondition::Clear, false);
        let lights = ChristmasLights::new();
        assert!(!lights.is_active(&fixture.ctx()));
        christmas(&mut fixture);
        assert!(lights.is_active(&fixture.ctx()));
    }

    #[test]
    fn the_lights_are_drawn_on_the_eave() {
        let mut fixture = Fixture::new(WeatherCondition::Clear, false);
        christmas(&mut fixture);
        let mut lights = ChristmasLights::new();
        let mut renderer = TerminalRenderer::headless(110, 32);
        lights.render(&mut renderer, &fixture.ctx()).unwrap();
        let text = renderer.text();
        let eave = text.lines().nth(25 - 10 + 4).unwrap();
        assert!(eave.matches('o').count() >= 15, "{eave}");
    }
}
