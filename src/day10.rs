use std::collections::HashSet;
use std::fs;
use std::sync::OnceLock;
static GRID: OnceLock<Vec<Vec<u32>>> = OnceLock::new();
const FILE_NAME: &str = "input_day10.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input_raw = fs::read_to_string(file_path).expect("Should have been able to read the file");
    init_grid(&input_raw);
    let mut probes = Probe::generate_probes(get_grid());
    let result: usize = probes
        .iter_mut()
        .map(|probe| {
            probe.solve();
            probe.count_trailheads()
        })
        .sum();
    println!("{}", result)
}

fn init_grid(input: &str) {
    GRID.get_or_init(|| {
        input
            .split_whitespace()
            .map(|str| str.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    });
}

fn get_grid() -> &'static Vec<Vec<u32>> {
    GRID.get().expect("get grid before init!!")
}
fn get_value(x: usize, y: usize) -> Option<u32> {
    if y >= get_grid().len() || x >= get_grid()[0].len() {
        return None;
    }
    Some(get_grid()[y][x])
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cell {
    x: usize,
    y: usize,
    value: u32,
}

impl Cell {
    fn try_new(x: usize, y: usize) -> Option<Cell> {
        get_value(x, y).map(|value| Cell { x, y, value })
    }
    fn search_neighbors(&self, value: u32) -> HashSet<Cell> {
        HashSet::from([
            Cell::try_new(self.x.saturating_sub(1), self.y),
            Cell::try_new(self.x, self.y.saturating_sub(1)),
            Cell::try_new(self.x + 1, self.y),
            Cell::try_new(self.x, self.y + 1),
        ])
        .into_iter()
        .flatten()
        .filter(|cell| cell.value == value)
        .collect()
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Probe {
    status: Status,
    cells: HashSet<Cell>,
}
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Pending,
    Running,
    Ended,
    Error,
}
impl Probe {
    fn generate_probes(grid: &[Vec<u32>]) -> Vec<Probe> {
        (0..grid.len())
            .flat_map(|y| {
                grid[y]
                    .iter()
                    .enumerate()
                    .filter(|(_, value)| **value == 0)
                    .map(move |(x, _)| Probe::new(x, y))
            })
            .collect()
    }

    fn new(x: usize, y: usize) -> Probe {
        let init_cell = Cell::try_new(x, y).unwrap();
        let status = match init_cell.value {
            0 => Status::Pending,
            _ => Status::Error,
        };
        Probe {
            status,
            cells: HashSet::from([init_cell]),
        }
    }

    fn solve(&mut self) {
        self.status = Status::Running;
        for step in 1..=9 {
            self.cells = self.search_all_neighbors(step)
        }
        self.status = Status::Ended;
    }

    fn search_all_neighbors(&self, value: u32) -> HashSet<Cell> {
        self.cells
            .iter()
            .flat_map(|cell| cell.search_neighbors(value))
            .collect()
    }

    fn count_trailheads(&self) -> usize {
        match self.status {
            Status::Ended => self.cells.len(),
            _ => panic!("status isn't ended!"),
        }
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
        let expected = HashSet::from([Cell::try_new(1, 0).unwrap()]);
        assert_eq!(result, expected);
        let cell = Cell::try_new(0, 0).unwrap();
        let result = cell.search_neighbors(1);
        let expected = HashSet::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn run_prob_no_split() {
        let mut prob = Probe::new(12, 7);
        prob.solve();
        assert_eq!(prob.count_trailheads(), 1);
    }
    #[test]
    fn run_prob_split_once() {
        let mut prob = Probe::new(0, 10);
        prob.solve();
        assert_eq!(prob.count_trailheads(), 1);
        let mut prob = Probe::new(34, 0);
        prob.solve();
        assert_eq!(prob.count_trailheads(), 1);
    }
    #[test]
    fn run_prob_split_multiple() {
        let mut prob = Probe::new(16, 0);
        prob.solve();
        assert_eq!(prob.count_trailheads(), 3);
    }
}
