use super::catalogue::Sprite;

const PAIRS: [(char, char); 6] = [
    ('/', '\\'),
    ('(', ')'),
    ('<', '>'),
    ('[', ']'),
    ('{', '}'),
    ('`', '\''),
];
const SYMMETRIC: &str = " -_=|.,\"*:;~^+#!@%08AHIMOTUVWXYilovwx";

pub fn mirror_char(ch: char) -> Option<char> {
    if SYMMETRIC.contains(ch) {
        return Some(ch);
    }
    PAIRS.iter().find_map(|&(a, b)| {
        if ch == a {
            Some(b)
        } else if ch == b {
            Some(a)
        } else {
            None
        }
    })
}

pub fn mirror_sprite(sprite: &Sprite) -> Option<Sprite> {
    mirror_sprite_at(sprite, sprite.width() as usize)
}

pub fn mirror_sprite_at(sprite: &Sprite, width: usize) -> Option<Sprite> {
    let rows = sprite
        .rows
        .iter()
        .map(|row| {
            format!("{row:<width$}")
                .chars()
                .rev()
                .map(mirror_char)
                .collect::<Option<String>>()
                .map(|mirrored| mirrored.trim_end().to_string())
        })
        .collect::<Option<Vec<String>>>()?;
    Some(Sprite { rows })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::flyers::catalogue::{CATALOGUE, Fill, parse_asset};

    #[test]
    fn pairs_swap_and_symmetric_characters_stay() {
        assert_eq!(mirror_char('/'), Some('\\'));
        assert_eq!(mirror_char(')'), Some('('));
        assert_eq!(mirror_char('>'), Some('<'));
        assert_eq!(mirror_char('o'), Some('o'));
        assert_eq!(mirror_char('`'), Some('\''));
        assert_eq!(mirror_char('\''), Some('`'));
        assert_eq!(mirror_char('P'), None);
    }

    #[test]
    fn mirroring_reverses_rows_and_twice_gives_the_original() {
        let sprite = Sprite {
            rows: vec!["  /\\".to_string(), "<(__)".to_string()],
        };
        let mirrored = mirror_sprite(&sprite).unwrap();
        assert_eq!(mirrored.rows, vec![" /\\".to_string(), "(__)>".to_string()]);
        assert_eq!(mirror_sprite(&mirrored), Some(sprite));
        assert_eq!(
            mirror_sprite(&Sprite {
                rows: vec!["AIR CO".to_string()]
            }),
            None
        );
    }

    #[test]
    fn the_mirrored_set_matches_the_rule() {
        let mut safe: Vec<&str> = CATALOGUE
            .iter()
            .filter(|def| def.right.is_none() != def.left.is_none())
            .filter(|def| def.fill != Fill::Same)
            .filter(|def| !matches!(def.id, "trijet" | "banner"))
            .filter(|def| {
                parse_asset(def.right.or(def.left).unwrap())
                    .iter()
                    .all(|sprite| mirror_sprite(sprite).is_some())
            })
            .map(|def| def.id)
            .collect();
        safe.sort();
        let mut flagged: Vec<&str> = CATALOGUE
            .iter()
            .filter(|def| def.fill == Fill::Mirror)
            .map(|def| def.id)
            .collect();
        flagged.sort();
        assert_eq!(safe, flagged);
        assert_eq!(
            flagged,
            [
                "airship",
                "biplane",
                "blimp",
                "fighter",
                "mini-plane",
                "prop",
                "sleigh"
            ]
        );
    }
}
