use std::collections::HashMap;
use crossterm::event::{KeyEvent, MouseEvent};
use crate::actions_register::Actions;

pub struct InputManager {
    pub is_wezterm: bool,
    keybinds: HashMap<KeyEvent, Actions>,
    mousebinds: HashMap<MouseEvent, Actions>,
    ui_buttons: HashMap<String, ratatui::layout::Rect>,
}

// impl InputManager {
//     pub fn new() -> InputManager {
//
//     }
// }