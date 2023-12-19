use std::collections::{HashMap, VecDeque};

mod input;

enum Action {
    Add,
    Remove,
}

fn parse<'a>(input: &'a str) -> Vec<&'a str> {
    input.split(",")
        .map(|s| s)
        .collect()
}

fn hash(input: &str) -> u32 {
    /*Determine the ASCII code for the current character of the string.
    Increase the current value by the ASCII code you just determined.
    Set the current value to itself multiplied by 17.
    Set the current value to the remainder of dividing itself by 256.*/
    let mut result = 0;
    for c in input.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }

    result
}

fn process_lenses<'a>(input: &'a Vec<&'a str>) -> u32 {
    let mut boxes: HashMap<u32, VecDeque<(&str, u32)>> = HashMap::new();
    for item in input {
        let mut action = None;
        let mut split = if item.contains('-') {
            action = Some(Action::Remove);
            item.split('-')
        } else {
            action = Some(Action::Add);
            item.split('=')
        };

        let label = split.next().unwrap();
        let box_number = hash(label);

        let lens_list = boxes.entry(box_number).or_insert(VecDeque::new());

        match action {
            Some(Action::Add) => {
                let number = split.next().unwrap().parse::<u32>().unwrap();
                if let Some(pos) = lens_list.iter().position(|&x| x.0 == label) {
                    lens_list[pos].1 = number;
                } else {
                    lens_list.push_back((label, number));
                }
            },
            Some(Action::Remove) => {
                lens_list.iter().position(|&x| x.0 == label).map(|i| lens_list.remove(i));
            },
            None => panic!("No action found"),
        }
    }

    println!("{:?}", boxes);
    calculate_focusing_power(boxes)
}

fn calculate_focusing_power(boxes: HashMap<u32, VecDeque<(&str, u32)>>) -> u32 {
    let mut total_power = 0;
    boxes.iter().for_each(|(k, v)| {
        //total_power = (k + 1) +
        v.iter().enumerate().for_each(|(i, (_, power))| {
            println!("{} * {} * {} = {}", i + 1, k + 1, power, ((k + 1) * power * i as u32));
            total_power = total_power + ((k + 1) * power * (i + 1) as u32);
        });
        println!("Total Power after box {}: {}", k, total_power);
    });

    println!("Total power: {}", total_power);
    total_power
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_example_input() {
        let parsed = parse(input::EXAMPLE_INPUT);
        assert_eq!(parsed.len(), 11);
    }

    #[test]
    fn can_hash_example_input() {
        let parsed = parse(input::EXAMPLE_INPUT);
        let hashed = parsed.iter().map(|i| hash(i)).sum::<u32>();
        assert_eq!(hashed, 1320);
    }

    #[test]
    fn can_hash_puzzle_input() {
        let parsed = parse(input::INPUT);
        let hashed = parsed.iter().map(|i| hash(i)).sum::<u32>();
        assert_eq!(hashed, 514025);
    }

    #[test]
    fn can_fix_lens_input_example() {
        let parsed = parse(input::EXAMPLE_INPUT);
        let power = process_lenses(&parsed);
        assert_eq!(power, 145);
    }

    #[test]
    fn can_fix_lens_input() {
        let parsed = parse(input::INPUT);
        let power = process_lenses(&parsed);
        assert_eq!(power, 244461);
    }
}
