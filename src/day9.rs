use std::fs;

const FILE_NAME: &str = "input_day9.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("input: {:?}", input);
}

// fn unite_free_space(s: &str) -> String {
//     let mut free_space_index = 0;
//     let mut block_index = s.len().saturating_sub(1);
//     let mut chars: Vec<_> = s.chars().collect();
//     while free_space_index < block_index {
//         if is_free_space(chars[block_index]) {
//             block_index -= 1;
//         } else if is_free_space(chars[free_space_index]) {
//             chars.swap(free_space_index, block_index);
//             block_index -= 1;
//         } else {
//             free_space_index += 1;
//         }
//     }
//     chars.into_iter().collect()
// }
//
// fn is_free_space(c: char) -> bool {
//     c == '.'
// }
//
// fn check_sum(s: &str) -> usize{
//
//     todo!()
// }

fn parse_string(s: &str) -> Vec<i32>{


    todo!()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse_string(){
        let string = "143023";
        let parsed = vec![0,-1-1-1-1,1,1,1,2,2,-1-1-1];
        assert_eq!(parse_string(string), parsed);
    }

    // #[test]
    // fn test_unite_free_space() {
    //     let s = "0..111....22222";
    //     let answer = String::from("022111222......");
    //     assert_eq!(unite_free_space(s), answer);
    //
    //     let s2 = "00...111...2...333.44.5555.6666.777.888899";
    //     let answer2 = String::from("0099811188827773336446555566..............");
    //     assert_eq!(unite_free_space(s2), answer2);
    // }
    //
    // #[test]
    // fn test_check_sum() {
    //     let s = String::from("0099811188827773336446555566..............");
    //     assert_eq!(check_sum(&s), 1928);
    // }
}
