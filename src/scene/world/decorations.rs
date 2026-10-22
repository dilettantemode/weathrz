use crate::hide::HiddenDecorations;
use crate::render::TerminalRenderer;
use crate::scene::world::style::WorldSceneStyle;
use crate::scene::world::trees::{Garden, LEAVES, TreeArt, WOOD, row_offset};
use crate::scene::world::yard::{self, Bit, Part};
use std::io;

const FENCE_ASCII: &str = include_str!("assets/fence.txt");
const MAILBOX_ASCII: &str = include_str!("assets/mailbox.txt");

pub struct Decorations {
    hidden: HiddenDecorations,
}

pub struct DecorationLayout {
    pub horizon_y: u16,
    pub house_x: u16,
    pub house_width: u16,
    pub width: u16,
    pub garden: Garden,
}

impl Decorations {
    pub fn new(hidden: HiddenDecorations) -> Self {
        Self { hidden }
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &DecorationLayout,
        style: &WorldSceneStyle,
    ) -> io::Result<()> {
        if !self.hidden.tree {
            self.render_tree(renderer, layout, style)?;
        }
        if !self.hidden.fence {
            self.render_fence(renderer, layout, style)?;
        }
        if !self.hidden.mailbox {
            self.render_mailbox(renderer, layout, style)?;
        }
        if layout.width > 120 && !self.hidden.pine_tree {
            self.render_pine_tree(renderer, layout, style)?;
        }
        if !layout.garden.yard.road {
            if !self.hidden.pond {
                let x = i32::from(layout.house_x) + yard::POND_FROM_HOUSE;
                let y = i32::from(layout.horizon_y) + yard::POND_BELOW_GROUND;
                if x >= 0 {
                    render_bits(
                        renderer,
                        &yard::pond(&layout.garden.yard),
                        (x, y),
                        layout,
                        style,
                    )?;
                }
            }
            if !self.hidden.flowers {
                for (offset, units) in yard::BEDS_FROM_HOUSE {
                    let x = i32::from(layout.house_x) + offset;
                    let y = i32::from(layout.horizon_y) + yard::BED_BELOW_GROUND;
                    let seed = layout.garden.yard.seed.rotate_left(offset as u32);
                    render_bits(renderer, &yard::bed(units, seed), (x, y), layout, style)?;
                }
            }
        }

        Ok(())
    }

    fn render_tree(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &DecorationLayout,
        style: &WorldSceneStyle,
    ) -> io::Result<()> {
        let tree_x = layout.house_x.saturating_sub(20);
        if tree_x == 0 {
            return Ok(());
        }
        render_tree_art(
            renderer,
            layout.garden.main,
            tree_x,
            layout.horizon_y,
            layout.garden.sway,
            layout.garden.ornaments,
            style,
        )
    }

    fn render_fence(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &DecorationLayout,
        style: &WorldSceneStyle,
    ) -> io::Result<()> {
        let fence_x = layout.house_x + layout.house_width + 2;
        if fence_x >= layout.width {
            return Ok(());
        }
        let line_count = FENCE_ASCII.lines().count() as u16;
        let fence_y = layout.horizon_y.saturating_sub(line_count);
        render_art(renderer, FENCE_ASCII, fence_x, fence_y, style.fence)
    }

    fn render_mailbox(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &DecorationLayout,
        style: &WorldSceneStyle,
    ) -> io::Result<()> {
        let tree_x = layout.house_x.saturating_sub(20);
        let Some(mailbox_x) = tree_x.checked_sub(10) else {
            return Ok(());
        };
        if mailbox_x >= layout.width {
            return Ok(());
        }
        let line_count = MAILBOX_ASCII.lines().count() as u16;
        let mailbox_y = layout.horizon_y.saturating_sub(line_count);
        render_art(renderer, MAILBOX_ASCII, mailbox_x, mailbox_y, style.mailbox)
    }

    fn render_pine_tree(
        &self,
        renderer: &mut TerminalRenderer,
        layout: &DecorationLayout,
        style: &WorldSceneStyle,
    ) -> io::Result<()> {
        let second = layout.garden.second;
        let second_x = layout.house_x + layout.house_width + 18;
        if second_x + art_width(second) >= layout.width {
            return Ok(());
        }
        render_tree_art(
            renderer,
            second,
            second_x,
            layout.horizon_y,
            layout.garden.sway,
            layout.garden.ornaments,
            style,
        )
    }
}

const BAUBLES: [char; 4] = ['r', 'y', 'b', 'm'];

fn is_ornament(row: usize, column: usize) -> bool {
    (row * 7 + column * 3) % 11 == 0
}

fn art_width(tree: &TreeArt) -> u16 {
    tree.art.lines().map(|row| row.len()).max().unwrap_or(0) as u16
}

fn render_tree_art(
    renderer: &mut TerminalRenderer,
    tree: &TreeArt,
    x: u16,
    ground_y: u16,
    sway: i16,
    ornaments: Option<u64>,
    style: &WorldSceneStyle,
) -> io::Result<()> {
    let rows: Vec<&str> = tree.art.lines().collect();
    let masks: Vec<&str> = tree.mask.lines().collect();
    let top = ground_y.saturating_sub(rows.len() as u16);
    let leafless = !tree.mask.contains(LEAVES);
    let holds_a_bauble = |letter: char, index: usize| {
        letter == LEAVES || (leafless && letter == WOOD && index < tree.sway_rows)
    };
    for (index, (row, letters)) in rows.iter().zip(&masks).enumerate() {
        let offset = row_offset(index, tree.sway_rows, sway);
        for ((column, ch), letter) in row.chars().enumerate().zip(letters.chars()) {
            if ch == ' ' {
                continue;
            }
            let Some(cell_x) = (x as i32 + column as i32 + i32::from(offset))
                .try_into()
                .ok()
            else {
                continue;
            };
            if let Some(phase) =
                ornaments.filter(|_| holds_a_bauble(letter, index) && is_ornament(index, column))
            {
                let bauble = BAUBLES[(index + column + phase as usize) % BAUBLES.len()];
                let color = crate::animation::flyers::mask::colour(bauble, &style.palette)
                    .unwrap_or(style.tree_foliage);
                renderer.render_char(cell_x, top + index as u16, 'o', color)?;
                continue;
            }
            let color = match letter {
                LEAVES => style.tree_foliage,
                WOOD => style.wood,
                other => crate::animation::flyers::mask::colour(other, &style.palette)
                    .unwrap_or(style.tree_foliage),
            };
            renderer.render_char(cell_x, top + index as u16, ch, color)?;
        }
    }
    Ok(())
}

fn render_bits(
    renderer: &mut TerminalRenderer,
    bits: &[Bit],
    (x, y): (i32, i32),
    layout: &DecorationLayout,
    style: &WorldSceneStyle,
) -> io::Result<()> {
    let palette = |letter: char| {
        crate::animation::flyers::mask::colour(letter, &style.palette).unwrap_or(style.path)
    };
    for bit in bits {
        let (cell_x, cell_y) = (x + bit.dx, y + bit.dy);
        if cell_x < 0 || cell_x >= i32::from(layout.width) || cell_y < 0 {
            continue;
        }
        let color = match bit.part {
            Part::Rim => style.path,
            Part::Water => palette('b'),
            Part::Lily => palette('g'),
            Part::Ring => palette('c'),
            Part::Bloom(index) => style.flower_colors[index % style.flower_colors.len()],
            Part::Stem => style.grass_primary,
        };
        renderer.render_char(cell_x as u16, cell_y as u16, bit.ch, color)?;
    }
    Ok(())
}

fn render_art(
    renderer: &mut TerminalRenderer,
    ascii: &str,
    x: u16,
    y: u16,
    color: crossterm::style::Color,
) -> io::Result<()> {
    for (i, line) in ascii.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch != ' ' {
                renderer.render_char(x + j as u16, y + i as u16, ch, color)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::hide::HiddenDecorations;
    use crate::render::TerminalRenderer;
    use crate::scene::world::WorldScene;
    use crate::scene::world::trees::{BARE, FRUIT, Garden, ROUND};
    use crate::scene::{Scene, SceneContext};
    use crate::theme::catalogue::default_colors;

    fn draw(garden: Garden, width: u16) -> Vec<String> {
        let colors = default_colors().day;
        let scene = WorldScene::new(width, 32, HiddenDecorations::default());
        let mut renderer = TerminalRenderer::headless(width, 32);
        scene
            .render(
                &mut renderer,
                &SceneContext {
                    colors: &colors,
                    garden,
                },
            )
            .unwrap();
        renderer.text().lines().map(String::from).collect()
    }

    fn garden(sway: i16) -> Garden {
        Garden {
            main: &ROUND,
            second: &FRUIT,
            sway,
            ornaments: None,
            yard: crate::scene::world::yard::Yard::default(),
        }
    }

    #[test]
    fn christmas_hangs_ornaments_on_the_crown_only() {
        let plain = draw(garden(0), 110);
        let festive = draw(
            Garden {
                ornaments: Some(0),
                ..garden(0)
            },
            110,
        );
        let crown = plain
            .iter()
            .position(|row| row.contains(".-\"`.-'-."))
            .unwrap()
            - 1;
        let column = plain[crown + 1].find(".-\"`").unwrap().saturating_sub(4);
        let count = |rows: &[String], range: std::ops::Range<usize>| {
            rows[range]
                .iter()
                .map(|row| {
                    row.get(column..column + 16)
                        .unwrap_or("")
                        .matches('o')
                        .count()
                })
                .sum::<usize>()
        };
        assert_eq!(count(&plain, crown..crown + 6), 0);
        assert!(count(&festive, crown..crown + 6) >= 3);
        assert_eq!(plain[crown + 6..crown + 9], festive[crown + 6..crown + 9]);
    }

    #[test]
    fn a_leafless_winter_tree_still_gets_baubles_on_its_branches() {
        let bare = |ornaments| Garden {
            main: &BARE,
            second: &BARE,
            sway: 0,
            ornaments,
            yard: crate::scene::world::yard::Yard::default(),
        };
        let plain = draw(bare(None), 110);
        let festive = draw(bare(Some(0)), 110);
        let trunk = plain
            .iter()
            .position(|row| row.contains("\\//"))
            .expect("the bare tree");
        let baubles = |rows: &[String]| {
            rows[trunk - 4..trunk]
                .iter()
                .map(|row| row[..20].matches('o').count())
                .sum::<usize>()
        };
        assert_eq!(baubles(&plain), 0);
        assert!(baubles(&festive) >= 2, "{:#?}", &festive[trunk - 4..trunk]);
        assert_eq!(plain[trunk..trunk + 4], festive[trunk..trunk + 4]);
    }

    #[test]
    fn the_crown_sways_and_its_trunk_stands() {
        let still = draw(garden(0), 110);
        let swayed = draw(garden(1), 110);
        let crown = still
            .iter()
            .position(|row| row.contains(".-\"`.-'-."))
            .expect("the round tree");
        let column = still[crown].find(".-\"`").unwrap();
        assert_eq!(swayed[crown].find(".-\"`"), Some(column + 1));
        let trunk = crown + 5;
        assert_eq!(
            still[trunk].get(column..column + 12),
            swayed[trunk].get(column..column + 12)
        );
        assert!(still[trunk].contains("%%"), "{}", still[trunk]);
    }

    #[test]
    fn the_second_tree_needs_a_wide_pane() {
        let narrow = draw(garden(0), 120);
        assert!(!narrow.iter().any(|row| row.contains("#o#\\#|#/###")));
        let wide = draw(garden(0), 126);
        assert!(wide.iter().any(|row| row.contains("#o#\\#|#/###")));
    }
}
