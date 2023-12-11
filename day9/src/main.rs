mod input;

#[derive(Debug)]
struct Sequence {
    numbers: Vec<i32>
}

fn parse_input() -> Vec<Sequence> {
    let mut sequences: Vec<Sequence> = Vec::new();
    for line in input::input.lines() {
        let mut numbers: Vec<i32> = Vec::new();
        for number in line.split_whitespace() {
            numbers.push(number.parse::<i32>().unwrap());
        }
        sequences.push(Sequence { numbers });
    }
    sequences
}

fn main() {
    let parsed = parse_input();
    println!("Parsed: {:?}", parsed);
    part_one_solve(&parsed);
}

fn part_one_solve(sequences: &Vec<Sequence>) {
    let mut future_difference = 0;
    let mut past_difference = 0;
    for sequence in sequences {
        let mut differences: Vec<Vec<i32>> = Vec::new();
        // populate the differences.
        differences.push(sequence.numbers.clone());
        //println!("Started with: {:?}", differences);
        // calculate the items difference.
        loop {
            let mut difference: Vec<i32> = Vec::new();
            let last_difference = differences.last().unwrap();
            for i in 0..last_difference.len() {
                if i == last_difference.len() - 1 {
                    break;
                }
                let diff = last_difference[i+1] - last_difference[i];
                difference.push(diff);
            }
            if difference.iter().all(|x| *x == 0) {
                differences.push(difference);
                //println!("Sequence calculated!");
                future_difference += extrapolate_future(&differences);
                past_difference += extrapolate_past(&differences);
                break;
            } else {
                //println!("Sequence needs more work: {:?}", difference);
            }
            differences.push(difference);
        }
    }

    println!("Total future difference: {}", future_difference);
    println!("Total past difference: {}", past_difference);
}

fn extrapolate_future(differences: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for difference in differences {
        total += difference.last().unwrap();
    }
    total
}

fn extrapolate_past(differences: &Vec<Vec<i32>>) -> i32 {
    let mut binding = differences.clone();
    let mut appended: Vec<_> = binding.iter_mut().map(|mut x | { x.insert(0, 0); x }).collect();
    for i in (0..appended.len()).rev() {
        if i == 0 { break; }
        appended[i-1][0] = appended[i-1][1] - appended[i][0];
    }
    println!("Input: {:?}", appended);
    println!("Extrapolated Past: {}", appended[0][0]);
    appended[0][0]
}