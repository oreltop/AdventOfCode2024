use crate::day6::Direction::{Down, Left, Right, Up};
use std::fs;

const FILE_NAME: &str = "input_day1.txt";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
struct Size {
    rows: usize,
    columns: usize,
}
#[derive(Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Debug, PartialEq)]

enum Cell {
    Empty,
    Obstruction,
    InitialGuardPosition(Direction),
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            Cell::InitialGuardPosition(_) => true,
            Cell::Obstruction => false,
        }
    }
}

struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn rotate(&mut self) {
        let new_direction = match self.direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        };
        self.direction = new_direction;
    }
}

struct World {
    size: Size,
    map: Vec<Vec<Cell>>,
    frame: usize,
    guard: Guard,
}
impl World {
    fn from(input: &str) -> World {
        let input_lines: Vec<&str> = input.trim().lines().collect();
        let size = (input_lines.len(), input_lines[0].len());
        // let map: = input_lines.iter().map(
        //     // |line| line.ma
        // )
        todo!()
    }
}

struct WorldBuilder();
impl WorldBuilder {
    fn build(input: &str) -> World {
        let size = Self::get_size(input);
        let map = Self::build_map(input);
        let guard = Self::build_guard(&map);
        World {
            size,
            map,
            frame: 0,
            guard,
        }
    }

    fn get_size(input: &str) -> Size {
        let input_lines: Vec<&str> = input.lines().collect();
        Size {
            rows: input_lines.len(),
            columns: input_lines[0].len(),
        }
    }

    fn build_line(line: &str) -> Vec<Cell> {
        line.chars()
            .map(|c| match c {
                '.' => Cell::Empty,
                '^' => Cell::InitialGuardPosition(Up),
                '>' => Cell::InitialGuardPosition(Right),
                '<' => Cell::InitialGuardPosition(Left),
                'V' => Cell::InitialGuardPosition(Down),
                '#' => Cell::Obstruction,
                _ => panic!("invalid character!"),
            })
            .collect()
    }
    fn build_map(input: &str) -> Vec<Vec<Cell>> {
        input
            .lines()
            .map(|line| WorldBuilder::build_line(line))
            .collect()
    }
    fn build_guard(map: &[Vec<Cell>]) -> Guard {
        let (x, y) = map
            .iter()
            .enumerate()
            .find_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .find(|(y, cell)| matches!(cell, Cell::InitialGuardPosition(_)))
                    .map(|(y, _)| (x, y))
            })
            .unwrap();
        let Cell::InitialGuardPosition(direction) = map[x][y] else {
            panic!("this shouldn't happen")
        };

        Guard {
            position: Position { x, y },
            direction,
        }
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

fn parse_string(input: &str) -> Vec<i32> {
    let mut column1: Vec<i32> = Vec::new();
    column1
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn new_from_input() {
        let input = r"
...^
####
....
";
        let world = WorldBuilder::build(input.trim());
        assert_eq!(
            world.size,
            Size {
                rows: 3,
                columns: 4
            }
        );
        assert_eq!(world.guard.position, Position { x: 0, y: 3 });
        assert_eq!(world.guard.direction, Up);
        assert!(!world.map[1][3].is_empty());
        assert!(world.map[0][3].is_empty());
    }

    #[test]
    fn test_guard_rotate() {
        let mut guard = Guard {
            position: Position { x: 0, y: 0 },
            direction: Up,
        };
        guard.rotate();
        assert_eq!(guard.direction, Right);
        guard.rotate();
        assert_eq!(guard.direction, Down);
        guard.rotate();
        assert_eq!(guard.direction, Left);
        guard.rotate();
        assert_eq!(guard.direction, Up);
    }

    #[test]
    fn test_guard_walk(){
        let mut guard = Guard{position: Position{x:2,y:2}, direction: Up};
        guard.walk();
        assert_eq!(guard.position, Position{x:2,y:3});
        guard.rotate();
        guard.walk();
        guard.walk();
        assert_eq!(guard.position, Position{x:4,y:3});

    }
}
