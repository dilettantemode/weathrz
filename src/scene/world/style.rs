use crate::scene::SceneContext;
use crossterm::style::Color;

#[derive(Clone, Copy)]
pub struct WorldSceneStyle {
    pub roof: Color,
    pub wood: Color,
    pub door: Color,
    pub window: Color,
    pub trim: Color,
    pub path: Color,
    pub grass_primary: Color,
    pub grass_secondary: Color,
    pub flower_colors: [Color; 4],
    pub soil: Color,
    pub tree_foliage: Color,
    pub fence: Color,
    pub mailbox: Color,
    pub palette: [Color; 15],
}

impl WorldSceneStyle {
    pub fn resolve(ctx: &SceneContext<'_>) -> Self {
        let colors = ctx.colors;
        Self {
            roof: colors.roof,
            wood: colors.wood,
            door: colors.door,
            window: colors.window,
            trim: colors.trim,
            path: colors.path,
            grass_primary: colors.grass,
            grass_secondary: colors.grass_alt,
            flower_colors: colors.flowers,
            soil: colors.soil,
            tree_foliage: colors.tree,
            fence: colors.fence,
            mailbox: colors.mailbox,
            palette: colors.palette,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::catalogue::default_colors;

    #[test]
    fn style_copies_the_half_it_is_given() {
        let colors = default_colors();
        let night = WorldSceneStyle::resolve(&SceneContext {
            garden: crate::scene::world::trees::default_garden(),
            colors: &colors.night,
            glow_windows: false,
        });
        assert_eq!(night.roof, Color::DarkMagenta);
        assert_eq!(night.tree_foliage, Color::Rgb { r: 0, g: 50, b: 0 });
        let day = WorldSceneStyle::resolve(&SceneContext {
            garden: crate::scene::world::trees::default_garden(),
            colors: &colors.day,
            glow_windows: false,
        });
        assert_eq!(day.roof, Color::DarkRed);
        assert_eq!(day.path, Color::Reset);
        assert_eq!(day.flower_colors[3], Color::Yellow);
    }
}
