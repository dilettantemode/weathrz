use crossterm::style::Color;

const DIM: f64 = 0.55;

pub fn dim(color: Color) -> Color {
    match color {
        Color::Rgb { r, g, b } => {
            let scale = |channel: u8| (f64::from(channel) * DIM).round() as u8;
            Color::Rgb {
                r: scale(r),
                g: scale(g),
                b: scale(b),
            }
        }
        Color::Red => Color::DarkRed,
        Color::Green => Color::DarkGreen,
        Color::Yellow => Color::DarkYellow,
        Color::Blue => Color::DarkBlue,
        Color::Magenta => Color::DarkMagenta,
        Color::Cyan => Color::DarkCyan,
        Color::White => Color::Grey,
        Color::Grey => Color::DarkGrey,
        other => other,
    }
}

pub fn parse_concrete(raw: &str) -> Option<Color> {
    let value = raw.trim();
    if value == "default" {
        return Some(Color::Reset);
    }
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex(hex);
    }
    if let Some(name) = value.strip_prefix("ansi:") {
        return parse_ansi(name);
    }
    None
}

fn parse_hex(hex: &str) -> Option<Color> {
    if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let byte = |start: usize| u8::from_str_radix(&hex[start..start + 2], 16).ok();
    Some(Color::Rgb {
        r: byte(0)?,
        g: byte(2)?,
        b: byte(4)?,
    })
}

fn parse_ansi(name: &str) -> Option<Color> {
    let color = match name {
        "black" => Color::Black,
        "dark_red" => Color::DarkRed,
        "dark_green" => Color::DarkGreen,
        "dark_yellow" => Color::DarkYellow,
        "dark_blue" => Color::DarkBlue,
        "dark_magenta" => Color::DarkMagenta,
        "dark_cyan" => Color::DarkCyan,
        "grey" => Color::Grey,
        "dark_grey" => Color::DarkGrey,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => return None,
    };
    Some(color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dims_colours_for_night() {
        assert_eq!(
            dim(Color::Rgb {
                r: 200,
                g: 100,
                b: 50
            }),
            Color::Rgb {
                r: 110,
                g: 55,
                b: 28
            }
        );
        assert_eq!(dim(Color::Magenta), Color::DarkMagenta);
        assert_eq!(dim(Color::Yellow), Color::DarkYellow);
        assert_eq!(dim(Color::White), Color::Grey);
        assert_eq!(dim(Color::DarkRed), Color::DarkRed);
        assert_eq!(dim(Color::Reset), Color::Reset);
    }

    #[test]
    fn parses_hex_in_any_case_with_whitespace() {
        assert_eq!(
            parse_concrete("#7aa2f7"),
            Some(Color::Rgb {
                r: 0x7a,
                g: 0xa2,
                b: 0xf7
            })
        );
        assert_eq!(
            parse_concrete("  #FFFFFF "),
            Some(Color::Rgb {
                r: 255,
                g: 255,
                b: 255
            })
        );
    }

    #[test]
    fn parses_every_ansi_name() {
        let cases = [
            ("black", Color::Black),
            ("dark_red", Color::DarkRed),
            ("dark_green", Color::DarkGreen),
            ("dark_yellow", Color::DarkYellow),
            ("dark_blue", Color::DarkBlue),
            ("dark_magenta", Color::DarkMagenta),
            ("dark_cyan", Color::DarkCyan),
            ("grey", Color::Grey),
            ("dark_grey", Color::DarkGrey),
            ("red", Color::Red),
            ("green", Color::Green),
            ("yellow", Color::Yellow),
            ("blue", Color::Blue),
            ("magenta", Color::Magenta),
            ("cyan", Color::Cyan),
            ("white", Color::White),
        ];
        for (name, expected) in cases {
            assert_eq!(
                parse_concrete(&format!("ansi:{name}")),
                Some(expected),
                "{name}"
            );
        }
    }

    #[test]
    fn parses_default_as_reset() {
        assert_eq!(parse_concrete("default"), Some(Color::Reset));
    }

    #[test]
    fn rejects_malformed_values() {
        for raw in [
            "",
            "#12345",
            "#1234567",
            "#gggggg",
            "#ééé",
            "ansi:orange",
            "ansi:",
            "red",
            "rgb(1,2,3)",
            "Default",
        ] {
            assert_eq!(parse_concrete(raw), None, "{raw:?}");
        }
    }
}
