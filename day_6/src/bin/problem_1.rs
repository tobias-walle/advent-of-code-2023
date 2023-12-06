use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, space0, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use utils::{
    parsing::{self, parse_with_nom},
    read_input_file_as_string,
};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<i32> {
    let problem = parse(input)?;
    let mut result = 1;
    for race in &problem.races {
        let mut wins = 0;
        for ms in 1..race.time_ms {
            let time_left = race.time_ms - ms;
            let distance = ms * time_left;
            if distance > race.min_distance_mm {
                wins += 1;
            }
        }
        result *= wins;
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct Problem {
    races: Vec<Race>,
}

#[derive(Debug, Clone)]
pub struct Race {
    time_ms: i32,
    min_distance_mm: i32,
}

fn parse(input: &str) -> Result<Problem> {
    let parse_times = preceded(
        tuple((tag("Time:"), space0)),
        separated_list1(space1, parsing::number),
    );
    let parse_distances = preceded(
        tuple((tag("Distance:"), space0)),
        separated_list1(space1, parsing::number),
    );
    let parse_problem = all_consuming(map(
        tuple((parse_times, multispace1, parse_distances)),
        |(times, _, distances)| Problem {
            races: times
                .into_iter()
                .zip(distances.into_iter())
                .map(|(time_ms, min_distance_mm)| Race {
                    time_ms,
                    min_distance_mm,
                })
                .collect(),
        },
    ));
    let problem = parse_with_nom(input.trim(), parse_problem)?;
    Ok(problem)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 288);
    }
}
