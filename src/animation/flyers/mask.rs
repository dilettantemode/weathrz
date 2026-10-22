use super::catalogue::Sprite;
use crossterm::style::Color;

pub const LETTERS: [char; 15] = [
    'f', 'u', 's', 'r', 'o', 'y', 'g', 'G', 'c', 'b', 'B', 'm', 'n', 'N', 't',
];

pub fn colour(letter: char, palette: &[Color; 15]) -> Option<Color> {
    LETTERS
        .iter()
        .position(|&known| known == letter)
        .map(|slot| palette[slot])
}

pub fn mirror_mask(mask: &Sprite, width: usize) -> Sprite {
    Sprite {
        rows: mask
            .rows
            .iter()
            .map(|row| {
                format!("{row:<width$}")
                    .chars()
                    .rev()
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::colors::SLOT_NAMES;

    #[test]
    fn letters_name_the_palette_slots_in_order() {
        let named: Vec<(char, &str)> = LETTERS.iter().copied().zip(SLOT_NAMES).collect();
        assert_eq!(
            named,
            [
                ('f', "fg"),
                ('u', "muted"),
                ('s', "subtle"),
                ('r', "red"),
                ('o', "orange"),
                ('y', "yellow"),
                ('g', "green"),
                ('G', "green_dark"),
                ('c', "cyan"),
                ('b', "blue"),
                ('B', "blue_dark"),
                ('m', "magenta"),
                ('n', "brown"),
                ('N', "brown_dark"),
                ('t', "tan"),
            ]
        );
    }

    #[test]
    fn a_letter_picks_its_palette_colour() {
        let palette: [Color; 15] = std::array::from_fn(|i| Color::AnsiValue(i as u8));
        assert_eq!(colour('f', &palette), Some(Color::AnsiValue(0)));
        assert_eq!(colour('G', &palette), Some(Color::AnsiValue(7)));
        assert_eq!(colour('t', &palette), Some(Color::AnsiValue(14)));
        assert_eq!(colour('x', &palette), None);
        assert_eq!(colour(' ', &palette), None);
    }
}
