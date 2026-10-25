use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer};
use crate::events::Look;
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use crate::theme::colors::ElementColors;
use crossterm::style::Color;
use rand::{Rng, RngExt};
use std::io;

const PUMPKIN: &str = include_str!("../assets/halloween/pumpkin.txt");
const WIDTH: i32 = 8;
const HEIGHT: i32 = 3;
const GLOW_CHANCE: f64 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Body,
    Face,
    Stem,
}

fn part_of(ch: char) -> Part {
    match ch {
        '^' | 'v' => Part::Face,
        '\\' | '/' => Part::Stem,
        _ => Part::Body,
    }
}

fn solid_cells() -> Vec<(i32, i32, char)> {
    let mut cells = Vec::new();
    for (dy, line) in PUMPKIN.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let (Some(first), Some(last)) = (
            chars.iter().position(|ch| *ch != ' '),
            chars.iter().rposition(|ch| *ch != ' '),
        ) else {
            continue;
        };
        for (dx, &ch) in chars.iter().enumerate().take(last + 1).skip(first) {
            cells.push((dx as i32, dy as i32, ch));
        }
    }
    cells
}

pub fn origins(door: DoorAnchor) -> [(i32, i32); 2] {
    let top = door.ground_y as i32 - 1;
    [(door.x as i32 - 1 - WIDTH, top), (door.x as i32 + 6, top)]
}

pub struct HalloweenPumpkins {
    cells: Vec<(i32, i32, char)>,
    lit: [bool; 2],
    frames_until_flicker: [u32; 2],
}

impl HalloweenPumpkins {
    pub fn new() -> Self {
        Self {
            cells: solid_cells(),
            lit: [true; 2],
            frames_until_flicker: [0; 2],
        }
    }

    fn flicker(&mut self, rng: &mut (impl Rng + ?Sized)) {
        for (lit, wait) in self
            .lit
            .iter_mut()
            .zip(self.frames_until_flicker.iter_mut())
        {
            if *wait == 0 {
                *lit = rng.random_bool(GLOW_CHANCE);
                *wait = rng.random_range(3..=12);
            } else {
                *wait -= 1;
            }
        }
    }

    pub(crate) fn cells(
        &self,
        door: DoorAnchor,
        width: u16,
        height: u16,
        is_day: bool,
        colors: &ElementColors,
    ) -> Vec<(u16, u16, char, Color)> {
        let [body, glow, stem] = colors.pumpkin;
        let mut out = Vec::new();
        for (index, (ox, oy)) in origins(door).into_iter().enumerate() {
            if ox < 0 || oy < 0 || ox + WIDTH > width as i32 || oy + HEIGHT > height as i32 {
                continue;
            }
            for &(dx, dy, ch) in &self.cells {
                let color = match part_of(ch) {
                    Part::Stem => stem,
                    Part::Face if !is_day && self.lit[index] => glow,
                    _ => body,
                };
                out.push(((ox + dx) as u16, (oy + dy) as u16, ch, color));
            }
        }
        out
    }
}

impl Default for HalloweenPumpkins {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for HalloweenPumpkins {
    fn id(&self) -> &'static str {
        "halloween_pumpkins"
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.events.has(Look::Halloween) && ctx.door.is_some()
    }

    fn update(
        &mut self,
        _ctx: &FrameContext<'_>,
        rng: &mut dyn Rng,
        _commands: &mut FrameCommands,
    ) {
        self.flicker(rng);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        let Some(door) = ctx.door else {
            return Ok(());
        };
        for (x, y, ch, color) in self.cells(
            door,
            ctx.size.width,
            ctx.size.height,
            ctx.conditions.sun.is_day,
            ctx.colors,
        ) {
            renderer.render_char(x, y, ch, color)?;
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

    const BODY: Color = Color::AnsiValue(1);
    const GLOW: Color = Color::AnsiValue(2);
    const STEM: Color = Color::AnsiValue(3);

    fn door() -> DoorAnchor {
        DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 25,
        }
    }

    fn colors() -> ElementColors {
        ElementColors {
            pumpkin: [BODY, GLOW, STEM],
            ..crate::theme::catalogue::default_colors().night
        }
    }

    #[test]
    fn pumpkins_sit_two_columns_either_side_of_the_path() {
        let cells = HalloweenPumpkins::new().cells(door(), 110, 32, false, &colors());
        let left: Vec<u16> = cells.iter().filter(|c| c.0 < 39).map(|c| c.0).collect();
        let right: Vec<u16> = cells.iter().filter(|c| c.0 > 39).map(|c| c.0).collect();
        assert_eq!(
            (left.iter().min(), left.iter().max()),
            (Some(&30), Some(&37))
        );
        assert_eq!(
            (right.iter().min(), right.iter().max()),
            (Some(&45), Some(&52))
        );
        assert!(cells.iter().all(|c| (24..=26).contains(&c.1)));
    }

    #[test]
    fn faces_are_solid_so_soil_never_shows_through() {
        let cells = HalloweenPumpkins::new().cells(door(), 110, 32, true, &colors());
        let middle: String = cells
            .iter()
            .filter(|c| c.1 == 25 && c.0 < 39)
            .map(|c| c.2)
            .collect();
        assert_eq!(middle, "( ^  ^ )");
    }

    #[test]
    fn faces_glow_at_night_and_stay_plain_by_day() {
        let mut pumpkins = HalloweenPumpkins::new();
        let day = pumpkins.cells(door(), 110, 32, true, &colors());
        assert!(
            day.iter()
                .filter(|c| part_of(c.2) == Part::Face)
                .all(|c| c.3 == BODY)
        );
        let mut rng = StdRng::seed_from_u64(5);
        let (mut glowed, mut dimmed) = (false, false);
        for _ in 0..300 {
            pumpkins.flicker(&mut rng);
            for c in pumpkins.cells(door(), 110, 32, false, &colors()) {
                match part_of(c.2) {
                    Part::Face => {
                        glowed |= c.3 == GLOW;
                        dimmed |= c.3 == BODY;
                    }
                    Part::Stem => assert_eq!(c.3, STEM),
                    Part::Body => assert_eq!(c.3, BODY),
                }
            }
        }
        assert!(glowed && dimmed, "glowed={glowed} dimmed={dimmed}");
    }

    #[test]
    fn a_pumpkin_past_the_pane_edge_is_skipped() {
        let near_left = DoorAnchor {
            x: 5,
            width: 4,
            ground_y: 25,
        };
        let cells = HalloweenPumpkins::new().cells(near_left, 110, 32, false, &colors());
        assert!(!cells.is_empty());
        assert!(cells.iter().all(|c| c.0 >= 11));
        let near_right = DoorAnchor {
            x: 100,
            width: 4,
            ground_y: 25,
        };
        let cells = HalloweenPumpkins::new().cells(near_right, 110, 32, false, &colors());
        assert!(!cells.is_empty());
        assert!(cells.iter().all(|c| c.0 <= 98));
    }

    #[test]
    fn pumpkins_show_all_halloween_in_any_weather() {
        for &condition in WeatherCondition::ALL.iter() {
            for is_day in [true, false] {
                let fixture = Fixture::new(condition, is_day);
                assert!(
                    HalloweenPumpkins::new().is_active(&fixture.ctx()),
                    "{condition:?} {is_day}"
                );
            }
        }
        let mut fixture = Fixture::new(WeatherCondition::Clear, false);
        fixture.door = None;
        assert!(!HalloweenPumpkins::new().is_active(&fixture.ctx()));
        fixture.door = Some(door());
        fixture.events = ActiveEvents::default();
        assert!(!HalloweenPumpkins::new().is_active(&fixture.ctx()));
    }
}
