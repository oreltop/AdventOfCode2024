use regex::Regex;
use std::fs;

const FILE_NAME: &'static str = "input_day3.txt";
const REGEX_PART_1: &str = r"mul\((\d+),(\d+)\)";
const REGEX_PART_2: &str = r"mul\((\d+),(\d+)\)|don't|do";

pub fn main() {
    println!("{}", "this is main");
    let file_path = format!("artifacts/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    main_part2(&input);
}

fn main_part1(input: &str){
    let result: i32 = find_regex_in_str(REGEX_PART_1, input)
        .iter()
        .map(|expression| parse_mul_arg(expression))
        .sum();
    println!("{}", result);
}

fn main_part2(input: &str){
    let mut active = true;
    
    let matches = find_regex_in_str(REGEX_PART_2,input);
    let mut result = 0;
    for item in matches{
        match item{
            "do" => active = true,
            "don't" => active = false,
            _ => if active{
                result += parse_mul_arg(item)
            } else {  }
        }
    }
    println!("{}", result)
    
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
    fn test_find_regex_part1() {
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let matches = find_regex_in_str(REGEX_PART_1, text);
        assert_eq!(
            matches,
            vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]
        );
    }
    #[test]
    fn test_find_regex_part2(){
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            find_regex_in_str(REGEX_PART_2, text),
            vec!["mul(2,4)", "do", "mul(5,5)", "mul(11,8)", "mul(8,5)"]
        );
        let text = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(
            find_regex_in_str(REGEX_PART_2, text),
            vec!["mul(2,4)", "don't", "mul(5,5)", "mul(11,8)", "do", "mul(8,5)"]
        );
    }
    
}
