use Direction::{Down, Left, Right, Up};
use std::cmp::PartialEq;
use std::fs;

const FILE_NAME: &str = "input_day6.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut world = WorldBuilder::build(&input);
    world.run(10_000);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    NotVisited,
    Visited(usize),
    Obstruction,
    InitialGuardPosition(Direction),
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::NotVisited => true,
            Cell::Visited(_) => true,
            Cell::InitialGuardPosition(_) => true,
            Cell::Obstruction => false,
        }
    }
    fn visit(&self) -> Cell {
        match self {
            Cell::InitialGuardPosition(_) => self.clone(),
            Cell::Visited(i) => Cell::Visited(i + 1),
            Cell::NotVisited => Cell::Visited(1),
            Cell::Obstruction => panic!("can't visit obstruction!"),
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn next_position(&self) -> Result<Position, String> {
        let mut next_position = self.position;
        match self.direction {
            Up => next_position.y += 1,
            Right => next_position.x += 1,
            Down => {
                if next_position.y == 0 {
                    return Err("Invalid position!".into());
                }
                next_position.y -= 1
            }
            Left => {
                if next_position.x == 0 {
                    return Err("Invalid position!".into());
                }
                next_position.x -= 1
            }
        }
        Ok(next_position)
    }

    fn walk(&mut self) {
        self.position = self.next_position().unwrap();
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
    Loop,
    GuardExited,
}

#[derive(Debug)]
struct World {
    map: Vec<Vec<Cell>>,
    guard: Guard,
    state: State,
}

impl World {
    fn run(&mut self, timeout: usize) {
        let mut frame = 0;

        while !self.is_done() && frame < timeout {
            self.next_frame();
            frame += 1;
        }
    }

    fn visit(&mut self, position: &Position) {
        self.map[position.y][position.x] = self.get_cell(position).unwrap().visit();
    }

    fn is_done(&self) -> bool {
        match self.state {
            State::Loop => true,
            State::GuardExited => true,
            State::NotDone => false,
        }
    }

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
        // println!("world: {:?}", self);

        if self.is_done() {
            return;
        }

        let Ok(next_position) = &self.guard.next_position() else {
            self.state = State::GuardExited;
            return;
        };

        match self.get_cell(next_position) {
            Err(_) => self.state = State::GuardExited,
            Ok(Cell::Visited(i)) if i > 4 => self.state = State::Loop,
            Ok(pos) => {
                if pos.is_empty() {
                    self.visit(&self.guard.next_position().unwrap());
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
    fn build(input_raw: &str) -> World {
        let input = input_raw.trim();
        let map = Self::build_map(input);
        let guard = Self::build_guard(&map);
        World {
            map,
            guard,
            state: State::NotDone,
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
                '#' | 'O' => Cell::Obstruction,
                _ => panic!("invalid character {c}!"),
            })
            .collect()
    }
    fn build_map(input: &str) -> Vec<Vec<Cell>> {
        input.lines().rev().map(WorldBuilder::build_line).collect()
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
        assert_eq!(world.guard.position, Position { x: 3, y: 2 });
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
        assert_eq!(next_pos, Ok(Position { x: 2, y: 3 }));
        assert_eq!(guard.position, Position { x: 2, y: 2 });
        guard.rotate();
        let next_pos = guard.next_position();
        assert_eq!(next_pos, Ok(Position { x: 3, y: 2 }));
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
        assert_eq!(world.state, State::GuardExited);
        assert_eq!(world.guard.position, Position { x: 3, y: 0 });
    }
    #[test]
    fn run_simulation_2() {
        let input = r">.#.";
        let mut world = WorldBuilder::build(input);
        world.run(5);
        assert_eq!(world.state, State::GuardExited);
        assert_eq!(world.guard.position, Position { x: 1, y: 0 });
    }
    #[test]
    fn run_simulation_3() {
        let input = r"
>.#.
....
.#..";
        let mut world = WorldBuilder::build(input);
        world.run(10);
        assert_eq!(world.state, State::GuardExited);
        assert_eq!(world.guard.position, Position { x: 0, y: 1 });
    }

    #[test]
    fn simulation_loop() {
        let input = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#.O^.....
........#.
#.........
......#...";
        let mut world = WorldBuilder::build(input);
        world.run(1000);
        assert_eq!(world.state, State::Loop)

    }
    #[test]
    fn simulation_loop_2() {
        let input = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#O..";
        let mut world = WorldBuilder::build(input);
        world.run(1000);
        assert_eq!(world.state, State::Loop)

    }
}
