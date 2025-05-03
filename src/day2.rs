use std::fs;

const FILE_NAME: &'static str = "input_day2.txt";
const MAX_STEP: i32 = 3;
const MIN_STEP: i32 = 1;

pub fn main() {
    let file_path = format!("artifacts/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let parsed = parse_string(&input);
    println!("{:?}", &count_safe_lists(&parsed));
}

fn parse_string(input: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let new_list: Vec<i32> = parse_line(line);
        result.push(new_list)
    }
    result
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|num| num.parse::<i32>().expect("Invalid number"))
        .collect()
}
fn is_safe_part1(list: &Vec<i32>) -> bool {
    let mut reversed_list = list.clone();
    reversed_list.reverse();
    check_safety_one_direction_part1(&list) || check_safety_one_direction_part1(&reversed_list)
}

fn check_safety_one_direction_part1(list: &Vec<i32>) -> bool {
    use std::ops::Range;

    for i in (Range {
        start: 0,
        end: list.len() - 1,
    }) {
        if list[i + 1] - list[i] > MAX_STEP {
            return false;
        }
        if list[i + 1] - list[i] < MIN_STEP {
            return false;
        }
    }

    true
}

fn is_safe_part2(list: &Vec<i32>) -> bool {
    let mut reversed_list = list.clone();
    reversed_list.reverse();
    check_safety_one_direction_part2(&list) || check_safety_one_direction_part2(&reversed_list)
}

fn check_safety_one_direction_part2(list: &Vec<i32>) -> bool {
    if let Some(index) =  find_unsafe_index(list){
        tolerate_a_single_bad_level(list, index)
    }
    else { true }
}

fn tolerate_a_single_bad_level(list: &Vec<i32>, bad_level: usize) -> bool {
    tolerate_ith_level(list, bad_level) || tolerate_ith_level(list, bad_level+1)
}

fn find_unsafe_index(list: &Vec<i32>) -> Option<usize> {


    for i in 0..list.len() - 1{
        if list[i + 1] - list[i] > MAX_STEP {

            return Some(i);
        }
        if list[i + 1] - list[i] < MIN_STEP {
            return Some(i);
        }
    }

    None
}

fn tolerate_ith_level(list: &Vec<i32>, i: usize) -> bool{
    let new_list = clone_vec_without_ith_item(list, i);
    check_safety_one_direction_part1(&new_list)
}

fn clone_vec_without_ith_item<T: Clone>(list: &Vec<T>, i: usize) -> Vec<T> {
    list.iter()
        .enumerate()
        .filter(|&(index, _)| index != i) // Exclude item at index `i`
        .map(|(_, item)| item.clone()) // Clone values to avoid move issues
        .collect()
}

fn count_safe_lists(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .filter(|list| is_safe_part2(list))
        .count()
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_safe_part1() {
        assert_eq!(is_safe_part1(&parse_line("7 6 4 2 1")), true);
        assert_eq!(is_safe_part1(&parse_line("1 3 6 7 9")), true);
    }
    #[test]
    fn test_unsafe_part1() {
        assert_eq!(is_safe_part1(&parse_line("1 2 7 8 9")), false);
        assert_eq!(is_safe_part1(&parse_line("9 7 6 2 1")), false);
        assert_eq!(is_safe_part1(&parse_line("1 3 2 4 5")), false);
        assert_eq!(is_safe_part1(&parse_line("8 6 4 4 1")), false);
    }
    #[test]
    fn test_safe_part2() {
        assert_eq!(is_safe_part2(&parse_line("7 6 4 2 1")), true);
        assert_eq!(is_safe_part2(&parse_line("1 3 6 7 9")), true);
        assert_eq!(is_safe_part2(&parse_line("1 3 2 4 5")), true);
        assert_eq!(is_safe_part2(&parse_line("8 6 4 4 1")), true);
    }
    #[test]
    fn test_unsafe_part2() {
        assert_eq!(is_safe_part2(&parse_line("1 2 7 8 9")), false);
        assert_eq!(is_safe_part2(&parse_line("9 7 6 2 1")), false);
    }
}
