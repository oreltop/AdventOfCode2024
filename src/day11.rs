use std::fs;
use std::time::SystemTime;

const FILE_NAME: &str = "input_day11.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("input: {:?}", input);
    let parsed = parse_string(&input);
    println!("input parsed: {:?}", &parsed);
}

#[derive(PartialEq, Debug)]
struct Stone {
    number: u32,
}

impl Stone {
    fn new(number: u32) -> Stone {
        Stone { number }
    }

    fn change(&self) -> Vec<Stone> {
        todo!()
    }
    fn is_even_digits(&self) -> bool {
        if self.number == 0 {
            return false;
        }
        let digit_count = (self.number as f64).log10().floor() as u32 + 1;
        digit_count %2 == 0

    }
}

fn parse_string(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|str| Stone::new(str.parse().expect("not a number!")))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn change_stone_0() {
        let stone = Stone::new(0);
        let result = stone.change();
        let expected = vec![Stone::new(1)];
        assert_eq!(result, expected);
    }
    #[test]
    fn change_stone_split() {
        let stone = Stone::new(1000);
        let result = stone.change();
        let expected = vec![Stone::new(10), Stone::new(0)];
        assert_eq!(result, expected);
    }
    #[test]
    fn change_stone_multiply() {
        let stone = Stone::new(2);
        let result = stone.change();
        let expected = vec![Stone::new(4048)];
        assert_eq!(result, expected);
    }
}
