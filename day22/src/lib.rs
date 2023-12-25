mod input;

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[derive(Clone, Debug)]
struct Ranger {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}
impl Ranger {
    fn xy_overlaps(&self, settled: &Ranger) -> bool {
        ranges_overlap(&self.x, &settled.x) && ranges_overlap(&self.y, &settled.y)
    }
}

fn ranges_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.end() >= b.start() && b.end() >= a.start()
}

#[derive(Debug)]
struct Tower {
    bricks: Vec<Ranger>,
    rests_on: HashMap<usize, Vec<usize>>,
    supports: HashMap<usize, Vec<usize>>,
}

impl Tower {
    fn safe_to_disintegrate(&self, idx: usize) -> bool {
        let supported = self.supports.get(&idx);
        match supported {
            Some(supported) => {
                supported.iter().all(|supported| {
                    self
                        .rests_on
                        .get(supported)
                        .expect("supported brick should have rests_on")
                        .len() > 1
                })
            }
            None => return true,
        }
    }

    fn settle(&mut self) {
        for settling in 0..self.bricks.len() {
            let brick = self.bricks[settling].clone();
            let mut dest_z = 1;
            let mut may_rest_on = Vec::new();
            for (idx, settled) in self.bricks[0..settling].iter().enumerate() {
                if brick.xy_overlaps(settled) {
                    may_rest_on.push(idx);
                    dest_z = dest_z.max(*settled.z.end() + 1)
                }
            }
            for (idx, settled) in may_rest_on
                .into_iter()
                .map(|elem| (elem, &self.bricks[elem]))
            {
                if settled.z.end() + 1 == dest_z {
                    self.rests_on.entry(settling).or_default().push(idx);
                    self.supports.entry(idx).or_default().push(settling);
                }
            }
            let brick = &mut self.bricks[settling];
            brick.z = dest_z..=(brick.z.end() - brick.z.start() + dest_z);
        }
    }

    fn count_fallen_if_disintegrated(&self, idx: usize) -> usize {
        let mut falling = HashSet::from([idx]);

        let mut check = vec![];
        let mut check_next = self.supports.get(&idx).cloned().unwrap_or_default();

        while !check_next.is_empty() {
            std::mem::swap(&mut check, &mut check_next);
            for check in check.drain(..) {
                if self.rests_on[&check]
                    .iter()
                    .all(|elem| falling.contains(elem))
                {
                    falling.insert(check);
                    check_next.extend(
                        self.supports
                            .get(&check)
                            .iter()
                            .flat_map(|elem| elem.iter())
                            .copied(),
                    );
                }
            }
        }

        // -1 as we include the disintegrated brick
        falling.len() - 1
    }
}

fn parse_input(input: &str) -> Tower {
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start_parts = start.splitn(3, ',').collect::<Vec<_>>();
            let end_parts = end.splitn(3, ',').collect::<Vec<_>>();
            let [x1, y1, z1] = start_parts.as_slice() else {
                panic!()
            };
            let [x2, y2, z2] = end_parts.as_slice() else {
                panic!()
            };
            let [x1, x2, y1, y2, z1, z2] =
                [x1, x2, y1, y2, z1, z2].map(|elem| elem.parse().unwrap());
            Ranger {
                x: x1..=x2,
                y: y1..=y2,
                z: z1..=z2,
            }
        })
        .collect();
    bricks.sort_by_key(|brick| *brick.z.start());
    Tower {
        bricks,
        supports: HashMap::default(),
        rests_on: HashMap::default(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{input, parse_input};

    #[test]
    fn can_solve_part_1_example() {
        let mut tower = parse_input(input::EXAMPLE_INPUT);
        tower.settle();
        let count =  (0..tower.bricks.len())
            .filter(|&idx| tower.safe_to_disintegrate(idx))
            .count();
        assert_eq!(count, 5);
    }

    #[test]
    fn can_solve_part_1() {
        let mut tower = parse_input(input::INPUT);
        tower.settle();
        let count = (0..tower.bricks.len())
            .filter(|&idx| tower.safe_to_disintegrate(idx))
            .count();
        assert_eq!(count, 505);
    }

    #[test]
    fn can_solve_part_2_example() {
        let mut tower = parse_input(input::EXAMPLE_INPUT);
        tower.settle();
        let count: usize = (0..tower.bricks.len())
            .map(|idx| tower.count_fallen_if_disintegrated(idx))
            .sum();
        assert_eq!(count, 7);
    }

    #[test]
    fn can_solve_part_2() {
        let mut tower = parse_input(input::INPUT);
        tower.settle();
        let count: usize = (0..tower.bricks.len())
            .map(|idx| tower.count_fallen_if_disintegrated(idx))
            .sum();
        assert_eq!(count, 71002);
    }
}
