
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Actions {
    QUIT,
    
    FILE_SAVE,
    FILE_CREATE,

    FRAME_CREATE,
    FRAME_DELETE,
    // FRAME_CHOOSE(usize),
    FRAME_NEXT,
    FRAME_PREVIOUS,

    TOOLS_CHOOSE(u8),

    PIXEL_SET(u16, u16, bool), // x, y, is_secondary
    // PIXEL_SET_ADVANCED(u16, u16, bool, BrushShape),
    PIXEL_ERASE(u16, u16), // x, y

    PALETTE_SET_FG(u8),
    PALETTE_SET_BG(u8),
    PALETTE_SET_SYMBOL(char),
    PALETTE_SWAP,
}