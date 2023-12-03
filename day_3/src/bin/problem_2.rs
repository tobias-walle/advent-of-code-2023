use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    result,
    str::FromStr,
};

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
    let mut gears = HashMap::new();
    for (row, line) in grid.lines.iter().enumerate() {
        let mut combined_digit = 0;
        let mut gear_neightbours = HashSet::new();
        for (col, _) in line.iter().enumerate() {
            let coord = Coord::new(row as i32, col as i32);
            let char = grid.get(coord).unwrap();
            if let Some(digit) = char.to_digit(10) {
                combined_digit = combined_digit * 10 + digit;
                for (n_coord, n_char) in grid.neighbours(coord) {
                    if n_char == '*' {
                        gear_neightbours.insert(n_coord);
                    }
                }
            } else {
                for gear in gear_neightbours {
                    gears.entry(gear).or_insert(Vec::new()).push(combined_digit)
                }
                combined_digit = 0;
                gear_neightbours = HashSet::new();
            }
        }
        for gear in gear_neightbours {
            gears.entry(gear).or_insert(Vec::new()).push(combined_digit)
        }
    }
    let mut result = 0;
    for values in gears.values() {
        if values.len() == 2 {
            result += values[0] * values[1];
        }
    }
    Ok(result)
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

    fn neighbours(&self, coord: Coord) -> impl Iterator<Item = (Coord, char)> {
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
        .flat_map(|coord| self.get(coord).map(|char| (coord, *char)))
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Sub, Add, Mul, Div)]
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
        assert_eq!(result, 467835);
    }
}
