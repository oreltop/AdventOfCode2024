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
fn get_value(x:usize, y:usize)->u32{
    get_grid()[y][x]
}

struct Cell{
    x: usize,
    y: usize,
    value: u32
}

impl Cell{
    fn new(x: usize, y:usize) -> Cell{
        Cell{x,y, value:get_value(x,y)}
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn new_cell() {
        let cell = Cell::new(0,0);
        assert_eq!(cell.value,5);
        let cell = Cell::new(1,0);
        assert_eq!(cell.value,6);
        let cell = Cell::new(0,1);
        assert_eq!(cell.value,4);
        let cell = Cell::new(1,1);
        assert_eq!(cell.value,1);
    }
}
