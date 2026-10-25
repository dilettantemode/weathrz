use crate::text::HELP_KEYS as KEYS;
const MIN_INNER: usize = 20;

fn clip(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    let mut out: String = text.chars().take(width.saturating_sub(1)).collect();
    out.push('~');
    out
}

pub fn rows(info: &[(String, String)]) -> Vec<String> {
    let mut pairs: Vec<(String, String)> = KEYS
        .iter()
        .map(|(key, what)| (key.to_string(), what.to_string()))
        .collect();
    pairs.push((String::new(), String::new()));
    pairs.extend(info.iter().cloned());
    aligned(&pairs)
}

pub fn aligned(pairs: &[(String, String)]) -> Vec<String> {
    let label_width = pairs
        .iter()
        .map(|(label, _)| label.chars().count())
        .max()
        .unwrap_or(0);
    pairs
        .iter()
        .map(|(label, value)| {
            format!("{label:<label_width$}  {value}")
                .trim_end()
                .to_string()
        })
        .collect()
}

pub fn framed(title: &str, rows: &[String], width: u16, height: u16) -> Vec<String> {
    let widest = rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0);
    let inner = widest
        .max(MIN_INNER)
        .min((width as usize).saturating_sub(6));
    if inner < 4 || (height as usize) < 5 {
        return Vec::new();
    }
    let room = (height as usize).saturating_sub(4);
    let title = format!(" {title} ");
    let mut top = format!("+-{title}");
    top.push_str(&"-".repeat((inner + 1).saturating_sub(title.chars().count())));
    top.push('+');
    let mut out = vec![top];
    for row in rows.iter().take(room) {
        let text = clip(row, inner);
        out.push(format!("| {text:<inner$} |"));
    }
    out.push(format!("+{}+", "-".repeat(inner + 2)));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info() -> Vec<(String, String)> {
        vec![
            (
                "Location".to_string(),
                "Jakarta (-6.2088, 106.8456)".to_string(),
            ),
            (
                "Config".to_string(),
                "~/.config/weathrz/config.toml".to_string(),
            ),
        ]
    }

    #[test]
    fn keys_come_first_then_the_facts_lined_up() {
        let rows = rows(&info());
        assert_eq!(rows[0], "q, Ctrl-C  quit");
        assert_eq!(rows[4], "");
        assert_eq!(rows[5], "Location   Jakarta (-6.2088, 106.8456)");
    }

    #[test]
    fn the_box_is_square_and_clips_long_rows() {
        let rows = rows(&info());
        let boxed = framed("weathrz", &rows, 120, 30);
        let widths: Vec<usize> = boxed.iter().map(|line| line.chars().count()).collect();
        assert!(
            widths.windows(2).all(|pair| pair[0] == pair[1]),
            "{boxed:#?}"
        );
        assert!(boxed[0].starts_with("+- weathrz -"));
        let narrow = framed("weathrz", &rows, 30, 30);
        assert!(narrow.iter().all(|line| line.chars().count() <= 28));
        assert!(narrow.iter().any(|line| line.contains('~')));
    }

    #[test]
    fn a_short_pane_shows_what_fits_and_a_tiny_one_nothing() {
        let rows = rows(&info());
        assert_eq!(framed("weathrz", &rows, 120, 6).len(), 4);
        assert!(framed("weathrz", &rows, 120, 4).is_empty());
        assert!(framed("weathrz", &rows, 8, 30).is_empty());
    }
}
