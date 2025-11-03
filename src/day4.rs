use std::fs;
use std::sync::OnceLock;

static INPUT_SIZE: OnceLock<(usize, usize)> = OnceLock::new();

pub fn main() {
    println!("this is main");
    let file_path = "artifacts/input_files/input_day4.txt";
    let raw_input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let result = count_all(&raw_input);
    println!("{result}")
}

fn count_all(raw_input: &String) -> usize {
    let forward_input = parse_string(&raw_input);
    let reverse_input = reverse_vec(&forward_input);
    let line_size = count_columns(&raw_input);
    let result: usize = [forward_input, reverse_input]
        .iter()
        .map(|input| {
            count_xmas(&input, 1, line_size)
                + count_xmas(&input, line_size, line_size)
                + count_xmas(&input, line_size + 1, line_size)
            // + count_xmas(&input, get_shape().1 - 1)
        })
        .sum();
    result
}

fn init_shape(input: &str) -> &(usize, usize) {
    INPUT_SIZE.get_or_init(|| {
        let split: Vec<&str> = input.split_whitespace().collect();
        (split.len(), split[0].len())
    })
}

fn get_shape() -> &'static (usize, usize) {
    INPUT_SIZE.get().unwrap()
}

fn count_columns(input: &str) -> usize {
    input.split_once('\r').unwrap().0.len()
}

fn parse_string(input: &str) -> Vec<char> {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

fn count_xmas(input: &[char], jump: usize, line_size: usize) -> usize {
    let search_up_to = line_size - jump % line_size * 3;
    input
        .windows(1 + jump * 3)
        .enumerate()
        .filter(|&(i, _)| i % line_size < search_up_to)
        .filter(|&(_, window)| {
            window[0] == 'X'
                && window[jump] == 'M'
                && window[jump * 2] == 'A'
                && window[jump * 3] == 'S'
        })
        .count()
}

fn reverse_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    vec.iter().cloned().rev().collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input_shape() {
        let file_path = "artifacts/test_files/day4/day4-one-vertical.txt";
        let input = fs::read_to_string(file_path).unwrap();
        let result = init_shape(&input);
        assert_eq!(*result, (5, 5));
    }
    #[test]
    fn test_count_columns() {
        let file_path = "artifacts/test_files/day4/day4-one-vertical.txt";
        let input = fs::read_to_string(file_path).unwrap();
        let result = count_columns(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_parse_string() {
        let file_path = "artifacts/test_files/day4/day4-one-vertical.txt";
        let input = fs::read_to_string(file_path).unwrap();
        let result = parse_string(&input);
        println!("{:?}", result);
        assert_eq!(result.len(), 25);
        assert_eq!(result[0], 'O');
        assert_eq!(result[7], 'X');
        assert_eq!(result[12], 'M');
        assert_eq!(result[17], 'A');
        assert_eq!(result[22], 'S');
    }

    #[test]
    fn test_find_horizontal_backwards() {
        let file_path = "artifacts/test_files/day4/day4-one-horizontal-backwards.txt";
        let raw_input = &fs::read_to_string(file_path).unwrap();
        let input = reverse_vec(&parse_string(&raw_input));
        assert_eq!(count_xmas(&input, 1, count_columns(raw_input)), 1);
    }

    #[test]
    fn test_find_horizontal() {
        let file_path = "artifacts/test_files/day4/day4-one-horizontal.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let input = parse_string(&raw_input);
        println!("{}", count_columns(&raw_input));
        assert_eq!(count_xmas(&input, 1, count_columns(&raw_input)), 1);
    }
    #[test]
    fn test_dont_find_wraps() {
        let file_path = "artifacts/test_files/day4/day4-one-horizontal-wrap.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let input = parse_string(&raw_input);
        assert_eq!(count_xmas(&input, 1, count_columns(&raw_input)), 1);
    }
    #[test]
    fn test_dont_find_wraps_backwards() {
        let file_path = "artifacts/test_files/day4/day4-one-horizontal-backwards-wrap.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let input = reverse_vec(&parse_string(&raw_input));
        assert_eq!(count_xmas(&input, 1, count_columns(&raw_input)), 1);
    }

    #[test]
    fn test_find_vertical() {
        let file_path = "artifacts/test_files/day4/day4-one-vertical.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let input = parse_string(&raw_input);
        assert_eq!(
            count_xmas(&input, count_columns(&raw_input), count_columns(&raw_input)),
            1
        );
    }
    #[test]
    fn test_find_vertical_backwards() {
        let file_path = "artifacts/test_files/day4/day4-one-verticalc-backwards.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let input = reverse_vec(&parse_string(&raw_input));
        assert_eq!(
            count_xmas(&input, count_columns(&raw_input), count_columns(&raw_input)),
            1
        );
    }

    #[test]
    fn test_find_diagonal() {
        let file_path = "artifacts/test_files/day4/day4-one-diagonal.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let input = parse_string(&raw_input);
        assert_eq!(
            count_xmas(
                &input,
                count_columns(&raw_input) + 1,
                count_columns(&raw_input)
            ),
            1
        );
    }
    #[test]
    fn test_find_diagonal_backwards() {
        let file_path = "artifacts/test_files/day4/day4-one-diagonal-backwards.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let input = reverse_vec(&parse_string(&raw_input));
        assert_eq!(
            count_xmas(
                &input,
                count_columns(&raw_input) + 1,
                count_columns(&raw_input)
            ),
            1
        );
    }
    #[test]
    fn test_count_all() {
        let file_path = "artifacts/test_files/day4/full_example.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();

        let result = count_all(&raw_input);
        assert_eq!(result, 18);
    }
}
