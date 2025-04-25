use std::fs;

const FILE_NAME: &'static str = "input_day2.txt";
const MAX_STEP: &'static i32 = &3;
const MIN_STEP: &'static i32 = &1;

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
        if list[i + 1] - list[i] > *MAX_STEP {
            return false;
        }
        if list[i + 1] - list[i] < *MIN_STEP {
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
    use std::ops::Range;

    for i in (Range {
        start: 0,
        end: list.len() - 1,
    }) {
        if list[i + 1] - list[i] > *MAX_STEP {
            return false;
        }
        if list[i + 1] - list[i] < *MIN_STEP {
            return false;
        }
    }

    true
}

fn count_safe_lists(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter()
        .map(|list| is_safe_part1(list))
        .filter(|x| *x == true)
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
