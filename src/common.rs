
#[derive(Debug)]
pub enum PieceState {
    NoPiece,
    Blue,
    Black,
    Orange,
}

pub static mut GRID_OFFSET: (i16, i16) = (50, 50);

pub const SIDE_LENGTH: u16 = 69;
