use std::path::Path;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_polygon_mut, draw_text_mut};
use imageproc::point::Point;
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use crate::Pos;

pub fn draw_to_image(grid: &Vec<Vec<usize>>, file_path: &Path, pos_path: Option<&Vec<Pos>>) {
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    const CELL_WIDTH: u32 = 100;
    const CELL_HEIGHT: u32 = 100;
    const CELL_SHADING: Option<u8> = Some(10);
    let mut image = RgbImage::new(grid_width as u32 * CELL_WIDTH, grid_height as u32 * CELL_HEIGHT);
    image.fill(255u8);
    const BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);
    const DODGER_BLUE: Rgb<u8> = Rgb([30u8, 144u8, 255u8]);
    const LIME_GREEN: Rgb<u8> = Rgb([50u8, 205u8, 50u8]);
    const LIGHT_GRAY: Rgb<u8> = Rgb([150u8, 150u8, 150u8]);

    // draw inner border lines
    for i in 1..grid_width {
        draw_line_segment_mut(&mut image, (i as f32 * CELL_WIDTH as f32, 0.0), (i as f32 * CELL_WIDTH as f32, grid_height as f32 * CELL_HEIGHT as f32), BLACK);
    }
    for i in 1..grid_height {
        draw_line_segment_mut(&mut image, (0.0, i as f32 * CELL_HEIGHT as f32), (grid_width as f32 * CELL_WIDTH as f32, i as f32 * CELL_HEIGHT as f32), BLACK);
    }

    let font = Vec::from(include_bytes!("../DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let height = 48.0;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };
    let no_costs = grid.iter().all(|row| row.iter().all(|cell| *cell == 0));
    let start_pos = pos_path.and_then(|v| v.first());
    let end_pos = pos_path.and_then(|v| v.last());
    fn get_cell_background_color(board_value: u8) -> Option<Rgb<u8>> {
        CELL_SHADING.map(|shading| {
            Rgb([255u8, 255u8 - (board_value - 1) * shading, 255u8 - (board_value - 1) * shading])
        })
    }
    // draw the numbers/walls (with start and end positions)
    for y in 0..grid_height {
        for x in 0..grid_width {
            let board_value = grid[y as usize][x as usize];
            let cur_pos = Pos::new();
            let mut cur_color: &Rgb<u8> = &BLACK;
            // This would be a nice place to use is_some_and(), but it's still unstable
            // https://github.com/rust-lang/rust/issues/93050
            if let Some(start_pos_real) = start_pos {
                if start_pos_real == &cur_pos {
                    cur_color = &DODGER_BLUE;
                }
            }
            if let Some(end_pos_real) = end_pos {
                if end_pos_real == &cur_pos {
                    cur_color = &LIME_GREEN;
                }
            }
            if !no_costs {
                if let Some(cell_background_color) = get_cell_background_color(board_value as u8) {
                    // cells on the left/top border need an extra pixel at the left/top
                    let start_x = if x == 0 { 0 } else {x as i32 * CELL_WIDTH as i32 + 1};
                    let x_size = if x == 0 { CELL_WIDTH } else { CELL_WIDTH - 1};
                    let start_y = if y == 0 { 0 } else {y as i32 * CELL_HEIGHT as i32 + 1};
                    let y_size = if y == 0 { CELL_HEIGHT } else { CELL_HEIGHT - 1};
                    draw_filled_rect_mut(&mut image,
                                         Rect::at(start_x, start_y).of_size(x_size, y_size),
                                         cell_background_color);
                }
                draw_text_mut(&mut image,
                              *cur_color,
                              x as i32 * CELL_WIDTH as i32 + 26,
                              y as i32 * CELL_HEIGHT as i32 + 26,
                              scale,
                              &font,
                              &format!("{}", board_value));
            }
            else {
                // draw a rectangle for the start and end positions
                if cur_color != &BLACK {
                    draw_filled_rect_mut(&mut image,
                                         Rect::at(x as i32 * CELL_WIDTH as i32 + 30, y as i32 * CELL_HEIGHT as i32 + 30).of_size(CELL_WIDTH - 30 * 2, CELL_HEIGHT - 30 * 2),
                                         *cur_color);
                }
            }
        }
    }

    fn get_line_endpoint(start: &Pos, end: &Pos) -> (f32, f32) {
        let x_delta = 20.0 * match end.x.cmp(&start.x) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1
        } as f32;
        let y_delta = 20.0 * match end.y.cmp(&start.y) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1
        } as f32;

        ((start.x as f32 + 0.5) * CELL_WIDTH as f32 + x_delta, (start.y as f32 + 0.5) * CELL_HEIGHT as f32 + y_delta)
    }
    fn get_points_for_rectangle_around_line(start: &(f32, f32), end: &(f32, f32), width: f32, space_for_arrow: f32) -> Vec<Point<i32>> {
        let (x1, y1) = start;
        let (x2, y2) = end;
        let x_delta = x2 - x1;
        let y_delta = y2 - y1;
        let x_delta_norm = x_delta / x_delta.hypot(y_delta);
        let y_delta_norm = y_delta / x_delta.hypot(y_delta);

        vec![
            Point::new((x1 - y_delta_norm * (width / 2.0)) as i32, (y1 + x_delta_norm * (width / 2.0)) as i32),
            Point::new((x1 + y_delta_norm * (width / 2.0)) as i32, (y1 - x_delta_norm * (width / 2.0)) as i32),
            Point::new((x2 + y_delta_norm * (width / 2.0) - x_delta_norm * space_for_arrow) as i32, (y2 - x_delta_norm * (width / 2.0) - y_delta_norm * space_for_arrow) as i32),
            Point::new((x2 - y_delta_norm * (width / 2.0) - x_delta_norm * space_for_arrow) as i32, (y2 + x_delta_norm * (width / 2.0) - y_delta_norm * space_for_arrow) as i32),
        ]
    }
    fn get_points_for_arrowhead(start: &(f32, f32), end: &(f32, f32), width: f32, length: f32) -> Vec<Point<i32>> {
        //
        //    start
        //    ***
        //    * *
        //    * *
        //  ******* <- midpoint of this line is arrow_middle
        //    ***
        //    end

        let (x1, y1) = start;
        let (x2, y2) = end;
        let x_delta = x2 - x1;
        let y_delta = y2 - y1;
        let x_delta_norm = x_delta / x_delta.hypot(y_delta);
        let y_delta_norm = y_delta / x_delta.hypot(y_delta);
        let arrow_middle_x = x2 - x_delta_norm * length;
        let arrow_middle_y = y2 - y_delta_norm * length;

        vec![
            Point::new(*x2 as i32, *y2 as i32),
            Point::new((arrow_middle_x - y_delta_norm * width) as i32, (arrow_middle_y + x_delta_norm * width) as i32),
            Point::new((arrow_middle_x + y_delta_norm * width) as i32, (arrow_middle_y - x_delta_norm * width) as i32),
        ]
    }
    // Draw the path
    if let Some(pos_path) = pos_path {
        pos_path.windows(2).for_each(|pair| {
            let start_pos = &pair[0];
            let end_pos = &pair[1];
            let start_line_endpoint = get_line_endpoint(start_pos, end_pos);
            let end_line_endpoint = get_line_endpoint(end_pos, start_pos);
            draw_polygon_mut(&mut image, &get_points_for_rectangle_around_line(&start_line_endpoint, &end_line_endpoint, 10.0, 24.0), LIGHT_GRAY);
            draw_polygon_mut(&mut image, &get_points_for_arrowhead(&start_line_endpoint, &end_line_endpoint, 14.0, 24.0), LIGHT_GRAY);
        });
    }

    image.save(file_path).unwrap();
}