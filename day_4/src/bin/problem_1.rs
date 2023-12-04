use std::collections::HashSet;

use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
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

fn solve_problem(input: &str) -> Result<i32> {
    let mut score = 0;
    let problem = parse(input).context("Failed to parse input")?;
    for card in &problem.cards {
        let mut card_score = 0;
        for n in &card.my_numbers {
            if card.winning_numbers.contains(n) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        score += card_score;
    }
    Ok(score)
}

#[derive(Debug, Clone)]
struct Problem {
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Card {
    pub id: usize,
    pub winning_numbers: HashSet<i32>,
    pub my_numbers: Vec<i32>,
}

fn parse(input: &str) -> Result<Problem> {
    let parse_card = map(
        tuple((
            preceded(tuple((tag("Card"), space1)), parsing::number),
            tuple((tag(":"), space1)),
            separated_list1(space1, parsing::number),
            delimited(space0, tag("|"), space0),
            separated_list1(space1, parsing::number),
        )),
        |(id, _, winning_numbers, _, my_numbers)| Card {
            id,
            winning_numbers: winning_numbers.into_iter().collect(),
            my_numbers,
        },
    );
    let parse_cards = all_consuming(separated_list1(tag("\n"), parse_card));
    let cards = parse_with_nom(input.trim(), parse_cards)?;
    Ok(Problem { cards })
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 13);
    }
}
