use std::fs;

const FILE_NAME: &str = "input_day7.txt";

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
    fn test_parse_string(){
        println!("{}", "this is test dummy")
    }
}
