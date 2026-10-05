use super::tools::{Tools, ToolsChoose};
use crate::ui::picker::PickerState;
use ascm::{ColorPair, Pixel, Frame};
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::ShouldDo;
use crate::common::CommonFields;
use crate::actions_register::Actions;

const EMPTY_PIXEL: Pixel = Pixel {
    symbol: ' ',
    color: ColorPair {
        fg: 0,
        bg: 0,
    }
};

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

pub struct Widgets {
    pub picker: PickerState,
}

impl Widgets {
    pub fn new(primary_pixel: Pixel, secondary_pixel: Pixel) -> Widgets {
        Self {
            picker: PickerState::new(primary_pixel, secondary_pixel),
        }
    }
}
pub struct App {
    pub tools: Tools,
    pub tools_state: ListState,
    pub should_do: ShouldDo,
    pub frames: Vec<HashMap<(u16, u16), Pixel>>,
    pub delays: Vec<u32>,
    pub current_frame: usize,
    pub canvas: (u8, u8),
    pub canvas_area: Rect,
    pub widgets: Widgets,
    pub primary_pixel: Pixel,
    pub secondary_pixel: Pixel,
    pub mouse_pos: (u16, u16),
    pub keybinds: HashMap<KeyEvent, Actions>,
    pub last_pressed: KeyCode,
}

impl crate::tui::RenderableApp for App {
    fn render(&mut self, frame: &mut ratatui::Frame) {
        super::layout::render(self, frame);
    }
}

impl CommonFields for App {
    fn should_do(&self) -> ShouldDo {self.should_do.clone()}
    fn keybinds(&self) -> HashMap<KeyEvent, Actions> {self.keybinds.clone()}
    fn process_action(&mut self, action: &Actions) {
        match action {
            Actions::QUIT => self.quit(),
            Actions::FILE_SAVE => self.save_file(),
            Actions::FRAME_CREATE => {self.new_frame(); self.current_frame += 1},
            Actions::FRAME_NEXT => {
                self.current_frame = self.current_frame.saturating_add(1).clamp(0, self.frames.len() - 1);
            },
            Actions::FRAME_PREVIOUS => {
                self.current_frame = self.current_frame.saturating_sub(1).clamp(0, self.frames.len() - 1);
            },
            Actions::TOOLS_CHOOSE(id) => {
                match id {
                    0 => {self.tools.update_tool(ToolsChoose::Pencil); self.tools_state.select(Some(0))},
                    1 => {self.tools.update_tool(ToolsChoose::Brush); self.tools_state.select(Some(1))},
                    2 => {self.tools.update_tool(ToolsChoose::Eraser); self.tools_state.select(Some(2))},
                    _ => {}
                }
            },
            _ => {}
        }
    }
    fn configure_keybinds(&mut self) {
        self.keybinds.insert(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL), Actions::QUIT);
        self.keybinds.insert(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL), Actions::FILE_SAVE);

        self.keybinds.insert(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT), Actions::FRAME_CREATE);

        self.keybinds.insert(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::empty()), Actions::TOOLS_CHOOSE(0));
        self.keybinds.insert(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::empty()), Actions::TOOLS_CHOOSE(1));
        self.keybinds.insert(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::empty()), Actions::TOOLS_CHOOSE(2));

        self.keybinds.insert(KeyEvent::new(KeyCode::Left, KeyModifiers::empty()), Actions::FRAME_PREVIOUS);
        self.keybinds.insert(KeyEvent::new(KeyCode::Right, KeyModifiers::empty()), Actions::FRAME_NEXT);
    }
    fn quit(&mut self) {self.should_do = ShouldDo::Quit}
}

impl App {
    pub fn new(width: u16, height: u16) -> Self {
        let primary_pixel = Pixel {
            symbol: '█',
            color: ColorPair {
                fg: 15,
                bg: 0,
            }
        };

        let secondary_pixel = Pixel {
            symbol: '&',
            color: ColorPair {
                fg: 2,
                bg: 0,
            }
        };
        
        let mut frames = Vec::new();

        let capacity = width as usize * height as usize;
        let mut map = HashMap::with_capacity(capacity);

        map.extend(
            (0..width)
                .flat_map(|x| (0..height).map(move |y| ((x, y), EMPTY_PIXEL.clone())))
        );

        frames.push(map);

        Self {
            tools: Tools::default(),
            tools_state: ListState::default(),
            should_do: ShouldDo::default(),
            frames,
            delays: Vec::new(),
            current_frame: 0,
            canvas: (width as u8, height as u8),
            canvas_area: Rect::default(),
            widgets: Widgets::new(primary_pixel, secondary_pixel),
            primary_pixel,
            secondary_pixel,
            mouse_pos: (0, 0),
            keybinds: HashMap::new(),
            last_pressed: KeyCode::Null,
        }
    }

    pub fn save_file(&mut self) {
        let file_path = rfd::FileDialog::new()
            .set_title("Save File")
            .set_file_name("animation.ascm")
            .add_filter("ASCII-Motion files", &["ascm"])
            .save_file();

        if let Some(path) = file_path {
            let mut frames = Vec::new();

            for pixels in self.frames.iter() {
                let frame = Frame::from_pixels(pixels.clone(), 16);
                frames.push(frame);
            }

            let _ = ascm::create_file(
                path,
                self.canvas.0,
                self.canvas.1,
                frames,
            );
        };
    }

    pub fn new_frame(&mut self) {
        let width = self.canvas.0 as u16;
        let height = self.canvas.1 as u16;

        let capacity = width as usize * height as usize;
        let mut map = HashMap::with_capacity(capacity);

        map.extend(
            (0..width)
                .flat_map(|x| (0..height).map(move |y| ((x, y), EMPTY_PIXEL.clone())))
        );

        self.frames.push(map);
    }
    
    pub fn tick(&self) {}
}