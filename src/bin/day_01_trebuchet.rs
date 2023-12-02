use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let ans_1: u32 = input.lines().filter_map(calibration_value).sum();
    let ans_2: u32 = input.lines().filter_map(calibration_value_p2).sum();
    println!("{ans_1} {ans_2}");
    Ok(())
}

fn calibration_value(line: &str) -> Option<u32> {
    let digits: Vec<_> = line.chars().filter_map(|ch| ch.to_digit(10)).collect();
    Some(digits.first()? * 10 + digits.last()?)
}

fn calibration_value_p2(line: &str) -> Option<u32> {
    let num_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits: Vec<_> = line
        .char_indices()
        .filter_map(|(i, ch)| {
            ch.to_digit(10).or_else(|| {
                let p = num_words.iter().position(|w| line[i..].starts_with(w))?;
                Some((p + 1) as u32)
            })
        })
        .collect();
    Some(digits.first()? * 10 + digits.last()?)
}
