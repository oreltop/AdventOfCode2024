use crate::day6::Direction::{Down, Left, Right, Up};
use std::cmp::PartialEq;
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

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    NotVisited,
    Visited,
    Obstruction,
    InitialGuardPosition(Direction),
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::NotVisited => true,
            Cell::Visited => true,
            Cell::InitialGuardPosition(_) => true,
            Cell::Obstruction => false,
        }
    }
    fn is_visited(&self) -> bool {
        match self {
            Cell::Visited => true,
            Cell::InitialGuardPosition(_) => true,
            Cell::NotVisited => false,
            Cell::Obstruction => false,
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn next_position(&self) -> Position {
        let mut next_position = self.position.clone();
        match self.direction {
            Up => next_position.y += 1,
            Right => next_position.x += 1,
            Down => next_position.y -= 1,
            Left => next_position.x -= 1,
        }
        next_position
    }

    fn walk(&mut self) {
        self.position = self.next_position();
    }

    fn rotate(&mut self) {
        let new_direction = match self.direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
        self.direction = new_direction;
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum State {
    NotDone,
    Done,
}
#[derive(Debug)]
struct World {
    size: Size,
    map: Vec<Vec<Cell>>,
    guard: Guard,
    state: State,
}

impl World {
    fn run(&mut self, timeout: usize) {
        let mut step = 0;

        while self.state != State::Done && step < timeout {
            self.next_frame();
            step += 1;
        }
    }

    fn visit(&mut self, position: &Position) {
        self.map[position.y][position.x] = Cell::Visited;
    }
    fn count_visited_cells(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|&cell| cell.is_visited()).count())
            .sum()
    }

    fn is_done(&self) -> bool {
        self.state == State::Done
    }
}

impl World {
    fn get_cell(&self, position: &Position) -> Result<Cell, String> {
        if position.y >= self.map.len() {
            return Err("Row index out of bounds".into());
        }
        if position.x >= self.map[position.y].len() {
            return Err("Column index out of bounds".into());
        }
        Ok(self.map[position.y][position.x])
    }
    fn next_frame(&mut self) {
        println!("world: {:?}", self);

        if self.state == State::Done {
            return;
        }
        match self.get_cell(&self.guard.next_position()) {
            Err(_) => self.state = State::Done,
            Ok(pos) => {
                if pos.is_empty() {
                    self.visit(&self.guard.next_position());
                    self.guard.walk()
                } else {
                    self.guard.rotate()
                }
            }
        }
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
            guard,
            state: State::NotDone,
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
                '.' => Cell::NotVisited,
                '^' => Cell::InitialGuardPosition(Up),
                '>' => Cell::InitialGuardPosition(Right),
                '<' => Cell::InitialGuardPosition(Left),
                'V' => Cell::InitialGuardPosition(Down),
                '#' => Cell::Obstruction,
                _ => panic!("invalid character {c}!"),
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
        let (y, x) = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, cell)| matches!(cell, Cell::InitialGuardPosition(_)))
                    .map(|(x, _)| (y, x))
            })
            .unwrap();
        let Cell::InitialGuardPosition(direction) = map[y][x] else {
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
        assert_eq!(world.guard.position, Position { x: 3, y: 0 });
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
    fn test_guard_walk() {
        let mut guard = Guard {
            position: Position { x: 2, y: 2 },
            direction: Up,
        };
        guard.walk();
        assert_eq!(guard.position, Position { x: 2, y: 3 });
        guard.rotate();
        guard.walk();
        guard.walk();
        assert_eq!(guard.position, Position { x: 4, y: 3 });
    }
    #[test]
    fn test_next_position() {
        let mut guard = Guard {
            position: Position { x: 2, y: 2 },
            direction: Up,
        };
        let next_pos = guard.next_position();
        assert_eq!(next_pos, Position { x: 2, y: 3 });
        assert_eq!(guard.position, Position { x: 2, y: 2 });
        guard.rotate();
        let next_pos = guard.next_position();
        assert_eq!(next_pos, Position { x: 3, y: 2 });
        assert_eq!(guard.position, Position { x: 2, y: 2 });
    }
    #[test]
    fn next_frame_walk() {
        let input = r"..>.";
        let mut world = WorldBuilder::build(input);
        assert_eq!(world.guard.position, Position { x: 2, y: 0 });

        world.next_frame();

        assert_eq!(world.guard.position, Position { x: 3, y: 0 });
    }
    #[test]
    fn next_frame_rotate() {
        let input = r"..>#";
        let mut world = WorldBuilder::build(input);
        assert_eq!(world.guard.position, Position { x: 2, y: 0 });
        assert_eq!(world.guard.direction, Right);

        world.next_frame();

        assert_eq!(world.guard.position, Position { x: 2, y: 0 });
        assert_eq!(world.guard.direction, Down);
    }

    #[test]
    fn test_done() {
        let input = r"...>";
        let mut world = WorldBuilder::build(input);
        assert!(!world.is_done());
        world.next_frame();
        assert!(world.is_done());
    }

    #[test]
    fn run_simulation() {
        let input = r">...";
        let mut world = WorldBuilder::build(input);
        world.run(5);
        assert_eq!(world.state, State::Done);
        assert_eq!(world.guard.position, Position { x: 3, y: 0 });
        assert_eq!(world.count_visited_cells(), 4);
    }
    #[test]
    fn run_simulation2() {
        let input = r">.#.";
        let mut world = WorldBuilder::build(input);
        world.run(5);
        assert_eq!(world.state, State::Done);
        assert_eq!(world.guard.position, Position { x: 1, y: 0 });
        assert_eq!(world.count_visited_cells(), 2);
    }
    #[test]
    fn run_simulation3() {
        let input = r"
>.#.
....
.#..";
        let mut world = WorldBuilder::build(input);
        world.run(5);
        assert_eq!(world.state, State::Done);
        assert_eq!(world.guard.position, Position { x: 0, y: 1 });
        assert_eq!(world.count_visited_cells(), 4);
    }
}
