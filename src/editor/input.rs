use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::layout::Position;
use crate::common::{to_ascii_layout, CommonFields, trace_log};

use ascm::ColorPair;

use super::app::App;
use super::tools::ToolsChoose;

pub fn process_keycode(app: &mut App, key_event: KeyEvent) {
    app.last_pressed = key_event.code;
    let code = if key_event.code.as_char().is_some() {
        KeyCode::Char(to_ascii_layout(key_event.code.as_char().unwrap()))
    } else {
        key_event.code
    };
    
    let event = KeyEvent::new(code, key_event.modifiers);
    let keybind = app.keybinds.get(&event).cloned();

    match keybind {
        Some(id) => app.process_action(&id),
        None => {},
    }
}

pub fn process_mouse(app: &mut App, mouse_event: MouseEvent) {
    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).create(true).open("mouse_debug.txt") {
        use std::io::Write;
        let _ = writeln!(file, "Event: {:?}", mouse_event.kind);
    }

    match mouse_event.kind {
        MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left) => {
            let col = mouse_event.column;
            let row = mouse_event.row;
            let pos = Position::new(col, row);

            if app.canvas_area.contains(pos) {
                let canvas_col = col - app.canvas_area.x;
                let canvas_row = row - app.canvas_area.y;

                match app.tools.current_tool {
                    ToolsChoose::Pencil => app.tools.pencil.draw(app.primary_pixel, &mut app.frames[app.current_frame], canvas_col, canvas_row),
                    // ToolsChoose::Brush => app.draw_pixels.push((mouse_event.column, mouse_event.row)),ё
                    ToolsChoose::Eraser => app.tools.eraser.draw(&mut app.frames[app.current_frame], canvas_col, canvas_row),
                    _ => {}
                };
            } else if app.widgets.picker.area.contains(pos) {
                if app.widgets.picker.colors_area.contains(pos) {
                    if let Some(clicked_color) = app.widgets.picker.get_color_at(col, row) {
                        app.widgets.picker.primary_pixel.color.fg = clicked_color;
                        app.primary_pixel.color.fg = clicked_color;
                        trace_log(&format!("3. ЛОГИКА: Цвет изменен на {:?}", clicked_color));
                    }
                } else if app.widgets.picker.symbols_area.contains(pos) {
                    if let Some(clicked_symbol) = app.widgets.picker.get_symbol_at(col, row) {
                        app.widgets.picker.primary_pixel.symbol = clicked_symbol;
                        app.primary_pixel.symbol = clicked_symbol;
                    }
                }
            }
        },
        MouseEventKind::Down(MouseButton::Right) | MouseEventKind::Drag(MouseButton::Right) => {
            if app.canvas_area.contains(Position::new(mouse_event.column, mouse_event.row)) {
                let canvas_col = mouse_event.column - app.canvas_area.x;
                let canvas_row = mouse_event.row - app.canvas_area.y;

                match app.tools.current_tool {
                    ToolsChoose::Pencil => app.tools.pencil.draw(app.secondary_pixel, &mut app.frames[app.current_frame], canvas_col, canvas_row),
                    // ToolsChoose::Brush => app.draw_pixels.push((mouse_event.column, mouse_event.row)),
                    _ => {}
                };
            } else if app.widgets.picker.colors_area.contains(Position::new(mouse_event.column, mouse_event.row)) {
                if let Some(clicked_color) = app.widgets.picker.get_color_at(mouse_event.column, mouse_event.row) {
                    app.widgets.picker.primary_pixel.color.bg = clicked_color;
                    app.primary_pixel.color.bg = clicked_color;
                }
            }
        },
        MouseEventKind::Moved => {
            app.mouse_pos = (mouse_event.column, mouse_event.row);
        }
        _ => {}
    };
}