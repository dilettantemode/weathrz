use crate::theme::Theme;
use crate::theme::colors::ThemeColors;
use crate::theme::load;

pub fn default_colors() -> ThemeColors {
    load::resolve("default", None).colors
}

pub fn default_theme() -> Theme {
    Theme {
        id: "default",
        display_name: "Default",
        scene_id: "world",
        overlay_id: None,
        colors: default_colors(),
    }
}
