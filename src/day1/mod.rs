use std::collections::{BTreeMap, HashMap};

fn part_one(input: &str) {
    let mut count = 0;

    // split the string into lines
    let lines = input.split("\n");
    for line in lines {
        let trimmed = line.trim();
        // get rid of all of the letters
        let digits = trimmed.chars().filter(|c| c.is_digit(10));
        // convert the digits into a string
        let digits_str: String = digits.collect();
        // only use the first and last digits
        let digits_str = digits_str.chars().take(1).chain(digits_str.chars().rev().take(1)).collect::<String>();
        // convert the string into a number
        let number = digits_str.parse::<u32>().unwrap();
        // add the number to the count
        count += number;
    }

    println!("Part One number: {}", count);
}

fn part_two(input: &str) {
    // see if the first or last in the string is a match
    let mut possible_matches: HashMap<&str, i32> = HashMap::new();
    possible_matches.insert("one", 1);
    possible_matches.insert("two", 2);
    possible_matches.insert("three", 3);
    possible_matches.insert("four", 4);
    possible_matches.insert("five", 5);
    possible_matches.insert("six", 6);
    possible_matches.insert("seven", 7);
    possible_matches.insert("eight", 8);
    possible_matches.insert("nine", 9);

    let mut count = 0;
    let lines = input.split("\n");
    for line in lines {
        let trimmed = line.trim();
        let mut hashmap: BTreeMap<usize, usize> = BTreeMap::new();
        // check trimmed for possible matches of all digits and strings
        for ind in 0..trimmed.len() - 1 {
            for (key, value) in &possible_matches {
                //println!("Looking for: {} in {} ({})", key, &trimmed[ind..], &trimmed);
                if trimmed[ind..].starts_with(key) {
                    //println!("Found {} at index: {}", key, ind);
                    if !hashmap.contains_key(&ind) {
                        hashmap.insert(ind, *value as usize);
                    }
                }
            }
        }
        // get each digit from the trimmed and index it
        for (index, digit) in trimmed.chars().enumerate() {
            // if the digit is a match, add it to the count
            if digit.is_digit(10) {
                //count += digit.to_digit(10).unwrap() as i32;
                hashmap.insert(index, digit.to_digit(10).unwrap() as usize);
            }
        }
        // get the values and add them together.
        let output = format!("{}{}", hashmap.first_key_value().unwrap().1, hashmap.last_key_value().unwrap().1);
        let number = output.parse::<u32>().unwrap();
        println!("input: {}, numbers: {:?}, output: {}, number: {}", trimmed, hashmap, output, number);
        count += number;
    }

    println!("Part Two number: {}", count);
}

#[cfg(test)]
mod tests {
    use crate::read_test_file;
    use super::{part_one, part_two};

    #[test]
    fn can_run_part_one() {
        let input = read_test_file("day1/input.txt");
        assert_eq!(part_one(input.as_str()), 55834);
    }

    #[test]
    fn can_run_part_two() {
        let input = read_test_file("day1/input.txt");
        assert_eq!(part_two(input.as_str()), 53221);

    }
}