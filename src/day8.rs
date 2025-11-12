use std::collections::HashMap;
use std::fs;

const FILE_NAME: &str = "input_day8.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("input: {:?}", input);
    let parsed = parse_string(&input);
    println!("input parsed: {:?}", &parsed);
}

#[derive(PartialEq, Debug)]
struct Point(i32, i32);

fn parse_string(input: &str) -> HashMap<char, Vec<Point>> {
    let mut column1: Vec<i32> = Vec::new();
    column1
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

        let mut parsed = parse_string(input);

        let values_0 = parsed.get(&'0').unwrap();
        let result_0 = vec![Point(4, 7), Point(7, 8), Point(5, 9), Point(8, 10)];
        assert!(result_0.iter().all(|p| values_0.iter().contains(p)));
        assert_eq!(values_0.len(), result_0.len());

        let values_A = parsed.get(&'A').unwrap();
        let result_A = vec![Point(2, 9), Point(3, 8), Point(6, 6)];
        assert!(result_A.iter().all(|p| values_0.iter().contains(p)));
        assert_eq!(values_A.len(), result_0.len());
    }
}
