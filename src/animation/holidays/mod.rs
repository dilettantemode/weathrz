pub mod bonfire;
pub mod bunting;
pub mod confetti;
pub mod flagpole;
pub mod ground;
pub mod hangings;
pub mod holi;
pub mod lanterns;
pub mod mudik;
pub mod songkran;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cell {
    pub dx: i32,
    pub dy: i32,
    pub ch: char,
    pub letter: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Placed {
    pub x: u16,
    pub y: u16,
    pub ch: char,
    pub letter: char,
    pub halves: Option<(char, char)>,
}

pub(crate) fn place(
    x: i32,
    y: i32,
    ch: char,
    letter: char,
    width: u16,
    height: u16,
) -> Option<Placed> {
    ((0..i32::from(width)).contains(&x) && (0..i32::from(height)).contains(&y)).then_some(Placed {
        x: x as u16,
        y: y as u16,
        ch,
        letter,
        halves: None,
    })
}

pub(crate) fn cells(art: &str, mask: &str) -> Vec<Cell> {
    let mut out = Vec::new();
    for (dy, (art_row, mask_row)) in art.lines().zip(mask.lines()).enumerate() {
        for (dx, (ch, letter)) in art_row.chars().zip(mask_row.chars()).enumerate() {
            if ch != ' ' {
                out.push(Cell {
                    dx: dx as i32,
                    dy: dy as i32,
                    ch,
                    letter,
                });
            }
        }
    }
    out
}

#[cfg(test)]
pub(crate) fn assert_mask_fits(name: &str, art: &str, mask: &str) {
    let shape = |text: &str| -> Vec<Vec<bool>> {
        text.lines()
            .map(|row| row.trim_end().chars().map(|ch| ch != ' ').collect())
            .collect()
    };
    assert_eq!(shape(art), shape(mask), "{name}: mask must copy the art");
    for letter in mask.chars().filter(|ch| !ch.is_whitespace()) {
        assert!(
            crate::animation::flyers::mask::LETTERS.contains(&letter),
            "{name}: unknown mask letter {letter:?}"
        );
    }
}
