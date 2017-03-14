extern crate rand;
extern crate sdl2;

use self::rand::{Rng, SeedableRng, StdRng, Rand};
use sdl2_platform;
use sdl2_platform::Sdl2Platform as Platform;
use axial_hex;
use consts;

use common;
use common::GRID_OFFSET;
use common::PieceState;
use common::PieceState::*;

use platform::Event;
use platform::Platform as PlatformTrait;

static mut RNG: Option<StdRng> = None;
use std;

pub fn go() {
    let mut platform = sdl2_platform::new();

    let mut rng;
    unsafe {
        if RNG.is_none() {
            let seed: &[_] = &[42, 42];
            RNG = Some(SeedableRng::from_seed(seed));
        }

        rng = RNG.as_mut().unwrap();
    }


    let mut grid = axial_hex::Grid::new(10);
    for _ in 0..48 {
        let tile = rng.gen::<u16>() % 8;
        let piece = rng.gen::<PieceState>();


        grid.push((tile, piece));
    }

    let mut current_axial = (0, 0);

    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 {
        match (args[1].parse(), args[2].parse()) {
            (Ok(x), Ok(y)) => unsafe {
                GRID_OFFSET = (x, y);
                println!("set GRID_OFFSET to {:?}", GRID_OFFSET);
            },
            _ => {
                println!("could not parse GRID_OFFSET.");
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
                Event::MouseMove { x, y } => current_axial = (x, y),
                // _ => {}
            };
        }
        platform.draw_background();

        for ((x, y), &(tile_type, ref piece)) in grid.indices() {
            platform.draw_hexagon((x as i16, y as i16),
                                  tile_type,
                                  if current_axial == (x as i16, y as i16) {
                                      0xFFFFFFFF
                                  } else {
                                      0xFFDDDDDD
                                  });


            platform.draw_piece((x as i16, y as i16), piece);


        }

        platform.flip_frame();
    }
}



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
