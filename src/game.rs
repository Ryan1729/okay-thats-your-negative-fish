extern crate rand;
extern crate sdl2;

use self::rand::{Rng, SeedableRng, StdRng};
use platform::Platform;
use platform::Event;
use axial_hex;

static mut RNG: Option<StdRng> = None;
use std;

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
    //floor((short_diameter / 2) / cos(PI/6))
    let side_length: u16 = 69;

    let mut grid = axial_hex::Grid::new(10);
    for _ in 0..48 {
        let c = 0xFF000000u32 | rng.gen::<u32>();
        let u = rng.gen::<u16>() % 3;
        let v = rng.gen::<u16>() % 2;

        grid.push(((u, v), c));
    }

    let mut current_axial = (0, 0);

    let mut grid_offset = (0, 50);

    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 {
        match (args[1].parse(), args[2].parse()) {
            (Ok(x), Ok(y)) => {
                grid_offset = (x, y);
                println!("set grid_offset to {:?}", grid_offset);
            }
            _ => {
                println!("could not parse grid_offset.");
                println!("recieved {:?}", args);
            }
        }
    } else {
        if args.len() != 1 {
            println!("usage: [x y]");
            println!("example {} 20 40", args[0]);
        }
    }

    'running: loop {
        let events = platform.get_events();
        for event in events {
            match event {
                Event::Quit => break 'running,
                Event::MouseUp { x, y } |
                Event::MouseMove { x, y } => {
                    current_axial = axial_hex::pixel_to_axial_pointy(side_length,
                                                                     sub((x as i16, y as i16),
                                                                         grid_offset));
                    platform.render_text(&format!("   pixel: {:?} hex: {:?}",
                                                  (x as i16, y as i16),
                                                  current_axial));
                }
                // _ => {}
            };
        }

        for ((x, y), &(texture_coords, colour)) in grid.indices() {
            let pixel_coords = add(axial_hex::axial_to_pixel_pointy(side_length,
                                                                    (x as i16, y as i16)),
                                   grid_offset);


            platform.draw_bitmap_hexagon(pixel_coords,
                                         texture_coords,
                                         if current_axial == (x as i16, y as i16) {
                                             0xFFFFFFFF
                                         } else {
                                             colour
                                         });
        }

        for ((x, y), _) in grid.indices() {
            let pixel_coords = add(axial_hex::axial_to_pixel_pointy(side_length,
                                                                    (x as i16, y as i16)),
                                   grid_offset);



            let c = 0xFF000000 | ((y & 1) * 0xFFFFFF) as u32;
            platform.draw_box(pixel_coords,
                              axial_hex::short_diameter(side_length),
                              axial_hex::long_diameter(side_length),
                              c)
        }

        platform.flip_frame();
        // The rest of the game loop goes here...
    }
}

use std::ops::Add;
fn add<T: Add<Output = T>>((x1, y1): (T, T), (x2, y2): (T, T)) -> (T, T) {
    (x1 + x2, y1 + y2)
}

use std::ops::Sub;
fn sub<T: Sub<Output = T>>((x1, y1): (T, T), (x2, y2): (T, T)) -> (T, T) {
    (x1 - x2, y1 - y2)
}
