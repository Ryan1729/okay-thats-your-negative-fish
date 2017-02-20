extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event as PlatformEvent;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use dw_hex;
pub struct Platform<'a> {
    renderer: Renderer<'a>,
    event_pump: EventPump,
    window_width: i16,
    window_height: i16,
}

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);



impl<'a> Platform<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window_width = 800;
        let window_height = 600;

        let window = video_subsystem.window("rust-sdl2 demo: Video", window_width, window_height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        renderer.set_draw_color(Color::RGB(250, 224, 55));
        renderer.clear();

        // // Load a font
        // let ttf_context = sdl2::ttf::init().unwrap();
        // let font = ttf_context.load_font("fantasquesansmono-regular-webfont.ttf", 128).unwrap();

        renderer.present();

        let event_pump = sdl_context.event_pump().unwrap();

        Platform {
            renderer: renderer,
            event_pump: event_pump,
            window_width: window_width as i16,
            window_height: window_height as i16,
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

    pub fn render_text(&mut self, text: &str) {
        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font("fantasquesansmono-regular-webfont.ttf", 128).unwrap();
        // render a surface, and convert it to a texture bound to the renderer
        let surface = font.render(text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();
        let mut texture = self.renderer.create_texture_from_surface(&surface).unwrap();

        self.renderer.clear();

        let target = rect!(400, 400, 250, 100);

        self.renderer.copy(&mut texture, None, Some(target)).unwrap();

        self.renderer.present();
    }

    pub fn draw_coloured_hexagon(&mut self, (x, y): (i16, i16), side_length: u16, colour: u32) {
        let mut xs = &mut [5, -5, -10, -5, 5, 10, 5];
        let mut ys = &mut [-9, -9, 0, 9, 9, 0, -9];

        let radius = dw_hex::short_radius(side_length) as i16;

        for i in 0..xs.len() {
            xs[i] = (xs[i] + x) * radius;
            ys[i] = self.window_height - ((ys[i] + y) * radius);
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
                  {result.push(Event::MouseUp{ x: x as i16, y: self.window_height - y as i16 });}
                _ => {}
                // e => {println!("{:?}", e);}
            }
        }

        result
    }
}

#[derive(Debug)]
pub enum Event {
    Quit,
    MouseUp { x: i16, y: i16 },
}
use self::Event::*;
