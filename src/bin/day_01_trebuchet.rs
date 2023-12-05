fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let ans_1: u32 = input.lines().filter_map(calibration_value).sum();
    let ans_2: u32 = input.lines().filter_map(calibration_value_p2).sum();
    println!("{ans_1} {ans_2}");
    Ok(())
}

fn calibration_value(s: &str) -> Option<u32> {
    let digits: Vec<_> = s.chars().filter_map(|ch| ch.to_digit(10)).collect();
    Some(digits.first()? * 10 + digits.last()?)
}

fn calibration_value_p2(s: &str) -> Option<u32> {
    let digits: Vec<_> = s
        .char_indices()
        .filter_map(|(i, ch)| ch.to_digit(10).or_else(|| word_digit_at(s, i)))
        .collect();
    Some(digits.first()? * 10 + digits.last()?)
}

fn word_digit_at(s: &str, pos: usize) -> Option<u32> {
    let num_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let d = num_words.iter().position(|w| s[pos..].starts_with(w))? + 1;
    Some(d as u32)
}
