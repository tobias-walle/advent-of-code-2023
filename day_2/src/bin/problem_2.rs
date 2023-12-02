use std::collections::HashMap;

use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, space0},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
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

fn solve_problem(input: &str) -> Result<u32> {
    let problem = parse(input).context("Failed to parse input")?;
    let mut result = 0;
    for game in &problem.games {
        let mut min_cubes = Cubes::new(&[]);
        for round in &game.rounds {
            for (color, count) in &round.counts {
                let min_count = min_cubes.counts.entry(color.clone()).or_insert(0);
                *min_count = (*min_count).max(*count);
            }
        }
        let power = min_cubes
            .counts
            .into_values()
            .reduce(|a, b| a * b)
            .unwrap_or(0);
        dbg!(&power);
        result += power;
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Cubes>,
}

#[derive(Debug, Clone)]
pub struct Cubes {
    pub counts: HashMap<Color, Count>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Color {
    Blue,
    Red,
    Green,
}

type Count = u32;

impl Cubes {
    fn new(counts: &[(Color, u32)]) -> Self {
        Self {
            counts: counts.iter().cloned().collect(),
        }
    }
}

fn parse(input: &str) -> Result<Problem> {
    let parse_color = alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ));
    let parse_color_count = map(
        delimited(
            space0,
            tuple((parsing::number, space0, parse_color)),
            space0,
        ),
        |(n, _, color)| (color, n),
    );
    let parse_cubes = map(separated_list1(tag(","), parse_color_count), |counts| {
        Cubes::new(&counts)
    });
    let parse_game = map(
        tuple((
            preceded(tag("Game "), parsing::number),
            tuple((tag(":"), multispace1)),
            separated_list1(tag(";"), parse_cubes),
        )),
        |(id, _, rounds)| Game { id, rounds },
    );
    let parse_games = all_consuming(separated_list1(tag("\n"), parse_game));
    let games = parse_with_nom(input.trim(), parse_games)?;
    Ok(Problem { games })
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 2286);
    }
}
