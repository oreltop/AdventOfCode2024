use std::fs;

pub fn main() {
    println!("this is main");
    let file_path = "artifacts/input_files/input_day4.txt";
    let raw_input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let result = count_all(&raw_input);
    println!("{result}")
}

fn count_all(raw_input: &str) -> usize {
    let forward_input = parse_string(raw_input);
    let reverse_input = reverse_vec(&forward_input);
    let line_size = count_columns(raw_input);
    let result: usize = [forward_input, reverse_input]
        .iter()
        .map(|input| {
            count_xmas(input, 1, line_size)
                + count_xmas(input, line_size, line_size)
                + count_xmas(input, line_size + 1, line_size)
                + count_xmas(input, line_size - 1, line_size)
        })
        .sum();
    result
}

fn count_columns(input: &str) -> usize {
    input.split_once('\r').unwrap().0.len()
}

fn parse_string(input: &str) -> Vec<char> {
    input.chars().filter(|&c| c != '\r').collect()
}
fn count_xmas(input: &[char], jump: usize, line_size: usize) -> usize {
    let letter_count = 1 + jump * 3;
    let new_line = if letter_count >= line_size { 1 } else { 0 };
    input
        .windows(letter_count + new_line * 3)
        .filter(|&window| window.iter().filter(|&&c| c == '\n').count() == new_line * 3)
        .filter(|&window| {
            window[0] == 'X'
                && window[jump + new_line] == 'M'
                && window[(jump + new_line) * 2] == 'A'
                && window[(jump + new_line) * 3] == 'S'
        })
        .count()
}

fn reverse_vec<T: Clone>(vec: &[T]) -> Vec<T> {
    vec.iter().cloned().rev().collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

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
        assert_eq!(result.len(), 29);
        assert_eq!(result[0], 'O');
        assert_eq!(result[8], 'X');
        assert_eq!(result[14], 'M');
        assert_eq!(result[20], 'A');
        assert_eq!(result[26], 'S');
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
        assert_eq!(count_xmas(&input, 1, count_columns(&raw_input)), 1);
    }
    #[test]
    fn test_find_something() {
        let file_path = "artifacts/test_files/day4/day4-one-horizontal.txt";
        let raw_input = fs::read_to_string(file_path).unwrap();
        let input = parse_string(&raw_input);
        assert!(count_xmas(&input, 1, count_columns(&raw_input)) >= 1);
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
