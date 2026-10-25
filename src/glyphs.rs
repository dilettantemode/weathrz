#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Glyphs {
    Ascii,
    #[default]
    Unicode,
    Fine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Rain,
    Snow,
    Star,
    Fog,
    Cloth,
}

pub fn swap(glyphs: Glyphs, element: Element, ch: char) -> char {
    if glyphs == Glyphs::Ascii {
        return ch;
    }
    match (element, ch) {
        (Element::Rain, '|') => '│',
        (Element::Rain, ':') => '╎',
        (Element::Rain, '/') => '╱',
        (Element::Rain, '\\') => '╲',
        (Element::Rain, '.') => '╷',
        (Element::Snow | Element::Star | Element::Fog, '.') => '·',
        (Element::Snow, '*') => '❄',
        (Element::Star, '*') => '✦',
        (Element::Star, '+') => '✧',
        (Element::Fog, '-' | '~') => '░',
        (Element::Fog, ',') => '·',
        (Element::Cloth, '=') => '█',
        (Element::Cloth, '~') => '▓',
        _ => ch,
    }
}

pub fn parse(text: &str) -> Result<Glyphs, String> {
    match text.trim() {
        "ascii" => Ok(Glyphs::Ascii),
        "" | "unicode" => Ok(Glyphs::Unicode),
        "fine" => Ok(Glyphs::Fine),
        other => Err(format!(
            "glyphs = \"{other}\" is not valid (use \"unicode\", \"fine\" or \"ascii\"); using \"unicode\""
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_swaps_only_the_listed_characters() {
        assert_eq!(swap(Glyphs::Ascii, Element::Rain, '|'), '|');
        assert_eq!(swap(Glyphs::Unicode, Element::Rain, '|'), '│');
        assert_eq!(swap(Glyphs::Unicode, Element::Rain, '\\'), '╲');
        assert_eq!(swap(Glyphs::Unicode, Element::Star, '*'), '✦');
        assert_eq!(swap(Glyphs::Unicode, Element::Snow, '*'), '❄');
        assert_eq!(swap(Glyphs::Unicode, Element::Fog, '~'), '░');
        assert_eq!(swap(Glyphs::Unicode, Element::Cloth, '='), '█');
        assert_eq!(swap(Glyphs::Unicode, Element::Cloth, '@'), '@');
        assert_eq!(swap(Glyphs::Unicode, Element::Rain, ','), ',');
    }

    #[test]
    fn glyph_styles_parse_and_unknown_ones_warn() {
        assert_eq!(parse(""), Ok(Glyphs::Unicode));
        assert_eq!(parse("ascii"), Ok(Glyphs::Ascii));
        assert_eq!(parse(" unicode "), Ok(Glyphs::Unicode));
        assert!(parse("braille").unwrap_err().contains("braille"));
    }
}
