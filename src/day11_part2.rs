use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;

const FILE_NAME: &str = "input_day11.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut line = StonesLine::from(&input);
    line.blink_n_times(75);
    println!("{}", line.count_stones());
}

struct StonesLine {
    stones: HashMap<u64, u64>,
    calculator: Calculator,
}

impl StonesLine {
    fn new() -> StonesLine {
        StonesLine {
            stones: HashMap::new(),
            calculator: Calculator::new(),
        }
    }

    fn from(input: &str) -> StonesLine {
        let stones = input
            .split_whitespace()
            .map(|str| str.parse().expect("not a number!"));

        let mut result = StonesLine::new();
        for stone in stones {
            result.add_one(stone);
        }
        result
    }
    fn add(&mut self, number: u64, amount: u64) {
        *self.stones.entry(number).or_insert(0) += amount;
    }

    fn add_one(&mut self, number: u64) {
        self.add(number, 1)
    }
    fn add_multiple(&mut self, numbers: &[u64], amount: u64) {
        for number in numbers {
            self.add(*number, amount);
        }
    }

    fn blink(&mut self) {
        let mut next_line = StonesLine::new();
        for (stone, amount) in &self.stones {
            next_line.add_multiple(self.calculator.change(*stone), *amount)
        }
        self.stones = next_line.stones
    }

    fn blink_n_times(&mut self, times: usize) {
        for _ in 0..times {
            self.blink()
        }
    }

    fn count_stones(&self) -> u64 {
        self.stones.iter().fold(0, |acc, (_, amount)| acc + amount)
    }
}
impl Debug for StonesLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.stones.fmt(f)
    }
}
impl PartialEq<Self> for StonesLine {
    fn eq(&self, other: &Self) -> bool {
        self.stones == other.stones
    }
}

#[derive(Clone)]
struct Calculator {
    cache: HashMap<u64, Vec<u64>>,
}
impl Calculator {
    fn new() -> Calculator {
        Calculator {
            cache: HashMap::new(),
        }
    }

    fn change(&mut self, number: u64) -> &[u64] {
        self.cache
            .entry(number)
            .or_insert(Calculator::calculate_change(number))
    }

    fn calculate_change(number: u64) -> Vec<u64> {
        if number == 0 {
            return vec![1];
        }
        if Calculator::is_even_digits(number) {
            return Calculator::split(number);
        }
        vec![number * 2024]
    }
    fn count_digits(number: u64) -> u64 {
        if number == 0 {
            return 1;
        }
        (number as f64).log10().floor() as u64 + 1
    }
    fn is_even_digits(number: u64) -> bool {
        Calculator::count_digits(number).is_multiple_of(2)
    }

    fn split_number(number: u64) -> (u64, u64) {
        let divisor = 10u64.pow((Calculator::count_digits(number) / 2) as u32);
        let left = number / divisor;
        let right = number % divisor;
        (left, right)
    }
    fn split(number: u64) -> Vec<u64> {
        let (left, right) = Calculator::split_number(number);
        vec![left, right]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn change_stone_0() {
        let stone = 0;
        let result = Calculator::calculate_change(stone);
        let expected = vec![1];
        assert_eq!(result, expected);
    }
    #[test]
    fn change_stone_split() {
        let stone = 1000;
        let result = Calculator::calculate_change(stone);
        let expected = vec![10, 0];
        assert_eq!(result, expected);
    }
    #[test]
    fn change_stone_multiply() {
        let stone = 2;
        let result = Calculator::calculate_change(stone);
        let expected = vec![4048];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bling() {
        let mut line = StonesLine::from("125 17");
        line.blink();
        let expected = StonesLine::from("253000 1 7");
        assert_eq!(line, expected);
        let mut line = StonesLine::from("253 0 2024 14168");
        line.blink();
        let expected = StonesLine::from("512072 1 20 24 28676032");
        assert_eq!(line, expected);
        let mut line = StonesLine::from("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
        line.blink();
        let expected = StonesLine::from(
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
        );
        assert_eq!(line, expected);
    }

    #[test]
    fn blink_6_times() {
        let mut line = StonesLine::from("125 17");
        line.blink_n_times(6);
        let expected = StonesLine::from(
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
        );
        assert_eq!(line, expected);
    }
}
