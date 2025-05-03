use regex::Regex;
use std::fs;

const FILE_NAME: &'static str = "input_day3.txt";
const REGEX: &str = r"mul\((\d+),(\d+)\)";
pub fn main() {
    println!("{}", "this is main");
    let file_path = format!("artifacts/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let result: i32 = find_regex_in_str(REGEX, &input)
        .iter()
        .map(|expression| parse_mul_arg(expression))
        .sum();
    println!("{}", result);
}

fn parse_mul_arg(input: &str) -> i32 {
    let parts: Vec<_> = input
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split(",")
        .collect();
    let a: i32 = convert_str_to_i32(parts[0]);
    let b: i32 = convert_str_to_i32(parts[1]);
    a * b
}

fn convert_str_to_i32(s: &str) -> i32 {
    s.parse().expect(&format!("{}{}", "can't parse", s))
}
fn find_regex_in_str<'a>(pattern: &'a str, text: &'a str) -> Vec<&'a str> {
    let re = Regex::new(pattern).expect("Invalid regex pattern");
    re.find_iter(text).map(|m| m.as_str()).collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_parse_mul_arg() {
        assert_eq!(parse_mul_arg("mul(2,4)"), 8);
    }
    #[test]
    fn test_find_regex_rand() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let pattern = r"\b\w{5}\b";
        let matches = find_regex_in_str(pattern, text);
        assert_eq!(matches, vec!["quick", "brown", "jumps"]);
    }
    #[test]
    fn test_find_regex() {
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let matches = find_regex_in_str(REGEX, text);
        assert_eq!(
            matches,
            vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]
        );
    }
}
