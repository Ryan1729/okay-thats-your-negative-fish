extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Renderer;
use sdl2::EventPump;

pub struct Platform<'a> {
    renderer: Renderer<'a>,
    event_pump: EventPump,
}

impl<'a> Platform<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        renderer.set_draw_color(Color::RGB(250, 224, 55));
        renderer.clear();

        renderer.present();

        let event_pump = sdl_context.event_pump().unwrap();

        Platform {
            renderer: renderer,
            event_pump: event_pump,
        }
    }

    pub fn draw_hexagon(&mut self, x: i32, y: i32) {
        self.renderer.set_draw_color(Color::RGB(66, 66, 66));
        let mut points = &mut [Point::new(5, -9),
                               Point::new(-5, -9),
                               Point::new(-10, 0),
                               Point::new(-5, 9),
                               Point::new(5, 9),
                               Point::new(10, 0),
                               Point::new(5, -9)];

        // points = points.iter().map(|point| point.scale(10).offset(x, y)).collect();
        for point in points.iter_mut() {
            *point = point.scale(10).offset(x, y);
        }

        self.renderer.draw_lines(points).unwrap();

        self.renderer.present();
    }

    pub fn quit_on_keypress(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { /*keycode: Some(Keycode::Escape),*/ .. } => {return true;},
                _ => {}
            }
        }

        false
    }
}
