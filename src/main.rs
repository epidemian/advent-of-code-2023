use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let ans_1: i32 = input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|ch| ch.is_numeric()).unwrap_or('0');
            let last_digit = line.chars().rev().find(|ch| ch.is_numeric()).unwrap_or('0');
            String::from_iter([first_digit, last_digit])
                .parse::<i32>()
                .unwrap()
        })
        .sum();
    println!("{ans_1}");

    let num_names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let ans_2: usize = input
        .lines()
        .map(|line| {
            let mut first_digit = 0;
            for i in 0..line.len() {
                let ch = line.as_bytes()[i] as char;
                if ch.is_numeric() {
                    first_digit = String::from(ch).parse().unwrap();
                    break;
                };
                if let Some(pos) = num_names
                    .iter()
                    .position(|num_name| line[i..].starts_with(num_name))
                {
                    first_digit = pos + 1;
                    break;
                }
            }
            let mut last_digit = 0;
            for i in (0..line.len()).rev() {
                let ch = line.as_bytes()[i] as char;
                if ch.is_numeric() {
                    last_digit = String::from(ch).parse().unwrap();
                    break;
                };
                if let Some(pos) = num_names
                    .iter()
                    .position(|num_name| line[i..].starts_with(num_name))
                {
                    last_digit = pos + 1;
                    break;
                }
            }
            first_digit * 10 + last_digit
        })
        .sum();

    println!("{ans_2}");
    Ok(())
}
