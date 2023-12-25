mod input;
//
// use std::collections::{HashMap, VecDeque};
// use crate::Tile::Rock;
//
// #[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
// struct Point(isize, isize);
// impl std::ops::Add for Point {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Point(self.0 + rhs.0, self.1 + rhs.1)
//     }
// }
//
// #[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }
//
// impl Direction {
//     pub fn inc(&self) -> (isize, isize) {
//         match self {
//             Direction::Up => (0, -1),
//             Direction::Down => (0, 1),
//             Direction::Left => (-1, 0),
//             Direction::Right => (1, 0),
//         }
//     }
// }
//
// #[derive(Debug, PartialEq)]
// enum Tile {
//     Rock,
//     Plot,
//     Start
// }
//
// #[derive(Debug)]
// struct Map {
//     grid: Vec<Tile>,
//     start: Point,
//     dimensions: (isize, isize),
// }
//
// impl Map {
//     fn is_rock(&self, &Point(x, y): &Point) -> bool {
//         self.grid[y as usize * 132 + x as usize] == Rock
//     }
// }
//
// fn parse_input(input: &str) -> Map {
//     let grid = input.lines().map(|line| {
//         line.chars().map(|c| {
//             match c {
//                 '.' => Tile::Plot,
//                 '#' => Rock,
//                 'S' => Tile::Start,
//                 _ => panic!("Invalid input")
//             }
//         }).collect::<Vec<Tile>>()
//     }).flatten().collect();
//     Map {
//         grid,
//         start: Point(65, 65),
//         dimensions: (131, 131),
//     }
// }
//
// fn solve(map: &Map) {
//     let mut frontier = VecDeque::<(usize, Point)>::new();
//     let mut visited = HashMap::new();
//     frontier.push_back((0, map.start));
//
//     while let Some((dist, coord)) = frontier.pop_front() {
//         if visited.contains_key(&coord) {
//             continue;
//         }
//
//         visited.insert(coord, dist);
//
//         for d in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
//             let inc = d.inc();
//             let next = coord + Point(inc.0, inc.1);
//
//             if next.0 >= 0 && next.0 < map.dimensions.0 && next.1 >= 0 && next.1 < map.dimensions.1 {
//                 if !visited.contains_key(&next) && !map.is_rock(&next) {
//                     frontier.push_back((dist + 1, next));
//                 }
//             }
//         }
//     }
//
//     let p1 = visited
//         .values()
//         .filter(|v| **v <= 64 && **v % 2 == 0)
//         .count();
//
//     println!("Part 1: {}", p1);
//
//     let even_corners = visited
//         .values()
//         .filter(|v| **v % 2 == 0 && **v > 65)
//         .count();
//     let odd_corners = visited
//         .values()
//         .filter(|v| **v % 2 == 1 && **v > 65)
//         .count();
//
//     // This is 202300 but im writing it out here to show the process
//     let n = ((26501365 - (map.dimensions.0 / 2)) / map.dimensions.0) as usize;
//     assert_eq!(n, 202300);
//
//     let even = n * n;
//     let odd = (n + 1) * (n + 1);
//
//     let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count()
//         + even * visited.values().filter(|v| **v % 2 == 0).count()
//         - ((n + 1) * odd_corners)
//         + (n * even_corners);
//
//     println!("Part 2: {}", p2);
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::{input, parse_input, solve};
//
//     #[test]
//     fn test_day21() {
//         let map = parse_input(input::INPUT);
//         solve(&map);
//     }
// }

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn add_coords(c1: (isize, isize), c2: (isize, isize)) -> (isize, isize) {
    (c1.0 + c2.0, c1.1 + c2.1)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coord(pub isize, pub isize);

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

impl Dir {
    pub fn increment(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    pub fn cincrement(&self) -> Coord {
        match self {
            Dir::Up => Coord(0, -1),
            Dir::Down => Coord(0, 1),
            Dir::Left => Coord(-1, 0),
            Dir::Right => Coord(1, 0),
        }
    }

    pub fn from_char(c: char) -> Dir {
        match c {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!(),
        }
    }
}

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Env<'a> {
    map: &'a [u8],
    start: Coord,
    dim: (isize, isize),
}

impl Env<'_> {
    fn is_rock(&self, &Coord(x, y): &Coord) -> bool {
        self.map[y as usize * 132 + x as usize] == b'#'
    }
}

fn parse_input(input: &str) -> Env {
    Env {
        map: input.as_bytes(),
        start: Coord(65, 65),
        dim: (131, 131),
    }
}

fn solve(env: &Env) {
    let mut frontier = VecDeque::<(usize, Coord)>::new();
    let mut visited = HashMap::new();
    frontier.push_back((0, env.start));

    while let Some((dist, coord)) = frontier.pop_front() {
        if visited.contains_key(&coord) {
            continue;
        }

        visited.insert(coord, dist);

        for d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let inc = d.increment();
            let next = coord + Coord(inc.0, inc.1);

            if next.0 >= 0 && next.0 < env.dim.0 && next.1 >= 0 && next.1 < env.dim.1 {
                if !visited.contains_key(&next) && !env.is_rock(&next) {
                    frontier.push_back((dist + 1, next));
                }
            }
        }
    }

    let p1 = visited
        .values()
        .filter(|v| **v <= 64 && **v % 2 == 0)
        .count();

    println!("Part 1: {}", p1);

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    // This is 202300 but im writing it out here to show the process
    let n = ((26501365 - (env.dim.0 / 2)) / env.dim.0) as usize;
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count()
        + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);

    println!("Part 2: {}", p2);
}

pub fn day21(input: String) {
    let env = parse_input(&input);
    solve(&env);
}

#[cfg(test)]
mod tests {
    use crate::{day21, input};

    #[test]
    fn test_day21() {
        day21(input::INPUT.to_string());
    }
}