const FILE_NAME: &str = "input_day9.txt";
use std::fs;
use std::iter::Map;

struct Disk {
    space: Vec<i32>,
}

impl Disk {
    fn new(input: &str) -> Disk {
        todo!()
    }
    fn present(&self) -> String {
        todo!()
    }
}

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("input: {:?}", input);
    // let parsed = parse_string(&input);
    // println!("input parsed: {:?}", &parsed);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_parse_string() {
        let string = "143023";
        let expected = "0....11122...";
        let result = Disk::new(string).present();
        assert_eq!(expected, result);
    }
}
