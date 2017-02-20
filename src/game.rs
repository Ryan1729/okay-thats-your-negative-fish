extern crate rand;
extern crate sdl2;

use self::rand::{Rng, SeedableRng, StdRng};
use platform::Platform;
use dw_hex;

static mut RNG: Option<StdRng> = None;


pub fn go() {
    let mut platform = Platform::new();

    let mut rng;
    unsafe {
        if RNG.is_none() {
            let seed: &[_] = &[42, 42];
            RNG = Some(SeedableRng::from_seed(seed));
        }

        rng = RNG.as_mut().unwrap();
    }

    let mut grid = dw_hex::Grid::new(6);
    for _ in 0..24 {
        grid.push(0xFF000000u32 | rng.gen::<u32>());
    }

    for ((x, y), &colour) in grid.indices() {
        platform.draw_coloured_hexagon(15 * (x + 5) as i16, 9 * (y + 1) as i16, colour);
    }

    'running: loop {
        let should_quit = platform.quit_on_keypress();

        if should_quit {
            break 'running;
        }

        let mouse_state = platform.mouse_state();

        platform.flip_frame();
        // The rest of the game loop goes here...
    }
}
