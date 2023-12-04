use std::{collections::HashSet, error::Error, io};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let cards: Vec<_> = input.lines().map(parse_card).try_collect()?;
    let win_counts: Vec<_> = cards
        .iter()
        .map(|(winning_numbers, my_numbers)| my_numbers.intersection(winning_numbers).count())
        .collect();

    // Part 1
    let total_points: u32 = win_counts
        .iter()
        .map(|&count| if count == 0 { 0 } else { 1 << (count - 1) })
        .sum();

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

fn parse_card(line: &str) -> Result<(HashSet<u32>, HashSet<u32>), Box<dyn Error>> {
    let (_, numbers_part) = line.split_once(": ").ok_or("malformed card line")?;
    let (left, right) = numbers_part
        .split_once(" | ")
        .ok_or("malformed card line")?;
    let winning_numbers = left.split_whitespace().map(str::parse).try_collect()?;
    let my_numbers = right.split_whitespace().map(str::parse).try_collect()?;
    Ok((winning_numbers, my_numbers))
}
