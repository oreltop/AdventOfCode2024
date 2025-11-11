use crate::day7::Operation::{Addition, Multiplication};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const FILE_NAME: &str = "input_day7.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let per = Permutator::new();
    // println!("input: {:?}", input);
    // let parsed = parse_string(&input);
    // println!("input parsed: {:?}", &parsed);
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
}

struct Permutator {
    cache: HashMap<usize, Vec<Operation>>,
}
impl Permutator {
    fn new() -> Permutator {
        Permutator {
            cache: HashMap::new(),
        }
    }
    fn all_permutation(&self, length: usize) -> Vec<Vec<Operation>> {
        let possibilities_iters =
            itertools::repeat_n(vec![Addition, Multiplication].into_iter(), length);

        possibilities_iters.multi_cartesian_product().collect()

    }
}

fn calculate_result(parts: &[i64], operations: &[Operation]) -> i64 {
    let mut operations_iter = operations.iter();
    parts.iter().copied().reduce(
        |a, b| match operations_iter.next() {
            None => { panic!("this shouldn't happen!") }
            Some(Addition) => { a + b },
            Some(Multiplication) => { a * b }
        }
    ).unwrap()

}

fn could_possibly_be_true(result: i64, parts: Vec<i64>) -> bool {
    todo!()
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
        assert!(could_possibly_be_true(result, parts));
    }
    #[test]
    fn all_permutations_length_3() {
        let permutations = Permutator::new().all_permutation(3);
        assert_eq!(permutations.len(), 8);
        assert!(permutations.contains(&vec![Addition, Addition, Addition]));
        assert!(permutations.contains(&vec![Multiplication, Addition, Addition]));
        assert!(permutations.contains(&vec![Addition, Multiplication, Addition]));
        assert!(permutations.contains(&vec![Addition, Addition, Multiplication]));
        assert!(permutations.contains(&vec![Multiplication, Addition, Multiplication]));
        assert!(permutations.contains(&vec![Addition, Multiplication, Multiplication]));
        assert!(permutations.contains(&vec![Multiplication, Multiplication, Addition]));
        assert!(permutations.contains(&vec![Multiplication, Multiplication, Multiplication]));
    }
    #[test]
    fn make_a_simple_calculation(){
        let parts = vec![1,2,3];
        let operations = vec![Addition, Multiplication];
        assert_eq!(calculate_result(&parts, &operations), 9);
    }
}
