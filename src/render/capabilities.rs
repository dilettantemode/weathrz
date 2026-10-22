use crossterm::style::Color;
use std::env;
use std::io::IsTerminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    None,
    Basic,
    Ansi256,
    TrueColor,
}

#[derive(Debug, Clone)]
pub struct TerminalCapabilities {
    pub color_support: ColorSupport,
    #[allow(dead_code)]
    pub is_tty: bool,
}

impl TerminalCapabilities {
    pub fn detect() -> Self {
        let is_tty = std::io::stdout().is_terminal();

        if env::var("NO_COLOR").is_ok() {
            return Self {
                color_support: ColorSupport::None,
                is_tty,
            };
        }

        if env::var("TERM").is_ok_and(|term| term == "dumb") {
            return Self {
                color_support: ColorSupport::None,
                is_tty,
            };
        }

        if !is_tty {
            return Self {
                color_support: ColorSupport::None,
                is_tty,
            };
        }

        let color_support = if let Ok(colorterm) = env::var("COLORTERM") {
            if colorterm == "truecolor" || colorterm == "24bit" {
                ColorSupport::TrueColor
            } else {
                check_term_for_256()
            }
        } else {
            check_term_for_256()
        };

        Self {
            color_support,
            is_tty,
        }
    }

    pub fn adjust_color(&self, color: Color) -> Color {
        match (self.color_support, color) {
            (ColorSupport::None, _) => Color::Reset,
            (ColorSupport::Basic, Color::Rgb { r, g, b }) => nearest_basic(r, g, b),
            (ColorSupport::Basic, Color::AnsiValue(_)) => Color::White,
            (ColorSupport::Ansi256, Color::Rgb { r, g, b }) => {
                Color::AnsiValue(nearest_256(r, g, b))
            }
            _ => color,
        }
    }

    pub fn adjust_background(&self, color: Color) -> Color {
        self.adjust_color(color)
    }
}

const CUBE_LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];

const BASIC: [(Color, (u8, u8, u8)); 16] = [
    (Color::Black, (0, 0, 0)),
    (Color::DarkRed, (205, 0, 0)),
    (Color::DarkGreen, (0, 205, 0)),
    (Color::DarkYellow, (205, 205, 0)),
    (Color::DarkBlue, (0, 0, 238)),
    (Color::DarkMagenta, (205, 0, 205)),
    (Color::DarkCyan, (0, 205, 205)),
    (Color::Grey, (229, 229, 229)),
    (Color::DarkGrey, (127, 127, 127)),
    (Color::Red, (255, 0, 0)),
    (Color::Green, (0, 255, 0)),
    (Color::Yellow, (255, 255, 0)),
    (Color::Blue, (92, 92, 255)),
    (Color::Magenta, (255, 0, 255)),
    (Color::Cyan, (0, 255, 255)),
    (Color::White, (255, 255, 255)),
];

fn distance(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    let d = |x: u8, y: u8| (i32::from(x) - i32::from(y)).unsigned_abs();
    d(a.0, b.0).pow(2) + d(a.1, b.1).pow(2) + d(a.2, b.2).pow(2)
}

fn nearest_level(value: u8) -> usize {
    CUBE_LEVELS
        .iter()
        .enumerate()
        .min_by_key(|(_, level)| (i32::from(**level) - i32::from(value)).unsigned_abs())
        .map_or(0, |(index, _)| index)
}

pub fn nearest_256(r: u8, g: u8, b: u8) -> u8 {
    let (ri, gi, bi) = (nearest_level(r), nearest_level(g), nearest_level(b));
    let cube = (CUBE_LEVELS[ri], CUBE_LEVELS[gi], CUBE_LEVELS[bi]);
    let average = ((u32::from(r) + u32::from(g) + u32::from(b)) / 3) as u8;
    let grey_index = (i32::from(average) - 8).clamp(0, 230) as u32 / 10;
    let grey_value = (8 + 10 * grey_index) as u8;
    let grey = (grey_value, grey_value, grey_value);
    if distance((r, g, b), grey) < distance((r, g, b), cube) {
        232 + grey_index as u8
    } else {
        16 + 36 * ri as u8 + 6 * gi as u8 + bi as u8
    }
}

pub fn nearest_basic(r: u8, g: u8, b: u8) -> Color {
    BASIC
        .iter()
        .min_by_key(|(_, rgb)| distance((r, g, b), *rgb))
        .map_or(Color::White, |(color, _)| *color)
}

fn check_term_for_256() -> ColorSupport {
    if env::var("TERM").is_ok_and(|term| term.contains("256color")) {
        return ColorSupport::Ansi256;
    }
    ColorSupport::Basic
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::Color;

    #[test]
    fn test_adjust_color_none() {
        let caps = TerminalCapabilities {
            color_support: ColorSupport::None,
            is_tty: true,
        };
        assert_eq!(caps.adjust_color(Color::Red), Color::Reset);
        assert_eq!(
            caps.adjust_color(Color::Rgb { r: 255, g: 0, b: 0 }),
            Color::Reset
        );
    }

    #[test]
    fn test_adjust_color_basic() {
        let caps = TerminalCapabilities {
            color_support: ColorSupport::Basic,
            is_tty: true,
        };
        assert_eq!(caps.adjust_color(Color::Red), Color::Red);
        assert_eq!(
            caps.adjust_color(Color::Rgb { r: 255, g: 0, b: 0 }),
            Color::Red
        );
        assert_eq!(
            caps.adjust_color(Color::Rgb {
                r: 30,
                g: 30,
                b: 46
            }),
            Color::Black
        );
        assert_eq!(
            caps.adjust_color(Color::Rgb {
                r: 122,
                g: 162,
                b: 247
            }),
            Color::Blue
        );
    }

    #[test]
    fn test_adjust_color_ansi256() {
        let caps = TerminalCapabilities {
            color_support: ColorSupport::Ansi256,
            is_tty: true,
        };
        assert_eq!(caps.adjust_color(Color::Red), Color::Red);
        let rgb = Color::Rgb { r: 255, g: 0, b: 0 };
        assert_eq!(caps.adjust_color(rgb), Color::AnsiValue(196));
        assert_eq!(nearest_256(128, 128, 128), 244);
        assert_eq!(nearest_256(0, 0, 0), 16);
        assert_eq!(nearest_256(255, 255, 255), 231);
        assert_eq!(nearest_256(122, 162, 247), 111);
    }

    #[test]
    fn test_adjust_color_truecolor() {
        let caps = TerminalCapabilities {
            color_support: ColorSupport::TrueColor,
            is_tty: true,
        };
        assert_eq!(caps.adjust_color(Color::Red), Color::Red);
        let rgb = Color::Rgb { r: 255, g: 0, b: 0 };
        assert_eq!(caps.adjust_color(rgb), rgb);
    }
}
