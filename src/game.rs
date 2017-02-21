extern crate rand;
extern crate sdl2;

use self::rand::{Rng, SeedableRng, StdRng};
use platform::Platform;
use platform::Event;
use axial_hex;

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
    let side_length: u16 = 32;

    let mut grid = axial_hex::Grid::new(10);
    for _ in 0..64 {
        grid.push(0xFF000000u32 | rng.gen::<u32>());
    }
    println!("{:?}",
             grid.indices().map(|(a, _)| a).collect::<Vec<(usize, usize)>>());


    'running: loop {
        let events = platform.get_events();
        for event in events {
            match event {
                Event::Quit => break 'running,
                Event::MouseUp { x, y } |
                Event::MouseMove { x, y } => {
                    let axial = axial_hex::pixel_to_axial(side_length, (x as i16, y as i16));
                    platform.render_text(&format!("   pixel: {:?} hexagon: {:?}",
                                                  (x as i16, y as i16),
                                                  axial));
                }
                // _ => {}
            };
        }

        for ((x, y), &colour) in grid.indices() {
            let pixel_coords = add(axial_hex::axial_to_pixel(side_length, (x as i16, y as i16)),
                                   (40, 40));
            platform.draw_coloured_hexagon(pixel_coords, side_length, colour);
        }

        for ((x, y), &colour) in grid.indices() {
            let box_size = axial_hex::corner_height(side_length) * 2 + side_length;

            let pixel_coords = add(axial_hex::axial_to_pixel(side_length, (x as i16, y as i16)),
                                   (40, 40));

            let c = 0xFFFF0000 | ((y & 1) * 0xFFFF) as u32;
            platform.draw_box(pixel_coords, box_size, box_size, c)
        }

        platform.flip_frame();
        // The rest of the game loop goes here...
    }
}

use std::ops::Add;
fn add<T: Add<Output = T>>((x1, y1): (T, T), (x2, y2): (T, T)) -> (T, T) {
    (x1 + x2, y1 + y2)
}
