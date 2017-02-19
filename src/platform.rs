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

    pub fn draw_hexagon(&mut self, x: usize, y: usize) {
        self.renderer.set_draw_color(Color::RGB(66, 66, 66));
        self.renderer
            .draw_lines(&[Point::new(15, 1).scale(10),
                          Point::new(5, 1).scale(10),
                          Point::new(0, 10).scale(10),
                          Point::new(5, 19).scale(10),
                          Point::new(15, 19).scale(10),
                          Point::new(20, 10).scale(10),
                          Point::new(15, 1).scale(10)])
            .unwrap();



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
