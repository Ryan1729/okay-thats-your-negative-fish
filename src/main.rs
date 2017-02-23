#[macro_use]
extern crate lazy_static;
extern crate sdl2;

mod game;
mod platform;
mod axial_hex;
mod consts;

fn main() {
    game::go();
}
