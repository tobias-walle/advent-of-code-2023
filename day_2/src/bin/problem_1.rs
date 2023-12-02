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

fn solve_problem(input: &str) -> Result<usize> {
    let problem = parse(input).context("Failed to parse input")?;
    let mut result = 0;
    'game: for game in &problem.games {
        for round in &game.rounds {
            let mut bag = problem.bag.clone();
            for (color, count) in &round.counts {
                let cube_in_bag = bag
                    .cubes
                    .counts
                    .get_mut(color)
                    .with_context(|| format!("Color '{color:?}' not found on bag"))?;
                *cube_in_bag = match cube_in_bag.checked_sub(*count) {
                    Some(new_count) => new_count,
                    None => {
                        // Count cannot be subtracted anymore, cancel game
                        continue 'game;
                    }
                };
            }
        }
        // All rounds were possible, add to result
        result += game.id;
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub bag: Bag,
    pub games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub struct Bag {
    pub cubes: Cubes,
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

impl Bag {
    fn new(cubes: &[(Color, u32)]) -> Self {
        Self {
            cubes: Cubes::new(cubes),
        }
    }
}

impl Cubes {
    fn new(counts: &[(Color, u32)]) -> Self {
        Self {
            counts: counts.iter().cloned().collect(),
        }
    }
}

fn parse(input: &str) -> Result<Problem> {
    let bag = Bag::new(&[(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
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
    Ok(Problem { bag, games })
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 8);
    }
}
