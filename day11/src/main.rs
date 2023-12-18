use itertools::Itertools;

mod input;
mod test;

#[derive(Debug, Eq, PartialEq, Clone)]
enum GalaxyEntry {
    Galaxy,
    Empty
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Point(usize, usize);
impl Point {
    /// X is the row number
    #[inline]
    fn x(&self) -> usize {
        self.0
    }
    /// Y is the column number
    #[inline]
    fn y(&self) -> usize {
        self.1
    }

    fn manhattan(&self, p: Point) -> usize {
        let (x1, y1) = (self.x(), self.y());
        let (x2, y2) = (p.x(), p.y());
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }
}

fn parse_input(input: &str) -> Vec<Vec<GalaxyEntry>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| if c == '#' { GalaxyEntry::Galaxy } else { GalaxyEntry::Empty }).collect())
        .collect()
}

fn expand_universe(map: &mut Vec<Vec<GalaxyEntry>>, how_far_expand: usize) {
    // Expand Rows
    let mut where_to_insert_row: Vec<usize> = Vec::new();
    map.iter_mut().enumerate().for_each(|(index, row)| {
        if !row.contains(&GalaxyEntry::Galaxy) {
            where_to_insert_row.push(index);
        }
    });
    where_to_insert_row.reverse();
    for index in &where_to_insert_row {
        for _ in 0..how_far_expand {
            map.insert(*index, vec![GalaxyEntry::Empty; map[0].len()]);
        }
    }
    // Expand Columns
    let mut where_to_insert_column: Vec<usize> = Vec::new();
    for x in 0..map[0].len() {
        let mut galaxy_found = false;
        for y in 0..map.len() {
            if map[y][x] == GalaxyEntry::Galaxy {
                galaxy_found = true;
                break;
            }
        }
        if !galaxy_found {
            where_to_insert_column.push(x);
        }
    }
    where_to_insert_column.reverse();
    for index in &where_to_insert_column {
        for row in map.iter_mut() {
            for _ in 0..how_far_expand {
                row.insert(*index, GalaxyEntry::Empty);
            }
        }
    }
}

fn part_one(map: &Vec<Vec<GalaxyEntry>>) -> usize {
    let pairs = get_galaxy_pairs(get_galaxy_positions(map));
    pairs.into_iter().map(|(src, dst)| src.manhattan(dst)).sum()
}

fn get_galaxy_pairs(galaxy_positions: Vec<Point>) -> Vec<(Point, Point)> {
    galaxy_positions
        .into_iter()
        .tuple_combinations()
        .collect_vec()
}

fn get_galaxy_positions(map: &Vec<Vec<GalaxyEntry>>) -> Vec<Point> {
    map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().filter_map(|(x, col)| {
            if col == &GalaxyEntry::Galaxy {
                Some(Point{ 0: x, 1: y })
            } else {
                None
            }
        }).collect::<Vec<Point>>()
    }).flatten().collect()
}

fn main() {
    let mut map = parse_input(input::input);
    expand_universe(&mut map, 1);
    println!("Part One: {}", part_one(&map));

    let mut map = parse_input(input::input);
    expand_universe(&mut map, 1000000);
    println!("Part Two: {}", part_one(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut map = parse_input(input::input_example);
        expand_universe(&mut map, 1000000);
        println!("Part Two: {}", part_one(&map));
    }
}