use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

pub mod input;
pub mod test;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn can_travel(&self, s: &Tile, can_climb_slopes: bool) -> bool {
        if can_climb_slopes {
            match s {
                Tile::Wall => false,
                Tile::Path => true,
                Tile::Slope(_) => true,
            }
        } else {
            match s {
                Tile::Wall => false,
                Tile::Path => true,
                Tile::Slope(d) => d == self,
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Tile {
    Wall,
    Path,
    Slope(Direction),
}

fn find_start_or_end(tiles: &Vec<Vec<Tile>>, y: i32) -> (i32, i32) {
    for (i, tile) in tiles[y as usize].iter().enumerate() {
        if let Tile::Path = tile {
            return (i as i32, y);
        }
    }
    panic!("No pos found at y: {}", y);
}

#[derive(Debug, Clone)]
struct PosInstance {
    x: i32,
    y: i32,
    direction: Direction,
    last_move: Option<Direction>,
    path_history: Vec<(i32, i32)>,
}

pub fn can_move(
    deque: &mut VecDeque<PosInstance>,
    c: &PosInstance,
    tiles: &Vec<Vec<Tile>>,
    skip_slopes: bool,
    direction: Direction,
    (x, y): (i32, i32),
) -> Option<(Direction, i32, i32)> {
    // make sure x & y are in bounds
    if x < 0 || y < 0 || y >= tiles.len() as i32 || x >= tiles[y as usize].len() as i32 {
        return None;
    }

    if direction.can_travel(&tiles[y as usize][x as usize], skip_slopes)
        && c.last_move != Some(direction.opposite())
        && c.path_history
            .iter()
            .find(|(x1, y1)| *x1 == x && *y1 == y)
            .is_none()
    {
        //println!("We can move {:?} from ({}, {}) to ({}, {})", direction, c.x, c.y, x, y);
        return Some((direction, x, y));
    }
    None
}

pub fn find_path(tiles: &Vec<Vec<Tile>>, can_climb_slopes: bool) -> usize {
    let mut deque = VecDeque::new();
    // find the first . in the first row
    let start = find_start_or_end(tiles, 0);
    let end = find_start_or_end(tiles, (tiles.len() - 1) as i32);

    deque.push_back(PosInstance {
        x: start.0,
        y: start.1,
        direction: Direction::Down,
        last_move: None,
        path_history: vec![],
    });

    let mut final_paths = Vec::new();

    let start = Instant::now();
    let mut next = 0;
    loop {
        let current_move = { deque.pop_front() };
        if current_move.is_none() {
            break;
        }
        let mut current_move = current_move.unwrap();

        // check if we're at the end
        if current_move.x == end.0 && current_move.y == end.1 {
            //println!("We've found the end!");
            final_paths.push(current_move);
            continue;
        }

        let duration = start.elapsed();
        if duration.as_secs() > next {
            let mut max = 0;
            let mut max_index = 0;
            if final_paths.len() > 100 {
                // take the largest of the final paths
                for (i, p) in final_paths.iter().enumerate() {
                    if p.path_history.len() > max {
                        max = p.path_history.len();
                        max_index = i;
                    }
                }
                // remove the rest
                final_paths = vec![final_paths[max_index].clone()];
            }
            println!(
                "We've been running for {} seconds and have final {} paths, with {} more to solve...largest is {}",
                duration.as_secs(),
                final_paths.len(),
                deque.len(),
                max
            );
            next = duration.as_secs() + 5;
        }

        let x = current_move.x;
        let y = current_move.y;

        current_move.path_history.push((x, y));
        let mut v = Vec::new();
        v.push(can_move(
            &mut deque,
            &current_move,
            tiles,
            can_climb_slopes,
            Direction::Up,
            (x, y - 1),
        ));
        v.push(can_move(
            &mut deque,
            &current_move,
            tiles,
            can_climb_slopes,
            Direction::Down,
            (x, y + 1),
        ));
        v.push(can_move(
            &mut deque,
            &current_move,
            tiles,
            can_climb_slopes,
            Direction::Left,
            (x - 1, y),
        ));
        v.push(can_move(
            &mut deque,
            &current_move,
            tiles,
            can_climb_slopes,
            Direction::Right,
            (x + 1, y),
        ));
        if v.len() == 0 {
            continue;
        } else if v.len() == 1 {
            let (direction, x, y) = v[0].unwrap();
            deque.push_back(PosInstance {
                x,
                y,
                direction,
                last_move: Some(current_move.direction),
                path_history: current_move.path_history,
            });
            continue;
        } else {
            for i in v {
                if let Some((direction, x, y)) = i {
                    deque.push_back(PosInstance {
                        x,
                        y,
                        direction,
                        last_move: Some(current_move.direction.clone()),
                        path_history: current_move.path_history.clone(),
                    });
                }
            }
        }
    }
    //println!("{:?}", deque);
    // find the longest path from what's left
    println!(
        "{:?}",
        final_paths
            .iter()
            .map(|p| p.path_history.len())
            .collect::<Vec<usize>>()
    );
    final_paths
        .iter()
        .map(|p| p.path_history.len())
        .max()
        .unwrap()
}

pub fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Path,
                    'v' => Tile::Slope(Direction::Down),
                    '^' => Tile::Slope(Direction::Up),
                    '<' => Tile::Slope(Direction::Left),
                    '>' => Tile::Slope(Direction::Right),
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part_one_example() {
        let tiles = parse(input::EXAMPLE_INPUT);
        assert_eq!(find_path(&tiles, false), 94);
    }

    #[test]
    fn can_solve_part_one() {
        let tiles = parse(input::INPUT);
        assert_eq!(find_path(&tiles, false), 2386);
    }

    #[test]
    fn can_solve_part_two_example() {
        let tiles = parse(input::EXAMPLE_INPUT);
        assert_eq!(find_path(&tiles, true), 154);
    }

    #[test]
    fn can_solve_part_two() {
        let tiles = parse(input::INPUT);
        assert_eq!(find_path(&tiles, true), 6246);
    }
}
