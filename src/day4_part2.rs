use std::fs;

pub fn main() {
    println!("this is main");
    let file_path = "artifacts/input_files/input_day4.txt";
    let raw_input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let result = count_xmas(parse_string(&raw_input), count_columns(&raw_input));
    println!("{result}")
}

fn count_columns(input: &str) -> usize {
    input.split_once('\r').unwrap().0.len()
}

fn parse_string(input: &str) -> Vec<char> {
    input.chars().filter(|&c| c != '\r').collect()
}
fn count_xmas(input: Vec<char>, line_size: usize) -> usize {
    input
        .windows(line_size * 2 + 5)
        .filter(|&window| {
            [window[0], window[line_size * 2 + 4]].contains(&'M')
                && [window[0], window[line_size * 2 + 4]].contains(&'S')
                && [window[2], window[line_size * 2 + 2]].contains(&'M')
                && [window[2], window[line_size * 2 + 2]].contains(&'S')
                && window[line_size + 2] == 'A'
        })
        .count()
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
    fn test_trivial() {
        let file_path = "artifacts/test_files/day4/part2-3x3.txt";
        let raw_input = &fs::read_to_string(file_path).unwrap();
        let input = parse_string(&raw_input);
        assert_eq!(count_xmas(input, count_columns(raw_input)), 1);
    }
    #[test]
    fn test_10x10() {
        let file_path = "artifacts/test_files/day4/part2-10x10.txt";
        let raw_input = &fs::read_to_string(file_path).unwrap();
        let input = parse_string(&raw_input);
        assert_eq!(count_xmas(input, count_columns(raw_input)), 9);
    }
}
