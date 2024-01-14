use anyhow::Context;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let cards: Vec<_> = input.lines().map(parse_card).try_collect()?;
    let win_counts: Vec<_> = cards
        .iter()
        .map(|(winning_numbers, my_numbers)| my_numbers.intersection(winning_numbers).count())
        .collect();

    // Part 1
    let total_points: u32 = win_counts.iter().map(|&count| 1 << count >> 1).sum();

    // Part 2
    let mut card_copies = vec![1; win_counts.len()];
    for (i, &win_count) in win_counts.iter().enumerate() {
        for j in i + 1..=i + win_count {
            card_copies[j] += card_copies[i];
        }
    }
    let total_cards: usize = card_copies.iter().sum();

    println!("{total_points} {total_cards}");
    Ok(())
}

fn parse_card(line: &str) -> aoc::Result<(HashSet<u32>, HashSet<u32>)> {
    let (_, numbers_part) = line.split_once(": ").context("malformed card line")?;
    let (left, right) = numbers_part
        .split_once(" | ")
        .context("malformed card line")?;
    let winning_numbers = HashSet::from_iter(aoc::parse_numbers(left)?);
    let my_numbers = HashSet::from_iter(aoc::parse_numbers(right)?);

    Ok((winning_numbers, my_numbers))
}
