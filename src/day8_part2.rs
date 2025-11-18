use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::{Add, Sub};

const FILE_NAME: &str = "input_day8.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let result = count_antinodes(&input);
    println!("{}", &result);
}

#[derive(PartialEq, Debug, Hash, Eq, Copy, Clone)]
struct Point(i32, i32);
impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl Sub for &Point {
    type Output = (i32, i32);

    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0, self.1 - other.1)
    }
}

impl Point {
    fn on_grid(&self, grid_size: &(usize, usize)) -> bool {
        let grid_size = (grid_size.0 as i32, grid_size.1 as i32);
        self.0 >= 0 && self.0 < grid_size.0 && self.1 >= 0 && self.1 < grid_size.1
    }
}

#[derive(Debug)]
struct ResonantHarmonic {
    vector: (i32, i32),
    grid_size: (usize, usize),
    current: Point,
    timeout: usize,
}
impl ResonantHarmonic {
    fn new(
        init_point: &Point,
        vector: &(i32, i32),
        grid_size: &(usize, usize),
    ) -> ResonantHarmonic {
        ResonantHarmonic {
            vector: *vector,
            grid_size: *grid_size,
            current: *init_point,
            timeout: 100,
        }
    }
}

impl Iterator for ResonantHarmonic {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vector == (0, 0) {
            return None;
        }

        let result = match self.current.on_grid(&self.grid_size) {
            true => Some(self.current),
            false => None,
        };
        self.current = Point(
            self.current.0 + self.vector.0,
            self.current.1 + self.vector.1,
        );
        self.timeout -= 1;
        println!("{:?}", self);
        if self.timeout == 0 {
            panic!("timeout reached!")
        }

        result
    }
}

fn parse_string(input: &str) -> HashMap<char, Vec<Point>> {
    input
        .trim()
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|(_x, c)| c != &'.')
                .map(move |(x, c)| (c, Point(x as i32, y as i32)))
        })
        .fold(HashMap::new(), |mut antennas, (c, point)| {
            antennas.entry(c).or_default().push(point);
            antennas
        })
}

fn get_size(input: &str) -> (usize, usize) {
    let lines: Vec<_> = input.trim().lines().collect();
    (lines.len(), lines[0].trim().len())
}

fn calculate_antinodes_for_frequency(
    antennas: &Vec<Point>,
    size: &(usize, usize),
) -> HashSet<Point> {
    let mut result = HashSet::new();

    let pairs = antennas.iter().cartesian_product(antennas);
    for point_pair in pairs {
        let distance = point_pair.0 - point_pair.1;
        println!("point pair: {:?}, distance: {:?}", point_pair, distance);
        result.extend(ResonantHarmonic::new(point_pair.0, &distance, size));
    }
    result
}

fn find_all_antinodes(
    antennas: &HashMap<char, Vec<Point>>,
    size: &(usize, usize),
) -> HashSet<Point> {
    antennas
        .iter()
        .flat_map(|(_, points)| calculate_antinodes_for_frequency(points, size))
        .collect()
}

fn count_antinodes(input: &str) -> usize {
    let size = get_size(input);
    let antennas = parse_string(input);
    find_all_antinodes(&antennas, &size).len()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use itertools::Itertools;
    #[test]
    fn test_parse_string() {
        let input = r"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............";

        let parsed = parse_string(input);
        println!("{:?}", parsed);

        let values_0 = parsed.get(&'0').unwrap();
        let answer_0 = vec![Point(4, 7), Point(7, 8), Point(5, 9), Point(8, 10)];
        assert!(answer_0.iter().all(|p| values_0.iter().contains(p)));
        assert_eq!(values_0.len(), answer_0.len());

        let values_a = parsed.get(&'A').unwrap();
        let answer_a = vec![Point(9, 2), Point(8, 3), Point(6, 6)];
        assert!(answer_a.iter().all(|p| values_a.iter().contains(p)));
        assert_eq!(values_a.len(), answer_a.len());
    }

    #[test]
    fn test_calculate_antinodes() {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        let a_antennas = vec![Point(1, 1), Point(2, 2)];
        antennas.insert('a', a_antennas);
        let size = (8, 8);
        let antinodes = find_all_antinodes(&antennas, &size);
        let answers = HashSet::from([
            Point(0, 0),
            Point(1, 1),
            Point(2, 2),
            Point(3, 3),
            Point(4, 4),
            Point(5, 5),
            Point(6, 6),
            Point(7, 7),
        ]);
        assert_eq!(antinodes, answers);
    }
    #[test]
    fn test_count_antinodes() {
        let input = r"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............";
        assert_eq!(count_antinodes(input), 34);
    }

    #[test]
    fn resonant_harmonic() {
        let harmonic_series: Vec<_> =
            ResonantHarmonic::new(&Point(0, 0), &(2, 1), &(8, 8)).collect();
        let answer = [Point(0, 0), Point(2, 1), Point(4, 2), Point(6, 3)];
        assert_eq!(harmonic_series, answer)
    }
}
