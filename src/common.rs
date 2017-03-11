
#[derive(Debug)]
pub enum PieceState {
    NoPiece,
    Blue,
    Black,
    Orange,
}

pub static mut grid_offset: (i16, i16) = (50, 50);

pub const side_length: u16 = 69;
