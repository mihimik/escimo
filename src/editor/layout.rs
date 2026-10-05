use ratatui::{
    Frame,
    layout::{Position, Rect},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph, Widget},
};

use super::app::{App, Widgets};
use ascm::ColorPair;
use crate::ui::picker::PickerWidget;
use crate::common::trace_log;
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

    let timeline_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title("Timeline");

    frame.render_widget(title, chunks[0]);
    frame.render_stateful_widget(tools, middle_chunks[0], &mut app.tools_state);
    frame.render_widget(canvas, middle_chunks[1]);

    let inner_area = Rect {
        x: middle_chunks[1].x + 1,
        y: middle_chunks[1].y + 1,
        width: middle_chunks[1].width - 2,
        height: middle_chunks[1].height - 2,
    };

    let canvas_area = draw_canvas(frame, inner_area, app);
    app.canvas_area = canvas_area;

    draw_timeline(frame, timeline_block.clone(), chunks[2], app);

    let ar = middle_chunks[1].clone().resize(Size {width: 30, height: 30}).offset(Offset {x: 70, y: 1});

    let picker = PickerWidget {state: &mut app.widgets.picker};
    if middle_chunks[1].intersects(ar) {
        frame.render_widget(picker, ar)
    };
}

fn draw_canvas(f: &mut Frame, area: Rect, app: &App) -> Rect {
    let style_border = Style::default().fg(Color::Indexed(237));
    // let style_fill = Style::default().fg(Color::Indexed(235));

    let inner_width = (app.canvas.0 + 1) as u16;
    let inner_height = (app.canvas.1 - 1) as u16;

    let child_area = Rect {
        x: area.x,
        y: area.y,
        width: inner_width.min(area.width),
        height: inner_height.min(area.height),
    };

    let child_block = Block::default()
        .borders(Borders::RIGHT | Borders::BOTTOM)
        .border_style(style_border);
    f.render_widget(&child_block, child_area);

    let drawing_area = child_block.inner(child_area);

    let main_buffer = f.buffer_mut();

    for ((global_x, global_y), &pixel) in &app.frames[app.current_frame] {
        let local_x = global_x + drawing_area.x;
        let local_y = global_y + drawing_area.y;

        if drawing_area.contains(Position::new(local_x, local_y)) {
            if let Some(cell) = main_buffer.cell_mut((local_x, local_y)) {
                cell.set_symbol(&pixel.symbol.to_string());
                cell.set_fg(Color::Indexed(pixel.color.fg));
                cell.set_bg(Color::Indexed(pixel.color.bg));
            }
        }
    }

    drawing_area
}

fn draw_timeline(f: &mut Frame, block: Block, area: Rect, app: &App) {
    let frames_len = app.frames.len();
    let text = (1..=frames_len)
        .map(|i| format!("[ {} | {}ms ]", i, app.delays.get(i).unwrap_or(&0)))
        .collect::<Vec<String>>()
        .join(" ");

    let timeline = Paragraph::new(Text::styled(
        format!(" {} ||||| frames: {}; current: {}; last pressed key: {}", text, app.frames.len(), app.current_frame, app.last_pressed.to_string()),
        Style::default().fg(Color::White),
    ))
        .block(block);

    f.render_widget(timeline, area);
}