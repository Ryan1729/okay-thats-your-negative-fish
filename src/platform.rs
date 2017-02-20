extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event as PlatformEvent;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::gfx::primitives::DrawRenderer;

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

    pub fn flip_frame(&mut self) {
        self.renderer.present();
    }

    // pub fn draw_hexagon(&mut self, x: i32, y: i32) {
    //     self.renderer.set_draw_color(Color::RGB(66, 66, 66));
    //     let mut points = &mut [Point::new(5, -9),
    //                            Point::new(-5, -9),
    //                            Point::new(-10, 0),
    //                            Point::new(-5, 9),
    //                            Point::new(5, 9),
    //                            Point::new(10, 0),
    //                            Point::new(5, -9)];
    //
    //     for point in points.iter_mut() {
    //         *point = point.scale(10).offset(x, y);
    //     }
    //
    //     self.renderer.draw_lines(points).unwrap();
    //
    //     self.renderer.present();
    // }

    pub fn draw_coloured_hexagon(&mut self, x: i16, y: i16, colour: u32) {
        let mut xs = &mut [5, -5, -10, -5, 5, 10, 5];
        let mut ys = &mut [-9, -9, 0, 9, 9, 0, -9];

        for i in 0..xs.len() {
            xs[i] = (xs[i] + x) * 5;
            ys[i] = (ys[i] + y) * 5;

        }

        self.renderer.filled_polygon(xs, ys, colour).unwrap();

        self.renderer.present();
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        let mut result = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                PlatformEvent::Quit { .. } |
                PlatformEvent::KeyDown
                { /*keycode: Some(Keycode::Escape),*/
                     .. } => {result.push(Quit)},
                PlatformEvent::MouseButtonUp{ x, y, .. } =>
                  {result.push(Event::MouseUp{ x: x, y: y });}
                _ => {}
                // e => {println!("{:?}", e);}
            }
        }

        result
    }


    pub fn mouse_state(&self) -> MouseState {
        let platform_mouse_state = self.event_pump.mouse_state();

        MouseState {
            x: platform_mouse_state.x(),
            y: platform_mouse_state.y(),
            left: platform_mouse_state.left(),
            middle: platform_mouse_state.middle(),
            right: platform_mouse_state.right(),
        }
    }
}

pub enum Event {
    Quit,
    MouseUp { x: i32, y: i32 },
}
use self::Event::*;

#[derive(Debug)]
pub struct MouseState {
    x: i32,
    y: i32,
    left: bool,
    middle: bool,
    right: bool,
}
