use crate::day9_part2::DiskSpace::{Block, FreeSpace};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs;
use std::iter::once;

const FILE_NAME: &str = "input_day9.txt";

#[derive(Debug, PartialEq, Clone)]
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
impl Display for DiskSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskSpace::FreeSpace { size } => {
                write!(f, "{}", ".".repeat(*size))
            }
            DiskSpace::Block { size, id } => {
                write!(f, "{}", format!("{}", id).repeat(*size))
            }
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
    let mut disk = disk.to_vec();

    for block_idx in (0..disk.len()).rev() {
        if disk[block_idx].is_empty() {
            continue;
        }
        let empty_slot = (0..block_idx).find(|empty_slot| {
            let source = &disk[block_idx];
            let target = &disk[*empty_slot];
            target.is_empty() && target.size() >= source.size()
        });
        if let Some(target) = empty_slot {
            move_block(&mut disk, block_idx, target)
        }
    }
    disk
}

fn format_disk(disk: &[DiskSpace]) -> String {
    disk.iter().map(|space| space.to_string()).collect()
}

fn move_block(disk: &mut Vec<DiskSpace>, source: usize, target: usize) {
    if !disk[target].is_empty() {
        return;
    }
    match disk[source].size().cmp(&disk[target].size()) {
        Ordering::Less => {
            let block = disk.remove(source);
            disk.insert(source, FreeSpace { size: block.size() });
            let free_space = disk.remove(target);
            disk.insert(
                target,
                FreeSpace {
                    size: free_space.size() - block.size(),
                },
            );
            disk.insert(target, block);
        }
        Ordering::Equal => {
            disk.swap(source, target);
        }
        Ordering::Greater => {}
    }
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

        let answer = vec![
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

        let result = format_disk(&unite_free_space(&input));

        println!("\nResult: {}", result);
        println!("Should be: {}", format_disk(&answer));

        assert_eq!(result, format_disk(&answer));
    }
}
