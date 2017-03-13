use common::PieceState;

pub trait Platform {
    fn flip_frame(&mut self);

    fn draw_hexagon(&mut self, (i16, i16), u16, u32);

    fn draw_piece(&mut self, (i16, i16), &PieceState);

    fn get_events(&mut self) -> Vec<Event>;
}


#[derive(Debug)]
pub enum Event {
    Quit,
    MouseUp { x: i16, y: i16 },
    MouseMove { x: i16, y: i16 },
}
