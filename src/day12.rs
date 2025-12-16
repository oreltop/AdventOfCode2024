use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::ops::Deref;
use itertools::Itertools;

const FILE_NAME: &str = "input_day12.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("input: {:?}", input);
    // let parsed = parse_string(&input);
    // println!("input parsed: {:?}", &parsed);
}

fn calculate_fence_cost(regions: &[Region]) -> u32{
    regions.iter().map(|region: &Region| {region.fence_cost()}).sum()
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Cell {
    x: usize,
    y: usize,
    crop: char,
}
impl Cell {
    fn distance(&self, other: &Cell) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Eq, PartialEq)]
struct Region {
    crop: char,
    cells: HashSet<Cell>,
}

impl Region {
    fn from(cells: HashSet<Cell>) -> Region {
        Region {
            crop: cells.iter().next().unwrap().crop,
            cells,
        }
    }
    fn area(&self) -> u32 {
        self.cells.len() as u32
    }
    fn perimeter(&self) -> u32 {
        let cells_diameter = self.cells.len() * 4;
        let edges_to_substruct = self.cells.iter().cartesian_product(self.cells.iter())
            .filter(|(cell1, cell2)| {cell1.distance(cell2)==1}).count();
        (cells_diameter - edges_to_substruct) as u32
    }

    fn fence_cost(&self) -> u32{
        self.area() * self.perimeter()    }
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
                    .map(move |(x, crop)| Cell { x, y, crop })
                    .collect::<Vec<_>>()
            })
            .collect();
        let shape = (data[0].len(), data.len());
        Grid { data, shape }
    }
    fn iter(&self) -> impl Iterator<Item=&Cell> {
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

    fn bfs<'a>(&'a self, start: &'a Cell) -> HashSet<&'a Cell> {
        let mut visited: HashSet<&Cell> = HashSet::from([start]);
        let mut queue = VecDeque::from([start]);

        while let Some(cell) = queue.pop_front() {
            for neighbor in self.get_identical_neighbors(cell) {
                if visited.insert(&*neighbor) {
                    queue.push_back(neighbor)
                }
            }
        }
        visited
    }

    fn calculate_regions(&mut self) -> Vec<Region> {
        let mut result = Vec::new();
        let mut visited: HashSet<&Cell> = HashSet::new();
        for cell in self.iter() {
            if visited.contains(&cell) {
                continue;
            } else {
                let cells_in_region = self.bfs(cell);
                visited.extend(&cells_in_region);
                result.push(Region::from(cells_in_region.into_iter().cloned().collect()))
            }
        }

        result
    }
}

fn parse_string(input: &str) -> Vec<Region> {
    let mut grid = Grid::from(input);
    grid.calculate_regions()
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
        let bfs_result = grid.bfs(cell);
        assert_eq!(bfs_result.len(), 4);

        let cell = grid.get_cell(1, 1).unwrap();
        let bfs_result = grid.bfs(cell);
        assert_eq!(bfs_result.len(), 4);

        let cell = grid.get_cell(3, 1).unwrap();
        let bfs_result = grid.bfs(cell);
        assert_eq!(bfs_result.len(), 1);

        let cell = grid.get_cell(1, 3).unwrap();
        let bfs_result = grid.bfs(cell);
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
        assert_eq!(result.len(), 5);
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

    #[test]
    fn fence_cost(){
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
        let regions = parse_string(input);
        let cost = calculate_fence_cost(&regions);
        assert_eq!(cost, 1930);

    }
}
