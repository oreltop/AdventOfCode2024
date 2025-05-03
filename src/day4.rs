use std::fs;

const FILE_NAME: &'static str = "input_day4.txt";

pub fn main() {
    println!("{}", "this is main");
    let file_path = format!("artifacts/{}",FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("input: {:?}", input);

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
