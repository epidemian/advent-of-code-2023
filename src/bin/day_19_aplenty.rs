use itertools::Itertools;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (workflows, parts) = parse_input(&input)?;
    let ratings_sum: u64 = parts
        .iter()
        .filter(|p| process_part(p, &workflows))
        .map(|p| p.iter().sum::<u64>())
        .sum();
    let accept_combinations = get_accept_ratings_combinations(&workflows);
    println!("{ratings_sum} {accept_combinations}");
    Ok(())
}

type Part = [u64; 4];

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    fallback: Output<'a>,
}

struct Rule<'a> {
    condition: Condition,
    output: Output<'a>,
}

struct Condition {
    category_index: usize,
    op: Operator,
    value: u64,
}

enum Operator {
    Gt,
    Lt,
}

type RatingsIntervals = [Interval; 4];

#[derive(Clone, Copy)]
struct Interval {
    start: u64,
    end: u64,
}

#[derive(Clone, Copy)]
enum Output<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

fn process_part(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut workflow_id = "in";
    loop {
        // TODO: handle this error better.
        let workflow = workflows.get(workflow_id).expect("invalid workflow ID");
        let output = workflow.process(part);
        match output {
            Output::Accept => return true,
            Output::Reject => return false,
            Output::Workflow(id) => workflow_id = id,
        }
    }
}

fn get_accept_ratings_combinations(workflows: &HashMap<&str, Workflow>) -> u64 {
    let full_rating_interval = Interval {
        start: 1,
        end: 4000,
    };
    let start_intervals = [full_rating_interval; 4];
    let accept_intervals = collect_accept_intervals("in", start_intervals, workflows);

    let ratings_cartesian_product = |intervals: &RatingsIntervals| {
        intervals
            .iter()
            .map(|interval| interval.size())
            .product::<u64>()
    };
    accept_intervals.iter().map(ratings_cartesian_product).sum()
}

fn collect_accept_intervals(
    workflow_id: &str,
    intervals: RatingsIntervals,
    workflows: &HashMap<&str, Workflow>,
) -> Vec<RatingsIntervals> {
    let workflow = &workflows[workflow_id];
    let mut intervals = intervals;
    let mut accept_intervals = vec![];
    for rule in workflow.rules.iter() {
        let matching_intervals = rule.condition.apply_to_intervals(intervals);
        match rule.output {
            Output::Accept => accept_intervals.push(matching_intervals),
            Output::Reject => {}
            Output::Workflow(id) => {
                accept_intervals.extend(collect_accept_intervals(id, matching_intervals, workflows))
            }
        }
        // Continue with the rest of the rules considering this rule's condition not matched.
        intervals = rule.condition.negated().apply_to_intervals(intervals);
    }
    match workflow.fallback {
        Output::Accept => accept_intervals.push(intervals),
        Output::Reject => {}
        Output::Workflow(id) => {
            accept_intervals.extend(collect_accept_intervals(id, intervals, workflows))
        }
    }
    accept_intervals
}

impl Workflow<'_> {
    fn parse(s: &str) -> aoc::Result<Workflow> {
        let rules = s.split(',').collect_vec();
        let [rules @ .., fallback] = &rules[..] else {
            Err("expected at least one fallback rule")?
        };
        let rules = rules.iter().map(|s| Rule::parse(s)).try_collect()?;
        let fallback = Output::parse(fallback);
        Ok(Workflow { rules, fallback })
    }

    fn process(&self, part: &Part) -> Output {
        self.rules
            .iter()
            .find_map(|r| r.process(part))
            .unwrap_or(self.fallback)
    }
}

impl Rule<'_> {
    fn parse(s: &str) -> aoc::Result<Rule> {
        let (condition, output) = s.split_once(':').ok_or("invalid rule")?;
        let condition = Condition::parse(condition)?;
        let output = Output::parse(output);
        Ok(Rule { condition, output })
    }

    fn process(&self, part: &Part) -> Option<Output> {
        if self.condition.matches(part) {
            Some(self.output)
        } else {
            None
        }
    }
}

impl Condition {
    fn parse(s: &str) -> aoc::Result<Condition> {
        let category = &s[0..1];
        let op = &s[1..2];
        let value = &s[2..];
        let category_index = match category {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => Err(format!("invalid category '{category}'"))?,
        };
        let op = Operator::parse(op)?;
        let value = value.parse()?;
        Ok(Condition {
            category_index,
            op,
            value,
        })
    }

    fn matches(&self, part: &Part) -> bool {
        let part_value = part[self.category_index];
        self.op.apply(part_value, self.value)
    }

    fn apply_to_intervals(&self, intervals: RatingsIntervals) -> RatingsIntervals {
        let mut intervals = intervals;
        let interval = &mut intervals[self.category_index];
        match self.op {
            Operator::Gt => interval.start = interval.start.max(self.value + 1),
            Operator::Lt => interval.end = interval.end.min(self.value - 1),
        }
        intervals
    }

    fn negated(&self) -> Condition {
        let (op, value) = match self.op {
            Operator::Gt => (Operator::Lt, self.value + 1),
            Operator::Lt => (Operator::Gt, self.value - 1),
        };
        Condition { op, value, ..*self }
    }
}

impl Operator {
    fn parse(s: &str) -> aoc::Result<Operator> {
        Ok(match s {
            "<" => Operator::Lt,
            ">" => Operator::Gt,
            _ => Err(format!("invalid operator '{s}'"))?,
        })
    }

    fn apply(&self, lhs: u64, rhs: u64) -> bool {
        match self {
            Operator::Gt => lhs > rhs,
            Operator::Lt => lhs < rhs,
        }
    }
}

impl Output<'_> {
    fn parse(s: &str) -> Output {
        match s {
            "A" => Output::Accept,
            "R" => Output::Reject,
            _ => Output::Workflow(s),
        }
    }
}

impl Interval {
    fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

fn parse_input(input: &str) -> aoc::Result<(HashMap<&str, Workflow>, Vec<Part>)> {
    let (workflows_input, parts_input) = input.split_once("\n\n").ok_or("invalid input")?;
    let workflows = workflows_input.lines().map(parse_workflow).try_collect()?;
    let parts = parts_input.lines().map(parse_part).try_collect()?;
    Ok((workflows, parts))
}

fn parse_workflow(s: &str) -> aoc::Result<(&str, Workflow)> {
    let (id, workflow) = s
        .split(['{', '}'])
        .filter(|s| !s.is_empty())
        .collect_tuple()
        .ok_or("invalid workflow")?;
    Ok((id, Workflow::parse(workflow)?))
}

fn parse_part(s: &str) -> aoc::Result<Part> {
    let part = aoc::parse_numbers(s)?[..].try_into()?;
    Ok(part)
}
