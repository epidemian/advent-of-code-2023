use std::{error::Error, io};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let cards: Vec<_> = input.lines().map(parse_card).try_collect()?;
    let score_sum: u32 = cards.iter().map(card_score).sum();
    println!("{score_sum}");
    Ok(())
}

fn parse_card(line: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let (_, numbers_part) = line.split_once(": ").ok_or("malformed card line")?;
    let (left, right) = numbers_part
        .split_once(" | ")
        .ok_or("malformed card line")?;
    let winning_numbers: Vec<u32> = left.split_whitespace().map(str::parse).try_collect()?;
    let my_numbers: Vec<u32> = right.split_whitespace().map(str::parse).try_collect()?;
    Ok((winning_numbers, my_numbers))
}

fn card_score((winning_numbers, my_numbers): &(Vec<u32>, Vec<u32>)) -> u32 {
    let count = my_numbers
        .iter()
        .filter(|my_num| winning_numbers.contains(my_num))
        .count();
    if count == 0 {
        0
    } else {
        1 << count - 1
    }
}
