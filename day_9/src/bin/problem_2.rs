use anyhow::{Context, Result};
use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<i64> {
    let histories = parse(input);
    let result = histories
        .iter()
        .map(|h| compute_result_for_history(h))
        .sum();
    Ok(result)
}

fn compute_result_for_history(history: &[i64]) -> i64 {
    let mut steps = vec![history.to_vec()];
    loop {
        let last = steps.last().unwrap();
        let pairs = last.iter().zip(last.iter().skip(1));
        let step: Vec<_> = pairs.map(|(a, b)| b - a).collect();
        let all_zeros = step.iter().all(|n| *n == 0);
        steps.push(step);
        if all_zeros {
            break;
        }
    }
    let result = steps.iter().rev().fold(0, |result, step| step[0] - result);
    result
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .split(' ')
                .map(|n| n.parse().expect("Invalid number"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 2);
    }
}
