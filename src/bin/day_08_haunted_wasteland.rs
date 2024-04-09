use anyhow::{bail, Context};
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (instructions, nodes) = input.split_once("\n\n").context("invalid input")?;
    let nodes = nodes.lines().map(parse_node).try_collect()?;

    let ans_1 = count_steps("AAA", instructions, &nodes)?;

    // This part 2 solution assumes:
    // 1. That the amount of steps to reach the end from start_id is a multiple of the instructions
    //    count for each start_id.
    // 2. That those amounts of steps are also equal to the amount of steps that it takes to reach
    //    the end again if we keep going.
    //
    // With these assumptions —which are true for the puzzle input— the answer is then the least
    // common multiple of all these counts.
    let ans_2 = nodes
        .keys()
        .filter(|id| id.ends_with('A'))
        .map(|start_id| count_steps(start_id, instructions, &nodes))
        .fold_ok(1, lcm)?;

    println!("{ans_1} {ans_2}");
    Ok(())
}

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn count_steps(start: &str, instructions: &str, nodes: &Graph) -> aoc::Result<u64> {
    let mut inst_iter = instructions.chars().cycle();
    let mut curr = start;
    let mut count = 0;
    loop {
        let (left, right) = nodes
            .get(curr)
            .with_context(|| format!("node '{curr}' not found"))?;
        let inst = inst_iter.next().context("no instructions found")?;
        match inst {
            'L' => curr = left,
            'R' => curr = right,
            _ => bail!("unexpected instruction char '{inst}'"),
        }
        count += 1;
        if curr.ends_with('Z') {
            return Ok(count);
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

fn parse_node(s: &str) -> aoc::Result<(&str, (&str, &str))> {
    let (id, connected_ids) = s.split_once(" = ").context("expected an = sign")?;
    let (left_id, right_id) = parse_connections(connected_ids).context("invalid input format")?;
    Ok((id, (left_id, right_id)))
}

fn parse_connections(connected_ids: &str) -> Option<(&str, &str)> {
    connected_ids
        .strip_prefix('(')?
        .strip_suffix(')')?
        .split_once(", ")
}
