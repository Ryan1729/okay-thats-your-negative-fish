extern crate sdl2;

use platform;

pub fn go() {
    platform::init();

    platform::draw_hexagon(10, 10);
}
