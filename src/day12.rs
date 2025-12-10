use std::collections::{HashMap, HashSet};
use std::fs;

const FILE_NAME: &str = "input_day12.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("input: {:?}", input);
    // let parsed = parse_string(&input);
    // println!("input parsed: {:?}", &parsed);
}

struct Cell {
    x: usize,
    y: usize,
    crop: char,
    group: Option<u64>,
}

struct Polygon {
    crop: char,
    cells: HashSet<Cell>
}

fn parse_string(input: &str) -> HashSet<Polygon> {
    todo!()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_parse_string() {
        let input = r"
            AAAA
            BBCD
            BBCC
            EEEC";
        let result = parse_string(input);
        assert_eq!(result.len(), 4);
        let input = r"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO";
        let result = parse_string(input);
        assert_eq!(result.len(), 5);
        let input = r"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE";
        let result = parse_string(input);
        assert_eq!(result.len(), 11);
    }
}
