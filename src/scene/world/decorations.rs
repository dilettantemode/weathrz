use crate::hide::HiddenDecorations;
use crate::render::TerminalRenderer;
use crate::scene::world::style::WorldSceneStyle;
use crate::scene::world::trees::{
    CANOPY, Garden, LEAVES, LEAVES_ON_CANOPY, TreeArt, WOOD, row_offset,
};
use crate::scene::world::yard::{self, Bit, Part};
use crossterm::style::Color;
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
    let top = i32::from(ground_y) - rows.len() as i32;
    let leafless = !tree.mask.contains(LEAVES);
    let solid_crown = tree.mask.contains(CANOPY);
    let holds_a_bauble = |letter: char, index: usize| {
        letter == LEAVES
            || letter == LEAVES_ON_CANOPY
            || (leafless && letter == WOOD && index < tree.sway_rows)
    };
    let darkest_sky = (0..rows.len())
        .filter_map(|index| u16::try_from(top + index as i32).ok())
        .filter_map(|y| renderer.sky_row(y))
        .min_by(|a, b| {
            let level = |color: Color| rgb(color).map_or(0.0, lightness);
            level(*a).total_cmp(&level(*b))
        });
    let crown = canopy_colours(darkest_sky, style.tree_foliage);
    let moon_ink = if crown.0 == Color::Reset {
        rgb(style.tree_foliage).map_or(Color::Reset, |leaf| {
            crate::render::mix(leaf, (0, 0, 0), 0.7)
        })
    } else {
        crown.0
    };
    for (index, (row, letters)) in rows.iter().zip(&masks).enumerate() {
        let Ok(y) = u16::try_from(top + index as i32) else {
            continue;
        };
        let offset = if !leafless && letters.contains(WOOD) {
            0
        } else {
            row_offset(index, tree.sway_rows, sway)
        };
        let filled =
            solid_crown && (letters.contains(LEAVES_ON_CANOPY) || letters.contains(CANOPY));
        let span = (
            row.find(|ch: char| ch != ' ').unwrap_or(0),
            row.rfind(|ch: char| ch != ' ').unwrap_or(0),
        );
        let (canopy, leaf) = if filled {
            crown
        } else {
            (Color::Reset, style.tree_foliage)
        };
        if letters.contains(LEAVES)
            || letters.contains(LEAVES_ON_CANOPY)
            || letters.contains(CANOPY)
        {
            for column in [span.0 as i32 - 1, span.1 as i32 + 1] {
                if let Ok(cell_x) = u16::try_from(x as i32 + column + i32::from(offset)) {
                    renderer.clear_beside(cell_x, y);
                }
            }
        }
        for ((column, ch), letter) in row.chars().enumerate().zip(letters.chars()) {
            let in_span = filled
                && (span.0..=span.1).contains(&column)
                && !((column == span.0 || column == span.1)
                    && crate::animation::solid::bare_edge(&rows, index, column, ch));
            if ch == ' ' && !in_span {
                continue;
            }
            let Some(cell_x) = (x as i32 + column as i32 + i32::from(offset))
                .try_into()
                .ok()
            else {
                continue;
            };
            if ch == ' ' {
                renderer.render_char_on(cell_x, y, ' ', Color::Reset, canopy)?;
                continue;
            }
            if let Some(phase) =
                ornaments.filter(|_| holds_a_bauble(letter, index) && is_ornament(index, column))
            {
                let bauble = BAUBLES[(index + column + phase as usize) % BAUBLES.len()];
                let color = crate::animation::flyers::mask::colour(bauble, &style.palette)
                    .unwrap_or(style.tree_foliage);
                renderer.render_char_on(cell_x, y, 'o', color, canopy)?;
                continue;
            }
            let color = match letter {
                LEAVES | LEAVES_ON_CANOPY => leaf,
                WOOD => style.wood,
                other => crate::animation::flyers::mask::colour(other, &style.palette)
                    .unwrap_or(style.tree_foliage),
            };
            if in_span {
                renderer.render_char_on(cell_x, y, ch, color, canopy)?;
            } else {
                renderer.render_char_over_disc_in(cell_x, y, ch, color, Some(moon_ink))?;
            }
        }
    }
    Ok(())
}

const SKY_IN_CANOPY: f32 = 0.15;
const SILHOUETTE: f32 = 0.7;
const LEAF_LIFT: f32 = 0.2;
const DARK_SKY: f32 = 40.0;

fn lightness((r, g, b): (u8, u8, u8)) -> f32 {
    0.3 * f32::from(r) + 0.59 * f32::from(g) + 0.11 * f32::from(b)
}

fn rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Rgb { r, g, b } => Some((r, g, b)),
        _ => None,
    }
}

fn canopy_colours(sky: Option<Color>, foliage: Color) -> (Color, Color) {
    let (Some(sky), Some(leaf)) = (sky.and_then(rgb), rgb(foliage)) else {
        return (Color::Reset, foliage);
    };
    let night = lightness(sky) < DARK_SKY;
    let shade = if night { 0.25 } else { 0.4 };
    let shadow = (
        (f32::from(leaf.0) * shade) as u8,
        (f32::from(leaf.1) * shade) as u8,
        (f32::from(leaf.2) * shade) as u8,
    );
    let mut canopy = crate::render::mix(shadow, sky, SKY_IN_CANOPY);
    if let Some(mixed) = rgb(canopy)
        && lightness(mixed) > SILHOUETTE * lightness(sky)
    {
        let scale = SILHOUETTE * lightness(sky) / lightness(mixed);
        let dim = |channel: u8| (f32::from(channel) * scale) as u8;
        canopy = Color::Rgb {
            r: dim(mixed.0),
            g: dim(mixed.1),
            b: dim(mixed.2),
        };
    }
    let lift = if night { LEAF_LIFT * 0.6 } else { LEAF_LIFT };
    (canopy, crate::render::mix(leaf, (255, 255, 200), lift))
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

    #[test]
    fn a_bare_crown_edge_stays_open() {
        let rows = [
            "    _.-~~-._",
            "  .-'.-\"`.-'-.",
            "   '-.\\%%/.-'",
            "       %%",
        ];
        let bare_edge = crate::animation::solid::bare_edge::<&str>;
        assert!(bare_edge(&rows, 1, 2, '.'), "nothing above the left corner");
        assert!(!bare_edge(&rows, 1, 4, '\''), "the next row runs under it");
        assert!(
            bare_edge(&rows, 2, 3, '\''),
            "nothing below the lower left edge"
        );
        assert!(!bare_edge(&rows, 2, 7, '\\'));
        assert!(bare_edge(&rows, 0, 4, '_'));
    }

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
                    glow_windows: false,
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
        let trunk = crown + ROUND.sway_rows;
        assert_eq!(count(&plain, crown..trunk), 0);
        assert!(count(&festive, crown..trunk) >= 3);
        assert_eq!(plain[trunk..trunk + 3], festive[trunk..trunk + 3]);
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
        let trunk = crown - 1 + ROUND.sway_rows;
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
