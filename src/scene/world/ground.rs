use crate::render::TerminalRenderer;
use crate::scene::world::style::WorldSceneStyle;
use crate::scene::world::yard::{self, Yard};
use std::io;

pub struct Ground;

impl Ground {
    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        (width, height, y_start): (u16, u16, u16),
        yard: &Yard,
        style: &WorldSceneStyle,
    ) -> io::Result<()> {
        let width = width as usize;
        let height = height as usize;

        for y in 0..height {
            for x in 0..width {
                let (ch, color) = if y == 0 {
                    let r = pseudo_rand(x, y);
                    if r < 5 {
                        (
                            '*',
                            style.flower_colors[(x + y) % style.flower_colors.len()],
                        )
                    } else if r < 15 {
                        (yard::grass(',', x as u16, yard), style.grass_secondary)
                    } else {
                        (yard::grass('^', x as u16, yard), style.grass_primary)
                    }
                } else if let Some(ch) = yard::road_row(y as u16).filter(|_| yard.road) {
                    (ch, style.path)
                } else {
                    let r = pseudo_rand(x, y);
                    let ch = if r < 20 {
                        '~'
                    } else if r < 25 {
                        '.'
                    } else {
                        ' '
                    };
                    (ch, style.soil)
                };

                renderer.render_char(x as u16, y_start + y as u16, ch, color)?;
            }
        }

        Ok(())
    }
}

fn pseudo_rand(x: usize, y: usize) -> u32 {
    ((x as u32 ^ 0x5DEECE6).wrapping_mul(y as u32 ^ 0xB)) % 100
}
