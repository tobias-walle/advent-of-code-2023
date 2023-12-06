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

fn solve_problem(input: &str) -> Result<i64> {
    let Problem { race } = parse(input)?;

    let min_ms = (1..race.time_ms)
        .find(|ms| is_winning_race(&race, ms))
        .unwrap();
    let max_ms = (1..race.time_ms)
        .rev()
        .find(|ms| is_winning_race(&race, ms))
        .unwrap();

    Ok((max_ms + 1) - min_ms)
}

fn is_winning_race(race: &Race, ms: &i64) -> bool {
    let time_left = race.time_ms - ms;
    let distance = ms * time_left;
    distance > race.min_distance_mm
}

#[derive(Debug, Clone)]
pub struct Problem {
    race: Race,
}

#[derive(Debug, Clone)]
pub struct Race {
    time_ms: i64,
    min_distance_mm: i64,
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
        |(times, _, distances): (Vec<i64>, _, Vec<i64>)| Problem {
            race: Race {
                time_ms: comnbine_numbers(times),
                min_distance_mm: comnbine_numbers(distances),
            },
        },
    ));
    let problem = parse_with_nom(input.trim(), parse_problem)?;
    Ok(problem)
}

fn comnbine_numbers(numbers: Vec<i64>) -> i64 {
    numbers
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 71503);
    }
}
