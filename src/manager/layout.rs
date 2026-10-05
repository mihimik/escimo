use ratatui::{
    Frame,
    layout::{Position, Rect},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph, Widget},
};

use super::app::{App};
use ascm::ColorPair;
use crate::ui::picker::PickerWidget;
use ratatui::layout::{Offset, Size};

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(17),
            Constraint::Min(20),
        ])
        .split(chunks[1]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        " [F1] File  [F2] Edit  [F3] View",
        Style::default().fg(Color::White),
    ))
        .block(title_block);

    let tools_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let tools_items = ["[P] Pencil *", "[B] Brush  █", "[E] Eraser ░"];

    let tools = List::new(tools_items)
        .highlight_style(Modifier::REVERSED)
        .block(tools_block);

    let canvas_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let canvas = Paragraph::new(Text::styled(
        ":P",
        Style::default().fg(Color::White),
    ))
        .block(canvas_block);

    frame.render_widget(title, chunks[0]);
    frame.render_widget(canvas, middle_chunks[1]);
}