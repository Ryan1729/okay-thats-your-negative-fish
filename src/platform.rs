
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event as PlatformEvent;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
use std::path::Path;
use sdl2::TimerSubsystem;
use sdl2::rect::Point;
use sdl2::image::LoadTexture;

use consts;

pub struct Platform<'a> {
    pub renderer: Renderer<'a>,
    event_pump: EventPump,
    pub window_width: i16,
    pub window_height: i16,
    timer: TimerSubsystem,
    spritesheet: sdl2::render::Texture,
    source_rect: sdl2::rect::Rect,
    dest_rect: sdl2::rect::Rect,
    hex_dimensions: (u32, u32),
}

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

macro_rules! red {
    ($x:expr) => ((($x & 0x00FF0000) >> 16) as u8)
}
macro_rules! green {
    ($x:expr) => ((($x & 0x0000FF00) >> 8) as u8)
}
macro_rules! blue {
    ($x:expr) => (($x & 0x000000FF) as u8)
}
macro_rules! alpha {
    ($x:expr) => ((($x & 0xFF000000) >> 24) as u8)
}

fn color_from_u32(bits: u32) -> Color {
    Color::RGBA(red!(bits), green!(bits), blue!(bits), alpha!(bits))
}

impl<'a> Platform<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window_width = 800;
        let window_height = 600;

        let hex_dimensions = (120, 140);

        let window = video_subsystem.window("rust-sdl2 demo: Video", window_width, window_height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().accelerated().build().unwrap();

        renderer.set_draw_color(Color::RGB(250, 224, 55));
        renderer.clear();

        let mut spritesheet = renderer.load_texture(Path::new("assets/hexagon-3to3.png"))
            .unwrap();

        let center = Point::new((window_width / 2) as i32, (window_height / 2) as i32);
        let source_rect = Rect::new(0, 0, hex_dimensions.0, hex_dimensions.1);
        let mut dest_rect = Rect::new(0, 0, hex_dimensions.0, hex_dimensions.1);
        dest_rect.center_on(center);

        // // Load a font
        // let ttf_context = sdl2::ttf::init().unwrap();
        // let font = ttf_context.load_font("fantasquesansmono-regular-webfont.ttf", 128).unwrap();

        renderer.present();

        let event_pump = sdl_context.event_pump().unwrap();

        let timer = sdl_context.timer().unwrap();

        Platform {
            renderer: renderer,
            event_pump: event_pump,
            window_width: window_width as i16,
            window_height: window_height as i16,
            timer: timer,
            spritesheet: spritesheet,
            source_rect: source_rect,
            dest_rect: dest_rect,
            hex_dimensions: hex_dimensions,
        }
    }

    pub fn flip_frame(&mut self) {
        self.renderer.present();
        self.renderer.set_draw_color(Color::RGB(250, 224, 55));
        self.spritesheet.set_color_mod(255, 255, 255);
    }

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

        let target = rect!(100, self.window_height - 600, text.len() * 20, 100);

        self.renderer.copy(&mut texture, None, Some(target)).unwrap();
    }

    pub fn draw_box(&mut self, (x, y): (i16, i16), width: u16, height: u16, colour: u32) {
        let ref mut r = self.renderer;

        let old_colour = r.draw_color();
        r.set_draw_color(color_from_u32(colour));

        r.draw_rect(rect!((x - (width as i16 / 2)),
                             self.window_height - (y + (height as i16 / 2)),
                             width,
                             height))
            .unwrap();

        r.set_draw_color(old_colour);
    }

    pub fn draw_bitmap_hexagon(&mut self,
                               (x, y): (i16, i16),
                               (u, v): (u16, u16),
                               mut colour: u32) {
        let (w, h) = self.hex_dimensions;
        let source_rect = rect!(u * w as u16, v * h as u16, w, h);
        let mut dest_rect = rect!(0, 0, w, h);
        dest_rect.center_on(Point::new(x as i32, (self.window_height - y) as i32));

        if alpha!(colour) == 0 {
            self.spritesheet.set_color_mod(255, 255, 255);
        } else {
            self.spritesheet.set_color_mod(red!(colour), green!(colour), blue!(colour))
        }

        self.renderer
            .copy(&self.spritesheet, Some(source_rect), Some(dest_rect))
            .unwrap();
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
                    {result.push(
                        Event::MouseUp{ x: x as i16, y: self.window_height - y as i16 });}

                PlatformEvent::MouseMotion{ x, y, .. } =>
                    {result.push(
                        Event::MouseMove{ x: x as i16, y: self.window_height - y as i16 });}
                _ => {}
                // e => {println!("{:?}", e);}
            }
        }

        result
    }


    pub fn render_to_buffer(&mut self, render_commands: &Fn(&mut [u8], usize)) {

        let mut texture = self.renderer
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .unwrap();
        // Create a red-green gradient
        texture.with_lock(None, render_commands)
            .unwrap();

        let platform_mouse_state = self.event_pump.mouse_state();

        self.renderer
            .copy(&texture,
                  None,
                  Some(Rect::new(platform_mouse_state.x(), platform_mouse_state.y(), 256, 256)))
            .unwrap();
    }

    pub fn mouse_state(&self) -> MouseState {
        let platform_mouse_state = self.event_pump.mouse_state();

        MouseState {
            x: platform_mouse_state.x(),
            y: self.window_height as i32 - platform_mouse_state.y(),
            left: platform_mouse_state.left(),
            middle: platform_mouse_state.middle(),
            right: platform_mouse_state.right(),
        }
    }
}


#[derive(Debug)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}



#[derive(Debug)]
pub enum Event {
    Quit,
    MouseUp { x: i16, y: i16 },
    MouseMove { x: i16, y: i16 },
}
use self::Event::*;
