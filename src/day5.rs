use std::fs;

const FILE_NAME: &'static str = "input_day5.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}",FILE_NAME);
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
    fn rule_not_breaking(){
        let rule = (1,2);
        let update = vec![1,2,3,4,5];
        assert!(does_break_rule(update, rule))
    }
}
