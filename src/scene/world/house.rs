use crate::render::TerminalRenderer;
use crate::scene::world::style::WorldSceneStyle;
use std::io;

const HOUSE_ASCII: &str = include_str!("assets/house.txt");
const VERANDA_FROM: usize = 28;

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
    ) -> io::Result<()> {
        for (i, line) in HOUSE_ASCII.lines().enumerate() {
            let row = y + i as u16;

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

#[cfg(test)]
mod tests {
    use super::*;

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
