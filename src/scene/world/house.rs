use crate::render::TerminalRenderer;
use crate::scene::world::style::WorldSceneStyle;
use std::io;

const HOUSE_ASCII: &str = include_str!("assets/house.txt");
const VERANDA_FROM: usize = 28;
const BODY_RIGHT: usize = 29;
const WINDOW_GLOW: (u8, u8, u8) = (64, 48, 24);

pub struct House;

impl House {
    pub const WIDTH: u16 = 64;
    pub const HEIGHT: u16 = 10;
    pub const CHIMNEY_X_OFFSET: u16 = 12;
    pub const DOOR_X_OFFSET: u16 = 16;
    pub const DOOR_WIDTH: u16 = 4;
    pub const EAVE_ROW: u16 = 4;

    pub fn width(&self) -> u16 {
        Self::WIDTH
    }

    pub fn roof_rows() -> Vec<Option<u16>> {
        let roof: Vec<&str> = HOUSE_ASCII
            .lines()
            .take(usize::from(Self::EAVE_ROW) + 1)
            .collect();
        (0..usize::from(Self::WIDTH))
            .map(|column| {
                roof.iter()
                    .position(|line| line.chars().nth(column).is_some_and(|ch| ch != ' '))
                    .map(|row| row as u16)
            })
            .collect()
    }

    pub fn light_spots(x: u16, y: u16) -> Vec<(u16, u16)> {
        let mut spots = Vec::new();
        for (row, line) in HOUSE_ASCII.lines().enumerate() {
            let eave = !line.trim().is_empty() && line.trim().chars().all(|ch| ch == '~');
            let veranda = line.contains("_._._");
            let first_tilde = line.find('~').unwrap_or(0);
            for (column, ch) in line.chars().enumerate() {
                let lit = (eave && ch == '~' && (column - first_tilde) % 2 == 0)
                    || (veranda && ch == '.' && column > VERANDA_FROM);
                if lit {
                    spots.push((x + column as u16, y + row as u16));
                }
            }
        }
        spots
    }

    pub fn height(&self) -> u16 {
        Self::HEIGHT
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        x: u16,
        y: u16,
        style: &WorldSceneStyle,
        glow_windows: bool,
    ) -> io::Result<()> {
        for (i, line) in HOUSE_ASCII.lines().enumerate() {
            let row = y + i as u16;
            let wall = renderer.sky_row(row).and_then(wall);
            for column in solid_columns(i, line) {
                renderer.mark_solid(x + column as u16, row);
                if let Some(wall) = wall {
                    renderer.set_background(x + column as u16, row, wall);
                }
            }

            match i {
                // Chimney top + roof slopes
                0..=3 => {
                    for (j, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            renderer.render_char(x + j as u16, row, ch, style.roof)?;
                        }
                    }
                }
                // Roof ridge
                4 => {
                    for (j, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            renderer.render_char(x + j as u16, row, ch, style.roof)?;
                        }
                    }
                }
                // Upper and mid window rows
                5..=7 => {
                    for (j, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            let color = match ch {
                                '[' | ']' => style.window,
                                '|' | '.' | '_' => style.wood,
                                '(' | ')' => style.door,
                                '=' => style.trim,
                                _ => style.wood,
                            };
                            renderer.render_char(x + j as u16, row, ch, color)?;
                            if glow_windows && matches!(ch, '[' | ']') {
                                let (r, g, b) = WINDOW_GLOW;
                                renderer.set_background(
                                    x + j as u16,
                                    row,
                                    crossterm::style::Color::Rgb { r, g, b },
                                );
                            }
                        }
                    }
                }
                // Base wall / fence
                8 => {
                    for (j, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            let color = match ch {
                                '=' | '|' => style.trim,
                                '(' | ')' => style.door,
                                _ => style.wood,
                            };
                            renderer.render_char(x + j as u16, row, ch, color)?;
                        }
                    }
                }
                // Grass / path row
                9 => {
                    for (j, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            let color = match ch {
                                '^' => style.grass_primary,
                                '=' => style.trim,
                                _ => style.path,
                            };
                            renderer.render_char(x + j as u16, row, ch, color)?;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

const WALL_SHADE: (u8, u8, u8) = (20, 22, 32);
const NIGHT_SHADE: (u8, u8, u8) = (6, 7, 12);
const WALL_SHARE: f32 = 0.55;
const DARK_SKY: f32 = 40.0;

fn wall(sky: crossterm::style::Color) -> Option<crossterm::style::Color> {
    let crossterm::style::Color::Rgb { r, g, b } = sky else {
        return None;
    };
    let lightness = 0.3 * f32::from(r) + 0.59 * f32::from(g) + 0.11 * f32::from(b);
    let day = (lightness / DARK_SKY).min(1.0);
    let shade =
        |night: u8, lit: u8| (f32::from(night) + (f32::from(lit) - f32::from(night)) * day) as u8;
    let toward = (
        shade(NIGHT_SHADE.0, WALL_SHADE.0),
        shade(NIGHT_SHADE.1, WALL_SHADE.1),
        shade(NIGHT_SHADE.2, WALL_SHADE.2),
    );
    Some(crate::render::mix((r, g, b), toward, WALL_SHARE))
}

fn solid_columns(row: usize, line: &str) -> Vec<usize> {
    let drawn: Vec<usize> = line
        .char_indices()
        .filter(|(_, ch)| *ch != ' ')
        .map(|(column, _)| column)
        .collect();
    let (Some(&first), Some(&last)) = (drawn.first(), drawn.last()) else {
        return Vec::new();
    };
    let last = match row {
        0 => return drawn,
        5..=8 => last.min(BODY_RIGHT),
        _ => last,
    };
    let mut columns: Vec<usize> = (first..=last).collect();
    columns.extend(drawn.into_iter().filter(|column| *column > last));
    columns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_house_hides_the_painted_sky_but_the_veranda_lets_it_through() {
        let mut renderer = TerminalRenderer::headless_truecolor(70, 12);
        let sky = crossterm::style::Color::Rgb {
            r: 20,
            g: 40,
            b: 90,
        };
        renderer.paint_backdrop(&[sky; 12]);
        let style = WorldSceneStyle::resolve(&crate::scene::SceneContext {
            colors: &crate::theme::ThemeRegistry::new().active().colors.night,
            garden: crate::scene::world::trees::default_garden(),
            glow_windows: true,
        });
        House.render(&mut renderer, 0, 0, &style, true).unwrap();
        let night_wall = wall(sky).unwrap();
        assert_eq!(
            night_wall,
            crossterm::style::Color::Rgb {
                r: 19,
                g: 30,
                b: 58
            }
        );
        assert_eq!(renderer.background_at(10, 6), night_wall, "inside the wall");
        assert_eq!(renderer.background_at(2, 6), sky, "left of the house");
        assert_eq!(renderer.background_at(40, 5), sky, "above the veranda");
        assert_eq!(renderer.background_at(31, 8), night_wall, "a veranda post");
        assert_eq!(
            renderer.background_at(13, 0),
            sky,
            "between chimney and peak"
        );
        let window = HOUSE_ASCII.lines().nth(5).unwrap().find('[').unwrap() as u16;
        assert_eq!(
            renderer.background_at(window, 5),
            crossterm::style::Color::Rgb {
                r: 64,
                g: 48,
                b: 24
            }
        );
    }

    #[test]
    fn the_roof_line_runs_from_the_eave_ends_up_to_the_chimney() {
        let roof = House::roof_rows();
        assert_eq!(roof.len(), usize::from(House::WIDTH));
        assert_eq!(roof[0], None);
        assert_eq!(roof[3], Some(House::EAVE_ROW));
        assert_eq!(roof[12], Some(0), "the chimney cap");
        assert_eq!(roof[40], None, "over the veranda the rain reaches the yard");
    }

    #[test]
    fn the_eave_row_is_the_line_of_tildes() {
        let eave = HOUSE_ASCII.lines().nth(House::EAVE_ROW as usize).unwrap();
        assert!(eave.trim().chars().all(|ch| ch == '~'), "{eave}");
    }

    #[test]
    fn lights_hang_on_the_eave_and_the_veranda() {
        let rows: Vec<&str> = HOUSE_ASCII.lines().collect();
        let spots = House::light_spots(0, 0);
        let eave: Vec<_> = spots.iter().filter(|(_, y)| *y == 4).collect();
        let veranda: Vec<_> = spots.iter().filter(|(_, y)| *y == 7).collect();
        assert_eq!(eave.len() + veranda.len(), spots.len());
        assert_eq!(eave.len(), 15);
        assert!(veranda.len() >= 15, "{}", veranda.len());
        for (x, y) in &spots {
            let ch = rows[*y as usize].chars().nth(*x as usize).unwrap();
            assert!(ch == '~' || ch == '.', "{x},{y} {ch:?}");
        }
        assert_eq!(
            House::light_spots(10, 5)[0],
            (spots[0].0 + 10, spots[0].1 + 5)
        );
    }

    #[test]
    fn door_constants_match_the_art() {
        let row: Vec<char> = HOUSE_ASCII.lines().nth(7).unwrap().chars().collect();
        let left = House::DOOR_X_OFFSET as usize;
        let right = left + House::DOOR_WIDTH as usize - 1;
        assert_eq!((row[left], row[right]), ('|', '|'));
        let path: String = HOUSE_ASCII
            .lines()
            .nth(9)
            .unwrap()
            .chars()
            .skip(left + 1)
            .take(3)
            .collect();
        assert_eq!(path, "===");
    }
}
