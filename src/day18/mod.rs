use std::collections::VecDeque;

mod input;

fn main() {}

#[derive(Debug, PartialEq)]
enum Direction {
    Up, Down, Left, Right
}
impl Direction {
    fn match_direction(a: &str) -> Direction {
        match a {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction")
        }
    }
    fn match_direction_hex(a: i32) -> Direction {
        match a {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction")
        }
    }
}

#[derive(Debug)]
struct Instruction {
    pub direction: Direction,
    pub number: i32
}

fn parse_hex(hex: String) -> (Direction, i32) {
    (Direction::match_direction_hex(hex[6..].parse().unwrap()), i32::from_str_radix(&hex[1..6], 16).unwrap())
}

fn parse(input: &str, should_parse_hex: bool) -> Vec<Instruction> {
    input.lines().map(|line| {
        let mut iter = line.split(' ');
        let direction = Direction::match_direction(iter.next().unwrap());
        let number = iter.next().unwrap().parse().unwrap();
        let hex = iter.next().unwrap().replace("(", "").replace(")", "");
        let (d, n) = if should_parse_hex {
            parse_hex(hex)
        } else {
            (direction, number)
        };
        Instruction {
            direction: d,
            number: n,
        }
    }).collect()
}

fn find_dimensions(instructions: &Vec<Instruction>) -> ((i32, i32), (i32, i32)) {
    let mut min_x: i32 = 0;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_y: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => y -= instruction.number,
            Direction::Down => y += instruction.number,
            Direction::Left => x -= instruction.number,
            Direction::Right => x += instruction.number,
        }
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    ((min_x, max_x), (min_y, max_y))
}

fn get_grid(x: (i32, i32), y: (i32, i32)) -> Vec<Vec<char>> {
    std::iter::repeat(
        Vec::from_iter(std::iter::repeat('.')
            .take((x.1+x.0.abs()+1) as usize)
            .collect::<Vec<char>>()
        ))
        .take((y.1+y.0.abs()+1) as usize)
        .collect::<Vec<Vec<char>>>()
}

fn solve_map(input: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>, dimensions: ((i32, i32), (i32, i32))) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                for _ in 0..instruction.number {
                    y -= 1;
                    let (tx, ty) = normalize_coord((x as isize, y as isize), dimensions);
                    input[ty][tx] = '#';
                }
            },
            Direction::Down => {
                for _ in 0..instruction.number {
                    y += 1;
                    let (tx, ty) = normalize_coord((x as isize, y as isize), dimensions);
                    input[ty][tx] = '#';
                }
            },
            Direction::Left => {
                for _ in 0..instruction.number {
                    x -= 1;
                    let (tx, ty) = normalize_coord((x as isize, y as isize), dimensions);
                    input[ty][tx] = '#';
                }
            },
            Direction::Right => {
                for _ in 0..instruction.number {
                    x += 1;
                    let (tx, ty) = normalize_coord((x as isize, y as isize), dimensions);
                    input[ty][tx] = '#';
                }
            },
        }
    }
}

fn draw_map(input: &Vec<Vec<char>>) {
    for line in input {
        println!("{}", line.iter().collect::<String>());
    }
}

fn normalize_coord(coord: (isize, isize), dimensions: ((i32, i32), (i32, i32))) -> (usize, usize) {
    ((coord.0 + (dimensions.0.0.abs()) as isize) as usize, (coord.1 + (dimensions.1.0.abs()) as isize) as usize)
}

fn find_fill_start_point(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (line_idx,line) in map.iter().enumerate() {
        let mut first = line.iter().position(|&c| c == '#').unwrap();
        let last = line.iter().rposition(|&c| c == '#').unwrap();
        while first < last {
            if line[first] == '.' {
                return Some((first, line_idx));
            }
            first += 1;
        }
    }
    None
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1)
];

fn flood_fill(map: &mut Vec<Vec<char>>, start: (usize,usize)) {
    let mut the_queue = VecDeque::new();
    the_queue.push_back(start);
    while !the_queue.is_empty() {
        let p = the_queue.pop_front().unwrap();
        if map[p.1][p.0] == '#' {
            continue;
        }

        //println!("---------------------------------");
        //println!("At ({},{})", p.0, p.1);
        // Process
        map[p.1][p.0] = '#';


        // Find neighbors
        for offset in OFFSETS {
            let neighbor = (p.0.checked_add_signed(offset.0), p.1.checked_add_signed(offset.1));
            if neighbor.0.is_none() || neighbor.1.is_none()
                || neighbor.0.unwrap() > map.first().unwrap().len()-1
                || neighbor.1.unwrap() > map.len()-1
                || map[neighbor.1.unwrap()][neighbor.0.unwrap()] == '#'
            {
                continue;
            }
            the_queue.push_back((neighbor.0.unwrap(), neighbor.1.unwrap()));
            //println!("\tAdding ({},{})", neighbor.0.unwrap(), neighbor.1.unwrap());
        }
    }
}

fn find_dimensions_vertices(instructions: &Vec<Instruction>, vertices: &mut Vec<(isize, isize)>) -> ((i32, i32), (i32, i32)) {
    let mut min_x: i32 = 0;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_y: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    vertices.push((x as isize,y as isize));
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => y -= instruction.number,
            Direction::Down => y += instruction.number,
            Direction::Left => x -= instruction.number,
            Direction::Right => x += instruction.number,
        }
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        vertices.push((x as isize, y as isize));
    }
    ((min_x, max_x), (min_y, max_y))
}

fn shoelace(vertices: &Vec<(isize,isize)>) -> usize {
    let mut area = vertices
        .windows(2)
        .map(|p| (p[0].0*p[1].1 - p[0].1*p[1].0)
            // Add edges
            + (p[0].0 - p[1].0).abs()
            + (p[0].1 - p[1].1).abs())
        .sum::<isize>();

    area += (vertices.last().unwrap().0*vertices.first().unwrap().1
        - vertices.last().unwrap().1*vertices.first().unwrap().0).abs();

    // Also add start pixel
    (area / 2 + 1) as usize
}

#[cfg(test)]
mod tests {
    use super::{find_fill_start_point, input, parse, shoelace};

    #[test]
    fn can_parse_input() {
        let parsed = parse(input::EXAMPLE_INPUT, false);
        assert_eq!(parsed.len(), 14);
        assert_eq!(parsed[0].direction, super::Direction::Right);
        assert_eq!(parsed[0].number, 6);
    }

    #[test]
    fn can_parse_input_hex() {
        let parsed = parse(input::EXAMPLE_INPUT, true);
        assert_eq!(parsed.len(), 14);
        assert_eq!(parsed[0].direction, super::Direction::Right);
        assert_eq!(parsed[0].number, 461937);
    }

    #[test]
    fn can_get_grid_for_example_input() {
        let parsed = parse(input::EXAMPLE_INPUT, false);
        let ((min_x, max_x), (min_y, max_y)) = super::find_dimensions(&parsed);
        let map = super::get_grid((min_x, max_x), (min_y, max_y));

        assert_eq!(map[0].len(), 7);
        assert_eq!(map.len(), 10);
    }

    #[test]
    fn can_get_grid_for_input() {
        let parsed = parse(input::INPUT, false);
        let ((min_x, max_x), (min_y, max_y)) = super::find_dimensions(&parsed);
        let map = super::get_grid((min_x, max_x), (min_y, max_y));

        assert_eq!(map[0].len(), 399);
        assert_eq!(map.len(), 285);
    }

    #[test]
    fn can_draw_outline_for_example_input() {
        let parsed = parse(input::EXAMPLE_INPUT, false);
        let ((min_x, max_x), (min_y, max_y)) = super::find_dimensions(&parsed);
        let mut map = super::get_grid((min_x, max_x), (min_y, max_y));
        super::solve_map(&mut map, &parsed, ((min_x, max_x), (min_y, max_y)));
        let map = map.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
        assert_eq!(map, input::EXAMPLE_INPUT_OUTLINE);
    }

    #[test]
    fn can_fill_outline_for_example_input() {
        let parsed = parse(input::EXAMPLE_INPUT, false);
        let ((min_x, max_x), (min_y, max_y)) = super::find_dimensions(&parsed);
        let mut map = super::get_grid((min_x, max_x), (min_y, max_y));
        super::solve_map(&mut map, &parsed, ((min_x, max_x), (min_y, max_y)));
        let cloned = map.clone();
        super::flood_fill(&mut map, find_fill_start_point(&cloned).unwrap());
        super::draw_map(&map);
        let map = map.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
        assert_eq!(map, input::EXAMPLE_INPUT_SOLUTION);
    }

    #[test]
    fn can_fill_outline_for_input() {
        let parsed = parse(input::INPUT, false);
        let ((min_x, max_x), (min_y, max_y)) = super::find_dimensions(&parsed);
        let mut map = super::get_grid((min_x, max_x), (min_y, max_y));
        super::solve_map(&mut map, &parsed, ((min_x, max_x), (min_y, max_y)));
        super::draw_map(&map);
        let cloned = map.clone();
        super::flood_fill(&mut map, find_fill_start_point(&cloned).unwrap());
        super::draw_map(&map);
        // get how many # there are
        let count: usize = map.iter().map(|line| line.iter().filter(|char| **char == '#').count()).sum();
        assert_eq!(count, 40745);
    }

    #[test]
    fn can_fill_outline_shoelace_for_input() {
        let parsed = parse(input::INPUT, false);
        let mut vertices: Vec<(isize, isize)> = Vec::new();
        let ((min_x, max_x), (min_y, max_y)) = super::find_dimensions_vertices(&parsed, &mut vertices);
        let area = shoelace(&vertices);
        assert_eq!(area, 40745);
    }

    #[test]
    fn can_fill_outline_for_input_pt2() {
        let parsed = parse(input::INPUT, true);
        let mut vertices: Vec<(isize, isize)> = Vec::new();
        let ((min_x, max_x), (min_y, max_y)) = super::find_dimensions_vertices(&parsed, &mut vertices);
        let area = shoelace(&vertices);
        assert_eq!(area, 90111113594927);
    }
}