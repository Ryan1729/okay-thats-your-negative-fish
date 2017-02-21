//see http://www.redblobgames.com/grids/hexagons for a description of the axial coordinate system

const SQRT_3: f32 = 1.7320508075688772935274;

use std::ops::Sub;
use std::ops::Neg;
fn axial_to_cube<T: Sub<Output = T> + Neg<Output = T> + Copy>((x, y): (T, T)) -> (T, T, T) {
    (x, -x - y, y)
}

fn cube_to_axial<T>((x, y, z): (T, T, T)) -> (T, T) {
    (x, z)
}

pub fn pixel_to_axial(side_length: u16, (x, y): (i16, i16)) -> (i16, i16) {
    let fx = x as f32;
    let fy = y as f32;
    let side = side_length as f32;
    axial_round((fx as f32 * (2f32 / 3f32) / side as f32, (-fx / 3f32 + SQRT_3 / 3f32 * fy) / side))
}

fn axial_round(coords: (f32, f32)) -> (i16, i16) {
    cube_to_axial(cube_round(axial_to_cube(coords)))
}

fn cube_round((x, y, z): (f32, f32, f32)) -> (i16, i16, i16) {
    let mut rx = x.round();
    let mut ry = y.round();
    let mut rz = z.round();

    let x_diff = f32::abs(rx - x);
    let y_diff = f32::abs(ry - y);
    let z_diff = f32::abs(rz - z);

    if x_diff > y_diff && x_diff > z_diff {
        rx = -ry - rz;
    } else if y_diff > z_diff {
        ry = -rx - rz;
    } else {
        rz = -rx - ry;
    }

    (rx as i16, ry as i16, rz as i16)
}


pub fn axial_to_pixel(side_length: u16, (x, y): (i16, i16)) -> (i16, i16) {
    let side = side_length as i16;

    (x * (side + side / 2), (SQRT_3 * side as f32 * (y as f32 + x as f32 / 2f32)) as i16)
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
