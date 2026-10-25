use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Envelope {
    pub spans: Vec<Option<(usize, usize)>>,
    pub stripe: Option<usize>,
}

pub(crate) fn envelope<S: AsRef<str>>(rows: &[S], solid: &[Vec<bool>]) -> Envelope {
    let grid: Vec<Vec<char>> = rows
        .iter()
        .map(|row| row.as_ref().chars().collect())
        .collect();
    let glyph = |x: usize, y: usize| grid[y].get(x).is_some_and(|ch| *ch != ' ');
    let inside = |x: usize, y: usize| {
        solid
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(false)
            && !glyph(x, y)
    };
    let mut seen: Vec<Vec<bool>> = solid.iter().map(|row| vec![false; row.len()]).collect();
    let mut largest: Vec<(usize, usize)> = Vec::new();
    for (y, row) in solid.iter().enumerate() {
        for x in 0..row.len() {
            if seen[y][x] || !inside(x, y) {
                continue;
            }
            let mut part = Vec::new();
            let mut queue = VecDeque::from([(x, y)]);
            seen[y][x] = true;
            while let Some((cx, cy)) = queue.pop_front() {
                part.push((cx, cy));
                let mut next = vec![(cx + 1, cy), (cx, cy + 1)];
                if cx > 0 {
                    next.push((cx - 1, cy));
                }
                if cy > 0 {
                    next.push((cx, cy - 1));
                }
                for (nx, ny) in next {
                    if ny < solid.len() && nx < solid[ny].len() && !seen[ny][nx] && inside(nx, ny) {
                        seen[ny][nx] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
            if part.len() > largest.len() {
                largest = part;
            }
        }
    }
    let mut spans = vec![None; grid.len()];
    let mut body_rows = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        let Some(low) = largest
            .iter()
            .filter(|cell| cell.1 == y)
            .map(|cell| cell.0)
            .min()
        else {
            continue;
        };
        let high = largest
            .iter()
            .filter(|cell| cell.1 == y)
            .map(|cell| cell.0)
            .max()
            .unwrap_or(low);
        let mut first = low;
        while first > 0 && glyph(first - 1, y) {
            first -= 1;
        }
        let mut last = high;
        while last + 1 < row.len() && glyph(last + 1, y) {
            last += 1;
        }
        spans[y] = Some((first, last));
        body_rows.push(y);
    }
    if let Some(&bottom) = body_rows.last() {
        for (y, row) in grid.iter().enumerate().skip(bottom + 1) {
            let first = row.iter().position(|ch| *ch != ' ');
            let last = row.iter().rposition(|ch| *ch != ' ');
            if let (Some(first), Some(last)) = (first, last) {
                spans[y] = Some((first, last));
            }
        }
    }
    Envelope {
        spans,
        stripe: body_rows.get(body_rows.len() / 2).copied(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_envelope_is_the_biggest_bag_and_leaves_the_fins_out() {
        let rows = [
            "     _..--=--..._",
            "  .-'            '-.  .-.",
            " /.'              '.\\/  /",
            "|=-                -=| (",
            " \\'.              .'/\\  \\",
            "  '-.,_____ _____.-'  '-'",
            "       [_____]=8",
        ];
        let solid = crate::animation::solid::solid_cells(&rows);
        let bag = envelope(&rows, &solid);
        assert_eq!(bag.spans[0], None, "the top line stays on the sky");
        assert_eq!(bag.spans[1], Some((2, 19)));
        assert_eq!(bag.spans[3], Some((0, 21)), "the fin stays out");
        assert_eq!(bag.spans[5], Some((2, 19)));
        assert_eq!(bag.spans[6], Some((7, 15)), "the gondola");
        assert_eq!(bag.stripe, Some(3));
    }

    #[test]
    fn a_drawing_with_no_hollow_has_no_envelope() {
        let rows = ["<==>", "=[]="];
        let solid = crate::animation::solid::solid_cells(&rows);
        let bag = envelope(&rows, &solid);
        assert!(bag.spans.iter().all(Option::is_none));
        assert_eq!(bag.stripe, None);
    }
}
