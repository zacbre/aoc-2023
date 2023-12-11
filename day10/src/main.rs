mod input;

#[derive(Debug)]
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
    let pipe = &pipe_grid.pipes[x][y];
    match pipe {
        PipeType::Vertical(c) => {
            if prev_y < y {
                (x, y + 1)
            } else {
                (x, y - 1)
            }
        },
        PipeType::Horizontal(c) => {
            if prev_x < x {
                (x + 1, y)
            } else {
                (x - 1, y)
            }
        },
        PipeType::NorthEast(c) => {
            if x + 1 == prev_x {
                (x, y + 1)
            } else {
                (x + 1, y)
            }
        },
        PipeType::NorthWest(c) => {
            if x - 1 == prev_x {
                (x, y + 1)
            } else {
                (x - 1, y)
            }
        },
        PipeType::SouthWest(c) => {
            if x + 1 == prev_x {
                (x, y - 1)
            } else {
                (x + 1, y)
            }
        },
        PipeType::SouthEast(c) => {
            if x - 1 == prev_x {
                (x, y - 1)
            } else {
                (x - 1, y)
            }
        },
        PipeType::StartingPosition(c) => (x + 1, y),
        _ => panic!("This shouldn't be here!")
    }
}

fn main() {
    let pipe_grid = parse_input(input::example_input);
    //println!("Pipe Grid: {:?}", pipe_grid);
    // find the x,y coordinate of the starting position
    let mut starting_position = (0, 0);
    for (x, row) in pipe_grid.pipes.iter().enumerate() {
        for (y, pipe) in row.iter().enumerate() {
            match pipe {
                PipeType::StartingPosition(c) => {
                    starting_position = (x, y);
                },
                _ => ()
            }
        }
    }
    println!("Pipe Grid: {:?}", pipe_grid);
    println!("Starting Position: {:?}", starting_position);
    let mut current_position = starting_position;
    let mut prev_x = 0;
    let mut prev_y = 0;
    loop {
        let next_position = check_next_pipe_direction(current_position.0, current_position.1, prev_x, prev_y, &pipe_grid);
        prev_x = current_position.0;
        prev_y = current_position.1;

        current_position = next_position;
        println!("Current Position: {:?}, Previous Position: {:?}, Current Pipe: {:?}, Previous Pipe: {:?}", current_position, (prev_x, prev_y), pipe_grid.pipes[current_position.0][current_position.1], pipe_grid.pipes[prev_x][prev_y]);
    }
}
