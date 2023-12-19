use std::collections::HashSet;
use crate::day10::PipeType::Ground;

mod input;

#[derive(Debug, Eq, PartialEq)]
enum PipeType {
    Vertical(char),
    Horizontal(char),
    NorthEast(char),
    NorthWest(char),
    SouthWest(char),
    SouthEast(char),
    Ground(char),
    StartingPosition(char)
}

#[derive(Debug)]
struct PipeGrid {
    pipes: Vec<Vec<PipeType>>,
}

fn parse_input(input: &str) -> PipeGrid {
    PipeGrid {
        pipes: input
            .lines()
            .map(|line| line.chars().map(match_to_pipe_type).collect())
            .collect(),
    }
}

fn match_to_pipe_type(c: char) -> PipeType {
    match c {
        '|' => PipeType::Vertical(c),
        '-' => PipeType::Horizontal(c),
        'L' => PipeType::NorthEast(c),
        'J' => PipeType::NorthWest(c),
        '7' => PipeType::SouthWest(c),
        'F' => PipeType::SouthEast(c),
        'S' => PipeType::StartingPosition(c),
        _ => PipeType::Ground(c)
    }
}

fn check_next_pipe_direction(x: usize, y: usize, prev_x: usize, prev_y: usize, pipe_grid: &PipeGrid) -> (usize, usize) {
    // get the pipe at x,y
    let pipe = &pipe_grid.pipes[y][x];
    match pipe {
        PipeType::Vertical(_c) => {
            if prev_y < y {
                (x, y + 1)
            } else {
                (x, y - 1)
            }
        },
        PipeType::Horizontal(_c) => {
            if prev_x < x {
                (x + 1, y)
            } else {
                (x - 1, y)
            }
        },
        PipeType::NorthEast(_c) => {
            if x + 1 == prev_x {
                (x, y - 1)
            } else {
                (x + 1, y)
            }
        },
        PipeType::NorthWest(_c) => {
            if x - 1 == prev_x {
                (x, y - 1)
            } else {
                (x - 1, y)
            }
        },
        PipeType::SouthWest(_c) => {
            if x - 1 == prev_x {
                (x, y + 1)
            } else {
                (x - 1, y)
            }
        },
        PipeType::SouthEast(_c) => {
            if x + 1 == prev_x {
                (x, y + 1)
            } else {
                (x + 1, y)
            }
        },
        PipeType::StartingPosition(_c) => {
            println!("Starting position!");
            (x + 1, y)
        },
        _ => panic!("This shouldn't be here!")
    }
}

fn main() {
    let mut pipe_grid = parse_input(input::input);
    let hashset = part_one(&pipe_grid);
    pipe_grid = clean_map(pipe_grid, &hashset);
    part_two(&pipe_grid);
}

fn clean_map(mut pipe_grid: PipeGrid, hashset: &HashSet<(usize, usize)>) -> PipeGrid {
    let pipes_cleaned = pipe_grid.pipes.into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col_idx, tile)| match tile {
                    pipe if hashset.contains(&(col_idx, row_idx)) => pipe,
                    _ => Ground('.'),
                })
                .collect()
        })
        .collect::<Vec<Vec<PipeType>>>();
    pipe_grid.pipes = pipes_cleaned;
    pipe_grid
}

fn part_two(pipe_grid: &PipeGrid) {
    let mut inside = false;
    let count = &pipe_grid.pipes.iter()
        .flatten()
        .filter(|tile| match tile {
            Ground(c) => inside,
            PipeType::Vertical(c) | PipeType::NorthWest(c) | PipeType::NorthEast(c) => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count();
    println!("{:?}", count);
}

fn part_one(pipe_grid: &PipeGrid) -> HashSet<(usize, usize)> {
    //println!("Pipe Grid: {:?}", pipe_grid);
    // find the x,y coordinate of the starting position
    let mut starting_position = (0, 0);
    for (x, row) in pipe_grid.pipes.iter().enumerate() {
        for (y, pipe) in row.iter().enumerate() {
            match pipe {
                PipeType::StartingPosition(c) => {
                    starting_position = (y, x);
                },
                _ => ()
            }
        }
    }
    //println!("Pipe Grid: {:?}", pipe_grid);
    println!("Starting Position: {:?}", starting_position);
    let mut current_position = starting_position;
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut total_pipes = 0;
    let mut hashset: HashSet<(usize,usize)> = HashSet::new();
    loop {
        hashset.insert((current_position.0, current_position.1));
        let next_position = check_next_pipe_direction(current_position.0, current_position.1, prev_x, prev_y, &pipe_grid);
        if pipe_grid.pipes[next_position.1][next_position.0] == PipeType::StartingPosition('S') {
            println!("Reached the end of the pipe!");
            println!("Total Pipes: {:?}", total_pipes);
            break;
        }
        prev_x = current_position.0;
        prev_y = current_position.1;

        current_position = next_position;
        total_pipes += 1;
        println!("Current Position: {:?}, Previous Position: {:?}, Current Pipe: {:?}, Previous Pipe: {:?}", current_position, (prev_x, prev_y), pipe_grid.pipes[current_position.1][current_position.0], pipe_grid.pipes[prev_y][prev_x]);
    }

    hashset
}
