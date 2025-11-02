use std::fs;
use std::sync::OnceLock;
const FILE_NAME: &str = "input_day4.txt";

static INPUT_SIZE: OnceLock<(usize, usize)> = OnceLock::new();

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    init_shape(&input);
    println!("input: {:?}", &input);
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

fn parse_string(input: &str) -> Vec<char> {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

fn count_xmas(input: Vec<char>, jump: usize) -> usize {
    let columns = get_shape().1;
    let search_up_to = columns - jump % columns * 3;
    input
        .windows(1 + jump * 3)
        .enumerate()
        .filter(|&(i, _)| i % columns < search_up_to)
        .filter(|&(_, window)| {
            window[0] == 'X'
                && window[jump] == 'M'
                && window[jump * 2] == 'A'
                && window[jump * 3] == 'S'
        })
        .count()
}

fn reverse_vec<T: Clone>(vec: Vec<T>) -> Vec<T> {
    vec.iter().cloned().rev().collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input_shape() {
        let file_path = "artifacts/test_files/day4-one-vertical.txt";
        let input = fs::read_to_string(file_path).unwrap();
        let result = init_shape(&input);
        assert_eq!(*result, (5, 5));
    }

    #[test]
    fn test_parse_string() {
        let file_path = "artifacts/test_files/day4-one-vertical.txt";
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
        let file_path = "artifacts/test_files/day4-one-horizontal-backwards.txt";
        let raw_input = &fs::read_to_string(file_path).unwrap();
        let input = reverse_vec(parse_string(&raw_input));
        assert_eq!(count_xmas(input, 1), 1);
    }

    #[test]
    fn test_find_horizontal() {
        let file_path = "artifacts/test_files/day4-one-horizontal.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        init_shape(&raw_input);
        let input = parse_string(&raw_input);
        assert_eq!(count_xmas(input, 1), 1);
    }
    #[test]
    fn test_dont_find_wraps() {
        let file_path = "artifacts/test_files/day4-one-horizontal-wrap.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        init_shape(&raw_input);
        let input = parse_string(&raw_input);
        assert_eq!(count_xmas(input, 1), 1);
    }
    #[test]
    fn test_dont_find_wraps_backwards() {
        let file_path = "artifacts/test_files/day4-one-horizontal-backwards-wrap.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        init_shape(&raw_input);
        let input = reverse_vec(parse_string(&raw_input));
        assert_eq!(count_xmas(input, 1), 1);
    }

    #[test]
    fn test_find_vertical() {
        let file_path = "artifacts/test_files/day4-one-vertical.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        init_shape(&raw_input);
        let input = parse_string(&raw_input);
        assert_eq!(count_xmas(input, get_shape().1), 1);
    }
    #[test]
    fn test_find_vertical_backwards() {
        let file_path = "artifacts/test_files/day4-one-verticalc-backwards.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        init_shape(&raw_input);
        let input = reverse_vec(parse_string(&raw_input));
        assert_eq!(count_xmas(input, get_shape().1), 1);
    }

    #[test]
    fn test_find_diagonal() {
        let file_path = "artifacts/test_files/day4-one-diagonal.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        init_shape(&raw_input);
        let input = parse_string(&raw_input);
        assert_eq!(count_xmas(input, get_shape().1 + 1), 1);
    }
    #[test]
    fn test_find_diagonal_backwards() {
        let file_path = "artifacts/test_files/day4-one-diagonal-backwards.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        init_shape(&raw_input);
        let input = reverse_vec(parse_string(&raw_input));
        assert_eq!(count_xmas(input, get_shape().1 + 1), 1);
    }
}
