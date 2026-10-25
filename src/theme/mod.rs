pub mod catalogue;
pub mod colors;
pub mod load;
pub mod value;

use std::collections::HashMap;
use std::fmt;

use colors::ThemeColors;
use load::ResolvedTheme;

pub struct Theme {
    pub id: &'static str,
    #[allow(dead_code)]
    pub display_name: &'static str,
    pub scene_id: &'static str,
    pub overlay_id: Option<&'static str>,
    pub colors: ThemeColors,
}

#[derive(Debug)]
pub enum ThemeError {
    NotFound(String),
    #[allow(dead_code)]
    SceneNotRegistered {
        theme: &'static str,
        scene: &'static str,
    },
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeError::NotFound(id) => write!(f, "theme '{}' is not registered", id),
            ThemeError::SceneNotRegistered { theme, scene } => {
                write!(
                    f,
                    "theme '{}' references unregistered scene '{}'",
                    theme, scene
                )
            }
        }
    }
}

pub struct ThemeRegistry {
    themes: HashMap<&'static str, Theme>,
    active: &'static str,
}

impl ThemeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            themes: HashMap::new(),
            active: "default",
        };
        registry.register(catalogue::default_theme());
        registry
    }

    pub fn register(&mut self, theme: Theme) {
        self.themes.insert(theme.id, theme);
    }

    pub fn activate_resolved(&mut self, resolved: ResolvedTheme) {
        let id: &'static str = Box::leak(resolved.id.into_boxed_str());
        let display_name: &'static str = Box::leak(resolved.display_name.into_boxed_str());
        self.register(Theme {
            id,
            display_name,
            scene_id: "world",
            overlay_id: None,
            colors: resolved.colors,
        });
        self.set_active(id)
            .expect("a theme that was just registered can be activated");
    }

    pub fn set_active(&mut self, id: &str) -> Result<(), ThemeError> {
        match self.themes.get_key_value(id) {
            Some((&static_id, _)) => {
                self.active = static_id;
                Ok(())
            }
            None => Err(ThemeError::NotFound(id.to_owned())),
        }
    }

    pub fn active(&self) -> &Theme {
        self.themes
            .get(self.active)
            .expect("active theme id must always reference a registered theme")
    }

    pub fn get(&self, id: &str) -> Option<&Theme> {
        self.themes.get(id)
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self::new()
    }
}
