use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::layout::Position;
use crate::common::{to_ascii_layout, CommonFields, trace_log};

use ascm::ColorPair;

use super::app::App;

pub fn process_keycode(app: &mut App, key_event: KeyEvent) {
    let event = KeyEvent::new(KeyCode::Char(to_ascii_layout(key_event.code.as_char().unwrap_or('a'))), key_event.modifiers);
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
            // let col = mouse_event.column;
            // let row = mouse_event.row;
            // let pos = Position::new(col, row);
        },
        MouseEventKind::Moved => {}
        _ => {}
    };
}