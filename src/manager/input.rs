use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::layout::Position;
use crate::common::{to_ascii_layout, CommonFields};

use ascm::ColorPair;
use crate::ShouldDo;
use super::app::App;


pub fn process_keycode(app: &mut App, key_event: KeyEvent) {
    // match key_event.code {
    //     KeyCode::Esc | KeyCode::Char('q') => app.quit(),
    //     KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
    //         app.quit()
    //     }
    //     _ => {}
    // };
    //
    // if let KeyCode::Char(ch) = key_event.code {
    //     match to_ascii_layout(ch) {
    //         'n' => app.should_do = ShouldDo::OpenEditor(None),
    //         _ => {}
    //     }
    // }

    let event = KeyEvent::new(KeyCode::Char(to_ascii_layout(key_event.code.as_char().unwrap_or('a'))), key_event.modifiers);
    let keybind = app.keybinds.get(&event).cloned();

    match keybind {
        Some(id) => app.process_action(&id),
        None => {},
    }
}

// pub fn process_mouse(app: &mut App, mouse_event: MouseEvent) {
//     match mouse_event.kind {
//         MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left) => {
//             if app.canvas_area.contains(Position::new(mouse_event.column, mouse_event.row)) {
//                 match app.tools.current_tool {
//                     ToolsChoose::Pencil => app.tools.pencil.draw(app.primary_pixel, &mut app.draw_pixels, mouse_event.column, mouse_event.row),
//                     // ToolsChoose::Brush => app.draw_pixels.push((mouse_event.column, mouse_event.row)),
//                     ToolsChoose::Eraser => app.tools.eraser.draw(&mut app.draw_pixels, mouse_event.column, mouse_event.row),
//                     _ => {}
//                 };
//             } else if app.widgets.picker.area.contains(Position::new(mouse_event.column, mouse_event.row)) {
//                 if app.widgets.picker.colors_area.contains(Position::new(mouse_event.column, mouse_event.row)) {
//                     if let Some(clicked_color) = app.widgets.picker.get_color_at(mouse_event.column, mouse_event.row) {
//                         app.widgets.picker.primary_pixel.color.fg = clicked_color;
//                         app.primary_pixel.color.fg = clicked_color;
//                     }
//                 } else if app.widgets.picker.symbols_area.contains(Position::new(mouse_event.column, mouse_event.row)) {
//                     if let Some(clicked_symbol) = app.widgets.picker.get_symbol_at(mouse_event.column, mouse_event.row) {
//                         app.widgets.picker.primary_pixel.symbol = Some(clicked_symbol);
//                         app.primary_pixel.symbol = Some(clicked_symbol);
//                     }
//                 }
//             }
//         },
//         MouseEventKind::Down(MouseButton::Right) | MouseEventKind::Drag(MouseButton::Right) => {
//             if app.canvas_area.contains(Position::new(mouse_event.column, mouse_event.row)) {
//                 match app.tools.current_tool {
//                     ToolsChoose::Pencil => app.tools.pencil.draw(app.secondary_pixel, &mut app.draw_pixels, mouse_event.column, mouse_event.row),
//                     // ToolsChoose::Brush => app.draw_pixels.push((mouse_event.column, mouse_event.row)),
//                     _ => {}
//                 };
//             } else // if app.widgets.picker.area.contains(Position::new(mouse_event.column, mouse_event.row)) {
//                 if app.widgets.picker.colors_area.contains(Position::new(mouse_event.column, mouse_event.row)) {
//                     if let Some(clicked_color) = app.widgets.picker.get_color_at(mouse_event.column, mouse_event.row) {
//                         app.widgets.picker.primary_pixel.color.bg = clicked_color;
//                         app.primary_pixel.color.bg = clicked_color;
//                     }
//                 }
//             // }
//         },
//         MouseEventKind::Moved => {
//             app.mouse_pos = (mouse_event.column, mouse_event.row);
//         }
//         _ => {}
//     };
// }