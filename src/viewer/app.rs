use ascm::{AscmView, Frame};
use ratatui::layout::Rect;
use std::collections::HashMap;
use std::path::PathBuf;
use ascm::integrations::ratatui_impl::AscmViewState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::ShouldDo;
use crate::common::{trace_log, CommonFields};

use ascm::read_file;
use crate::actions_register::Actions;
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

// pub struct Widgets {
//     pub picker: PickerState,
// }
//
// impl Widgets {
//     pub fn new(primary_pixel: ascm::Pixel, secondary_pixel: ascm::Pixel) -> Widgets {
//         Self {
//             picker: PickerState::new(primary_pixel, secondary_pixel),
//         }
//     }
// }
pub struct App {
    pub should_do: ShouldDo,
    pub frames: Vec<Frame>,
    pub current_frame: usize,
    pub current_file: PathBuf,
    pub canvas: (u8, u8),
    pub canvas_area: Rect,
    pub player: AscmViewState,
    pub keybinds: HashMap<KeyEvent, Actions>,
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
            _ => {}
        }
    }

    fn configure_keybinds(&mut self) {
        self.keybinds.insert(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL), Actions::QUIT);
    }

    fn quit(&mut self) {self.should_do = ShouldDo::Quit}
}

impl App {
    pub fn new(file: PathBuf) -> Self {
        let result = read_file(file.clone());
        match result {
            Ok(ref content) => {},
            Err(ref e) => trace_log(&format!("Error: {:?}", e))
        };
        let (header, frames) = result.expect("Unknown error while opened file");
        let player = AscmViewState::new((header.width, header.height), frames.clone());

        // let capacity = width as usize * height as usize;
        // let mut map = HashMap::with_capacity(capacity);
        //
        // map.extend(
        //     (0..width)
        //         .flat_map(|x| (0..height).map(move |y| ((x, y), empty_pixel.clone())))
        // );
        //
        // frames.push(map);

        trace_log(&format!("{:?}", frames));

        println!("{:?}", frames.len());

        Self {
            should_do: ShouldDo::default(),
            frames,
            current_frame: 0,
            current_file: file,
            canvas: (header.width, header.height),
            canvas_area: Rect::default(),
            player,
            keybinds: HashMap::new(),
        }
    }
    
    pub fn tick(&self) {}
}