fn main() {
    let input = r"Time:        44     70     70     80
Distance:    283   1134   1134   1491";

    let race_data = parse(input);
    println!("{:?}", race_data);
    solve_race(&race_data);
    let part_two_race_data = parse_part_two(input);
    println!("{:?}", part_two_race_data);
    solve_race(&part_two_race_data);
}

fn solve_race(data: &Vec<RaceData>) {
    let mut total_ways_to_win: Vec<usize> = Vec::new();
    for race in data {
        let mut current_race_ways_to_win = 0;
        for time in 0..race.time {
            // figure out if we can beat distance by holding the button by time
            let time_left = race.time - time;
            let result = time * time_left;
            //println!("Time Left: {}, Held Button: {}, Calculated Distance: {}, Record: {}", time_left, time, result, race.distance);
            if result > race.distance {
                current_race_ways_to_win += 1;
            }
        }
        println!("Race: {:?}, Total Ways to Win: {}", race, current_race_ways_to_win);
        total_ways_to_win.push(current_race_ways_to_win);
    }
    // multiply total_ways_to_win by each other
    let mut total = 0;
    for i in total_ways_to_win {
        if total == 0 {
            total = i;
        } else {
            total *= i;
        }
    }

    println!("Part One: {}", total);
}

#[derive(Debug)]
struct RaceData {
    time: usize,
    distance: usize,
}

fn parse(input: &str) -> Vec<RaceData> {
    let mut result = Vec::new();
    let mut lines = input.lines();
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    let binding = time.split(":").nth(1).unwrap().replace("  ", "");
    let times = binding.split(" ").collect::<Vec<&str>>();
    let binding = distance.split(":").nth(1).unwrap().replace("  ", "");
    let distances = binding.split(" ").collect::<Vec<&str>>();
    for i in 0..times.len() {
        result.push(RaceData {
            time: times[i].parse::<usize>().unwrap(),
            distance: distances[i].parse::<usize>().unwrap(),
        });
    }
    result
}

fn parse_part_two(input: &str) -> Vec<RaceData> {
    let mut result = Vec::new();
    let mut lines = input.lines();
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    let times = time.split(":").nth(1).unwrap().replace(" ", "");
    let distances = distance.split(":").nth(1).unwrap().replace(" ", "");
    result.push(RaceData {
        time: times.parse::<usize>().unwrap(),
        distance: distances.parse::<usize>().unwrap(),
    });
    result
}

