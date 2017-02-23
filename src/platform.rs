
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event as PlatformEvent;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
use std::path::Path;
use sdl2::TimerSubsystem;
use sdl2::rect::Point;
use sdl2::image::LoadTexture;

pub struct Platform<'a> {
    pub renderer: Renderer<'a>,
    event_pump: EventPump,
    pub window_width: i16,
    pub window_height: i16,
    timer: TimerSubsystem,
    anim_texture: sdl2::render::Texture,
    source_rect: sdl2::rect::Rect,
    dest_rect: sdl2::rect::Rect,
}

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

use std::f32::consts::PI;

const TAU_OVER_SIX: f32 = PI / 3f32;
const TAU_OVER_TWELEVE: f32 = PI / 6f32;

lazy_static!{
    static ref FLAT_UNIT_HEXAGON_XS: [f32 ; 7] =
        [f32::cos(TAU_OVER_SIX * 0f32),
         f32::cos(TAU_OVER_SIX * 1f32),
         f32::cos(TAU_OVER_SIX * 2f32),
         f32::cos(TAU_OVER_SIX * 3f32),
         f32::cos(TAU_OVER_SIX * 4f32),
         f32::cos(TAU_OVER_SIX * 5f32),
         f32::cos(TAU_OVER_SIX * 6f32)];

     static ref FLAT_UNIT_HEXAGON_YS: [f32 ; 7] =
         [f32::sin(TAU_OVER_SIX * 0f32),
          f32::sin(TAU_OVER_SIX * 1f32),
          f32::sin(TAU_OVER_SIX * 2f32),
          f32::sin(TAU_OVER_SIX * 3f32),
          f32::sin(TAU_OVER_SIX * 4f32),
          f32::sin(TAU_OVER_SIX * 5f32),
          f32::sin(TAU_OVER_SIX * 6f32)];

    static ref POINTY_UNIT_HEXAGON_XS: [f32 ; 7] =
        [f32::cos(TAU_OVER_TWELEVE * 1f32),
         f32::cos(TAU_OVER_TWELEVE * 3f32),
         f32::cos(TAU_OVER_TWELEVE * 5f32),
         f32::cos(TAU_OVER_TWELEVE * 7f32),
         f32::cos(TAU_OVER_TWELEVE * 9f32),
         f32::cos(TAU_OVER_TWELEVE * 11f32),
         f32::cos(TAU_OVER_TWELEVE * 13f32)];

    static ref POINTY_UNIT_HEXAGON_YS: [f32 ; 7] =
        [f32::sin(TAU_OVER_TWELEVE * 1f32),
         f32::sin(TAU_OVER_TWELEVE * 3f32),
         f32::sin(TAU_OVER_TWELEVE * 5f32),
         f32::sin(TAU_OVER_TWELEVE * 7f32),
         f32::sin(TAU_OVER_TWELEVE * 9f32),
         f32::sin(TAU_OVER_TWELEVE * 11f32),
         f32::sin(TAU_OVER_TWELEVE * 13f32)];
}


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

        let mut renderer = window.renderer().accelerated().build().unwrap();

        renderer.set_draw_color(Color::RGB(250, 224, 55));
        renderer.clear();

        let mut anim_texture = renderer.load_texture(Path::new("assets/hexagonPack_sheet.png"))
            .unwrap();
        anim_texture.set_color_mod(0, 255, 255);

        let center = Point::new((window_width / 2) as i32, (window_height / 2) as i32);
        let source_rect = Rect::new(0, 0, 120, 140);
        let mut dest_rect = Rect::new(0, 0, 120, 140);
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
            anim_texture: anim_texture,
            source_rect: source_rect,
            dest_rect: dest_rect,
        }
    }

    pub fn flip_frame(&mut self) {
        self.renderer.present();
        self.renderer.set_draw_color(Color::RGB(250, 224, 55));
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
    pub fn draw_box_upper_left(&mut self,
                               (x, y): (i16, i16),
                               width: u16,
                               height: u16,
                               colour: u32) {
        let ref mut r = self.renderer;

        let old_colour = r.draw_color();
        r.set_draw_color(color_from_u32(colour));

        r.draw_rect(rect!(x, self.window_height - y, width, height))
            .unwrap();

        r.set_draw_color(old_colour);
    }

    pub fn draw_coloured_hexagon(&mut self, (x, y): (i16, i16), side_length: u16, colour: u32) {
        let mut xs: Vec<i16> = Vec::new();
        let mut ys: Vec<i16> = Vec::new();


        let radius = side_length as f32;

        for i in 0..6 {
            xs.push((POINTY_UNIT_HEXAGON_XS[i] * radius + x as f32).round() as i16);
            ys.push(self.window_height -
                    ((POINTY_UNIT_HEXAGON_YS[i] * radius + y as f32).round() as i16));
        }

        self.renderer.filled_polygon(&xs, &ys, colour).unwrap();
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

    pub fn animate(&mut self) {
        // let ticks = self.timer.ticks();

        let next_x = (self.source_rect.x() + 1) % 360;
        self.source_rect.set_x(next_x);


        self.renderer.draw_rect(self.dest_rect).unwrap();

        self.renderer
            .copy_ex(&self.anim_texture,
                     Some(self.source_rect),
                     Some(self.dest_rect),
                     0.0,
                     None,
                     true,
                     false)
            .unwrap();
        self.renderer.present();
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

fn color_from_u32(bits: u32) -> Color {
    Color::RGBA((bits & 0x000000FF) as u8,
                (bits & 0x0000FF00 >> 8) as u8,
                (bits & 0x00FF0000 >> 16) as u8,
                (bits & 0xFF000000 >> 24) as u8)
}

#[derive(Debug)]
pub enum Event {
    Quit,
    MouseUp { x: i16, y: i16 },
    MouseMove { x: i16, y: i16 },
}
use self::Event::*;
