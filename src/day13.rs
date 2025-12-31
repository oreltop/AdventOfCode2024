use itertools::Itertools;
use nalgebra::{ComplexField, Matrix2, Vector2};
use std::fs;

const FILE_NAME: &str = "input_day13.txt";

pub fn main() {
    println!("this is main");
    let file_path = format!("artifacts/input_files/{}", FILE_NAME);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let parsed = parse_string(&input);
}

#[derive(PartialEq, Debug)]
enum NumberOfSolutions {
    None,
    One,
    Infinity,
}

impl NumberOfSolutions {
    fn calculate(matrix: &Matrix2<f32>, rhs: &Vector2<f32>) -> NumberOfSolutions{
        let option_solution = matrix.clone().lu().solve(rhs);
        match option_solution {
            None => {
                let cramer_matrix =
                    Matrix2::from_columns(&[matrix.column(0), rhs.column(0)]);
                if cramer_matrix.determinant().abs() < 1e-7 {
                    NumberOfSolutions::Infinity
                } else {
                    NumberOfSolutions::None
                }
            }
            Some(solution) => {
                if ClawMachine::is_vector_natural(solution) {
                    NumberOfSolutions::One
                } else {
                    NumberOfSolutions::None
                }
            }
        }
    }
}

struct ClawMachine {
    movement_matrix: Matrix2<f32>,
    target: Vector2<f32>,
    number_of_solutions: NumberOfSolutions
}

impl ClawMachine {
    fn new(a_movement: (f32, f32), b_movement: (f32, f32), target: (f32, f32)) -> ClawMachine {
        let movement_matrix = Matrix2::from_row_slice(&[
            a_movement.0,
            b_movement.0,
            a_movement.1,
            b_movement.1,
        ]);
        let target = Vector2::from_row_slice(&[target.0, target.1]);

        ClawMachine {
            movement_matrix,
            target,
            number_of_solutions: NumberOfSolutions::calculate(&movement_matrix,&target)
        }
    }

    fn from(input: &str) -> ClawMachine {
        let mut input_lines = input.trim().lines();
        let a_movement = Self::parse_line(input_lines.next(), "Button A: X+{d}, Y+{d}");
        let b_movement = Self::parse_line(input_lines.next(), "Button B: X+{d}, Y+{d}");
        let target = Self::parse_line(input_lines.next(), "Prize: X={d}, Y={d}");

        ClawMachine::new(a_movement, b_movement, target)
    }

    fn parse_line(line: Option<&str>, pattern: &str) -> (f32, f32) {
        let s = line.expect("missing line");
        let mut p = pattern.split("{d}");
        let (pre, mid) = (p.next().unwrap(), p.next().unwrap());

        let (n1, n2) = s
            .trim()
            .strip_prefix(pre)
            .and_then(|s| s.split_once(mid))
            .expect("pattern mismatch");
        (
            n1.parse().expect("invalid f32"),
            n2.parse().expect("invalid f32"),
        )
    }

    fn number_of_solutions(&self) -> NumberOfSolutions {
todo!()
    }

    fn is_vector_natural(v: Vector2<f32>) -> bool {
        v.iter()
            .all(|element| element.fract() == 0.0 && element >= &0.0)
    }
}

fn parse_string(input: &str) -> Vec<ClawMachine> {
    input
        .split("\r\n\r\n")
        .map(|s| ClawMachine::from(s))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn claw_machine_from_str() {
        let s = r"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400";

        let machine = ClawMachine::from(s);

        assert_eq!(machine.movement_matrix[(0, 0)], 94.0);
        assert_eq!(machine.movement_matrix[(0, 1)], 22.0);
        assert_eq!(machine.movement_matrix[(1, 0)], 34.0);
        assert_eq!(machine.movement_matrix[(1, 1)], 67.0);
        assert_eq!(machine.target[0], 8400.0);
        assert_eq!(machine.target[1], 5400.0);
    }

    #[test]
    fn parse_string_test() {
        let file_path = format!("artifacts/input_files/{}", FILE_NAME);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let parsed = parse_string(&input);
        assert_eq!(parsed.len(), 320);
    }
    #[test]
    fn single_solution() {
        let s = r"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400";

        let machine = ClawMachine::from(s);

        assert_eq!(machine.number_of_solutions, NumberOfSolutions::One)
    }
}
