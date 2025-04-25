use itertools::sorted;
use std::fs;

fn main() {
    let file_path = "artifacts/input_day1.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lists = parse_string(&input);
    let result = calculate_similarity(&lists.0, &lists.1);
    println!("{}", result)
}

fn parse_string(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect();

        if let [first, second] = &numbers[..] {
            column1.push(*first);
            column2.push(*second);
        }
    }
    (column1, column2)
}

fn calculate_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let list1_sorted = sorted(list1);
    let mut list2_sorted = sorted(list2);
    let mut result: i32 = 0;
    for i in list1_sorted {
        result += (i - list2_sorted.next().unwrap()).abs();
    }

    result
}

fn calculate_similarity(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for i in list1 {
        result += list2.iter().filter(|x| *x == i).sum::<i32>();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_string() {
        let data = "60236   87497
        27507   18604
        69810   73952
        60448   56269";
        let result = parse_string(data);
        let expected = (
            vec![60236, 27507, 69810, 60448],
            vec![87497, 18604, 73952, 56269],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_distance() {
        let result = calculate_distance(&vec![3, 1, 5, 2], &vec![5, 0, 2, 4]);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_calculate_similarity() {
        let result = calculate_similarity(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, 31);
    }
}
