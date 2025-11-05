use itertools::Update;
use std::fs;

const FILE_NAME: &'static str = "input_day5.txt";

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

fn does_break_rule(update: &[i32], rule: &(i32, i32)) -> bool {
    let first_pos = update.iter().position(|&item| item == rule.0);
    if first_pos.is_none() {
        return false;
    };

    let later_pos = update.iter().position(|&item| item == rule.1);
    if later_pos.is_none() {
        return false;
    };

    first_pos > later_pos
}

fn is_update_correct(update: &[i32], rules: &[(i32,i32)]) -> bool {
    !rules.iter().any(|rule| does_break_rule(update, rule))
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn rule_breaking() {
        let rule = (2, 1);
        let update = vec![1, 2, 3, 4, 5];
        assert!(does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_breaking() {
        let rule = (1, 2);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_apply() {
        let rule = (1, 6);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_apply2() {
        let rule = (9, 6);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }
    #[test]
    fn rule_not_apply3() {
        let rule = (9, 2);
        let update = vec![1, 2, 3, 4, 5];
        assert!(!does_break_rule(&update, &rule))
    }

    #[test]
    fn doesnt_break_any_rule(){
        let rules = vec![(1,2), (3,5), (1,9), (5,6), (8,4)];
        let update = vec![1,2,3,4,5];
        assert!(is_update_correct(&update, &rules))
    }
    #[test]
    fn break_one_rule(){
        let rules = vec![(1,2), (5,2), (1,9), (5,6), (8,4)];
        let update = vec![1,2,3,4,5];
        assert!(!is_update_correct(&update, &rules))
    }
}
