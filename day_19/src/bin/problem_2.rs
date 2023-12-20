use std::{collections::HashMap, ops::RangeInclusive};

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
    let result = Algorithmn::init(problem).solve();
    Ok(result)
}

impl Algorithmn {
    fn init(problem: Problem) -> Self {
        let initial_queue_item = QueueItem {
            range: PartRange {
                ratings: [
                    (RatingName::new("x"), 1..=4000),
                    (RatingName::new("m"), 1..=4000),
                    (RatingName::new("a"), 1..=4000),
                    (RatingName::new("s"), 1..=4000),
                ]
                .into_iter()
                .collect(),
            },
            next_workflow: WorkflowName::new("in"),
        };
        Self {
            problem,
            queue: vec![initial_queue_item],
            accepted: Vec::new(),
        }
    }

    fn solve(&mut self) -> i64 {
        while let Some(item) = self.queue.pop() {
            let workflow = self.problem.get_workflow(&item.next_workflow).clone();
            let mut fallback_range = item.range.clone();
            for rule in &workflow.rules {
                let mut next_range = fallback_range.clone();
                // Restrict the range by the rule
                if let Some(rating_range) = next_range.ratings.get_mut(&rule.rating_name) {
                    *rating_range = match rule.op {
                        RuleOperation::Less => *rating_range.start()..=(rule.n - 1),
                        RuleOperation::Greater => (rule.n + 1)..=*rating_range.end(),
                    };
                    self.handle_rule_outcome(next_range, rule.outcome.clone());
                }
                // Invert the rule for the fallback range
                if let Some(rating_range) = fallback_range.ratings.get_mut(&rule.rating_name) {
                    *rating_range = match rule.op {
                        RuleOperation::Less => rule.n..=*rating_range.end(),
                        RuleOperation::Greater => *rating_range.start()..=rule.n,
                    };
                }
            }
            self.handle_rule_outcome(fallback_range, workflow.fallback.clone());
        }
        self.accepted.iter().map(|r| r.score()).sum()
    }

    fn handle_rule_outcome(&mut self, range: PartRange, outcome: RuleOutcome) {
        match outcome {
            RuleOutcome::Accept => self.accepted.push(range),
            RuleOutcome::Reject => (),
            RuleOutcome::NextWorkflow(next_workflow) => self.queue.push(QueueItem {
                range,
                next_workflow,
            }),
        }
    }
}

impl Problem {
    fn get_workflow(&self, name: &WorkflowName) -> &Workflow {
        self.workflows
            .get(name)
            .unwrap_or_else(|| panic!("Workflow with {name:?} not found."))
    }
}

impl PartRange {
    fn score(&self) -> i64 {
        self.ratings
            .values()
            .map(|r| r.end() - r.start() + 1)
            .product()
    }
}

impl RatingName {
    fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl WorkflowName {
    fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

#[derive(Debug, Clone)]
struct Algorithmn {
    problem: Problem,
    queue: Vec<QueueItem>,
    accepted: Vec<PartRange>,
}

#[derive(Debug, Clone)]
struct QueueItem {
    pub range: PartRange,
    pub next_workflow: WorkflowName,
}

#[derive(Debug, Clone)]
struct PartRange {
    pub ratings: HashMap<RatingName, RangeInclusive<i64>>,
}

#[derive(Debug, Clone)]
struct Problem {
    pub workflows: HashMap<WorkflowName, Workflow>,
    #[allow(dead_code)]
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone)]
struct Workflow {
    pub name: WorkflowName,
    pub rules: Vec<WorkflowRule>,
    pub fallback: RuleOutcome,
}

#[derive(Debug, Clone)]
struct WorkflowRule {
    pub rating_name: RatingName,
    pub op: RuleOperation,
    pub n: i64,
    pub outcome: RuleOutcome,
}

#[derive(Debug, Clone)]
enum RuleOutcome {
    Accept,
    Reject,
    NextWorkflow(WorkflowName),
}

#[derive(Debug, Clone)]
enum RuleOperation {
    Less,
    Greater,
}

#[derive(Debug, Clone)]
struct Part {
    #[allow(dead_code)]
    pub ratings: HashMap<RatingName, i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WorkflowName(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RatingName(String);

fn parse(input: &str) -> Result<Problem> {
    let parse_rating_name = || map(alpha1, |s: &str| RatingName(s.into()));
    let parse_workflow_name = || map(alpha1, |s: &str| WorkflowName(s.into()));
    let parse_op = || {
        alt((
            map(tag("<"), |_| RuleOperation::Less),
            map(tag(">"), |_| RuleOperation::Greater),
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
                parse_op(),
                parsing::number,
                preceded(tag(":"), parse_rule_outcome()),
            )),
            |(part, op, n, outcome)| WorkflowRule {
                rating_name: part,
                op,
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
        assert_eq!(result, 167409079868000);
    }
}
