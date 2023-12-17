use std::collections::HashMap;

use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, multispace1},
    combinator::{all_consuming, map},
    multi::{count, many1, separated_list1},
    sequence::{delimited, preceded, tuple},
};
use utils::{parsing::parse_with_nom, read_input_file_as_string};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<usize> {
    let problem = parse(input)?;
    let start_node = Node(['A', 'A', 'A']);
    let target_node = Node(['Z', 'Z', 'Z']);
    let mut node = start_node;
    let mut count = 0;
    for direction in problem.directions.iter().cycle() {
        if node == target_node {
            break;
        }
        node = *problem.junctions[&node].get(direction);
        count += 1;
    }
    Ok(count)
}

#[derive(Debug, Clone)]
struct Problem {
    pub directions: Vec<Direction>,
    pub junctions: HashMap<Node, Junction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node([char; 3]);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Junction {
    pub node: Node,
    pub left: Node,
    pub right: Node,
}

impl Junction {
    fn get(&self, direction: &Direction) -> &Node {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> Result<Problem> {
    let parse_direction = || {
        alt((
            map(tag("L"), |_| Direction::Left),
            map(tag("R"), |_| Direction::Right),
        ))
    };
    let parse_node = || map(count(anychar, 3), |s| Node(s.try_into().unwrap()));
    let parse_junction = || {
        map(
            tuple((
                parse_node(),
                preceded(tag(" = ("), parse_node()),
                delimited(tag(", "), parse_node(), tag(")")),
            )),
            |(node, left, right)| Junction { node, left, right },
        )
    };
    let parse_problem = map(
        tuple((
            many1(parse_direction()),
            preceded(multispace1, separated_list1(multispace1, parse_junction())),
        )),
        |(directions, junctions)| Problem {
            directions,
            junctions: junctions.into_iter().map(|j| (j.node, j)).collect(),
        },
    );
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
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example2() {
        let input = read_to_string("./example2.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 6);
    }
}
