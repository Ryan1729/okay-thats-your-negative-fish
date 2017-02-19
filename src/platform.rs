extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

pub fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    // renderer.set_scale(10f32, 10f32).unwrap();

    renderer.set_draw_color(Color::RGB(250, 224, 55));
    renderer.clear();

    // cr.set_line_width(0.05);

    renderer.set_draw_color(Color::RGB(66, 66, 66));
    renderer.draw_lines(&[Point::new(15, 1).scale(10),
                      Point::new(5, 1).scale(10),
                      Point::new(0, 10).scale(10),
                      Point::new(5, 19).scale(10),
                      Point::new(15, 19).scale(10),
                      Point::new(20, 10).scale(10),
                      Point::new(15, 1).scale(10)])
        .unwrap();

    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { /*keycode: Some(Keycode::Escape),*/ .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}

pub fn draw_hexagon(x: usize, y: usize) {
    //TODO
}
