//dw_hex == double width hexagon
//see http://ondras.github.io/rot.js/manual/#hex/indexing

// sqrt(3)
const HEXAGON_HEIGHT: f32 = 1.7320508075688772935274;

fn dw_to_packed((x, y): (i16, i16)) -> (i16, i16) {
    (x / 2 - (y & 1), y)
}

fn packed_to_dw((x, y): (i16, i16)) -> (i16, i16) {
    (x * 2 + (y & 1), y)
}

// https://www.gamedev.net/resources/_/technical/game-programming/
// coordinates-in-hexagon-based-tile-maps-r1800
fn pixel_to_packed(side_length: u16, (x, y): (i16, i16)) -> (i16, i16) {
    let corner_h = corner_height(side_length) as i16;
    let radius = short_radius(side_length) as i16;

    let section_width = radius * 2;
    let section_height = corner_h + (side_length as i16);
    //splits the grind into (two different types of) sections each containing pieces of three hexes
    let x_section = x / section_width;
    let y_section = y / section_height;

    //the pixel value within a spacewith the origin at the upper left corner
    //of the current section
    let x_section_pixel: i16 = x - (x_section * section_width);
    let y_section_pixel: i16 = y - (y_section * section_height);

    let slope: i16 = corner_h / radius;

    if y_section & 1 == 0 {
        let left_top_hexagon_edge: i16 = corner_h - x_section_pixel * slope;

        if y_section_pixel < left_top_hexagon_edge {
            (y_section - 1, x_section - 1)
        } else if y_section_pixel < -left_top_hexagon_edge {
            (x_section, y_section - 1)
        } else {
            (x_section, y_section)
        }
    } else {
        if x_section_pixel >= radius {
            if x_section_pixel < 2 * corner_h - x_section_pixel * slope {
                (x_section - 1, y_section - 1)
            } else {
                (x_section, y_section)
            }
        } else {
            if x_section_pixel < x_section_pixel * slope {
                (x_section, y_section - 1)
            } else {
                (x_section - 1, y_section)
            }
        }

    }
}

fn packed_to_pixel(side_length: u16, (x, y): (i16, i16)) -> (i16, i16) {
    let radius = short_radius(side_length) as i16;
    let corner_h = corner_height(side_length) as i16;

    (x * 2 * radius + (x & 1) * radius, y * (corner_h + side_length as i16))
}

pub fn pixel_to_dw(side_length: u16, coords: (i16, i16)) -> (i16, i16) {
    packed_to_dw(pixel_to_packed(side_length, coords))
}
pub fn dw_to_pixel(side_length: u16, (x, y): (i16, i16)) -> (i16, i16) {
    let radius = short_radius(side_length) as i16;

    //we could skip a conversion from f32 to i16 before multiplying by HEXAGON_HEIGHT
    //but the truncation makes is necessary for the hexes to line up
    (x * (radius + radius / 2), y * (HEXAGON_HEIGHT / 2f32 * radius as f32) as i16)
}
// fn dw_to_linear((x, y): (usize, usize)) -> usize {
//     4
// }
fn linear_to_dw(width: usize, i: usize) -> (usize, usize) {
    //defaulting to 1 means we can't have width 1 grids,
    // but who needs one of those?
    let half_width = if width > 1 { width / 2 } else { 1 };

    let x = i / half_width;

    ((i % half_width) * 2 + (x & 1), x)
}

use std::f32::consts::PI;

pub fn corner_height(side_length: u16) -> u16 {
    (f32::sin(PI / 6f32) * side_length as f32) as u16
}
pub fn short_radius(side_length: u16) -> u16 {
    (f32::cos(PI / 6f32) * side_length as f32) as u16
}
pub fn long_diameter(side_length: u16) -> u16 {
    side_length + 2 * corner_height(side_length)
}
pub fn short_diameter(side_length: u16) -> u16 {
    2 * short_radius(side_length)
}


pub struct Grid<T> {
    pub vec: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize) -> Self {
        Grid {
            vec: Vec::new(),
            width: width,
        }
    }

    pub fn push(&mut self, element: T) {
        self.vec.push(element);
    }

    pub fn indices(&self) -> GridIndicesIterator<T> {
        GridIndicesIterator {
            grid: self,
            index: 0,
        }
    }
}

pub struct GridIndicesIterator<'a, T: 'a> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridIndicesIterator<'a, T> {
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.grid.vec.get(self.index) {
            Some(element) => Some((linear_to_dw(self.grid.width, self.index), element)),
            _ => return None,
        };

        self.index += 1;
        result
    }
}
