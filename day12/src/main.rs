mod input;

fn main() {

}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Gear {
    Empty,
    Gear,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct GearMap {
    map: Vec<Gear>,
    combinations: Vec<usize>,
}

fn parse(input: &str, part_two: bool) -> Vec<GearMap> {
    let mut maps = Vec::new();
    for line in input.lines() {
        let mut input_split = line.split(' ');
        let mut map = Vec::new();
        let mut combinations = Vec::new();
        for c in input_split.next().unwrap().chars() {
            match c {
                '?' => map.push(Gear::Unknown),
                '#' => map.push(Gear::Gear),
                _ => map.push(Gear::Empty)
            };
        }
        for c in input_split.next().unwrap().split(',') {
            combinations.push(c.parse().unwrap());
        }
        if part_two {
            combinations = combinations.repeat(5);
            map = [&map[..]; 5].join(&Gear::Unknown).to_vec();
        }
        maps.push(GearMap {
            map,
            combinations,
        });
    }
    maps
}

fn solve(gear_map: &Vec<GearMap>) -> usize {
    let mut total = 0;
    for item in gear_map {
        total += possible(item);
    }
    total
}

fn possible(gear_map: &GearMap) -> usize {
    let mut l = gear_map.map.clone();
    let n = gear_map.combinations.clone();
    if l.last() == Some(&Gear::Empty) { l.remove(l.len() - 1); }
    let mut vl = Vec::with_capacity(l.len() + 1);
    vl.push(Gear::Empty);
    vl.extend(l);
    let sz = vl.len() + 1;

    let (mut oldstate, mut newstate) = (vec![0; sz], vec![0; sz]);
    oldstate[0] = 1;

    for i in 1..vl.len() {
        if vl[i] != Gear::Gear { oldstate[i] = 1; } else { break; }
    }

    for cnt in n {
        let mut grp = 0;
        for (i, &ref c) in vl.iter().enumerate() {
            if c == &Gear::Empty { grp = 0; } else { grp += 1; }
            if c != &Gear::Gear {
                newstate[i + 1] += newstate[i];
            }
            if grp >= cnt && vl[i - cnt] != Gear::Gear {
                newstate[i + 1] += oldstate[i - cnt];
            }
        }
        oldstate.iter_mut().for_each(|x| *x = 0);
        (oldstate, newstate) = (newstate, oldstate);
    }

    oldstate[sz - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let input = "???.### 1,1,3";
        let parsed = parse(input, false);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].map.len(), 7);
        assert_eq!(parsed[0].combinations.len(), 3);
        let expected = vec![Gear::Unknown, Gear::Unknown, Gear::Unknown, Gear::Empty, Gear::Gear, Gear::Gear, Gear::Gear];
        assert_eq!(parsed[0].map, expected);
        assert_eq!(parsed[0].combinations, vec![1, 1, 3]);
    }

    #[test]
    fn test_example_1() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let parsed = parse(input, false);
        let total = super::solve(&parsed);
        println!("Example Total: {}", total);
        assert_eq!(total, 21);
    }

    #[test]
    fn part_one() {
        let total = super::solve(&parse(input::INPUT, false));
        println!("Part One Total: {}", total);
        assert_eq!(total, 8022);
    }

    #[test]
    fn part_two() {
        let total = super::solve(&parse(input::INPUT, true));
        println!("Part Two Total: {}", total);
        assert_eq!(total, 4968620679637);
    }
}