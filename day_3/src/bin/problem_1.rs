use std::{convert::Infallible, result, str::FromStr};

use anyhow::{Context, Result};
use derive_more::{Add, Div, Mul, Sub};
use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<u32> {
    let grid: Grid = input.parse().unwrap();
    let mut sum = 0;
    let mut add_to_sum_if_needed = |digit_has_symbol_as_neighbour: &mut bool,
                                    combined_digit: &mut u32| {
        if *digit_has_symbol_as_neighbour {
            sum += *combined_digit;
        }
        *combined_digit = 0;
        *digit_has_symbol_as_neighbour = false;
    };
    for (row, line) in grid.lines.iter().enumerate() {
        let mut combined_digit = 0;
        let mut digit_has_symbol_as_neighbour = false;
        for (col, _) in line.iter().enumerate() {
            let coord = Coord::new(row as i32, col as i32);
            let char = grid.get(coord).unwrap();
            if let Some(digit) = char.to_digit(10) {
                combined_digit = combined_digit * 10 + digit;
                digit_has_symbol_as_neighbour |= grid
                    .neighbours(coord)
                    .any(|c| c != '.' && !c.is_ascii_digit());
            } else {
                add_to_sum_if_needed(&mut digit_has_symbol_as_neighbour, &mut combined_digit)
            }
        }
        add_to_sum_if_needed(&mut digit_has_symbol_as_neighbour, &mut combined_digit)
    }
    Ok(sum)
}

#[derive(Debug, Clone)]
struct Grid {
    lines: Vec<Vec<char>>,
}

impl Grid {
    fn new(lines: Vec<Vec<char>>) -> Self {
        Self { lines }
    }

    fn get(&self, coord: Coord) -> Option<&char> {
        let row: usize = coord.row.try_into().ok()?;
        let col: usize = coord.col.try_into().ok()?;
        self.lines.get(row)?.get(col)
    }

    fn neighbours(&self, coord: Coord) -> impl Iterator<Item = char> {
        [
            coord + Coord::new(-1, -1), // Up/Left
            coord + Coord::new(-1, 0),  // Up
            coord + Coord::new(-1, 1),  // Up/Right
            coord + Coord::new(0, -1),  // Left
            coord + Coord::new(0, 1),   // Right
            coord + Coord::new(1, -1),  // Down/Left
            coord + Coord::new(1, 0),   // Down
            coord + Coord::new(1, 1),   // Down/Right
        ]
        .into_iter()
        .flat_map(|c| self.get(c))
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(Self::new(
            s.trim().lines().map(|l| l.chars().collect()).collect(),
        ))
    }
}

#[derive(Debug, Clone, Copy, Sub, Add, Mul, Div)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 4361);
    }
}
