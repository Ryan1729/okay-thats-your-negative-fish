
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event as SDL2_Event;
use sdl2::event::WindowEvent;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
use std::path::Path;
use sdl2::TimerSubsystem;
use sdl2::rect::Point;
use sdl2::image::LoadTexture;

use axial_hex;
use consts;

use std;

use common;
use common::PieceState;
use common::PieceState::*;

use platform::Platform;
use platform::Event;

pub struct Sdl2Platform<'a> {
    pub renderer: Renderer<'a>,
    event_pump: EventPump,
    pub window_width: i16,
    pub window_height: i16,
    timer: TimerSubsystem,
    spritesheet: sdl2::render::Texture,
    background: sdl2::render::Texture,
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

pub fn new<'a>() -> Sdl2Platform<'a> {
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

    let spritesheet = renderer.load_texture(Path::new("assets/hexagon-3to3.png"))
        .unwrap();
    let background = renderer.load_texture(Path::new("assets/background.png"))
        .unwrap();

    let center = Point::new((window_width / 2) as i32, (window_height / 2) as i32);
    let source_rect = Rect::new(0, 0, hex_dimensions.0, hex_dimensions.1);
    let mut dest_rect = Rect::new(0, 0, hex_dimensions.0, hex_dimensions.1);
    dest_rect.center_on(center);

    renderer.present();

    let event_pump = sdl_context.event_pump().unwrap();

    let timer = sdl_context.timer().unwrap();

    Sdl2Platform {
        renderer: renderer,
        event_pump: event_pump,
        window_width: window_width as i16,
        window_height: window_height as i16,
        timer: timer,
        spritesheet: spritesheet,
        background: background,
        source_rect: source_rect,
        dest_rect: dest_rect,
        hex_dimensions: hex_dimensions,
    }
}

const BACKGROUND_WIDTH: i16 = 1024;
const BACKGROUND_HEIGHT: i16 = 768;

impl<'a> Platform for Sdl2Platform<'a> {
    fn flip_frame(&mut self) {
        self.renderer.set_blend_mode(sdl2::render::BlendMode::Add);
        self.renderer.present();
        self.renderer.set_draw_color(Color::RGB(250, 224, 55));
        self.spritesheet.set_color_mod(255, 255, 255);
    }

    fn draw_background(&mut self) {
        let source_rect = rect!(0, 0, BACKGROUND_WIDTH, BACKGROUND_HEIGHT);
        let mut dest_rect;

        let mut x: i16 = 0;
        let mut y: i16 = 0;

        while y < self.window_height {
            dest_rect = rect!(x, y, BACKGROUND_WIDTH, BACKGROUND_HEIGHT);

            self.renderer
                .copy(&self.background, Some(source_rect), Some(dest_rect))
                .unwrap();

            if let Some(new_x) = x.checked_add(BACKGROUND_WIDTH) {
                x = new_x;
            } else {
                break;
            }

            if x >= self.window_width {
                x = 0;
                if let Some(new_y) = y.checked_add(BACKGROUND_HEIGHT) {
                    y = new_y;
                } else {
                    break;
                }
            }
        }
    }

    fn draw_hexagon(&mut self, (grid_x, grid_y): (i16, i16), tile_type: u16, colour: u32) {
        let (x, y) = add(axial_hex::axial_to_pixel_pointy(common::SIDE_LENGTH,
                                                          (grid_x as i16, grid_y as i16)),
                         unsafe { common::GRID_OFFSET });


        let (u, v) = (tile_type / 4, tile_type % 2);
        let (w, h) = self.hex_dimensions;
        let source_rect = rect!(u * w as u16, v * h as u16, w, h);
        let mut dest_rect = rect!(0,
                                  0,
                                  axial_hex::short_diameter(common::SIDE_LENGTH),
                                  axial_hex::long_diameter(common::SIDE_LENGTH));
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

    fn draw_piece(&mut self, (grid_x, grid_y): (i16, i16), piece_state: &PieceState) {
        let (x, y) = add(axial_hex::axial_to_pixel_pointy(common::SIDE_LENGTH,
                                                          (grid_x as i16, grid_y as i16)),
                         unsafe { common::GRID_OFFSET });
        let (u, v) = piece_uv(&piece_state);
        let (w, h) = consts::PIECE_DIMENSIONS;
        let source_rect = rect!(u, v, w, h);
        let mut dest_rect = rect!(0, 0, w * 2, h * 2);

        dest_rect.center_on(Point::new(x as i32, (self.window_height - y) as i32));

        self.renderer
            .copy(&self.spritesheet, Some(source_rect), Some(dest_rect))
            .unwrap();
    }

    fn get_events(&mut self) -> Vec<Event> {
        let mut result = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                SDL2_Event::Quit { .. } |
                SDL2_Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    result.push(Event::Quit)
                }
                SDL2_Event::Window { win_event: WindowEvent::Resized(w, h), .. } |
                SDL2_Event::Window { win_event: WindowEvent::SizeChanged(w, h), .. } => {
                    self.window_width = std::cmp::max(32767, w as i16);
                    self.window_width = std::cmp::max(32767, h as i16);
                }
                SDL2_Event::MouseButtonUp { x, y, .. } => {
                    let (ax, ay) = get_axial(x as i16, self.window_height - y as i16);
                    result.push(Event::MouseUp { x: ax, y: ay });
                }

                SDL2_Event::MouseMotion { x, y, .. } => {
                    let (ax, ay) = get_axial(x as i16, self.window_height - y as i16);
                    result.push(Event::MouseMove { x: ax, y: ay });
                }
                _ => {}
                // e => {println!("{:?}", e);}
            }
        }

        result
    }
}

impl<'a> Sdl2Platform<'a> {
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

    pub fn _mouse_state(&self) -> MouseState {
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

fn get_axial(x: i16, y: i16) -> (i16, i16) {
    axial_hex::pixel_to_axial_pointy(common::SIDE_LENGTH,
                                     sub((x, y), unsafe { common::GRID_OFFSET }))
}

use std::ops::Add;
fn add<T: Add<Output = T>>((x1, y1): (T, T), (x2, y2): (T, T)) -> (T, T) {
    (x1 + x2, y1 + y2)
}

use std::ops::Sub;
fn sub<T: Sub<Output = T>>((x1, y1): (T, T), (x2, y2): (T, T)) -> (T, T) {
    (x1 - x2, y1 - y2)
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}
