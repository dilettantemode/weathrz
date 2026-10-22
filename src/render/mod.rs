mod capabilities;

use crate::error::TerminalError;
use capabilities::TerminalCapabilities;
use crossterm::{
    cursor, event, execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        self, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{self, BufWriter, IsTerminal, Stdout, Write};
use std::time::Duration;

const STARTUP_SIZE_TIMEOUT: Duration = Duration::from_secs(2);
const STARTUP_SIZE_POLL: Duration = Duration::from_millis(25);

const MAX_TERMINAL_WIDTH: u16 = 1000;
const MAX_TERMINAL_HEIGHT: u16 = 500;

fn clamp_terminal_size(width: u16, height: u16) -> (u16, u16) {
    (
        width.min(MAX_TERMINAL_WIDTH),
        height.min(MAX_TERMINAL_HEIGHT),
    )
}

fn wait_for_size(
    mut query: impl FnMut() -> io::Result<(u16, u16)>,
    timeout: Duration,
    interval: Duration,
) -> io::Result<(u16, u16)> {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let (width, height) = query()?;
        if (width > 0 && height > 0) || std::time::Instant::now() >= deadline {
            return Ok((width, height));
        }
        std::thread::sleep(interval);
    }
}

fn printable(ch: char) -> char {
    if ch.is_control() { '?' } else { ch }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    character: char,
    color: Color,
    background: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            color: Color::Reset,
            background: Color::Reset,
        }
    }
}

pub struct TerminalRenderer {
    stdout: BufWriter<Stdout>,
    width: u16,
    height: u16,
    buffer: Vec<Cell>,
    last_buffer: Vec<Cell>,
    capabilities: TerminalCapabilities,
}

impl TerminalRenderer {
    pub fn new() -> Result<Self, TerminalError> {
        if !io::stdout().is_terminal() {
            return Err(TerminalError::NotATty);
        }

        let (width, height) =
            wait_for_size(terminal::size, STARTUP_SIZE_TIMEOUT, STARTUP_SIZE_POLL)
                .map_err(TerminalError::SizeError)?;

        let (width, height) = clamp_terminal_size(width, height);

        let stdout = BufWriter::new(io::stdout());
        let buffer_size = (width as usize) * (height as usize);
        let capabilities = TerminalCapabilities::detect();

        Ok(Self {
            stdout,
            width,
            height,
            buffer: vec![Cell::default(); buffer_size],
            last_buffer: vec![Cell::default(); buffer_size],
            capabilities,
        })
    }

    #[cfg(test)]
    pub(crate) fn headless(width: u16, height: u16) -> Self {
        let buffer_size = (width as usize) * (height as usize);
        Self {
            stdout: BufWriter::new(io::stdout()),
            width,
            height,
            buffer: vec![Cell::default(); buffer_size],
            last_buffer: vec![Cell::default(); buffer_size],
            capabilities: TerminalCapabilities::detect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn text(&self) -> String {
        (0..self.height)
            .map(|y| {
                let start = (y as usize) * (self.width as usize);
                self.buffer[start..start + self.width as usize]
                    .iter()
                    .map(|cell| cell.character)
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn init(&mut self) -> Result<(), TerminalError> {
        terminal::enable_raw_mode().map_err(TerminalError::RawModeError)?;
        execute!(
            self.stdout,
            EnterAlternateScreen,
            cursor::Hide,
            event::EnableFocusChange
        )
        .map_err(TerminalError::InitError)?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            event::DisableFocusChange,
            LeaveAlternateScreen,
            cursor::Show,
            ResetColor
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn manual_resize(&mut self, width: u16, height: u16) -> io::Result<()> {
        let (width, height) = clamp_terminal_size(width, height);
        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            let buffer_size = (width as usize) * (height as usize);
            self.buffer = vec![Cell::default(); buffer_size];
            self.last_buffer = vec![Cell::default(); buffer_size];
            execute!(self.stdout, Clear(ClearType::All))?;
        }
        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.buffer.fill(Cell::default());
        Ok(())
    }

    pub fn render_centered_colored(
        &mut self,
        lines: &[String],
        start_row: u16,
        color: Color,
    ) -> io::Result<()> {
        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let start_col = if self.width as usize > max_width {
            (self.width as usize - max_width) / 2
        } else {
            0
        };
        self.render_colored_at(lines, start_col as u16, start_row, color)
    }

    pub fn render_colored_at(
        &mut self,
        lines: &[String],
        start_col: u16,
        start_row: u16,
        color: Color,
    ) -> io::Result<()> {
        let adjusted_color = self.capabilities.adjust_color(color);

        for (idx, line) in lines.iter().enumerate() {
            let row = start_row + idx as u16;
            if row < self.height {
                for (char_idx, ch) in line.chars().enumerate() {
                    let col = start_col.saturating_add(char_idx as u16);
                    if col < self.width {
                        let buffer_idx = (row as usize) * (self.width as usize) + (col as usize);
                        if buffer_idx < self.buffer.len() {
                            self.buffer[buffer_idx] = Cell {
                                character: printable(ch),
                                color: adjusted_color,
                                background: Color::Reset,
                            };
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn render_line_colored(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        color: Color,
    ) -> io::Result<()> {
        if y >= self.height {
            return Ok(());
        }
        let adjusted_color = self.capabilities.adjust_color(color);

        for (idx, ch) in text.chars().enumerate() {
            let col = x + idx as u16;
            if col < self.width {
                let buffer_idx = (y as usize) * (self.width as usize) + (col as usize);
                if buffer_idx < self.buffer.len() {
                    self.buffer[buffer_idx] = Cell {
                        character: printable(ch),
                        color: adjusted_color,
                        background: Color::Reset,
                    };
                }
            }
        }
        Ok(())
    }

    pub fn render_char(&mut self, x: u16, y: u16, ch: char, color: Color) -> io::Result<()> {
        self.render_char_on(x, y, ch, color, Color::Reset)
    }

    pub fn render_char_on(
        &mut self,
        x: u16,
        y: u16,
        ch: char,
        color: Color,
        background: Color,
    ) -> io::Result<()> {
        if x < self.width && y < self.height {
            let buffer_idx = (y as usize) * (self.width as usize) + (x as usize);
            if buffer_idx < self.buffer.len() {
                self.buffer[buffer_idx] = Cell {
                    character: printable(ch),
                    color: self.capabilities.adjust_color(color),
                    background: self.capabilities.adjust_background(background),
                };
            }
        }
        Ok(())
    }

    pub fn flash_screen(&mut self, color: Color) -> io::Result<()> {
        let flash_color = self.capabilities.adjust_color(color);
        for cell in &mut self.buffer {
            cell.color = flash_color;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        let mut synchronized = false;
        let result = self.write_changes(&mut synchronized);
        if synchronized {
            queue!(self.stdout, EndSynchronizedUpdate)?;
        }
        result?;
        self.stdout.flush()?;
        self.last_buffer.copy_from_slice(&self.buffer);
        Ok(())
    }

    fn write_changes(&mut self, synchronized: &mut bool) -> io::Result<()> {
        let mut current_color = Color::Reset;
        let mut current_background = Color::Reset;
        let mut last_pos: Option<(u16, u16)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);

                if idx >= self.buffer.len() || idx >= self.last_buffer.len() {
                    continue;
                }

                let cell = self.buffer[idx];
                let last_cell = self.last_buffer[idx];

                if cell != last_cell {
                    if !*synchronized {
                        queue!(self.stdout, BeginSynchronizedUpdate)?;
                        *synchronized = true;
                    }
                    let expected_pos = last_pos.map(|(lx, ly)| (lx + 1, ly));
                    if expected_pos != Some((x, y)) {
                        queue!(self.stdout, cursor::MoveTo(x, y))?;
                    }

                    if cell.color != current_color {
                        queue!(self.stdout, SetForegroundColor(cell.color))?;
                        current_color = cell.color;
                    }
                    if cell.background != current_background {
                        queue!(self.stdout, SetBackgroundColor(cell.background))?;
                        current_background = cell.background;
                    }

                    queue!(self.stdout, Print(cell.character))?;
                    last_pos = Some((x, y));
                }
            }
        }

        if current_color != Color::Reset || current_background != Color::Reset {
            queue!(self.stdout, ResetColor)?;
        }
        Ok(())
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    const INJECTION: &str = "\u{1b}]2;INJECTED\u{7}Jakarta\u{9b}31m\u{7f}\n\t";

    fn renderer(width: u16, height: u16) -> TerminalRenderer {
        TerminalRenderer::headless(width, height)
    }

    fn row(renderer: &TerminalRenderer, y: u16) -> String {
        let start = (y as usize) * (renderer.width as usize);
        renderer.buffer[start..start + renderer.width as usize]
            .iter()
            .map(|cell| cell.character)
            .collect()
    }

    fn sizes(answers: Vec<(u16, u16)>) -> impl FnMut() -> io::Result<(u16, u16)> {
        let mut answers = answers.into_iter();
        let mut last = (0, 0);
        move || {
            if let Some(next) = answers.next() {
                last = next;
            }
            Ok(last)
        }
    }

    #[test]
    fn wait_for_size_returns_once_the_pane_has_a_size() {
        let size = wait_for_size(
            sizes(vec![(0, 0), (0, 0), (120, 30)]),
            Duration::from_secs(2),
            Duration::from_millis(1),
        )
        .unwrap();
        assert_eq!(size, (120, 30));
    }

    #[test]
    fn wait_for_size_returns_immediately_for_a_sized_terminal() {
        let started = Instant::now();
        let size = wait_for_size(
            sizes(vec![(80, 24)]),
            Duration::from_secs(2),
            Duration::from_millis(50),
        )
        .unwrap();
        assert_eq!(size, (80, 24));
        assert!(started.elapsed() < Duration::from_millis(50));
    }

    #[test]
    fn wait_for_size_gives_up_with_the_last_answer() {
        let size = wait_for_size(
            sizes(vec![(0, 0)]),
            Duration::from_millis(20),
            Duration::from_millis(1),
        )
        .unwrap();
        assert_eq!(size, (0, 0));
    }

    #[test]
    fn printable_replaces_control_characters() {
        for ch in ['\u{1b}', '\u{7}', '\u{9b}', '\u{7f}', '\n', '\t', '\0'] {
            assert_eq!(printable(ch), '?', "{:?}", ch);
        }
    }

    #[test]
    fn printable_keeps_visible_characters() {
        for ch in ['a', 'é', '°', '·', '│', '▀', '\u{e30d}', ' '] {
            assert_eq!(printable(ch), ch, "{:?}", ch);
        }
    }

    #[test]
    fn render_line_never_stores_control_characters() {
        let mut r = renderer(40, 3);
        r.render_line_colored(0, 1, INJECTION, Color::White)
            .unwrap();
        let line = row(&r, 1);
        assert!(!line.chars().any(char::is_control), "{line:?}");
        assert!(line.starts_with("?]2;INJECTED?Jakarta?31m???"), "{line:?}");
    }

    #[test]
    fn render_centered_never_stores_control_characters() {
        let mut r = renderer(40, 3);
        r.render_centered_colored(&[INJECTION.to_string()], 0, Color::White)
            .unwrap();
        assert!(!row(&r, 0).chars().any(char::is_control));
    }

    #[test]
    fn render_char_never_stores_control_characters() {
        let mut r = renderer(4, 1);
        r.render_char(0, 0, '\u{1b}', Color::White).unwrap();
        assert_eq!(row(&r, 0), "?   ");
    }
}
