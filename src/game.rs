extern crate rand;
extern crate sdl2;

use self::rand::{Rng, SeedableRng, StdRng};
use platform::Platform;
use platform::Event;
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
    let side_length: u16 = 5;

    let mut grid = dw_hex::Grid::new(4);
    for _ in 0..24 {
        grid.push(0xFF000000u32 | rng.gen::<u32>());
    }

    for ((x, y), &colour) in grid.indices() {
        platform.draw_coloured_hexagon(dw_hex::dw_to_pixel(side_length, (5 + x as i16, 1 + y as i16)),
                                       side_length,
                                       colour);
    }

    'running: loop {
        let events = platform.get_events();
        for event in events {
            match event {
                Event::Quit => break 'running,
                Event::MouseUp { x, y } => {
                    let dw = dw_hex::pixel_to_dw(side_length, (x as i16, y as i16));
                    platform.render_text(&format!("{:?}", dw));
                }
                // _ => {}
            };
        }

        platform.flip_frame();
        // The rest of the game loop goes here...
    }
}
