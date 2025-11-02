use std::fs;
use std::sync::OnceLock;
const FILE_NAME: &str = "input_day4.txt";
static INPUT_SIZE: OnceLock<(usize, usize)> = OnceLock::new();

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("input: {:?}", input);
}

fn count_rows(input: &str) -> usize {
    let split: Vec<&str> = input.split_whitespace().collect();
    split.len()
}

fn find_shape(input: &str) -> &(usize, usize) {
    INPUT_SIZE.get_or_init(||{
        let split: Vec<&str> = input.split_whitespace().collect();
        (split.len(), split[0].len())
    })


}

fn parse_string(input: &str) -> Vec<char> {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

fn count_xmas(input: Vec<char>, jump: usize) -> usize {
    let mut count = 0;
    let indexes = 1..input.len() - jump * 3;
    for index in indexes {
        if input[index] == 'X'
            && input[index + jump] == 'M'
            && input[index + jump * 2] == 'A'
            && input[index + jump * 3] == 'S'
        {
            count += 1;
        }
    }

    count
}

fn count_xmas_backwards(input: Vec<char>, jump: usize) -> usize {
    let mut count = 0;
    let indexes = 1..input.len() - jump * 3;
    for index in indexes {
        if input[input.len() - index] == 'X'
            && input[input.len() - index + jump] == 'M'
            && input[input.len() - index + jump * 2] == 'A'
            && input[input.len() - index + jump * 3] == 'S'
        {
            count += 1;
        }
    }

    count
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_count_rows() {
        let file_path = "artifacts/test_files/day4-one-vertical.txt";
        let input = fs::read_to_string(file_path).unwrap();
        let result = count_rows(&input);
        assert_eq!(result, 5);
    }
    #[test]
    fn test_input_shape() {
        let file_path = "artifacts/test_files/day4-one-vertical.txt";
        let input = fs::read_to_string(file_path).unwrap();
        let result = find_shape(&input);
        assert_eq!(*result, (5,5));
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
        let file_path = "artifacts/test_files/day4-one-horizontal.txt";
        let input = parse_string(&fs::read_to_string(file_path).unwrap());
        assert_eq!(count_xmas_backwards(input, 1), 1);
    }
}

#[test]
fn test_find_horizontal() {
    let file_path = "artifacts/test_files/day4-one-horizontal.txt";
    let input = parse_string(&fs::read_to_string(file_path).unwrap());
    assert_eq!(count_xmas(input, 1), 1);
}
#[test]
fn test_dont_find_wraps() {
    let file_path = "artifacts/test_files/day4-one-horizontal-wrap.txt";
    let input = parse_string(&fs::read_to_string(file_path).unwrap());
    assert_eq!(count_xmas(input, 1), 1);
}
