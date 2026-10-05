use crate::ui::Pixel;
use ascm::ColorPair;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub enum ToolsChoose {
    #[default]
    Pencil,
    Brush,
    Eraser,
}

impl ToolsChoose {
    pub fn to_index(&self) -> usize {
        match self {
            ToolsChoose::Pencil => 0,
            ToolsChoose::Brush => 1,
            ToolsChoose::Eraser => 2,
        }
    }
}

#[derive(Debug, Default)]
pub struct Tools {
    pub pencil: Pencil,
    pub brush: Brush,
    pub eraser: Eraser,
    pub current_tool: ToolsChoose,
}

impl Tools {
    pub fn update_tool(&mut self, choose: ToolsChoose) {
        match choose {
            ToolsChoose::Pencil => self.pencil.selected = true,
            ToolsChoose::Brush => self.brush.selected = true,
            ToolsChoose::Eraser => self.eraser.selected = true,
        }

        self.current_tool = choose;
    }
}

#[derive(Debug)]
pub struct Pencil {
    pub selected: bool,
}

impl Default for Pencil {
    fn default() -> Pencil {
        Self {
            selected: false,
        }
    }
}

impl Pencil {
    pub fn draw(&self, pixel: ascm::Pixel, draw_pixels: &mut HashMap<(u16, u16), ascm::Pixel>, column: u16, row: u16) {
        let pixel = ascm::Pixel {
            symbol: pixel.symbol,
            color: pixel.color,
        };

        draw_pixels.insert((column, row), pixel);
    }
}

#[derive(Debug, Default)]
pub struct Brush {
    pub selected: bool,
    pub symbol: char,
    pub color: ColorPair,
    pub bold: bool,
}

#[derive(Debug, Default)]
pub struct Eraser {
    pub selected: bool,
    pub fg: bool,
    pub bg: bool,
}

impl Eraser {
    pub fn draw(&self, draw_pixels: &mut HashMap<(u16, u16), ascm::Pixel>, column: u16, row: u16) {
        let pixel = ascm::Pixel {
            symbol: ' ',
            color: ColorPair::default(),
        };

        draw_pixels.insert((column, row), pixel);
    }
}