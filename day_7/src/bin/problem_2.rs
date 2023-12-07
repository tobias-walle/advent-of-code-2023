use std::cmp::Ordering;

use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::tuple,
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
    let mut problem = parse(input)?;
    problem.hands.sort();
    let result = problem
        .hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1))
        .sum();
    Ok(result)
}

#[derive(Debug, Clone)]
struct Problem {
    hands: Vec<Hand>,
}

type Cards = [Card; 5];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: Cards,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(|| {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| a.cmp(b))
                .find(|o| !o.is_eq())
                .unwrap_or(Ordering::Equal)
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    pub fn new(cards: Cards, bid: usize) -> Self {
        Self {
            hand_type: HandType::from_cards(&cards),
            cards,
            bid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &Cards) -> Self {
        Self::from_cards_without_joker(cards)
            .max(Self::from_cards_without_joker(&replace_jokers(cards)))
    }

    fn from_cards_without_joker(cards: &Cards) -> Self {
        let card_counts = cards.iter().counts();
        let counts_counts = card_counts.values().counts();
        let n_counts = |n: usize| *counts_counts.get(&n).unwrap_or(&0);
        if n_counts(5) == 1 {
            Self::FiveOfAKind
        } else if n_counts(4) == 1 {
            Self::FourOfAKind
        } else if n_counts(3) == 1 && n_counts(2) == 1 {
            Self::FullHouse
        } else if n_counts(3) == 1 {
            Self::ThreeOfAKind
        } else if n_counts(2) == 2 {
            Self::TwoPair
        } else if n_counts(2) == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

fn replace_jokers(cards: &Cards) -> Cards {
    let card_counts = cards.iter().counts();
    let card_max_count = card_counts
        .into_iter()
        .filter(|(c, _)| **c != Card::J)
        .max_by_key(|(_, count)| *count);
    match card_max_count {
        Some((card_max_count, _)) => cards.map(|c| if c == Card::J { *card_max_count } else { c }),
        None => *cards,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

fn parse(input: &str) -> Result<Problem> {
    let parse_card = || {
        alt((
            map(tag("2"), |_| Card::N2),
            map(tag("3"), |_| Card::N3),
            map(tag("4"), |_| Card::N4),
            map(tag("5"), |_| Card::N5),
            map(tag("6"), |_| Card::N6),
            map(tag("7"), |_| Card::N7),
            map(tag("8"), |_| Card::N8),
            map(tag("9"), |_| Card::N9),
            map(tag("T"), |_| Card::T),
            map(tag("J"), |_| Card::J),
            map(tag("Q"), |_| Card::Q),
            map(tag("K"), |_| Card::K),
            map(tag("A"), |_| Card::A),
        ))
    };
    let parse_cards = map(
        tuple((
            parse_card(),
            parse_card(),
            parse_card(),
            parse_card(),
            parse_card(),
        )),
        |cards| [cards.0, cards.1, cards.2, cards.3, cards.4],
    );
    let parse_hand = map(
        tuple((parse_cards, space1, parsing::number)),
        |(cards, _, bid)| Hand::new(cards, bid),
    );
    let parse_problem = map(separated_list1(multispace1, parse_hand), |hands| Problem {
        hands,
    });
    let problem = parse_with_nom(input.trim(), all_consuming(parse_problem))?;
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
        assert_eq!(result, 5905);
    }
}
