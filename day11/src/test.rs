use itertools::Itertools;
use std::collections::BTreeMap;
use crate::input;

fn part2(input: &str, n: usize) -> usize {
    let universe = Universe::create(input, n - 1);
    let pairs = universe.galaxy_pairs();
    pairs.into_iter().map(|(src, dst)| src.manhattan(dst)).sum()
}

#[derive(Debug)]
enum Space {
    Empty,
    Galaxy,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            val => panic!("invalid space: {val}"),
        }
    }
}

#[derive(Debug)]
struct Universe {
    universe: BTreeMap<Point, Space>,
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

impl Universe {
    fn create(input: &str, n: usize) -> Self {
        let universe = Expander::new(input).expand_universe(input, n);
        Self { universe }
    }

    fn galaxies(&self) -> Vec<Point> {
        self.universe
            .iter()
            .filter_map(|(&point, space)| match space {
                Space::Galaxy => Some(point),
                _ => None,
            })
            .collect()
    }

    fn galaxy_pairs(&self) -> Vec<(Point, Point)> {
        self.galaxies()
            .into_iter()
            .tuple_combinations()
            .collect_vec()
    }
}

#[derive(Debug)]
struct Expander {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Expander {
    fn new(input: &str) -> Self {
        // count rows
        let rows = input
            .lines()
            .enumerate()
            .filter_map(|(i, line)| if !line.contains('#') { Some(i) } else { None })
            .collect_vec();

        // count cols
        let col_count = input.lines().next().unwrap().chars().count();
        let cols = (0..col_count)
            .filter(|&c| {
                input
                    .lines()
                    .all(|line| line.chars().nth(c).unwrap() != '#')
            })
            .collect_vec();

        Self { rows, cols }
    }

    /// Expands universe, with empty row/cols expanded by factor n
    fn expand_universe(&self, input: &str, n: usize) -> BTreeMap<Point, Space> {
        let mut expanded_uni = BTreeMap::new();

        let (mut x_offset, mut y_offset) = (0, 0);
        for (i, line) in input.lines().enumerate() {
            if self.rows.contains(&i) {
                x_offset += n;
                continue;
            }
            for (j, space) in line.char_indices() {
                if self.cols.contains(&j) {
                    y_offset += n;
                    continue;
                }
                expanded_uni.insert(Point(i + x_offset, j + y_offset), space.into());
            }
            y_offset = 0;
        }

        expanded_uni
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let result = part2(input::input, 2);
        assert_eq!(result, 9591768);
    }

    #[test]
    fn test_part2() {
        let result = part2(input::input, 1_000_000);
        assert_eq!(result, 746962097860);
    }

    #[test]
    fn test_universe_pairs() {
        let input = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."};
        let universe = Universe::create(input, 1);
        assert_eq!(universe.galaxy_pairs().len(), 36);
    }

    #[test]
    fn test_expander() {
        let input = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."};
        let result = Expander::new(input);

        assert_eq!(result.rows, vec![3, 7]);
        assert_eq!(result.cols, vec![2, 5, 8]);
    }
}