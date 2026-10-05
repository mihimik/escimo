// ┌───────┬────Picker──┐
// │ [1]   │ abcdefghik │
// │   [2] │ lmnopqrstu │
// ├───────┴────────────┤
// │ █▓▒█░█▓▒█░█▓▒█░█▓▒ │
// │ ▒█░█▓▒█░█▓▒▓▒█░█▒█ │
// │ ▒█░█▓▒█░█▓▒▓▒█░█▒█ │
// └────────────────────┘

use ascm::Pixel;
use ratatui::layout::Offset;
use ratatui::style::Stylize;
use ratatui::widgets::Wrap;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Clone)]
pub struct PickerState {
    pub symbols: Vec<char>,
    pub colors: Vec<u8>,
    pub area: Rect,
    pub colors_area: Rect,
    pub symbols_area: Rect,

    pub primary_pixel: Pixel,
    pub secondary_pixel: Pixel,
}

impl PickerState {
    pub fn new(primary_pixel: Pixel, secondary_pixel: Pixel) -> Self {
        let mut colors = Vec::with_capacity(255 * 2);
        colors.extend((1..=255).flat_map(|i| [i, i]));
        Self {
            symbols: vec!['█', '▓', '▒', '░', '┌', '│', '┐', '┘', '─', '└', '|', '/', '\\', '-', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '%'],
            colors,
            area: Rect::default(),
            colors_area: Rect::default(),
            symbols_area: Rect::default(),

            primary_pixel,
            secondary_pixel,
        }
    }

    pub fn get_color_at(&self, click_x: u16, click_y: u16) -> Option<u8> {
        if !self.colors_area.contains(ratatui::layout::Position::new(click_x, click_y)) {
            return None;
        }

        let local_x = click_x as usize - self.colors_area.x as usize;
        let local_y = click_y as usize - self.colors_area.y as usize;

        let colors_per_row = self.colors_area.width as usize / 2;

        if colors_per_row == 0 { return None; }

        let actual_x = local_x / 2;

        let index = local_y * colors_per_row + actual_x;

        let color_value = index + 1;

        if color_value <= 255 {
            Some(color_value as u8)
        } else {
            None
        }
    }

    pub fn get_symbol_at(&self, click_x: u16, click_y: u16) -> Option<char> {
        if !self.symbols_area.contains(ratatui::layout::Position::new(click_x, click_y)) {
            return None;
        }

        let local_x = click_x as usize - self.symbols_area.x as usize;
        let local_y = click_y as usize - self.symbols_area.y as usize;

        let symbols_per_row = self.symbols_area.width as usize / 2;

        if symbols_per_row == 0 { return None; }

        let actual_x = local_x / 2;

        let index = local_y * symbols_per_row + actual_x;

        if index <= 255 {
            Some(self.symbols[index])
        } else {
            None
        }
    }
}

// ┌ ─ ┬ ─ ┐
// │   │   │
// ├ ─ ┼ ─ ┤
// │   │   │
// └ ─ ┴ ─ ┘

pub struct PickerWidget<'a> {
    pub state: &'a mut PickerState,
}

impl Widget for PickerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 15 || area.height < 4 { return; }

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let top_area = main_chunks[0];
        let bottom_area = main_chunks[1];

        let top_border = border::PLAIN;

        let top_block = Block::default()
            .title("Picker")
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .border_set(top_border);

        let top_inner = top_block.inner(top_area);
        top_block.render(top_area, buf);

        let top_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(9),
                Constraint::Min(0),
            ])
            .split(top_inner);

        for y in top_inner.y..=(top_inner.y + top_inner.height) {
            buf.set_string(top_inner.x + 7, y, "│", Style::default());
        }

        buf.set_string(top_area.x + 8, top_area.y, "┬", Style::default());

        let line1 = Line::from(vec![
            " [".fg(Color::Yellow).bold().into(),
            self.state.primary_pixel.symbol.to_string().fg(Color::Indexed(self.state.primary_pixel.color.fg))
                .bg(Color::Indexed(self.state.primary_pixel.color.bg)),
            "]↕ ".fg(Color::Yellow).bold().into(),
        ]);

        let line2 = Line::from(vec![
            "   [".into(),
            self.state.secondary_pixel.symbol.to_string().fg(Color::Indexed(self.state.secondary_pixel.color.fg))
                .bg(Color::Indexed(self.state.secondary_pixel.color.bg)),
            "]".into(),
        ]);

        let left_text = vec![line1, line2];
        let right_text = self.state.symbols.iter()
            .map(|&c| format!("{} ", c))
            .collect::<String>();

        let mut symbols_area = top_columns[1].offset(Offset {
            x: -1,
            y: 0
        });
        symbols_area.width += 1;

        Paragraph::new(left_text).render(top_columns[0], buf);
        Paragraph::new(right_text)
            .wrap(Wrap { trim: false })
            .render(symbols_area, buf);

        let mut bottom_border = border::PLAIN;
        bottom_border.top_left = "├";
        bottom_border.top_right = "┤";

        let bottom_block = Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .border_set(bottom_border);

        let bottom_inner = bottom_block.inner(bottom_area);
        bottom_block.render(bottom_area, buf);

        buf.set_string(top_area.x + 8, top_area.y + 3, "┴", Style::default());

        let mut spans = Vec::new();

        for (_, &col) in self.state.colors.iter().enumerate() {
            let styled_span = Span::styled(
                "█",
                Style::default().fg(Color::Indexed(col))
            );

            spans.push(styled_span);
        }

        let line = Line::from(spans);

        let colors = Paragraph::new(line).wrap(Wrap { trim: false });
        colors.render(bottom_inner, buf);

        self.state.area = area;
        self.state.colors_area = bottom_inner;
        self.state.symbols_area = symbols_area;
    }
}