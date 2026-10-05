use ascm::ColorPair;

pub mod picker;

#[derive(Debug, Default, Copy, Clone)]
pub struct Pixel {
    pub column: u16,
    pub row: u16,
    pub symbol: Option<char>,
    pub color: ColorPair,
}