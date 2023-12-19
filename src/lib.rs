mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

fn read_test_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    let filename = format!("{}/src/{}", std::env::current_dir().unwrap().to_str().unwrap().to_string(), filename);
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}