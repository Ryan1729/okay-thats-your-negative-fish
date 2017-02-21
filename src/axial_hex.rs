//see http://www.redblobgames.com/grids/hexagons for a description of the axial coordinate system

// sqrt(3)
const HEXAGON_HEIGHT: f32 = 1.7320508075688772935274;

pub fn section(side_length: u16, (x, y): (i16, i16)) -> (i16, i16, i16, i16) {
    let corner_h = corner_height(side_length) as i16;

    let section_width = long_diameter(side_length) as i16;
    let section_height = corner_h + (side_length as i16);
    //splits the grind into (two different types of) sections each containing pieces of three hexes
    let x_section = x / section_width;
    let y_section = y / section_height;

    //the pixel value within a spacewith the origin at the upper left corner
    //of the current section
    let x_section_pixel: i16 = x - (x_section * section_width);
    let y_section_pixel: i16 = y - (y_section * section_height);

    (x_section, y_section, x_section_pixel, y_section_pixel)
}

fn axial_to_cube((x, y): (i16, i16)) -> (i16, i16, i16) {
    (x, -x - y, y)
}

pub fn pixel_to_axial(side_length: u16, (x, y): (i16, i16)) -> (i16, i16) {
    (-1, -1)
}

pub fn axial_to_pixel(side_length: u16, (x, y): (i16, i16)) -> (i16, i16) {
    let side = side_length as i16;

    (x * (side + side / 2), (HEXAGON_HEIGHT * side as f32 * (y as f32 + x as f32 / 2f32)) as i16)
}
// fn dw_to_linear((x, y): (usize, usize)) -> usize {
//     4
// }
fn linear_to_axial(width: usize, i: usize) -> (usize, usize) {
    (i % width, i / width)
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

#[derive(Debug)]
pub struct Grid<T> {
    pub vec: Vec<Option<T>>,
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
        self.vec.push(Some(element));
    }

    pub fn indices(&self) -> IndicesIterator<T> {
        IndicesIterator {
            grid: self,
            index: 0,
        }
    }
}

pub struct IndicesIterator<'a, T: 'a> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for IndicesIterator<'a, T> {
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        self.index += 1;

        let result = match self.grid.vec.get(index) {
            Some(option) => {
                match option.as_ref() {
                    Some(element) => Some((linear_to_axial(self.grid.width, index), element)),
                    None => self.next(),
                }
            }
            _ => return None,
        };


        result
    }
}
