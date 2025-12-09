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
        if self.number == 0 {
            return vec![Stone::new(1)];
        }
        if self.is_even_digits() {
            return self.split();
        }
        vec![Stone::new(self.number * 2024)]
    }
    fn count_digits(&self) -> u32 {
        if self.number == 0 {
            return 1;
        }
        (self.number as f64).log10().floor() as u32 + 1
    }
    fn is_even_digits(&self) -> bool {
        self.count_digits() % 2 == 0
    }

    fn split_number(&self) -> (u32, u32) {
        let divisor = 10u32.pow(self.count_digits() / 2);
        let left = self.number / divisor;
        let right = self.number % divisor;
        (left, right)
    }
    fn split(&self) -> Vec<Stone> {
        let (left, right) = self.split_number();
        vec![Stone::new(left), Stone::new(right)]
    }
}

fn parse_string(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|str| Stone::new(str.parse().expect("not a number!")))
        .collect()
}

fn blink(line: Vec<Stone>) -> Vec<Stone>{
    todo!()
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
    #[test]
    fn test_count_digits() {
        let stone = Stone::new(1);
        let result = stone.count_digits();
        let expected = 1;
        assert_eq!(result, expected);
        let stone = Stone::new(12);
        let result = stone.count_digits();
        let expected = 2;
        assert_eq!(result, expected);
        let stone = Stone::new(1234);
        let result = stone.count_digits();
        let expected = 4;
        assert_eq!(result, expected);
        let stone = Stone::new(1234567);
        let result = stone.count_digits();
        let expected = 7;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_split_number() {
        let stone = Stone::new(10);
        let result = stone.split_number();
        let expected = (1, 0);
        assert_eq!(result, expected);
        let stone = Stone::new(1000);
        let result = stone.split_number();
        let expected = (10, 0);
        assert_eq!(result, expected);
        let stone = Stone::new(1234);
        let result = stone.split_number();
        let expected = (12, 34);
        assert_eq!(result, expected);
        let stone = Stone::new(12345678);
        let result = stone.split_number();
        let expected = (1234, 5678);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bling(){
        let line = parse_string("125 17");
        let result = blink(line);
        let expected = parse_string("253000 1 7");
        assert_eq!(result, expected);
        let line = parse_string("253 0 2024 14168");
        let result = blink(line);
        let expected = parse_string("512072 1 20 24 28676032");
        assert_eq!(result, expected);
        let line = parse_string("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
        let result = blink(line);
        let expected = parse_string("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2");
        assert_eq!(result, expected);
    }


}
