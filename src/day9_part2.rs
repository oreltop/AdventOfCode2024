use itertools::{Itertools, concat};
use std::fs;
use std::iter::once;
use crate::day9_part2::DiskSpace::{Block, FreeSpace};

const FILE_NAME: &str = "input_day9.txt";
const EMPTY_SPACE: i32 = -1;

#[derive(Debug)]
#[derive(PartialEq)]
enum DiskSpace {
    FreeSpace { size: usize },
    Block { size: usize, id: usize },
}

impl DiskSpace{
    fn size(&self) -> usize{
        match self {
            FreeSpace {size} => *size,
            Block {size, .. } => *size
        }
    }
}

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input_raw = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let input = parse_string(&input_raw);
    // let disk_sorted = unite_free_space(&input);
    // print!("{}", check_sum(&disk_sorted));
}

fn unite_free_space(disk: &[i32]) -> Vec<i32> {
    let mut free_space_index = 0;
    let mut block_index = disk.len().saturating_sub(1);
    let mut disk = disk.to_owned();
    while free_space_index < block_index {
        if is_free_space(disk[block_index]) {
            block_index -= 1;
        } else if is_free_space(disk[free_space_index]) {
            disk.swap(free_space_index, block_index);
            block_index -= 1;
        } else {
            free_space_index += 1;
        }
    }
    disk
}

fn is_free_space(item: i32) -> bool {
    item == EMPTY_SPACE
}

fn check_sum(disk: &[i32]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(index, content)| match is_free_space(*content) {
            true => 0,
            false => index * (*content as usize),
        })
        .sum()
}

fn parse_string(s: &str) -> Vec<DiskSpace> {
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
                Block {size: *block_size as usize, id: index},
                FreeSpace {size: *free_space as usize}
            ]
        })
        .filter(|disk| {disk.size()>0})
        .collect()

}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let string = "143023";
        let parsed = vec![
            Block { size: 1, id: 0 },
            FreeSpace { size: 4 },
            Block { size: 3, id: 1 },
            Block { size: 2, id: 2 },
            FreeSpace { size: 3 },
        ];
        assert_eq!(parse_string(string), parsed);
    }
    //
    // #[test]
    // fn test_unite_free_space() {
    //     let result = parse_for_testing("0..111....22222");
    //     let answer = parse_for_testing("022111222......");
    //     assert_eq!(unite_free_space(&result), answer);
    //
    //     let result2 = parse_for_testing("00...111...2...333.44.5555.6666.777.888899");
    //     let answer2 = parse_for_testing("0099811188827773336446555566..............");
    //     assert_eq!(unite_free_space(&result2), answer2);
    // }
    //
    // #[test]
    // fn test_check_sum() {
    //     let s = parse_for_testing("0099811188827773336446555566..............");
    //     assert_eq!(check_sum(&s), 1928);
    // }
    // fn parse_for_testing(s: &str) -> Vec<i32> {
    //     s.chars()
    //         .map(|c| match c == '.' {
    //             true => -1,
    //             false => c.to_digit(10).unwrap() as i32,
    //         })
    //         .collect()
    // }
}
