use crate::day9_part2::DiskSpace::{Block, FreeSpace};
use itertools::{Itertools, repeat_n};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs;
use std::iter::once;

const FILE_NAME: &str = "input_day9.txt";

#[derive(Debug, PartialEq, Clone)]
enum DiskSpace {
    FreeSpace {
        size: usize,
        starting_index: usize,
    },
    Block {
        size: usize,
        id: usize,
        starting_index: usize,
    },
}

impl DiskSpace {
    fn size(&self) -> usize {
        match self {
            FreeSpace { size, .. } => *size,
            Block { size, .. } => *size,
        }
    }
    fn starting_index(&self) -> usize {
        match self {
            FreeSpace { starting_index, .. } => *starting_index,
            Block { starting_index, .. } => *starting_index,
        }
    }
    fn ending_index(&self) -> usize {
        match self {
            FreeSpace {
                size,
                starting_index,
                ..
            } => *starting_index + size - 1,
            Block {
                size,
                starting_index,
                ..
            } => *starting_index + size - 1,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            FreeSpace { .. } => true,
            Block { .. } => false,
        }
    }

    fn check_sum(&self) -> usize {
        match self {
            FreeSpace { .. } => 0,
            Block {
                size,
                id,
                starting_index,
            } => (0..*size).map(|i| (i + starting_index) * id).sum(),
        }
    }
}
impl Display for DiskSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FreeSpace { size, .. } => {
                write!(f, "{}", ".".repeat(*size))
            }
            Block { size, id, .. } => {
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

struct FreeSpaceMap {
    map: Vec<bool>,
}

impl FreeSpaceMap {
    fn new(disk: &[DiskSpace]) -> FreeSpaceMap {
        let map = disk
            .iter()
            .flat_map(|disk_space| match disk_space {
                FreeSpace { size, .. } => repeat_n(true, *size),
                Block { size, .. } => repeat_n(false, *size),
            })
            .collect();
        FreeSpaceMap { map }
    }
    fn search_free_space(&self, size: usize) -> usize {
        let mut counter = 0;
        let mut index = 0;

        for i in &self.map {
            match i {
                false => {
                    counter = 0;
                }
                true => {
                    counter += 1;
                    if counter == size {
                        break;
                    }
                }
            }
            index += 1;
        }

        index
    }
    fn mark_as_block(&mut self, index: usize, size:usize){
        for i in 0..size{
            self.map[i] = false
        }
    }
}

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input_raw = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse_string(&input_raw);
    let disk_sorted = order_disk(&input);
    print!("{}", check_sum(&disk_sorted));
}

fn order_disk(disk: &[DiskSpace]) -> Vec<DiskSpace> {
    let free_space_mapping = FreeSpaceMap::new(disk);
    disk.iter().map(|disk_space: DiskSpace| {
        match disk_space {
            DiskSpace::FreeSpace { ..} => {}
            DiskSpace::Block { .. } => {
                disk_space.clone()
            }
        }
    })
    todo!()
}

// fn order_disk(disk: &[DiskSpace]) -> Vec<DiskSpace> {
//     let mut disk = disk.to_vec();
//
//     for block_idx in (0..disk.len()).rev() {
//         if disk[block_idx].is_empty() {
//             continue;
//         }
//         let empty_slot = (0..block_idx).find(|empty_slot| {
//             let source = &disk[block_idx];
//             let target = &disk[*empty_slot];
//             target.is_empty() && target.size() >= source.size()
//         });
//         if let Some(target) = empty_slot {
//             move_block(&mut disk, block_idx, target)
//         }
//     }
//     disk
// }

// fn combine_free_space(disk: &mut Vec<DiskSpace>, index: usize) {
//     let next_space = disk.get(index + 1);
//     if disk[index].is_empty() && next_space.is_some() && next_space.unwrap().is_empty() {
//         let combined_size = disk.remove(index + 1).size() + disk[index].size();
//         let _ = mem::replace(
//             &mut disk[index],
//             FreeSpace {
//                 size: combined_size,
//             },
//         );
//     }
// }

fn move_block(disk: &mut Vec<DiskSpace>, source: usize, target: usize) {
    todo!()
}
// }fn move_block(disk: &mut Vec<DiskSpace>, source: usize, target: usize) {
//     if !disk[target].is_empty() {
//         return;
//     }
//     let block_size = disk[source].size();
//     let free_size = disk[target].size();
//     match block_size.cmp(&disk[target].size()) {
//         Ordering::Less => {
//             let block = mem::replace(&mut disk[source], FreeSpace { size: block_size });
//
//             let _ = mem::replace(
//                 &mut disk[target],
//                 FreeSpace {
//                     size: free_size - block_size,
//                 },
//             );
//
//             disk.insert(target, block);
//             combine_free_space(disk, source);
//             combine_free_space(disk, source-1);
//             combine_free_space(disk, target+1);
//             combine_free_space(disk, target);
//
//         }
//         Ordering::Equal => {
//             disk.swap(source, target);
//         }
//         Ordering::Greater => {}
//     }
// }

fn check_sum(disk: &[DiskSpace]) -> usize {
    let mut index = 0;
    disk.iter()
        .map(|space| {
            let checksum = space.check_sum();
            index += space.size();
            checksum
        })
        .sum()
}

fn parse_string(s: &str) -> Vec<DiskSpace> {
    let mut result = Vec::new();
    let mut pos: usize = 0;
    let pairs: Vec<(i32, i32)> = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .chain(once(0)) // to avoid the last item being dropped if odd
        .tuples()
        .collect();

    for (id, (block_size, free_space)) in pairs.iter().enumerate() {
        let block_size = *block_size as usize;
        let free_size = *free_space as usize;

        if block_size > 0 {
            result.push(Block {
                size: block_size,
                id,
                starting_index: pos,
            });
            pos += block_size;
        }

        if free_size > 0 {
            result.push(FreeSpace {
                size: free_size,
                starting_index: pos,
            });
            pos += free_size;
        }
    }
    result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn format_disk(disk: &[DiskSpace]) -> String {
        disk.iter().map(|space| space.to_string()).collect()
    }

    #[test]
    fn test_parse_string() {
        let string = "143023";
        let parsed = vec![
            Block {
                size: 1,
                id: 0,
                starting_index: 0,
            },
            FreeSpace {
                size: 4,
                starting_index: 1,
            },
            Block {
                size: 3,
                id: 1,
                starting_index: 5,
            },
            Block {
                size: 2,
                id: 2,
                starting_index: 8,
            },
            FreeSpace {
                size: 3,
                starting_index: 10,
            },
        ];
        assert_eq!(parse_string(string), parsed);
    }

    #[test]
    fn test_order_disk() {
        let input = vec![
            Block {
                size: 2,
                id: 0,
                starting_index: 0,
            },
            FreeSpace {
                size: 3,
                starting_index: 2,
            },
            Block {
                size: 3,
                id: 1,
                starting_index: 5,
            },
            FreeSpace {
                size: 3,
                starting_index: 8,
            },
            Block {
                size: 1,
                id: 2,
                starting_index: 11,
            },
            FreeSpace {
                size: 3,
                starting_index: 12,
            },
            Block {
                size: 3,
                id: 3,
                starting_index: 15,
            },
            FreeSpace {
                size: 1,
                starting_index: 18,
            },
            Block {
                size: 2,
                id: 4,
                starting_index: 19,
            },
            FreeSpace {
                size: 1,
                starting_index: 21,
            },
            Block {
                size: 4,
                id: 5,
                starting_index: 22,
            },
            FreeSpace {
                size: 1,
                starting_index: 26,
            },
            Block {
                size: 4,
                id: 6,
                starting_index: 27,
            },
            FreeSpace {
                size: 1,
                starting_index: 31,
            },
            Block {
                size: 3,
                id: 7,
                starting_index: 32,
            },
            FreeSpace {
                size: 1,
                starting_index: 35,
            },
            Block {
                size: 4,
                id: 8,
                starting_index: 36,
            },
            Block {
                size: 2,
                id: 9,
                starting_index: 40,
            },
        ];

        let answer = vec![
            Block {
                size: 2,
                id: 0,
                starting_index: 0,
            },
            Block {
                size: 2,
                id: 9,
                starting_index: 2,
            },
            Block {
                size: 1,
                id: 2,
                starting_index: 4,
            },
            Block {
                size: 3,
                id: 1,
                starting_index: 5,
            },
            Block {
                size: 3,
                id: 7,
                starting_index: 8,
            },
            FreeSpace {
                size: 1,
                starting_index: 11,
            },
            Block {
                size: 2,
                id: 4,
                starting_index: 12,
            },
            FreeSpace {
                size: 1,
                starting_index: 14,
            },
            Block {
                size: 3,
                id: 3,
                starting_index: 15,
            },
            FreeSpace {
                size: 4,
                starting_index: 18,
            },
            Block {
                size: 4,
                id: 5,
                starting_index: 22,
            },
            FreeSpace {
                size: 1,
                starting_index: 26,
            },
            Block {
                size: 4,
                id: 6,
                starting_index: 27,
            },
            FreeSpace {
                size: 5,
                starting_index: 31,
            },
            Block {
                size: 4,
                id: 8,
                starting_index: 36,
            },
            FreeSpace {
                size: 2,
                starting_index: 40,
            },
        ];

        let result = format_disk(&order_disk(&input));

        println!("\nResult: {}", result);
        println!("Should be: {}", format_disk(&answer));

        assert_eq!(result, format_disk(&answer));
    }

    #[test]
    fn diskspace_check_sum() {
        let free = FreeSpace {
            size: 9,
            starting_index: 15,
        };
        assert_eq!(free.check_sum(), 0);

        let block = Block {
            id: 0,
            size: 1,
            starting_index: 15,
        };
        assert_eq!(block.check_sum(), 0);
        let block = Block {
            id: 2,
            size: 2,
            starting_index: 1,
        };
        assert_eq!(block.check_sum(), 6);
    }

    #[test]
    fn test_check_sum() {
        let input = vec![
            Block {
                size: 2,
                id: 0,
                starting_index: 0,
            },
            Block {
                size: 2,
                id: 9,
                starting_index: 2,
            },
            Block {
                size: 1,
                id: 2,
                starting_index: 4,
            },
            Block {
                size: 3,
                id: 1,
                starting_index: 5,
            },
            Block {
                size: 3,
                id: 7,
                starting_index: 8,
            },
            FreeSpace {
                size: 1,
                starting_index: 11,
            },
            Block {
                size: 2,
                id: 4,
                starting_index: 12,
            },
            FreeSpace {
                size: 1,
                starting_index: 14,
            },
            Block {
                size: 3,
                id: 3,
                starting_index: 15,
            },
            FreeSpace {
                size: 4,
                starting_index: 18,
            },
            Block {
                size: 4,
                id: 5,
                starting_index: 22,
            },
            FreeSpace {
                size: 1,
                starting_index: 26,
            },
            Block {
                size: 4,
                id: 6,
                starting_index: 27,
            },
            FreeSpace {
                size: 5,
                starting_index: 31,
            },
            Block {
                size: 4,
                id: 8,
                starting_index: 36,
            },
            FreeSpace {
                size: 2,
                starting_index: 40,
            },
        ];

        assert_eq!(check_sum(&input), 2858);
    }
}
