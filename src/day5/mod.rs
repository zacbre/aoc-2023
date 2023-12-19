use lazy_static::lazy_static;

mod input;

#[derive(Debug)]
struct RangeCalculator {
    source_range: usize,
    dest_range: usize,
    length: usize
}

impl RangeCalculator {
    fn new(source_range: usize, dest_range: usize, length: usize) -> RangeCalculator {
        RangeCalculator {
            source_range,
            dest_range,
            length
        }
    }
}

fn parse_range_to_calculator(input: &str) -> Vec<RangeCalculator> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let dest_range = iter.next().unwrap().parse::<usize>().unwrap();
        let source_range = iter.next().unwrap().parse::<usize>().unwrap();
        let length = iter.next().unwrap().parse::<usize>().unwrap();
        result.push(RangeCalculator::new(source_range, dest_range, length));
    }
    result
}

lazy_static! {
    static ref seed_to_soil_map: Vec<RangeCalculator> = parse_range_to_calculator(input::seed_to_soil_map);
    static ref soil_to_fertilizer_map: Vec<RangeCalculator> = parse_range_to_calculator(input::soil_to_fertilizer_map);
    static ref fertilizer_to_water_map: Vec<RangeCalculator> = parse_range_to_calculator(input::fertilizer_to_water_map);
    static ref water_to_light_map: Vec<RangeCalculator> = parse_range_to_calculator(input::water_to_light_map);
    static ref light_to_temperature_map: Vec<RangeCalculator> = parse_range_to_calculator(input::light_to_temperature_map);
    static ref temperature_to_humidity_map: Vec<RangeCalculator> = parse_range_to_calculator(input::temperature_to_humidity_map);
    static ref humidity_to_location_map: Vec<RangeCalculator> = parse_range_to_calculator(input::humidity_to_location_map);
}


fn convert_from_source_to_dest(input: usize, converter: &Vec<RangeCalculator>) -> usize {
    for range_calculator in converter {
        // find the one with the range in it
        if input >= range_calculator.source_range && input <= range_calculator.source_range + (range_calculator.length - 1) {
            //println!("Found range: {:?} for {}", range_calculator, input);
            // we found the range.
            let mut result = input - range_calculator.source_range;
            result = result + range_calculator.dest_range;
            return result;
        }
    }
    return input;
}

fn convert_from_source_to_dest_vec(input: Vec<usize>, converters: &Vec<&Vec<RangeCalculator>>) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    for item in input {
        let mut modified_value = item;
        for calc in converters {
            modified_value = convert_from_source_to_dest(modified_value, calc);
        }
        values.push(modified_value);
    }
    values
}

fn part_one() {
    let location_values = convert_from_source_to_dest_vec(input::seeds.to_vec(), &vec![&(*seed_to_soil_map), &(*soil_to_fertilizer_map), &(*fertilizer_to_water_map), &(*water_to_light_map), &(*light_to_temperature_map), &(*temperature_to_humidity_map), &(*humidity_to_location_map)]);
    println!("{:?}", location_values);
    // return the lowest of the location_values
    let lowest = location_values.iter().min().unwrap();
    println!("Lowest value: {}", lowest);
}

fn part_two() {
    // the seeds are now ranges.
    let mut seed_list: Vec<usize> = Vec::new();
    let mut first_seed = 0;
    for seed in input::seeds.to_vec() {
        if first_seed == 0 {
            first_seed = seed;
        } else {
            for i in first_seed..first_seed+seed {
                seed_list.push(i);
            }
            first_seed = 0;
        }
    }

    let location_values = convert_from_source_to_dest_vec(seed_list, &vec![&(*seed_to_soil_map), &(*soil_to_fertilizer_map), &(*fertilizer_to_water_map), &(*water_to_light_map), &(*light_to_temperature_map), &(*temperature_to_humidity_map), &(*humidity_to_location_map)]);
    let lowest = location_values.iter().min().unwrap();
    println!("Lowest value: {}", lowest);
}

#[cfg(test)]
mod tests {
    use crate::day5::{part_one, part_two};

    #[test]
    fn can_run_part_one() {
        part_one();
    }

    #[test]
    fn can_run_part_two() {
        part_two();
    }
}