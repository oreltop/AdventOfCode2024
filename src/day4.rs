use std::fs;

const FILE_NAME: &'static str = "input_day4.txt";

pub fn main() {
    println!("{}", "this is main");
    let file_path = format!("artifacts/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("input: {:?}", input);
}

fn parse_string(input: &str) -> Vec<&str> {
    let column1: Vec<&str> = Vec::new();
    column1
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_parse_string() {
        let file_path = "artifacts/test_files/day4-one-vertical.txt";
        let input = fs::read_to_string(file_path).unwrap();
        let result = parse_string(&input);
        
        assert_eq!(result.len(), 25); 
        assert_eq!(result[0], "O");
        assert_eq!(result[8], "X");
        assert_eq!(result[13], "M");
        assert_eq!(result[18], "A");
        assert_eq!(result[23], "S");
        
        
    }
}
