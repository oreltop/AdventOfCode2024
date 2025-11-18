use std::fs;

const FILE_NAME: &str = "input_day9.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("input: {:?}", input);
    let parsed = parse_string(&input);
    println!("input parsed: {:?}", &parsed);
}

fn parse_string(input: &str) -> Vec<i32> {
    let mut column1: Vec<i32> = Vec::new();
    column1
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_unite_free_space() {
        let s = "0..111....22222";
        let answer = String::from("022111222......");
        assert_eq!(unite_free_space(s), answer);

        let s2 = "00...111...2...333.44.5555.6666.777.888899";
        let answer2 = String::from("0099811188827773336446555566..............");
        assert_eq!(unite_free_space(s2), answer2);

    }
}
