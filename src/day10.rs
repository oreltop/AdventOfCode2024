use std::sync::OnceLock;
const INPUT: &str = include_str!("../artifacts/input_files/input_day10.txt");
static GRID: OnceLock<Vec<Vec<u32>>> = OnceLock::new();

pub fn main() {
    println!("this is main");
    let grid = get_grid();
}

fn get_grid() -> &'static Vec<Vec<u32>> {
    GRID.get_or_init(|| {
        INPUT
            .split_whitespace()
            .map(|str| str.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    })
}
fn get_value(x: usize, y: usize) -> Option<u32> {
    if y > get_grid().len() || x > get_grid()[0].len() {
        return None;
    }
    Some(get_grid()[y][x])
}

#[derive(Debug, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
    value: u32,
}

impl Cell {
    fn try_new(x: usize, y: usize) -> Option<Cell> {
        if let Some(value) = get_value(x, y) {
            Some(Cell { x, y, value })
        } else {
            None
        }
    }
    fn search_neighbors(&self, value: u32) -> Vec<Cell> {
        vec![
            Cell::try_new(self.x.saturating_sub(1), self.y),
            Cell::try_new(self.x, self.y.saturating_sub(1)),
            Cell::try_new(self.x + 1, self.y),
            Cell::try_new(self.x, self.y + 1),
        ]
        .into_iter()
        .filter_map(|x| x)
        .filter(|cell| cell.value == value)
        .collect()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn new_cell() {
        let cell = Cell::try_new(0, 0);
        assert_eq!(cell.unwrap().value, 5);
        let cell = Cell::try_new(1, 0);
        assert_eq!(cell.unwrap().value, 6);
        let cell = Cell::try_new(0, 1);
        assert_eq!(cell.unwrap().value, 4);
        let cell = Cell::try_new(1, 1);
        assert_eq!(cell.unwrap().value, 1);
    }

    #[test]
    fn test_search_neighbors() {
        let cell = Cell::try_new(0, 0).unwrap();
        let result = cell.search_neighbors(6);
        let expected = vec![Cell::try_new(1, 0).unwrap()];
        assert_eq!(result, expected);
        let cell = Cell::try_new(0, 0).unwrap();
        let result = cell.search_neighbors(1);
        let expected = Vec::new();
        assert_eq!(result, expected);
    }
}
