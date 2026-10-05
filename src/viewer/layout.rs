use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use super::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        " [F1] File  [F2] Edit  [F3] View",
        Style::default().fg(Color::White),
    ))
        .block(title_block);

    let canvas_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

        let canvas = Paragraph::new(Text::styled(
        format!(":P aaaaa: {}", app.frames.len()),
        Style::default().fg(Color::White),
    ))
        .block(canvas_block.clone());

    let info_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let info = Paragraph::new(Text::styled(
        format!(" File: {}  Dimensions: {}x{}", app.current_file.file_name().unwrap().to_str().unwrap().to_string(), app.canvas.0, app.canvas.1),
        Style::default().fg(Color::White),
    ))
        .block(info_block);

    let dimensions = app.canvas.clone();
    let area = canvas_block.inner(chunks[1]);

    frame.render_widget(title, chunks[0]);
    // frame.render_widget(canvas, chunks[1]);
    frame.render_widget(info, chunks[2]);
    frame.render_widget(ascm::AscmView { state: &mut app.player }, canvas_block.inner(chunks[1]));
}