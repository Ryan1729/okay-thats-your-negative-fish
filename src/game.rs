extern crate rand;
extern crate sdl2;

use self::rand::{Rng, SeedableRng, StdRng, Rand};
use platform::Platform;
use platform::Event;
use axial_hex;
use consts;

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
    let side_length: u16 = 69; //34;

    let mut grid = axial_hex::Grid::new(10);
    for _ in 0..48 {
        let u = rng.gen::<u16>() % 4;
        let v = rng.gen::<u16>() % 2;
        let piece = rng.gen::<PieceState>();


        grid.push(((u, v), piece));
    }

    let mut current_axial = (0, 0);

    let mut grid_offset = (50, 50);

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
                }
                // _ => {}
            };
        }

        for ((x, y), &(texture_coords, ref piece)) in grid.indices() {
            let pixel_coords = add(axial_hex::axial_to_pixel_pointy(side_length,
                                                                    (x as i16, y as i16)),
                                   grid_offset);


            platform.draw_bitmap_hexagon(pixel_coords,
                                         texture_coords,
                                         side_length,
                                         if current_axial == (x as i16, y as i16) {
                                             0xFFFFFFFF
                                         } else {
                                             0xFFDDDDDD
                                         });


            platform.draw_piece(pixel_coords, piece_uv(piece), side_length);


        }

        platform.flip_frame();
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

#[derive(Debug)]
pub enum PieceState {
    NoPiece,
    Blue,
    Black,
    Orange,
}
use self::PieceState::*;

impl Rand for PieceState {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen::<usize>() % 4 {
            1 => Blue,
            2 => Black,
            3 => Orange,
            _ => NoPiece,
        }
    }
}

fn piece_uv(piece: &PieceState) -> (u16, u16) {
    let offset = match *piece {
        NoPiece => 0,
        Blue => 1,
        Black => 2,
        Orange => 3,
    };

    (offset * consts::PIECE_DIMENSIONS.0, 280)

}
