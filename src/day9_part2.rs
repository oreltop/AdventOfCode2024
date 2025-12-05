const FILE_NAME: &str = "input_day9.txt";
const EMPTY_SPACE: i32 = -1;

use itertools::Itertools;
use std::fs;
use std::iter::once;

struct Disk {
    space: Vec<i32>,
}

impl Disk {
    fn order(&self) {
        todo!()
    }
    fn find_empty_space(&self, size: usize) -> Option<usize> {
        self.space
            .windows(size)
            .enumerate()
            .find(|(_, window)| window.iter().all(|&i| i == -1))
            .map(|(idx, _)| idx)
    }

    fn new(input: &str) -> Disk {
        Disk {
            space: Disk::parse_string(input),
        }
    }

    fn parse_string(s: &str) -> Vec<i32> {
        let pairs: Vec<(i32, i32)> = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .chain(once(0)) // to avoid the last item being dropped if odd
            .tuples()
            .collect();
        pairs
            .iter()
            .enumerate()
            .flat_map(|(index, (block_size, free_space))| {
                [
                    vec![index as i32; *block_size as usize],
                    vec![EMPTY_SPACE; *free_space as usize],
                ]
                .concat()
            })
            .collect()
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
        let expected = vec![0, -1, -1, -1, -1, 1, 1, 1, 2, 2, -1, -1, -1];
        let result = Disk::new(string).space;
        assert_eq!(expected, result);

        let string = "2333133121414131402";
        let expected = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];
        let result = Disk::new(string).space;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_order_disk() {
        let string = "2333133121414131402";
        let expected = vec![
            0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5, -1,
            6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1,
        ];
        let disk = Disk::new(string);
        disk.order();
        let result = disk.space;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_empty_space() {
        let string = "2333133121414131402";
        // 00...111...2...333.44.5555.6666.777.888899
        let disk = Disk::new(string);
        let empty_space_index = disk.find_empty_space(3);
        assert_eq!(empty_space_index, Some(2));
        let empty_space_index = disk.find_empty_space(1);
        assert_eq!(empty_space_index, Some(2));
        let empty_space_index = disk.find_empty_space(4);
        assert_eq!(empty_space_index, None);
    }
}
