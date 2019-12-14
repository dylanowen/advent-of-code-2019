use crate::coordinates::two_d::{Point, PointLike};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::RangeInclusive;
use std::path::Path;
use std::{fmt, mem};

pub mod two_d;

#[derive(Clone, Hash)]
pub struct Grid<T> {
    x_offset: isize,
    y_offset: isize,
    width: usize,

    grid: Vec<Vec<T>>,

    default: T,
}

impl<T> Grid<T> {
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    fn raw_x(&self, x: isize) -> isize {
        x - self.x_min()
    }

    fn raw_y(&self, y: isize) -> isize {
        y - self.y_min()
    }
}

impl<T: Default> Grid<T> {
    pub fn enumerate(&self) -> GridEnumerator<T> {
        GridEnumerator {
            grid: self,
            location: Point {
                x: self.x_min(),
                y: self.y_min(),
            },
        }
    }
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

impl<T: fmt::Display + Default> Grid<T> {
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

pub struct GridEnumerator<'a, T: Default> {
    grid: &'a Grid<T>,
    location: Point,
}

impl<'a, T: Default> Iterator for GridEnumerator<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        // check if we've iterated past the grid
        if self.location.y() < self.grid.y_max() {
            let result = Some((
                (self.location),
                self.grid.get(self.location.x, self.location.y),
            ));

            *self.location.x_mut() += 1;
            if self.location.x() >= self.grid.x_max() {
                *self.location.x_mut() = self.grid.x_min();
                *self.location.y_mut() += 1;
            }

            result
        } else {
            None
        }
    }
}

pub trait CanvasPixel {
    fn render(&self) -> &[u32];
    fn width() -> usize;
    fn height() -> usize;
}

static COLOR_BYTES: usize = 4;

impl<T: Default + CanvasPixel> Grid<T> {
    pub fn render(&self, img_data: &mut [u8]) {
        for (point, pixel) in self.enumerate() {
            self.render_pixel(pixel, point, img_data);
        }
    }

    pub fn build_img_data(&self) -> Vec<u8> {
        vec![0; (self.canvas_width() * self.canvas_height() * COLOR_BYTES) as usize]
    }

    fn render_pixel<P>(&self, pixel: &T, grid_start: P, img_data: &mut [u8])
    where
        P: PointLike + Sized,
    {
        let pixel_data = pixel.render();

        let canvas_start_x = self.raw_x(grid_start.x()) as usize * T::width();
        let canvas_start_y = self.raw_y(grid_start.y()) as usize * T::height();

        for y in 0..T::height() {
            for x in 0..T::width() {
                let color = pixel_data[y * T::width() + x];
                let r = (color >> 24 & 0xFF) as u8;
                let g = (color >> 16 & 0xFF) as u8;
                let b = (color >> 8 & 0xFF) as u8;
                let a = (color & 0xFF) as u8;

                let byte_offset = ((canvas_start_x + x)
                    + (canvas_start_y + y) * self.canvas_width())
                    * COLOR_BYTES;

                img_data[byte_offset] = r;
                img_data[byte_offset + 1] = g;
                img_data[byte_offset + 2] = b;
                img_data[byte_offset + 3] = a;
            }
        }
    }

    pub fn canvas_width(&self) -> usize {
        self.width() * T::width()
    }

    pub fn canvas_height(&self) -> usize {
        self.height() * T::width()
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
