use std::collections::{HashMap, HashSet};

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

fn solve_problem(input: &str) -> Result<usize> {
    let problem = parse(input).context("Failed to parse input")?;
    let mut cards: HashMap<CardId, usize> = HashMap::new();
    for card in &problem.cards {
        cards.insert(card.id, 1);
    }
    for card in &problem.cards {
        let mut next_id = card.id.increment();
        let n_cards = cards[&card.id];
        for n in &card.my_numbers {
            if card.winning_numbers.contains(n) {
                *cards.entry(next_id).or_insert(0) += n_cards;
                next_id = next_id.increment();
            }
        }
    }
    dbg!(&cards);
    let score = cards.values().sum();
    Ok(score)
}

#[derive(Debug, Clone)]
struct Problem {
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CardId(usize);

#[derive(Debug, Clone)]
struct Card {
    pub id: CardId,
    pub winning_numbers: HashSet<i32>,
    pub my_numbers: Vec<i32>,
}

impl CardId {
    fn increment(&self) -> Self {
        Self(self.0 + 1)
    }
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
            id: CardId(id),
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
        assert_eq!(result, 30);
    }
}
