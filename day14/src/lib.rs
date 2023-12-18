use std::fmt::{Display, Error, Formatter};

mod input;

#[derive(Debug, PartialEq, Clone)]
enum ItemType {
    RoundRock,
    SquareRock,
    Empty
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &ItemType::RoundRock => write!(f, "o"),
            &ItemType::SquareRock => write!(f, "#"),
            &ItemType::Empty => write!(f, ".")
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq, Clone)]
struct RockGrid {
    rows: Vec<Vec<ItemType>>
}

fn shift_rocks(direction: &Direction, grid: &RockGrid) -> RockGrid {
    // loop until there are no more shifted results.
    let mut grid_copy = grid.clone();
    loop {
        let mut shifted_count = 0;
        for (y, row) in grid.rows.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if grid_copy.rows[y][x] == ItemType::RoundRock {
                    match direction {
                        &Direction::North => {
                            if y > 0 && grid_copy.rows[y - 1][x] == ItemType::Empty {
                                grid_copy.rows[y - 1][x] = ItemType::RoundRock;
                                grid_copy.rows[y][x] = ItemType::Empty;
                                shifted_count += 1;
                            }
                        }
                        &Direction::West => {
                            if x > 0 && grid_copy.rows[y][x - 1] == ItemType::Empty {
                                grid_copy.rows[y][x - 1] = ItemType::RoundRock;
                                grid_copy.rows[y][x] = ItemType::Empty;
                                shifted_count += 1;
                            }
                        }
                        &Direction::South => {
                            if y < grid_copy.rows.len() - 1 && grid_copy.rows[y + 1][x] == ItemType::Empty {
                                grid_copy.rows[y + 1][x] = ItemType::RoundRock;
                                grid_copy.rows[y][x] = ItemType::Empty;
                                shifted_count += 1;
                            }
                        }
                        &Direction::East => {
                            if x < grid_copy.rows[0].len() - 1 && grid_copy.rows[y][x + 1] == ItemType::Empty {
                                grid_copy.rows[y][x + 1] = ItemType::RoundRock;
                                grid_copy.rows[y][x] = ItemType::Empty;
                                shifted_count += 1;
                            }
                        }
                        _ => ()
                    };
                }
            }
        }
        if shifted_count <= 0 {
            break;
        }
    }
    grid_copy
}

fn parse_rocks(input: &str) -> RockGrid {
    let mut rows = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut row = Vec::new();
        for c in line.trim().chars() {
            match c.to_ascii_lowercase() {
                '.' => row.push(ItemType::Empty),
                'o' => row.push(ItemType::RoundRock),
                '#' => row.push(ItemType::SquareRock),
                _ => panic!("Invalid character in input: {:?}", c)
            }
        }
        rows.push(row);
    }
    RockGrid { rows }
}

fn calculate_load(grid: &RockGrid, direction: &Direction) -> usize {
    let mut load = 0;
    let rock_load = grid.rows.len();
    match direction {
        &Direction::North => {
            for (y, row) in grid.rows.iter().enumerate() {
                for (x, item) in row.iter().enumerate() {
                    if grid.rows[y][x] == ItemType::RoundRock {
                        load += (rock_load - y)
                    }
                }
            }
            println!("Load: {}", load);
        }
        _ => ()
    }
    load
}

#[cfg(test)]
mod test {
    use crate::Direction;

    #[test]
    fn can_parse_input() {
        let input = r#"
            .o.
            o#o
            .o.
        "#;
        let grid = super::parse_rocks(input);
        assert_eq!(grid.rows.len(), 3);
        assert_eq!(grid.rows[0].len(), 3);
        assert_eq!(grid.rows[1].len(), 3);
        assert_eq!(grid.rows[2].len(), 3);
        assert_eq!(grid.rows[0], vec![super::ItemType::Empty, super::ItemType::RoundRock, super::ItemType::Empty]);
        assert_eq!(grid.rows[1], vec![super::ItemType::RoundRock, super::ItemType::SquareRock, super::ItemType::RoundRock]);
        assert_eq!(grid.rows[2], vec![super::ItemType::Empty, super::ItemType::RoundRock, super::ItemType::Empty]);
    }

    #[test]
    fn can_solve_example_input() {
        let grid = super::parse_rocks(super::input::EXAMPLE_INPUT);
        let solved_grid = super::parse_rocks(super::input::EXAMPLE_INPUT_SHIFTED_NORTH);

        let mut output_grid = super::shift_rocks(&super::Direction::North, &grid);

        for line in &output_grid.rows {
            for item in line {
                print!("{}", item);
            }
            println!("");
        }

        println!("Solved Grid:");
        for line in &solved_grid.rows {
            for item in line {
                print!("{}", item);
            }
            println!("");
        }

        assert_ne!(grid, output_grid);
        assert_eq!(output_grid, solved_grid);
    }

    #[test]
    fn can_calculate_load_for_example_input() {
        let grid = super::parse_rocks(super::input::EXAMPLE_INPUT);

        let mut output_grid = super::shift_rocks(&super::Direction::North, &grid);
        let load = super::calculate_load(&output_grid, &Direction::North);
        assert_eq!(load, 136);
    }

    #[test]
    fn can_calculate_load_for_part_one() {
        let grid = super::parse_rocks(super::input::INPUT);

        let mut output_grid = super::shift_rocks(&super::Direction::North, &grid);
        let load = super::calculate_load(&output_grid, &Direction::North);
        println!("Part One Load: {}", load);
        assert_eq!(load, 109939);
    }

    #[test]
    fn can_calculate_load_for_part_two_example() {
        let mut grid = super::parse_rocks(super::input::EXAMPLE_INPUT);
        // nwse
        let cycle_order = vec![super::Direction::North, super::Direction::West, super::Direction::South, super::Direction::East];
        for i in 0..1000 {
            for direction in &cycle_order {
                grid = super::shift_rocks(direction, &grid);
            }
        }
        let load = super::calculate_load(&grid, &Direction::North);
        assert_eq!(load, 64);
    }

    #[test]
    fn can_calculate_load_for_part_two() {
        let mut grid = super::parse_rocks(super::input::INPUT);
        // nwse
        let cycle_order = vec![super::Direction::North, super::Direction::West, super::Direction::South, super::Direction::East];
        for i in 0..1000 {
            for direction in &cycle_order {
                grid = super::shift_rocks(direction, &grid);
            }
        }
        let load = super::calculate_load(&grid, &Direction::North);
        println!("Part Two Load: {}", load);
        //assert_eq!(load, 64);
    }
}