use anyhow::{Context, Result};
use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<u32> {
    let mut result: u32 = 0;
    for line in input.trim().split('\n') {
        let mut numbers = Vec::<u32>::new();
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                numbers.push(digit);
            }
        }
        let first = numbers.first().context("No first number")?;
        let last = numbers.last().context("No last number")?;
        result += first * 10 + last;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 142);
    }
}
