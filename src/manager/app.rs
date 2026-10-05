use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::common::CommonFields;
use ascm::Pixel;
use crate::actions_register::Actions;
use crate::ui::picker::PickerState;
use crate::ShouldDo;

// pub struct DefaultPreset {
//     symbols: Vec<char>,
//     colors: Vec<u8>,
//     primary_pixel: Pixel,
//     secondary_pixel: Pixel,
// }
//
// impl DefaultPreset {
//     pub fn new() -> Self {
//         let mut colors = Vec::with_capacity(255 * 2);
//         colors.extend((1..=255).flat_map(|i| [i, i]));
//
//         Self {
//             symbols: vec!['█', '▓', '▒', '░', '┌', '│', '┐', '┘', '─', '└'],
//             colors,
//             primary_pixel: Pixel { column: 0, row: 0, symbol: Some('█'), color: ColorPair { fg: 15, bg: 0 } },
//             secondary_pixel: Pixel { column: 0, row: 0, symbol: Some('█'), color: ColorPair { fg: 2, bg: 0 } },
//         }
//     }
// }

// #[derive(Debug, Default)]
// pub struct Widgets {
//     pub picker: PickerState,
// }

// impl Widgets {
//     pub fn new(primary_pixel: Pixel, secondary_pixel: Pixel) -> Widgets {
//         Self {
//             picker: PickerState::new(primary_pixel, secondary_pixel),
//         }
//     }
// }

impl CommonFields for App {
    fn should_do(&self) -> ShouldDo {self.should_do.clone()}
    fn keybinds(&self) -> HashMap<KeyEvent, Actions> {self.keybinds.clone()}
    fn process_action(&mut self, action: &Actions) {
        match action {
            Actions::QUIT => self.quit(),
            Actions::FILE_CREATE => self.should_do = ShouldDo::OpenEditor(None),
            _ => {}
        }
    }

    fn configure_keybinds(&mut self) {
        self.keybinds.insert(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL), Actions::QUIT);
        self.keybinds.insert(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty()), Actions::FILE_CREATE);
    }

    fn quit(&mut self) {self.should_do = ShouldDo::Quit}
}

#[derive(Debug, Default)]
pub struct App {
    pub should_do: ShouldDo,
    // pub widgets: Widgets,
    pub mouse_pos: (u16, u16),
    pub keybinds: HashMap<KeyEvent, Actions>,
}

impl crate::tui::RenderableApp for App {
    fn render(&mut self, frame: &mut ratatui::Frame) {
        super::layout::render(self, frame);
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            should_do: ShouldDo::default(),
            // widgets: Widgets::default(),
            mouse_pos: (0, 0),
            keybinds: HashMap::new(),
        }
    }
    
    pub fn tick(&self) {}
}