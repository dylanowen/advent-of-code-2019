use crate::coordinates::two_d::Point;
use crate::coordinates::Grid;

pub fn render_grid<C, T>(pixel_size: usize, img_data: &mut [u8], grid: &Grid<T>, colorizer: C)
where
    C: Fn(&T) -> Option<[u8; 4]>,
    T: Default,
{
    for (Point { x, y }, value) in grid.enumerate() {
        if let Some(color) = colorizer(value) {
            set_grid_square(x, y, color, pixel_size, img_data, grid);
        }
    }

    //   for grid_y in grid.y_range() {
    //      for grid_x in grid.x_range() {
    //         match colorizer(grid.get(grid_x, grid_y)) {
    //            Some(color) => {
    //               set_grid_square(grid_x, grid_y, color, pixel_size, img_data, grid);
    //            }
    //            None => {}
    //         }
    //      }
    //   }
}

pub fn get_img_square_range(x: usize, y: usize, pixel_size: usize) -> (usize, usize, usize, usize) {
    let start_x = x * pixel_size;
    let end_x = start_x + pixel_size;
    let start_y = y * pixel_size;
    let end_y = start_y + pixel_size;

    (start_x, end_x, start_y, end_y)
}

pub fn set_grid_square<T>(
    grid_x: isize,
    grid_y: isize,
    color: [u8; 4],
    pixel_size: usize,
    img_data: &mut [u8],
    grid: &Grid<T>,
) where
    T: Default,
{
    let img_width = grid.width() * pixel_size;
    let x = grid.raw_x(grid_x) as usize;
    let y = grid.raw_y(grid_y) as usize;
    let (start_x, end_x, start_y, end_y) = get_img_square_range(x, y, pixel_size);

    for img_y in start_y..end_y {
        for img_x in start_x..end_x {
            set_img_pixel(img_x, img_y, color, img_width, img_data);
        }
    }
}

pub fn set_img_pixel(
    img_x: usize,
    img_y: usize,
    color: [u8; 4],
    img_width: usize,
    img_data: &mut [u8],
) {
    let pixel = ((img_y * img_width) + img_x) * 4;

    img_data[pixel] = color[0];
    img_data[pixel + 1] = color[1];
    img_data[pixel + 2] = color[2];
    img_data[pixel + 3] = color[3];
}
