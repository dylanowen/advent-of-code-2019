use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::RangeInclusive;
use std::path::Path;
use std::{fmt, mem};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub static ZERO_POINT: Point = Point { x: 0, y: 0 };

impl Point {
    pub fn distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Clone, Hash)]
pub struct Grid<T: Clone> {
    x_offset: isize,
    y_offset: isize,
    width: usize,

    grid: Vec<Vec<T>>,

    default: T,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(center_x: isize, center_y: isize) -> Grid<T> {
        Self::new_with_dimensions(
            (center_x - 8)..=(center_x + 8),
            (center_y - 8)..=(center_y + 8),
        )
    }

    pub fn new_with_dimensions(x: RangeInclusive<isize>, y: RangeInclusive<isize>) -> Grid<T> {
        let width: usize = (x.end() - x.start()) as usize;
        let height: usize = (x.end() - x.start()) as usize;

        Grid {
            x_offset: *x.start(),
            y_offset: *y.start(),
            width,
            grid: vec![vec![Default::default(); width]; height],
            default: Default::default(),
        }
    }

    pub fn get(&self, x: isize, y: isize) -> &T {
        let raw_x = self.raw_x(x);
        let raw_y = self.raw_y(y);

        if raw_x >= 0
            && raw_y >= 0
            && raw_y < self.grid.len() as isize
            && raw_x < self.grid[raw_y as usize].len() as isize
        {
            &self.grid[raw_y as usize][raw_x as usize]
        } else {
            &self.default
        }
    }

    pub fn set(&mut self, x: isize, y: isize, value: T) -> T {
        let mut raw_x = self.raw_x(x);
        let mut raw_y = self.raw_y(y);

        if raw_y < 0 {
            let mut to_prepend = vec![vec![Default::default(); 1]; raw_y.abs() as usize];
            to_prepend.append(&mut self.grid);
            self.grid = to_prepend;

            self.y_offset += raw_y;

            raw_y = self.raw_y(y);
        } else if raw_y >= self.grid.len() as isize {
            let mut to_append =
                vec![vec![Default::default(); 1]; (raw_y as usize) - self.grid.len() + 1];
            self.grid.append(&mut to_append);
        }

        let y_index = raw_y as usize;

        if raw_x < 0 {
            println!(
                "raw_x < 0, {} {} {} {} {}",
                self.width, self.x_offset, self.y_offset, raw_x, x
            );
            let new_columns = raw_x.abs() as usize;
            for i in 0..self.grid.len() {
                let mut to_prepend = vec![Default::default(); new_columns];

                to_prepend.append(&mut self.grid[i]);
                self.grid[i] = to_prepend;
            }
            self.x_offset += raw_x;
            self.width += new_columns;

            raw_x = self.raw_x(x);
        } else if raw_x >= self.grid[y_index].len() as isize {
            let mut to_add =
                vec![Default::default(); (raw_x as usize) - self.grid[y_index].len() + 1];
            self.grid[y_index].append(&mut to_add);

            self.width = self.width.max(self.grid[y_index].len())
        }

        let x_index = raw_x as usize;

        mem::replace(&mut self.grid[y_index][x_index], value)
    }

    pub fn enumerate(&self) -> GridEnumerator<T> {
        GridEnumerator {
            grid: self,
            x: self.x_min(),
            y: self.y_min(),
        }
    }

    pub fn x_min(&self) -> isize {
        self.x_offset
    }

    pub fn y_min(&self) -> isize {
        self.y_offset
    }

    // exclusive max
    pub fn x_max(&self) -> isize {
        self.x_min() + (self.width() as isize)
    }

    // exclusive max
    pub fn y_max(&self) -> isize {
        self.y_min() + (self.height() as isize)
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn raw_x(&self, x: isize) -> isize {
        x - self.x_min()
    }

    fn raw_y(&self, y: isize) -> isize {
        y - self.y_min()
    }

    pub fn write_image<F>(&self, path: &str, converter: F)
    where
        F: Fn(&T) -> [u8; 4],
    {
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width() as u32, self.height() as u32);

        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut stream_writer = writer.stream_writer();

        let mut buffer = vec![0; self.width() * 4];
        for y in (self.y_min()..self.y_max()).rev() {
            for x in self.x_min()..self.x_max() {
                let offset: usize = (self.raw_x(x) as usize) * 4;
                let rgba = converter(self.get(x as isize, y as isize));
                buffer.splice(offset..offset + 4, rgba.iter().copied());
            }

            stream_writer.write_all(buffer.as_slice()).unwrap();
        }

        stream_writer.finish().unwrap();
    }
}

impl<T: fmt::Display + Clone + Default> Grid<T> {
    pub fn print_bottom_up(&self) {
        for y in (self.y_min()..self.y_max()).rev() {
            for x in self.x_min()..self.x_max() {
                print!("{}", self.get(x as isize, y as isize));
            }
            println!();
        }
    }

    pub fn print_top_down(&self) {
        for y in self.y_min()..self.y_max() {
            for x in self.x_min()..self.x_max() {
                print!("{}", self.get(x as isize, y as isize));
            }
            println!();
        }
    }
}

pub struct GridEnumerator<'a, T: Clone + Default> {
    grid: &'a Grid<T>,
    x: isize,
    y: isize,
}

impl<'a, T: Clone + Default> Iterator for GridEnumerator<'a, T> {
    type Item = (isize, isize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        // check if we've iterated past the grid
        if self.y < self.grid.y_max() {
            let result = Some((self.x, self.y, self.grid.get(self.x, self.y)));

            self.x += 1;
            if self.x >= self.grid.x_max() {
                self.x = self.grid.x_min();
                self.y += 1;
            }

            result
        } else {
            None
        }
    }
}

//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn negative() {
//        let mut grid = Grid::new(0, 0);
//        grid.set(-1, -1, true);
//
//        grid.set(-1, -1, true);
//        grid.set(-1, -1, true);
//    }
//}
