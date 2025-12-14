use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::BufRead;
use std::ops::Sub;

const FILE_NAME: &str = "input_day12.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("input: {:?}", input);
    // let parsed = parse_string(&input);
    // println!("input parsed: {:?}", &parsed);
}

#[derive(Hash, Eq, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
    crop: char,
    region: Option<u64>,
}

struct Region {
    crop: char,
    id: u64,
    cells: HashSet<Cell>,
}

struct Grid {
    data: Vec<Vec<Cell>>,
    shape: (usize, usize),
}

impl Grid {
    fn from(input: &str) -> Grid {
        let data: Vec<_> = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(x, crop)| Cell {
                        x,
                        y,
                        crop,
                        region: None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let shape = (data[0].len(), data.len());
        Grid { data, shape }
    }
    fn iter(&self) -> impl Iterator {
        self.data.iter().flatten()
    }

    fn iter_mut(&mut self) -> impl Iterator {
        self.data.iter_mut().flatten()
    }
    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.shape.0 || y >= self.shape.1 {
            None
        } else {
            Some(&self.data[y][x])
        }
    }
    fn get_identical_neighbors(&self, cell: &Cell) -> HashSet<&Cell> {
        let candidates = [
            (cell.x, cell.y + 1),
            (cell.x + 1, cell.y),
            (cell.x.saturating_sub(1), cell.y),
            (cell.x, cell.y.saturating_sub(1)),
        ];
        candidates
            .into_iter()
            .filter_map(|(x, y)| self.get_cell(x, y))
            .filter(|&c| c != cell)
            .filter(|&c| c.crop == cell.crop)
            .collect()
    }

    fn bfs_region<'a>(&'a self, init_cell: &'a Cell) -> HashSet<&'a Cell> {
        let mut result = HashSet::new();
        let mut new_cells = HashSet::from([init_cell]);
        while true {
            if new_cells.sub(&result).is_empty() {
                break;
            }

            let mut discovered_cells = HashSet::new();
            for cell in &new_cells {
                discovered_cells.extend(self.get_identical_neighbors(cell));
            }
            result.extend(&new_cells);
            new_cells.extend(discovered_cells)
        }

        result
    }

    fn calculate_regions(&mut self) -> HashSet<Region> {
        let result = HashSet::new();
        for cell in self.iter_mut() {}

        result
    }
}

fn parse_string(input: &str) -> HashSet<Region> {
    let grid = Grid::from(input);

    todo!()
}
fn find_neighbors(grid: &[Vec<Cell>], cell: Cell) -> HashSet<Cell> {
    todo!()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn grid_find_region() {
        let input = r"
            AAAA
            BBCD
            BBCC
            EEEC";
        let grid = Grid::from(input);
        let cell = grid.get_cell(0, 0).unwrap();
        let bfs_result = grid.bfs_region(cell);
        assert_eq!(bfs_result.len(), 4);

        let cell = grid.get_cell(1, 1).unwrap();
        let bfs_result = grid.bfs_region(cell);
        assert_eq!(bfs_result.len(), 4);

        let cell = grid.get_cell(3, 1).unwrap();
        let bfs_result = grid.bfs_region(cell);
        assert_eq!(bfs_result.len(), 1);

        let cell = grid.get_cell(1, 3).unwrap();
        let bfs_result = grid.bfs_region(cell);
        assert_eq!(bfs_result.len(), 3);
    }

    #[test]
    fn test_parse_string() {
        let input = r"
            AAAA
            BBCD
            BBCC
            EEEC";
        let result = parse_string(input);
        assert_eq!(result.len(), 4);
        let input = r"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO";
        let result = parse_string(input);
        assert_eq!(result.len(), 5);
        let input = r"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE";
        let result = parse_string(input);
        assert_eq!(result.len(), 11);
    }
}
