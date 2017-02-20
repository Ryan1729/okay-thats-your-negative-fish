//dw_hex == double width hexagon
//see http://ondras.github.io/rot.js/manual/#hex/indexing

// fn dw_to_packed((x, y): (usize, usize)) -> (usize, usize) {}
//
// fn packed_to_dw((x, y): (usize, usize)) -> (usize, usize) {}

// fn dw_to_linear((x, y): (usize, usize)) -> usize {
//     4
// }
fn linear_to_dw(width: usize, i: usize) -> (usize, usize) {
    let x = i / width;
    (x, (i % width) * 2 + (x & 1))
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

struct GridIndicesIterator<'a, T: 'a> {
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
