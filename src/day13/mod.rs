mod input;

#[derive(Debug)]
struct PuzzleInput<'a> {
    map: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32
}

fn parse_input(input: &str) -> Vec<PuzzleInput> {
    let mut puzzle_inputs = Vec::new();
    let mut puzzle_input = PuzzleInput { map: Vec::new() };
    for line in input.lines() {
        if line.len() == 0 {
            puzzle_inputs.push(puzzle_input);
            puzzle_input = PuzzleInput { map: Vec::new() };
            continue;
        }
        puzzle_input.map.push(line);
    }
    puzzle_inputs.push(puzzle_input);
    puzzle_inputs
}

fn find_possible_reflections(puzzle_input: &PuzzleInput, use_smudge: bool) -> Vec<Point> {
    // look forward to see if there's a match
    let mut possible_reflections = Vec::new();

    // row
    for i in 0..puzzle_input.map.len() - 1 {
        if use_smudge {
            let matched = puzzle_input.map[i].chars().zip(puzzle_input.map[i+1].chars()).filter(|&(a, b)| a == b).count();
            if matched == puzzle_input.map[i].len() - 1 {
                println!("Found possible smudged reflection: {:?} - {:?}", puzzle_input.map[i], puzzle_input.map[i+1]);
                possible_reflections.push(Point { x: i as i32, y: -1 });
                continue;
            }
        }
        if puzzle_input.map[i] == puzzle_input.map[i+1] {
            possible_reflections.push(Point { x: i as i32, y: -1 });
        }
    }

    // col
    for i in 0..puzzle_input.map[0].len() - 1 {
        let col = get_col(puzzle_input, i as i32);
        let next_col = get_col(puzzle_input, (i + 1) as i32);
        if use_smudge {
            let matched = col.iter().zip(next_col.iter()).filter(|&(a, b)| a == b).count();
            if matched == puzzle_input.map.len() - 1 {
                println!("Found possible smudged reflection: {:?} - {:?}", col, next_col);
                possible_reflections.push(Point { x: -1, y: i as i32 });
                continue;
            }
        }
        if col == next_col {
            possible_reflections.push(Point { x: -1, y: i as i32 });
        }
    }

    possible_reflections
}

fn get_col(puzzle_input: &PuzzleInput, col: i32) -> Vec<char> {
    puzzle_input.map.iter()
        .map(|row| row.chars().nth(col as usize).unwrap())
        .collect()
}

fn find_mirror<'a>(puzzle_input: &PuzzleInput, possible_reflections: &'a Vec<Point>, use_smudge: bool) -> Option<&'a Point> {
    // we need to check forward and back for each possible reflection
    for (index, point) in possible_reflections.iter().enumerate() {
        println!("Possible Matches: {:?}", &possible_reflections[index..]);
        let x = point.x;
        let y = point.y;

        let mut smudged = false;

        let mut backward_match: i32 = 0;
        let mut forward_match: i32 = 0;
        loop {
            match (x, y) {
                (-1, _) => {
                    if backward_match == 0 && forward_match == 0 {
                        backward_match = y;
                        forward_match = y + 1;
                    }
                    if backward_match < 0 || forward_match > (puzzle_input.map[0].len() - 1) as i32 {
                        if use_smudge && !smudged {
                            break;
                        }
                        return Some(point);
                    }
                    let current_col = get_col(puzzle_input, backward_match);
                    let next_col = get_col(puzzle_input, forward_match);
                    // get how many of each match
                    let matching_in_count = current_col.iter().zip(&next_col).filter(|&(a, b)| a == b).count();
                    if matching_in_count == current_col.len() || use_smudge && matching_in_count == current_col.len() - 1 {
                        if use_smudge && matching_in_count == current_col.len() - 1 {
                            if smudged {
                                println!("Smudged twice, not picking this one: {:?}", point);
                                break;
                            }
                            smudged = true;
                            println!("Found a smudged match: {:?} - {:?}", current_col, next_col);
                        }
                        backward_match = backward_match - 1;
                        forward_match = forward_match + 1;
                    } else {
                        println!("col[{}] did not match col[{}]: {:?} != {:?}", backward_match, forward_match, current_col, next_col);
                        break;
                    }
                },
                (_, -1) => {
                    if backward_match == 0 && forward_match == 0 {
                        backward_match = x;
                        forward_match = x + 1;
                    }
                    if backward_match < 0 || forward_match > (puzzle_input.map.len() - 1) as i32 {
                        if use_smudge && !smudged {
                            break;
                        }
                        return Some(point);
                    }
                    let current_row = puzzle_input.map[backward_match as usize];
                    let next_row = puzzle_input.map[forward_match as usize];
                    let matching_in_count = current_row.chars().zip(next_row.chars()).filter(|&(a, b)| a == b).count();
                    if matching_in_count == current_row.len() || use_smudge && matching_in_count == current_row.len() - 1 {
                        if use_smudge && matching_in_count == current_row.len() - 1 {
                            if smudged {
                                println!("Smudged twice, not picking this one: {:?}", point);
                                break;
                            }
                            println!("Found a smudged match: {} - {}", current_row, next_row);
                            smudged = true;
                        }
                        backward_match = backward_match - 1;
                        forward_match = forward_match + 1;
                    } else {
                        break;
                    }
                },
                _ => {
                    println!("Invalid point: {:?}", point);
                    break;
                }
            }
        }
    }
    return None;
}

fn calculate_all_mirrors(puzzle_inputs: &Vec<PuzzleInput>, smudge: bool) -> i32 {
    puzzle_inputs.iter().enumerate().map(|(index, puzzle_input)| {
        let possible_matches = find_possible_reflections(puzzle_input, smudge);
        let point = find_mirror(puzzle_input, &possible_matches, smudge);
        println!("[{}] - Picked: {:?}", index, point);
        assert_eq!(point.is_some(), true);
        match point {
            Some(point) => {
                if point.x == -1 {
                    point.y + 1
                } else {
                    (point.x + 1) * 100
                }
            }
            None => 0
        }
    }).sum()
}

fn part_one(puzzle_inputs: &Vec<PuzzleInput>) -> i32 {
    calculate_all_mirrors(puzzle_inputs, false)
}

fn part_two(puzzle_inputs: &Vec<PuzzleInput>) -> i32 {
    calculate_all_mirrors(puzzle_inputs, true)
}

fn main() {
}

#[cfg(test)]
mod test {
    use super::{find_possible_reflections, input};

    #[test]
    fn can_parse_input_for_example_1() {
        let puzzle_inputs = super::parse_input(input::example_input_1);
        assert_eq!(puzzle_inputs.len(), 1);
        assert_eq!(puzzle_inputs[0].map.len(), 7);
    }

    #[test]
    fn can_parse_input_multi_puzzle() {
        let puzzle_inputs = super::parse_input(input::input);
        assert_eq!(puzzle_inputs.len(), 100);
    }

    #[test]
    fn can_find_mirror_for_example_1() {
        let puzzle_inputs = super::parse_input(input::example_input_1);
        let possible_matches = find_possible_reflections(&puzzle_inputs[0], false);
        let point = super::find_mirror(&puzzle_inputs[0], &possible_matches, false);
        assert_eq!(point, Some(super::Point { x: -1, y: 4 }).as_ref());
    }

    #[test]
    fn can_find_mirror_for_example_2() {
        let puzzle_inputs = super::parse_input(input::example_input_2);
        let possible_matches = find_possible_reflections(&puzzle_inputs[0], false);
        let point = super::find_mirror(&puzzle_inputs[0], &possible_matches, false);
        assert_eq!(point, Some(super::Point { x: 3, y: -1 }).as_ref());
    }

    #[test]
    fn can_find_mirror_for_combined_example_input() {
        let puzzle_inputs = super::parse_input(input::combined_example_input);
        let output: i32 = super::part_one(&puzzle_inputs);
        assert_eq!(output, 405);
    }

    #[test]
    fn can_find_mirror_for_combined_example_input_smudged() {
        let puzzle_inputs = super::parse_input(input::combined_example_input);
        let output: i32 = super::part_two(&puzzle_inputs);
        assert_eq!(output, 400);
    }

    #[test]
    fn can_find_mirror_for_input() {
        let puzzle_inputs = super::parse_input(input::input);
        let output: i32 = super::part_one(&puzzle_inputs);
        println!("Part One: {}", output);
        assert_eq!(output, 42974);
    }

    #[test]
    fn can_find_mirror_for_part_two() {
        let puzzle_inputs = super::parse_input(input::input);
        let output: i32 = super::part_two(&puzzle_inputs);
        println!("Part Two: {}", output);
        assert_eq!(output, 27587);
    }
}