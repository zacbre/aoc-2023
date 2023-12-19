use std::collections::BTreeMap;

#[derive(Default, Debug)]
struct Line {
    pub symbols: BTreeMap<usize, String>,
    pub parts: BTreeMap<usize, usize>,
}

#[derive(Default, Debug)]
struct Lines {
    pub lines: Vec<Line>
}

fn parse(input: &str) -> Lines{
    // split by lines, and check each previous/current/next line for being next to a symbol (anything but a .?)
    let mut total_lines = Lines::default();
    let mut lines = input.lines();
    for line in lines {
        let mut current_line = Line::default();
        let mut part = String::default();
        for (index, symbol) in line.chars().enumerate() {
            if symbol.is_digit(10) {
                part.push(symbol);
            } else {
                if part.len() > 0 {
                    current_line.parts.insert(index - part.len(), part.parse::<usize>().unwrap());
                    part = String::default();
                }
                if symbol != '.' {
                    current_line.symbols.insert(index, symbol.to_string());
                }
            }
        }
        if part.len() > 0 {
            current_line.parts.insert((line.len() - 1) - part.len(), part.parse::<usize>().unwrap());
        }
        total_lines.lines.push(current_line);
    }

    total_lines
}

fn part_one(lines: &Lines) {
    // check each line for a symbol, and then check the previous and next line for a symbol
    let mut total = 0;
    for (index, line) in lines.lines.iter().enumerate() {
        for (p_index, part) in &line.parts {
            let mut lines_to_process: Vec<&Line> = Vec::new();
            // check previous line
            if index > 0 {
                let previous_line = &lines.lines[index - 1];
                lines_to_process.push(previous_line);
            }

            lines_to_process.push(line);

            if index < lines.lines.len() - 1 {
                let next_line = &lines.lines[index + 1];
                lines_to_process.push(next_line);
            }

            let part_len = part.to_string().len();
            let mut is_match = false;
            for c_line in lines_to_process {
                let (result, _, _) = check_for_symbol(index, c_line, *part, *p_index, part_len);
                if result {
                    total += *part;
                    is_match = true;
                    break;
                }
            }
            if !is_match {
                println!("[{}] No Match: {}: {}", index + 4, p_index, part);
            }
        }
    }
    println!("Part One: {}", total);
}

fn part_two(lines: &Lines) {
    // find each * that is adjacent to another number and multiply them together, then add the result.
    let mut total = 0;
    let mut gear_symbols: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for (index, line) in lines.lines.iter().enumerate() {
        for (p_index, part) in &line.parts {
            let mut lines_to_process: BTreeMap<usize, &Line> = BTreeMap::new();
            // check previous line
            if index > 0 {
                let previous_line = &lines.lines[index - 1];
                lines_to_process.insert(index-1, previous_line);
            }

            lines_to_process.insert(index, line);

            if index < lines.lines.len() - 1 {
                let next_line = &lines.lines[index + 1];
                lines_to_process.insert(index + 1, next_line);
            }

            let part_len = part.to_string().len();
            for (n_index, c_line) in lines_to_process {
                let (result, symbol, r_index) = check_for_symbol(index, c_line, *part, *p_index, part_len);
                if result {
                    if symbol.as_str() == "*" {
                        if gear_symbols.contains_key(&(n_index, r_index)) {
                            let value = gear_symbols.get(&(n_index, r_index)).unwrap();
                            println!("[{}] {} * {} = {}", index + 4, value, *part, value * *part);
                            total += value * *part;
                        } else {
                            gear_symbols.insert((n_index, r_index), *part);
                        }
                    }
                }
            }
        }
    }
    //println!("{:?}", gear_symbols);
    println!("Part Two: {}", total);
}

fn check_for_symbol(line_num: usize, line: &Line, part: usize, p_index: usize, part_len: usize) -> (bool, String, usize) {
    let mut sub = 1;
    if p_index == 0 {
        sub = 0;
    }

    for i in p_index-sub..p_index+part_len+1 {
        if line.symbols.contains_key(&i) {
            //println!("[{}] {}: {} = {} -> {}", line_num + 4, line.symbols.get(&i).unwrap(), i, p_index, part);
            return (true, line.symbols.get(&i).unwrap().to_string(), i);
        }
    }
    (false, String::default(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_test_file;

    #[test]
    fn test_part1() {
        let input = read_test_file("day3/input.txt");
        let lines = parse(input.as_str());
        let mut index = 0;
        for line in &lines.lines {
            println!("{} - {:?}", index + 4, line);
            index += 1;
        }
        part_one(&lines);
    }

    #[test]
    fn test_part2() {
        let input = read_test_file("day3/input.txt");
        let lines = parse(input.as_str());
        let mut index = 0;
        for line in &lines.lines {
            println!("{} - {:?}", index + 4, line);
            index += 1;
        }
        part_two(&lines);
    }
}