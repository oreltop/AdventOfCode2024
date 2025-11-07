use std::fs;

const FILE_NAME: &str = "input_day1.txt";

#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct Position{
    x: usize,
    y: usize
}

enum Cell{
    Empty,
    Obstruction,
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            Cell::Obstruction => false,
        }
    }
}

struct Guard{
    position: Position,
    direction: Direction
}

struct World{
    size: (usize, usize),
    map: Vec<Vec<Cell>>,
    frame: usize,
    guard: Guard
}
impl World {
    fn from(input: &str) -> World{
        let input_lines = input.trim().lines();

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

fn parse_string(input: &str) -> Vec<i32> {
    let mut column1: Vec<i32> = Vec::new();
    column1
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn new_from_input(){
        let input = r"
...^
####
....
....";
       let world = World::from(input);
        assert_eq!(world.size, (4,4));
        assert_eq!(world.guard.position.x,0);
        assert_eq!(world.guard.position.y,3);
        assert_eq!(world.guard.direction, Direction::Up);
        assert!(!world.map[1][3].is_empty());
        assert!(world.map[0][3].is_empty());

    }
}
