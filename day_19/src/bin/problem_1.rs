use std::{cmp::Ordering, collections::HashMap};

use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::{all_consuming, map},
    multi::{separated_list0, separated_list1},
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

fn solve_problem(input: &str) -> Result<i64> {
    let problem = parse(input)?;
    let mut result = 0;
    for part in &problem.parts {
        if problem.is_accepted(part) {
            result += part.ratings.values().sum::<i64>()
        }
    }
    Ok(result)
}

impl Problem {
    fn is_accepted(&self, part: &Part) -> bool {
        let mut current_workflow_name = WorkflowName("in".into());
        loop {
            let workflow = self
                .workflows
                .get(&current_workflow_name)
                .unwrap_or_else(|| {
                    panic!("Workflow with name {current_workflow_name:?} not found.")
                });
            match workflow.apply(part) {
                RuleOutcome::Accept => return true,
                RuleOutcome::Reject => return false,
                RuleOutcome::NextWorkflow(next) => current_workflow_name = next,
            };
        }
    }
}

impl Workflow {
    fn apply(&self, part: &Part) -> RuleOutcome {
        match self.rules.iter().find(|r| r.is_applicable(part)) {
            Some(rule) => rule.outcome.clone(),
            None => self.fallback.clone(),
        }
    }
}

impl WorkflowRule {
    fn is_applicable(&self, part: &Part) -> bool {
        let Some(rating) = part.ratings.get(&self.rating_name) else {
            return false;
        };
        rating.cmp(&self.n) == self.expected_ord
    }
}

#[derive(Debug, Clone)]
struct Problem {
    pub workflows: HashMap<WorkflowName, Workflow>,
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone)]
struct Workflow {
    pub name: WorkflowName,
    pub rules: Vec<WorkflowRule>,
    pub fallback: RuleOutcome,
}

#[derive(Debug, Clone)]
enum RuleOutcome {
    Accept,
    Reject,
    NextWorkflow(WorkflowName),
}

#[derive(Debug, Clone)]
struct WorkflowRule {
    pub rating_name: RatingName,
    pub expected_ord: Ordering,
    pub n: i64,
    pub outcome: RuleOutcome,
}

#[derive(Debug, Clone)]
struct Part {
    pub ratings: HashMap<RatingName, i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WorkflowName(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RatingName(String);

fn parse(input: &str) -> Result<Problem> {
    let parse_rating_name = || map(alpha1, |s: &str| RatingName(s.into()));
    let parse_workflow_name = || map(alpha1, |s: &str| WorkflowName(s.into()));
    let parse_ordering = || {
        alt((
            map(tag("<"), |_| Ordering::Less),
            map(tag(">"), |_| Ordering::Greater),
        ))
    };
    let parse_rule_outcome = || {
        alt((
            map(tag("A"), |_| RuleOutcome::Accept),
            map(tag("R"), |_| RuleOutcome::Reject),
            map(parse_workflow_name(), RuleOutcome::NextWorkflow),
        ))
    };
    let parse_rule = || {
        map(
            tuple((
                parse_rating_name(),
                parse_ordering(),
                parsing::number,
                preceded(tag(":"), parse_rule_outcome()),
            )),
            |(part, expected_ord, n, outcome)| WorkflowRule {
                rating_name: part,
                expected_ord,
                n,
                outcome,
            },
        )
    };
    let parse_workflow = || {
        map(
            tuple((
                parse_workflow_name(),
                delimited(
                    tag("{"),
                    tuple((
                        separated_list0(tag(","), parse_rule()),
                        preceded(tag(","), parse_rule_outcome()),
                    )),
                    tag("}"),
                ),
            )),
            |(name, (rules, fallback))| Workflow {
                name,
                rules,
                fallback,
            },
        )
    };
    let parse_part = || {
        map(
            delimited(
                tag("{"),
                separated_list1(
                    tag(","),
                    tuple((parse_rating_name(), preceded(tag("="), parsing::number))),
                ),
                tag("}"),
            ),
            |ratings| Part {
                ratings: ratings.into_iter().collect(),
            },
        )
    };
    let parse_problem = || {
        map(
            tuple((
                separated_list1(tag("\n"), parse_workflow()),
                preceded(multispace0, separated_list1(tag("\n"), parse_part())),
            )),
            |(workflows, parts)| Problem {
                workflows: workflows.into_iter().map(|w| (w.name.clone(), w)).collect(),
                parts,
            },
        )
    };
    let problem = parse_with_nom(input.trim(), all_consuming(parse_problem()))?;
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
        assert_eq!(result, 19114);
    }
}
