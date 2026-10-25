pub(crate) fn bare_edge<S: AsRef<str>>(rows: &[S], index: usize, column: usize, ch: char) -> bool {
    let drawn = |row: Option<&S>| {
        row.and_then(|row| row.as_ref().chars().nth(column))
            .is_some_and(|ch| ch != ' ')
    };
    match ch {
        '_' | '.' | ',' => !drawn(index.checked_sub(1).and_then(|above| rows.get(above))),
        '\'' | '`' | '"' => !drawn(rows.get(index + 1)),
        _ => false,
    }
}

pub(crate) fn solid_cells<S: AsRef<str>>(rows: &[S]) -> Vec<Vec<bool>> {
    let grid: Vec<Vec<char>> = rows
        .iter()
        .map(|row| row.as_ref().chars().collect())
        .collect();
    let height = grid.len();
    let width = grid.iter().map(Vec::len).max().unwrap_or(0);
    let open = |x: usize, y: usize| {
        x == 0
            || y == 0
            || x == width + 1
            || y == height + 1
            || grid
                .get(y - 1)
                .and_then(|row| row.get(x - 1))
                .is_none_or(|ch| *ch == ' ')
    };
    let mut outside = vec![vec![false; width + 2]; height + 2];
    let mut stack = vec![(0, 0)];
    while let Some((x, y)) = stack.pop() {
        if outside[y][x] || !open(x, y) {
            continue;
        }
        outside[y][x] = true;
        if x > 0 {
            stack.push((x - 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if x <= width {
            stack.push((x + 1, y));
        }
        if y <= height {
            stack.push((x, y + 1));
        }
    }
    (0..height)
        .map(|y| (0..width).map(|x| !outside[y + 1][x + 1]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn picture(rows: &[&str]) -> Vec<String> {
        solid_cells(rows)
            .iter()
            .map(|row| row.iter().map(|on| if *on { '#' } else { '.' }).collect())
            .collect()
    }

    #[test]
    fn a_closed_outline_fills_and_an_open_one_does_not() {
        assert_eq!(
            picture(&[" .--. ", "(    )", " `--' "]),
            [".####.", "######", ".####."]
        );
        assert_eq!(picture(&["|  |", "|__|"]), ["#..#", "####"]);
    }

    #[test]
    fn short_rows_count_as_open_space() {
        assert_eq!(picture(&["(  )", "("]), ["#..#", "#..."]);
    }
}
