use std::fs;

const FILE_NAME: &'static str = "input_day2.txt";

pub fn main() {
    println!("{}", "this is main");
    let file_path = format!("artifacts/{}",FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let parsed = parse_string(&input);
    println!("input parsed: {:?}", parsed);

}

fn parse_string(input: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for line in input.lines(){
        let new_list: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect();
        result.push(new_list)
    }
    result
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_dummy(){
        println!("{}", "this is test dummy")
    }

}
