use pathfinding::prelude::dijkstra;
use crate::Direction::{Down, Left, Right, Up};

mod input;
mod image;

fn main() {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub steps: i32,
    pub direction: Direction,
}

impl Pos {
    fn new() -> Self {
        Pos {
            x: 0,
            y: 0,
            steps: 0,
            direction: Right,
        }
    }
    fn successors(&self, grid: &Vec<Vec<usize>>, min_steps: i32, max_steps: i32) -> Vec<(Pos, usize)> {
        // figure out which directions to move in
        let directions = vec![Up, Down, Left, Right];
        let mut output = Vec::new();
        for direction in directions {
            if direction == self.direction && self.steps == max_steps {
                continue;
            }
            if direction != self.direction && self.steps < min_steps {
                continue;
            }
            // prevent from going backwards
            if direction == Up && self.direction == Down ||
                direction == Down && self.direction == Up ||
                direction == Left && self.direction == Right ||
                direction == Right && self.direction == Left {
                continue;
            }

            // if we try to move up, but we're already at the top, skip, or rest of possibilities
            if direction == Up && self.y == 0 ||
                direction == Down && self.y == grid.len() as i32 - 1 ||
                direction == Left && self.x == 0 ||
                direction == Right && self.x == grid[0].len() as i32 - 1 {
                continue;
            }

            let next_steps = if direction == self.direction { self.steps + 1 } else { 1 };
            let (x, y) = match direction {
                Up => (self.x, self.y - 1),
                Down => (self.x, self.y + 1),
                Left => (self.x - 1, self.y),
                Right => (self.x + 1, self.y),
            };
            let cost = grid[y as usize][x as usize];
            output.push((Pos { x, y, steps: next_steps, direction }, cost));
        }

        output
    }

    fn goal(&self, grid: &Vec<Vec<usize>>) -> bool {
        self.x == grid[0].len() as i32 - 1 && self.y == grid.len() as i32 - 1
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}

fn find_path(start: &Pos, grid: &Vec<Vec<usize>>, min_steps: i32, max_steps: i32) -> (Vec<Pos>, usize) {
    let res = dijkstra(start, |p| p.successors(grid, min_steps, max_steps), |p| p.goal(&grid));
    res.expect("No path found!")
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::image::draw_to_image;
    use super::*;

    #[test]
    fn can_parse_example_input() {
    }

    #[test]
    fn can_find_example_path() {
        let input = parse(input::EXAMPLE_INPUT);
        let start = Pos::new();
        let answer = find_path(&start, &input, 0, 3);
        draw_to_image(&input, Path::new("example.png"), Some(&answer.0));
        assert_eq!(answer.1, 102);
    }

    #[test]
    fn can_find_path() {
        let input = parse(input::INPUT);
        let start = Pos::new();
        let answer = find_path(&start, &input, 0, 3);
        //draw_to_image(&input, Path::new("example.png"), Some(&answer.0));
        assert_eq!(answer.1, 1260);
    }

    #[test]
    fn can_find_example_path_pt2() {
        let input = parse(input::EXAMPLE_INPUT);
        let start = Pos::new();
        let answer = find_path(&start, &input, 4, 10);
        draw_to_image(&input, Path::new("example.png"), Some(&answer.0));
        assert_eq!(answer.1, 94);
    }

    #[test]
    fn can_find_path_pt2() {
        let input = parse(input::INPUT);

        let mut start = Pos::new();
        start.direction = Down;

        let possiblities = vec![Pos::new(), start];
        let answer = possiblities.iter().map(|p| find_path(p, &input, 4, 10).1).min();
        //draw_to_image(&input, Path::new("example.png"), Some(&answer.0));
        assert_eq!(answer.unwrap(), 1416);
    }
}