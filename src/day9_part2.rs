use crate::day9_part2::DiskSpace::{Block, FreeSpace};
use itertools::{Itertools, concat};
use std::cmp::Ordering;
use std::fs;
use std::iter::once;

const FILE_NAME: &str = "input_day9.txt";
const EMPTY_SPACE: i32 = -1;

#[derive(Debug, PartialEq)]
enum DiskSpace {
    FreeSpace { size: usize },
    Block { size: usize, id: usize },
}

impl DiskSpace {
    fn size(&self) -> usize {
        match self {
            FreeSpace { size } => *size,
            Block { size, .. } => *size,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            FreeSpace { .. } => true,
            Block { .. } => false,
        }
    }
}

impl PartialOrd for DiskSpace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size().partial_cmp(&other.size())
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

fn unite_free_space(disk: &[DiskSpace]) -> Vec<DiskSpace> {
    todo!()
}

// fn check_sum(disk: &[i32]) -> usize {
//     disk.iter()
//         .enumerate()
//         .map(|(index, content)| match is_free_space(*content) {
//             true => 0,
//             false => index * (*content as usize),
//         })
//         .sum()
// }

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
                Block {
                    size: *block_size as usize,
                    id: index,
                },
                FreeSpace {
                    size: *free_space as usize,
                },
            ]
        })
        .filter(|disk| disk.size() > 0)
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

    #[test]
    fn test_unite_free_space() {
        let input = vec![
            Block { size: 2, id: 0 },
            FreeSpace { size: 3 },
            Block { size: 3, id: 1 },
            FreeSpace { size: 3 },
            Block { size: 1, id: 2 },
            FreeSpace { size: 3 },
            Block { size: 3, id: 3 },
            FreeSpace { size: 1 },
            Block { size: 2, id: 4 },
            FreeSpace { size: 1 },
            Block { size: 4, id: 5 },
            FreeSpace { size: 1 },
            Block { size: 4, id: 6 },
            FreeSpace { size: 1 },
            Block { size: 3, id: 7 },
            FreeSpace { size: 1 },
            Block { size: 4, id: 8 },
            Block { size: 2, id: 9 },
        ];

        let result = vec![
            Block { size: 2, id: 0 },
            Block { size: 2, id: 9 },
            Block { size: 1, id: 2 },
            Block { size: 3, id: 1 },
            Block { size: 3, id: 7 },
            FreeSpace { size: 1 },
            Block { size: 2, id: 4 },
            FreeSpace { size: 1 },
            Block { size: 3, id: 3 },
            FreeSpace { size: 4 },
            Block { size: 4, id: 5 },
            FreeSpace { size: 1 },
            Block { size: 4, id: 6 },
            FreeSpace { size: 5 },
            Block { size: 4, id: 8 },
            FreeSpace { size: 2 },
        ];

        assert_eq!(unite_free_space(&input), result);
    }
    //
    // #[test]
    // fn test_check_sum() {
    //     let s = parse_for_testing("0099811188827773336446555566..............");
    //     assert_eq!(check_sum(&s), 1928);
    // }
}
