use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};
use rayon::prelude::*;

mod input;

#[derive(Debug)]
struct CoordinateLookup<'a> {
    pub left: &'a str,
    pub right: &'a str,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right
}

fn assemble_instructions() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for char in input::instructions.chars() {
        match char {
            'L' => instructions.push(Instruction::Left),
            'R' => instructions.push(Instruction::Right),
            _ => panic!("Invalid instruction")
        }
    }
    instructions
}

fn assemble_lookup_table<'a>() -> HashMap<&'a str, CoordinateLookup<'a>> {
    //AAA = (BBB, CCC)
    let mut lookup_table: HashMap<&'a str, CoordinateLookup> = HashMap::new();
    for line in input::input.lines() {
        // grab the first 3 characters of line
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        let lookup = CoordinateLookup {
            left: left,
            right: right
        };
        lookup_table.insert(key, lookup);
    }
    lookup_table
}

fn main() {
    let instructions = assemble_instructions();
    let lookup_table = assemble_lookup_table();
    part_one_solve(&instructions, &lookup_table);
    part_two_solve(&instructions, &lookup_table);
}

fn part_one_solve(instructions: &Vec<Instruction>, lookup_table: &HashMap<&str, CoordinateLookup>) {
    // get the first coordinate.
    let mut current_lookup = "AAA";
    let mut steps = 0;
    loop {
        let instruction = &instructions[steps % instructions.len()];
        let lookup = do_lookup(current_lookup, lookup_table);
        //println!("Looking for {} at {}, found {:?}, using {:?}", current_lookup, steps, lookup, instruction);
        match instruction {
            Instruction::Left => {
                current_lookup = lookup.left;
            },
            Instruction::Right => {
                current_lookup = lookup.right;
            }
        }
        steps += 1;
        if current_lookup == "ZZZ" {
            println!("Found ZZZ!");
            println!("Total: {}", steps);
            break;
        }
    }
    println!("Done!");
}

fn do_lookup<'a>(name: &str, lookup_table: &'a HashMap<&str, CoordinateLookup<'_>>) -> &'a CoordinateLookup<'a> {
    match lookup_table.get(name) {
        Some(lookup) => lookup,
        None => panic!("Invalid lookup")
    }
}

fn part_two_solve(instructions: &Vec<Instruction>, lookup_table: &HashMap<&str, CoordinateLookup>) {
    // find all in the lookup table that end in A
    let ending_in_a = get_all_that_end_in_character(lookup_table, 'A');
    println!("All that end in A: {:?}", ending_in_a);
    let ending_in_z = get_all_that_end_in_character(lookup_table, 'Z');
    println!("All that end in Z: {:?}", ending_in_z);

    //println!("Output of ending in z: {:?}", check_all_items_for_z(&ending_in_z));

    let mut map: Vec<(&str, usize)> = Vec::new();
    for item in &ending_in_a {
        map.push((item, 0));
    }


    let mut end_records: Arc<RwLock<BTreeMap<usize, &str>>> = Arc::new(RwLock::new(BTreeMap::new()));
    let total_steps: Vec<_> = map.par_iter_mut().map(|(item, steps)| {
        let cloned_end_records = end_records.clone();
        loop {
            let instruction = &instructions[*steps % instructions.len()];
            let lookup = do_lookup(item, lookup_table);
            match instruction {
                Instruction::Left => {
                    *item = lookup.left;
                },
                Instruction::Right => {
                    *item = lookup.right;
                }
            }
            *steps += 1;
            if item.ends_with("Z") {
                break;
            }
            // if check_all_items_for_z(&ending_in_a) {
            //     println!("All items end in Z!");
            //     println!("Steps: {}", steps);
            //     break;
            // }
        }
        println!("Took me {} steps to Z", steps);
        cloned_end_records.write().unwrap().insert(*steps, item);
        (item, steps)
    }).collect();

    let lcm = find_lcm(total_steps.iter().map(|(_, steps)| **steps).collect());
    if check_all_items_for_z(&total_steps) && lcm > 0 {
        println!("All items end in Z!");
        println!("LCM: {}", lcm);
        return;
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn find_lcm(numbers: Vec<usize>) -> usize {
    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = lcm(result, num);
    }

    result
}

fn check_all_items_for_z<'a>(items: &Vec<(&mut &str, &mut usize)>) -> bool {
    for (item, _) in items {
        if &item[item.len()-1..] != "Z" {
            return false;
        }
    }
    true
}

fn get_all_that_end_in_character<'a>(lookup_table: &'a HashMap<&str, CoordinateLookup<'_>>, character: char) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for (key, _) in lookup_table {
        if key.ends_with(character) {
            results.push(key);
        }
    }
    results
}