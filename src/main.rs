#[macro_use]
extern crate lazy_static;
extern crate sdl2;

mod game;
mod platform;
mod dw_hex;

fn main() {
    game::go();
}
