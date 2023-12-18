mod input;

fn main() {}

#[derive(Debug, PartialEq)]
enum GridItem {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    Mirror90Back,
    Mirror90Forward,
}

fn parse(input: &str) -> Vec<Vec<GridItem>> {
    input.lines()
        .map(|line| line.chars()
            .map(|c| match c {
                '.' => GridItem::Empty,
                '|' => GridItem::VerticalSplitter,
                '-' => GridItem::HorizontalSplitter,
                '\\' => GridItem::Mirror90Back,
                '/' => GridItem::Mirror90Forward,
                _ => panic!("Unknown character: {}", c),
            })
            .collect())
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Clone)]
enum LightDirection {
    Up,
    Down,
    Left,
    Right,
}

fn light_path(grid: &Vec<Vec<GridItem>>, mut start: Vec<(LightDirection, Point)>) -> Vec<(usize, usize, LightDirection)> {
    if start.len() == 0 {
        start.push((LightDirection::Right, Point{x: 0, y: 0}));
    }
    let mut light_paths = start;
    let mut energized_points: Vec<(usize, usize, LightDirection)> = Vec::new();

    loop {
        let mut new_light_paths: Vec<(LightDirection, Point)> = Vec::new();
        let mut remove_light_paths: Vec<(LightDirection, Point)> = Vec::new();
        if light_paths.len() == 0 {
            break;
        }
        //println!("Light paths: {:?}", light_paths);
        for (ref mut direction, ref mut path) in light_paths.iter_mut() {
            if path.x >= grid[0].len() as i32 ||
                path.y >= grid.len() as i32 ||
                path.x < 0 ||
                path.y < 0 {
                // this item is out of bounds, so we can't continue
                remove_light_paths.push((direction.clone(), path.clone()));
                continue;
            }

            if energized_points.contains(&(path.x as usize, path.y as usize, direction.clone())) {
                // this item has already been energized, so we can't continue
                remove_light_paths.push((direction.clone(), path.clone()));
                continue;
            }

            let grid_item = &grid[path.y as usize][path.x as usize];
            //println!("Direction: {:?}, Path: {:?}, Current: {:?}", direction, path, grid_item);

            if !energized_points.contains(&(path.x as usize, path.y as usize, direction.clone())) {
                energized_points.push((path.x as usize, path.y as usize, direction.clone()));
            }

            match grid_item {
                GridItem::VerticalSplitter => {
                    match direction {
                        LightDirection::Left | LightDirection::Right => {
                            new_light_paths.push((LightDirection::Up, Point{x: path.x, y: path.y - 1}));
                            *direction = LightDirection::Down;
                            path.y += 1;
                        }
                        LightDirection::Up => path.y -= 1,
                        LightDirection::Down => path.y += 1
                    }
                }
                GridItem::HorizontalSplitter => {
                    match direction {
                        LightDirection::Up | LightDirection::Down => {
                            new_light_paths.push((LightDirection::Left, Point{x: path.x-1, y: path.y}));
                            *direction = LightDirection::Right;
                            path.x += 1;
                        }
                        LightDirection::Right => path.x += 1,
                        LightDirection::Left => path.x -= 1
                    }
                }
                GridItem::Mirror90Back => {
                    match direction {
                        LightDirection::Up => {
                            *direction = LightDirection::Left;
                            path.x -= 1;
                        }
                        LightDirection::Down => {
                            *direction = LightDirection::Right;
                            path.x += 1;
                        }
                        LightDirection::Left => {
                            *direction = LightDirection::Up;
                            path.y -= 1;
                        }
                        LightDirection::Right => {
                            *direction = LightDirection::Down;
                            path.y += 1;
                        }
                    }
                }
                GridItem::Mirror90Forward => {
                    match direction {
                        LightDirection::Up => {
                            *direction = LightDirection::Right;
                            path.x += 1;
                        }
                        LightDirection::Down => {
                            *direction = LightDirection::Left;
                            path.x -= 1;
                        }
                        LightDirection::Left => {
                            *direction = LightDirection::Down;
                            path.y += 1;
                        }
                        LightDirection::Right => {
                            *direction = LightDirection::Up;
                            path.y -= 1;
                        }
                    }
                }
                _ => {
                    match direction {
                        LightDirection::Up => path.y -= 1,
                        LightDirection::Down => path.y += 1,
                        LightDirection::Left => path.x -= 1,
                        LightDirection::Right => path.x += 1,
                    }
                }
            }
        }
        for new_light_path in new_light_paths {
            light_paths.push(new_light_path);
        }
        for remove in remove_light_paths {
            light_paths.iter().position(|x| x == &remove).map(|e| light_paths.remove(e));
        }
    }
    energized_points
}

fn modify_start_position(grid: &Vec<Vec<GridItem>>) -> usize {
    // get a list of possible directions
    let mut start_positions: Vec<(LightDirection, Point)> = Vec::new();
    grid.iter().enumerate().for_each(|(index, row)| {
        if index == 0 {
           // generate one for each row.
           row.iter().enumerate().for_each(|(x_index, _)| {
                start_positions.push((LightDirection::Down, Point{x: x_index as i32, y: index as i32}));
           });
        } else if index == grid.len() - 1 {
            row.iter().enumerate().for_each(|(x_index, _)| {
                start_positions.push((LightDirection::Up, Point{x: x_index as i32, y: index as i32}));
            });
        } else {
            start_positions.push((LightDirection::Right, Point{x: 0, y: index as i32}));
            start_positions.push((LightDirection::Left, Point{x: row.len() as i32 - 1, y: index as i32}));
        }
    });
    let max = start_positions.iter().map(|(direction, point)| dedupe_energized(&light_path(&grid, vec![(direction.clone(), point.clone())])).len()).max().take();
    println!("Max: {:?}", max);
    max.unwrap()
}

fn dedupe_energized(energized_points: &Vec<(usize, usize, LightDirection)>) -> Vec<(usize, usize)> {
    let mut deduped_energized_points: Vec<(usize, usize)> = Vec::new();
    for (x, y, _) in energized_points {
        if !deduped_energized_points.contains(&(*x, *y)) {
            deduped_energized_points.push((*x, *y));
        }
    }
    deduped_energized_points
}

#[cfg(test)]
mod tests {
    use crate::{dedupe_energized, input, modify_start_position, parse};

    #[test]
    fn can_parse_example_input() {
        let parsed = parse(input::EXAMPLE_INPUT);
        assert_eq!(parsed.len(), 10);
        assert_eq!(parsed[0].len(), 10);
        assert_eq!(parsed[1], vec![
            super::GridItem::VerticalSplitter,
            super::GridItem::Empty,
            super::GridItem::HorizontalSplitter,
            super::GridItem::Empty,
            super::GridItem::Mirror90Back,
            super::GridItem::Empty,
            super::GridItem::Empty,
            super::GridItem::Empty,
            super::GridItem::Empty,
            super::GridItem::Empty,
        ]);
    }

    #[test]
    fn can_solve_example_input_part_one() {
        let parsed = parse(input::EXAMPLE_INPUT);
        let energized_points = super::light_path(&parsed, vec![]);
        let mut deduped_energized_points: Vec<(usize, usize)> = Vec::new();
        for (x, y, _) in energized_points {
            if !deduped_energized_points.contains(&(x, y)) {
                deduped_energized_points.push((x, y));
            }
        }
        //energized_points.iter().filter(|(x, y, _)| ).count();
        assert_eq!(deduped_energized_points.len(), 46);
    }

    #[test]
    fn can_solve_input_part_one() {
        let parsed = parse(input::INPUT);
        let energized_points = super::light_path(&parsed, vec![]);
        //energized_points.iter().filter(|(x, y, _)| ).count();
        assert_eq!(dedupe_energized(&energized_points).len(), 7046);
    }

    #[test]
    fn can_solve_example_input_part_two() {
        let parsed = parse(input::EXAMPLE_INPUT);
        let result = modify_start_position(&parsed);
        assert_eq!(result, 51);
    }

    #[test]
    fn can_solve_input_part_two() {
        let parsed = parse(input::INPUT);
        let result = modify_start_position(&parsed);
        assert_eq!(result, 7313);
    }
}
