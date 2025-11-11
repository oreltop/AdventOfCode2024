use crate::day7_part2::Operation::{Addition, Concatenation, Multiplication};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const FILE_NAME: &str = "input_day7.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input_parsed = parse_string(&input);
    let mut permutator = Permutator::new();
    let result: i64 = input_parsed
        .iter()
        .filter(|&(result, parts)| could_possibly_be_true(*result, &parts, &mut permutator))
        .map(|(result, _)| result)
        .sum();
    println!("{:?}", result);
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

struct Permutator {
    cache: HashMap<usize, Vec<Vec<Operation>>>,
}
impl Permutator {
    fn new() -> Permutator {
        Permutator {
            cache: HashMap::new(),
        }
    }
    fn all_permutation(&mut self, length: usize) -> Vec<Vec<Operation>> {
        self.cache
            .entry(length)
            .or_insert(Self::calculate_permutations(length))
            .to_owned()
    }

    fn calculate_permutations(length: usize) -> Vec<Vec<Operation>> {
        let possibilities_iters = itertools::repeat_n(
            vec![Addition, Multiplication, Concatenation].into_iter(),
            length,
        );

        possibilities_iters.multi_cartesian_product().collect()
    }
}

fn calculate_result(parts: &[i64], operations: &[Operation]) -> i64 {
    let mut operations_iter = operations.iter();
    parts
        .iter()
        .copied()
        .reduce(|a, b| match operations_iter.next() {
            None => {
                panic!("this shouldn't happen!")
            }
            Some(Addition) => a + b,
            Some(Multiplication) => a * b,
            Some(Concatenation) => format!("{}{}", a, b).parse().unwrap(),
        })
        .unwrap()
}

fn could_possibly_be_true(result: i64, parts: &[i64], mut permutator: &mut Permutator) -> bool {
    let permutations = permutator.all_permutation(parts.len());
    permutations
        .iter()
        .any(|operations| calculate_result(parts, operations) == result)
}

fn parse_string(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(result, parts)| {
            (
                result.parse::<i64>().unwrap(),
                parts
                    .trim()
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}
pub mod tests {
    use super::*;

    #[test]
    fn test_simple_addition_two_numbers() {
        let result = 6;
        let parts = vec![2, 3];
        let mut permutator = Permutator::new();
        assert!(could_possibly_be_true(result, &parts, &mut permutator));
    }
    #[test]
    fn all_permutations_length_3() {
        let permutations = Permutator::new().all_permutation(3);
        assert_eq!(permutations.len(), 27);
        assert!(permutations.contains(&vec![Addition, Addition, Concatenation]));
        assert!(permutations.contains(&vec![Multiplication, Concatenation, Addition]));
        assert!(permutations.contains(&vec![Addition, Multiplication, Addition]));
        assert!(permutations.contains(&vec![Addition, Concatenation, Multiplication]));
        assert!(permutations.contains(&vec![Multiplication, Concatenation, Multiplication]));
        assert!(permutations.contains(&vec![Multiplication, Multiplication, Concatenation]));
        assert!(permutations.contains(&vec![Multiplication, Concatenation, Concatenation]));
        assert!(permutations.contains(&vec![Multiplication, Multiplication, Multiplication]));
    }
    #[test]
    fn make_a_simple_calculation() {
        let parts = vec![1, 2, 3, 4];
        let operations = vec![Addition, Multiplication, Concatenation];
        assert_eq!(calculate_result(&parts, &operations), 94);
    }
}
