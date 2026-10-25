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

pub(crate) fn rgb(color: Color) -> Option<(u8, u8, u8)> {
    capabilities::approx_rgb(color)
}

pub fn mix(from: (u8, u8, u8), to: (u8, u8, u8), amount: f32) -> Color {
    let channel =
        |a: u8, b: u8| (f32::from(a) + (f32::from(b) - f32::from(a)) * amount).round() as u8;
    Color::Rgb {
        r: channel(from.0, to.0),
        g: channel(from.1, to.1),
        b: channel(from.2, to.2),
    }
}

const INK: (u8, u8, u8) = (16, 18, 34);
const VEIL: (u8, u8, u8) = (96, 100, 116);
const VEIL_SHARE: f32 = 0.62;
const FLASH_SHARE: f32 = 0.18;
const FLASH_SKY: (u8, u8, u8) = (120, 125, 150);
const INK_SHARE: f32 = 0.7;

const HALF_LIT: f32 = 0.5;

fn lit_share(ch: char) -> f32 {
    let quadrants = [
        ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
    ];
    if let Some(bits) = quadrants.iter().position(|quad| *quad == ch) {
        return (bits as u32).count_ones() as f32 / 4.0;
    }
    match ch as u32 {
        code @ 0x1FB00..=0x1FB3B => {
            let mut pattern = code - 0x1FB00 + 1;
            pattern += u32::from(pattern >= 21);
            pattern += u32::from(pattern >= 42);
            pattern.count_ones() as f32 / 6.0
        }
        0x2580..=0x259F => HALF_LIT,
        _ => 0.0,
    }
}

fn kept_fill(under: &Cell, ch: char) -> Color {
    if ch == ' ' { under.fill } else { Color::Reset }
}

fn printable(ch: char) -> char {
    if ch.is_control() { '?' } else { ch }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    character: char,
    color: Color,
    background: Color,
    fill: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            color: Color::Reset,
            background: Color::Reset,
            fill: Color::Reset,
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
    solid: Vec<bool>,
    reserved: Vec<bool>,
    protected: Vec<bool>,
    sky: Vec<Color>,
    sky_upper: Vec<Color>,
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
            solid: vec![false; buffer_size],
            reserved: vec![false; buffer_size],
            protected: vec![false; buffer_size],
            sky: Vec::new(),
            sky_upper: Vec::new(),
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
            solid: vec![false; buffer_size],
            reserved: vec![false; buffer_size],
            protected: vec![false; buffer_size],
            sky: Vec::new(),
            sky_upper: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn headless_truecolor(width: u16, height: u16) -> Self {
        let mut renderer = Self::headless(width, height);
        renderer.capabilities = TerminalCapabilities {
            color_support: capabilities::ColorSupport::TrueColor,
            is_tty: false,
        };
        renderer
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
            self.solid = vec![false; buffer_size];
            self.reserved = vec![false; buffer_size];
            self.protected = vec![false; buffer_size];
            execute!(self.stdout, Clear(ClearType::All))?;
        }
        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.buffer.fill(Cell::default());
        self.solid.fill(false);
        self.reserved.fill(false);
        self.protected.fill(false);
        self.sky.clear();
        self.sky_upper.clear();
        Ok(())
    }

    pub fn can_paint(&self) -> bool {
        self.capabilities.color_support == capabilities::ColorSupport::TrueColor
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        (x < self.width && y < self.height)
            .then(|| (y as usize) * (self.width as usize) + (x as usize))
            .filter(|idx| *idx < self.buffer.len())
    }

    pub fn paint_backdrop(&mut self, rows: &[Color]) {
        self.sky.clear();
        self.sky
            .extend(rows.iter().take(self.height as usize).copied());
        let width = self.width as usize;
        for (y, color) in self.sky.iter().enumerate() {
            let color = self.capabilities.adjust_background(*color);
            for idx in y * width..(y + 1) * width {
                if !self.solid[idx] {
                    self.buffer[idx].background = color;
                }
            }
        }
    }

    pub fn paint_backdrop_halves(&mut self, upper: &[Color], lower: &[Color]) {
        self.paint_backdrop(lower);
        self.sky_upper.clear();
        self.sky_upper
            .extend(upper.iter().take(self.sky.len()).copied());
        let width = self.width as usize;
        for (y, color) in self.sky_upper.iter().enumerate() {
            if *color == Color::Reset || self.sky.get(y) == Some(color) {
                continue;
            }
            let color = self.capabilities.adjust_background(*color);
            for idx in y * width..(y + 1) * width {
                if !self.solid[idx] {
                    self.buffer[idx].fill = color;
                }
            }
        }
    }

    pub fn mark_solid(&mut self, x: u16, y: u16) {
        if let Some(idx) = self.index(x, y) {
            self.solid[idx] = true;
            self.buffer[idx] = Cell::default();
        }
    }

    pub fn tint(&mut self, x: u16, y: u16, toward: Color, amount: f32) {
        let Some(idx) = self.index(x, y).filter(|idx| !self.solid[*idx]) else {
            return;
        };
        let Some(toward) = rgb(toward) else {
            return;
        };
        if let Some(sky) = self.sky.get(y as usize).copied().and_then(rgb) {
            self.buffer[idx].background = self
                .capabilities
                .adjust_background(mix(sky, toward, amount));
        }
        if self.buffer[idx].fill != Color::Reset
            && let Some(upper) = self.sky_upper.get(y as usize).copied().and_then(rgb)
        {
            self.buffer[idx].fill = self
                .capabilities
                .adjust_background(mix(upper, toward, amount));
        }
    }

    pub fn set_background(&mut self, x: u16, y: u16, color: Color) {
        if self.sky.is_empty() {
            return;
        }
        if let Some(idx) = self.index(x, y) {
            self.buffer[idx].background = self.capabilities.adjust_background(color);
            self.buffer[idx].fill = Color::Reset;
        }
    }

    #[cfg(test)]
    pub(crate) fn cell_at(&self, x: u16, y: u16) -> (char, Color) {
        self.index(x, y).map_or((' ', Color::Reset), |idx| {
            (self.buffer[idx].character, self.buffer[idx].color)
        })
    }

    #[cfg(test)]
    pub(crate) fn sky_at(&self, y: u16) -> Color {
        self.sky
            .get(usize::from(y))
            .copied()
            .unwrap_or(Color::Reset)
    }

    #[cfg(test)]
    pub(crate) fn ansi(&self) -> String {
        let sgr = |color: Color, foreground: bool| match capabilities::approx_rgb(color) {
            Some((r, g, b)) => format!("\x1b[{};2;{r};{g};{b}m", if foreground { 38 } else { 48 }),
            None => format!("\x1b[{}m", if foreground { 39 } else { 49 }),
        };
        let mut out = String::new();
        for y in 0..self.height {
            out.push_str(&format!("\x1b[{};1H", y + 1));
            for x in 0..self.width {
                let cell = self.buffer[usize::from(y) * usize::from(self.width) + usize::from(x)];
                let (character, color) = if cell.character == ' ' && cell.fill != Color::Reset {
                    ('▀', cell.fill)
                } else {
                    (cell.character, cell.color)
                };
                out.push_str(&sgr(color, true));
                out.push_str(&sgr(cell.background, false));
                out.push(character);
            }
        }
        out.push_str("\x1b[0m");
        out
    }

    #[cfg(test)]
    pub(crate) fn background_at(&self, x: u16, y: u16) -> Color {
        self.index(x, y)
            .map_or(Color::Reset, |idx| self.buffer[idx].background)
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
                                background: self.buffer[buffer_idx].background,
                                fill: kept_fill(&self.buffer[buffer_idx], ch),
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
                        background: self.buffer[buffer_idx].background,
                        fill: kept_fill(&self.buffer[buffer_idx], ch),
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
                let (background, fill) = if background == Color::Reset {
                    (
                        self.buffer[buffer_idx].background,
                        kept_fill(&self.buffer[buffer_idx], ch),
                    )
                } else {
                    (
                        self.capabilities.adjust_background(background),
                        Color::Reset,
                    )
                };
                self.buffer[buffer_idx] = Cell {
                    character: printable(ch),
                    color: self.capabilities.adjust_color(color),
                    background,
                    fill,
                };
            }
        }
        Ok(())
    }

    pub fn render_char_in_front(
        &mut self,
        x: u16,
        y: u16,
        ch: char,
        color: Color,
    ) -> io::Result<()> {
        let Some(idx) = self.index(x, y) else {
            return Ok(());
        };
        if self.reserved[idx] {
            return Ok(());
        }
        match self.buffer[idx].character {
            ' ' => self.render_char(x, y, ch, color),
            '█' => {
                self.ink_on_disc(idx, y, ch, None);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn disc_under(&self, x: u16, y: u16) -> Option<Color> {
        let cell = &self.buffer[self.index(x, y)?];
        (lit_share(cell.character) >= HALF_LIT).then_some(cell.color)
    }

    pub fn lit_from_behind(&mut self, x: u16, y: u16, disc: Color, amount: f32) {
        let Some(idx) = self.index(x, y) else {
            return;
        };
        let Some(light) = rgb(disc) else {
            return;
        };
        let cell = &mut self.buffer[idx];
        if let Some(back) = rgb(cell.background) {
            cell.background = self
                .capabilities
                .adjust_background(mix(back, light, amount));
        }
        if cell.character != ' '
            && let Some(ink) = rgb(cell.color)
        {
            cell.color = self.capabilities.adjust_color(mix(ink, light, amount));
        }
        cell.fill = Color::Reset;
    }

    pub fn protect(&mut self, x: u16, y: u16) {
        if let Some(idx) = self.index(x, y) {
            self.protected[idx] = true;
        }
    }

    pub fn is_protected(&self, x: u16, y: u16) -> bool {
        self.index(x, y).is_some_and(|idx| self.protected[idx])
    }

    pub fn clear_beside(&mut self, x: u16, y: u16) {
        let Some(idx) = self.index(x, y) else {
            return;
        };
        if self.protected[idx] {
            return;
        }
        self.reserved[idx] = true;
        let cell = &mut self.buffer[idx];
        if lit_share(cell.character) == 0.0 && !self.solid[idx] {
            cell.character = ' ';
            cell.color = Color::Reset;
        }
    }

    pub fn render_char_on_open_sky(
        &mut self,
        x: u16,
        y: u16,
        ch: char,
        color: Color,
    ) -> io::Result<()> {
        let Some(idx) = self.index(x, y) else {
            return Ok(());
        };
        let sky = self
            .sky
            .get(usize::from(y))
            .copied()
            .unwrap_or(Color::Reset);
        let cell = &self.buffer[idx];
        if cell.character == ' ' && cell.background == sky && !self.reserved[idx] {
            self.render_char(x, y, ch, color)
        } else {
            Ok(())
        }
    }

    pub fn render_char_over_disc(
        &mut self,
        x: u16,
        y: u16,
        ch: char,
        color: Color,
    ) -> io::Result<()> {
        self.render_char_over_disc_in(x, y, ch, color, None)
    }

    pub fn render_char_over_disc_in(
        &mut self,
        x: u16,
        y: u16,
        ch: char,
        color: Color,
        ink: Option<Color>,
    ) -> io::Result<()> {
        let Some(idx) = self.index(x, y) else {
            return Ok(());
        };
        if lit_share(self.buffer[idx].character) >= HALF_LIT {
            self.ink_on_disc(idx, y, ch, ink);
            Ok(())
        } else {
            self.render_char(x, y, ch, color)
        }
    }

    fn ink_on_disc(&mut self, idx: usize, y: u16, ch: char, ink: Option<Color>) {
        let disc = self.buffer[idx].color;
        let night = self
            .sky
            .get(usize::from(y))
            .copied()
            .and_then(rgb)
            .unwrap_or(INK);
        let ink = ink
            .filter(|ink| *ink != Color::Reset)
            .unwrap_or_else(|| rgb(disc).map_or(disc, |lit| mix(lit, night, INK_SHARE)));
        self.buffer[idx] = Cell {
            character: printable(ch),
            color: self.capabilities.adjust_color(ink),
            background: disc,
            fill: Color::Reset,
        };
    }

    pub fn clear_to_sky(&mut self, x: u16, y: u16) {
        let Some(idx) = self.index(x, y) else {
            return;
        };
        if self.solid[idx] {
            return;
        }
        let row = usize::from(y);
        let lower = self.sky.get(row).copied().unwrap_or(Color::Reset);
        let upper = self
            .sky_upper
            .get(row)
            .copied()
            .filter(|upper| *upper != lower)
            .unwrap_or(Color::Reset);
        self.buffer[idx] = Cell {
            character: ' ',
            color: Color::Reset,
            background: self.capabilities.adjust_background(lower),
            fill: self.capabilities.adjust_background(upper),
        };
    }

    pub fn sky_row(&self, y: u16) -> Option<Color> {
        self.sky
            .get(usize::from(y))
            .copied()
            .filter(|color| *color != Color::Reset)
    }

    pub fn veiled(&self, y: u16, color: Color) -> Color {
        let Some(disc) = rgb(color) else {
            return color;
        };
        let behind = self.sky_row(y).and_then(rgb).unwrap_or(VEIL);
        mix(disc, behind, VEIL_SHARE)
    }

    pub fn flash_screen(&mut self, color: Color) -> io::Result<()> {
        if self.sky.iter().any(|row| *row != Color::Reset) {
            let width = usize::from(self.width);
            for (y, row) in self.sky.clone().into_iter().enumerate() {
                if row == Color::Reset {
                    continue;
                }
                for idx in y * width..(y + 1) * width {
                    if self.solid[idx] {
                        continue;
                    }
                    let cell = &mut self.buffer[idx];
                    if let Some(lit) = rgb(cell.background) {
                        cell.background =
                            self.capabilities
                                .adjust_background(mix(lit, FLASH_SKY, FLASH_SHARE));
                    }
                    if let Some(upper) = rgb(cell.fill) {
                        cell.fill =
                            self.capabilities
                                .adjust_background(mix(upper, FLASH_SKY, FLASH_SHARE));
                    }
                }
            }
            return Ok(());
        }
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

                    let (character, color) = if cell.character == ' ' && cell.fill != Color::Reset {
                        ('▀', cell.fill)
                    } else {
                        (cell.character, cell.color)
                    };
                    if color != current_color {
                        queue!(self.stdout, SetForegroundColor(color))?;
                        current_color = color;
                    }
                    if cell.background != current_background {
                        queue!(self.stdout, SetBackgroundColor(cell.background))?;
                        current_background = cell.background;
                    }

                    queue!(self.stdout, Print(character))?;
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
    fn a_half_block_sky_draws_two_bands_until_something_covers_the_cell() {
        let mut renderer = TerminalRenderer::headless_truecolor(3, 2);
        let (top, low) = (
            Color::Rgb {
                r: 10,
                g: 20,
                b: 60,
            },
            Color::Rgb {
                r: 30,
                g: 40,
                b: 80,
            },
        );
        renderer.paint_backdrop_halves(&[top, Color::Reset], &[low, Color::Reset]);
        assert_eq!(renderer.background_at(0, 0), low);
        assert_eq!(renderer.buffer[0].fill, top);
        renderer.render_char(1, 0, '*', Color::White).unwrap();
        assert_eq!(
            renderer.buffer[1].fill,
            Color::Reset,
            "a star takes the whole cell"
        );
        assert_eq!(renderer.background_at(1, 0), low);
        renderer
            .render_line_colored(0, 0, " ", Color::White)
            .unwrap();
        assert_eq!(
            renderer.buffer[0].fill, top,
            "a space leaves the sky as it was"
        );
        renderer.tint(2, 0, Color::White, 0.5);
        assert_ne!(renderer.buffer[2].fill, top, "a cloud tints both halves");
        assert_eq!(renderer.buffer[3].fill, Color::Reset, "below the sky");
        assert_eq!(row(&renderer, 0), " * ", "the text view stays blank");
    }

    #[test]
    fn weather_in_front_of_a_solid_block_keeps_the_block_showing() {
        let mut renderer = TerminalRenderer::headless_truecolor(4, 1);
        let moon = Color::Rgb {
            r: 200,
            g: 210,
            b: 240,
        };
        let rain = Color::Rgb {
            r: 120,
            g: 160,
            b: 250,
        };
        renderer.render_char(0, 0, '█', moon).unwrap();
        renderer.render_char(1, 0, '▙', moon).unwrap();
        renderer.render_char_in_front(0, 0, '│', rain).unwrap();
        renderer.render_char_in_front(1, 0, '│', rain).unwrap();
        renderer.render_char_in_front(2, 0, '│', rain).unwrap();
        renderer.render_char(3, 0, '|', moon).unwrap();
        renderer.render_char_in_front(3, 0, '│', rain).unwrap();
        assert_eq!(row(&renderer, 0), "│▙│|", "a drop never replaces a stroke");
        assert_eq!(renderer.cell_at(3, 0).1, moon);
        assert_eq!(
            renderer.background_at(0, 0),
            moon,
            "the streak sits on the moon"
        );
        let (_, ink) = renderer.cell_at(0, 0);
        assert_ne!(ink, rain, "dark ink, not the rain colour, on the moon");
        assert_ne!(ink, moon);
        assert_eq!(renderer.background_at(2, 0), Color::Reset);
        renderer.render_char_over_disc(1, 0, '@', rain).unwrap();
        assert_eq!(
            renderer.cell_at(1, 0).0,
            '@',
            "a rider crosses the moon's edge"
        );
        assert_eq!(renderer.background_at(1, 0), moon);
    }

    #[test]
    fn clearing_to_the_sky_wipes_what_was_behind_but_not_the_house() {
        let mut renderer = TerminalRenderer::headless_truecolor(3, 1);
        let (top, low) = (
            Color::Rgb {
                r: 10,
                g: 20,
                b: 60,
            },
            Color::Rgb {
                r: 30,
                g: 40,
                b: 80,
            },
        );
        renderer.paint_backdrop_halves(&[top], &[low]);
        renderer.render_char(0, 0, '✦', Color::White).unwrap();
        renderer.clear_to_sky(0, 0);
        assert_eq!(renderer.cell_at(0, 0).0, ' ');
        assert_eq!(renderer.buffer[0].fill, top);
        assert_eq!(renderer.background_at(0, 0), low);
        renderer.mark_solid(1, 0);
        renderer.clear_to_sky(1, 0);
        assert_eq!(
            renderer.background_at(1, 0),
            Color::Reset,
            "the house stays"
        );
        assert_eq!(renderer.sky_row(0), Some(low));
    }

    #[test]
    fn the_painted_sky_survives_drawing_and_tints_toward_named_colours() {
        let mut renderer = TerminalRenderer::headless_truecolor(10, 3);
        let sky = Color::Rgb {
            r: 20,
            g: 40,
            b: 90,
        };
        renderer.paint_backdrop(&[sky, sky, Color::Reset]);
        renderer.render_char(1, 0, '*', Color::White).unwrap();
        assert_eq!(renderer.background_at(1, 0), sky);
        renderer.tint(2, 0, Color::White, 0.5);
        assert_eq!(
            renderer.background_at(2, 0),
            Color::Rgb {
                r: 138,
                g: 148,
                b: 173
            }
        );
        renderer.mark_solid(3, 1);
        renderer.tint(3, 1, Color::White, 0.5);
        assert_eq!(renderer.background_at(3, 1), Color::Reset);
        assert_eq!(renderer.background_at(4, 2), Color::Reset);
        assert!(!TerminalRenderer::headless(10, 3).can_paint());
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
