use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let sequences: Vec<_> = input.lines().map(aoc::parse_numbers).try_collect()?;
    let ans_1: i64 = sequences.iter().map(|s| extrapolate(s)).sum();
    let ans_2: i64 = sequences.iter().map(|s| extrapolate_back(s)).sum();
    println!("{ans_1} {ans_2}");
    Ok(())
}

fn extrapolate(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|&n| n == 0) {
        return 0;
    }
    let diffs = differences(sequence);
    extrapolate(&diffs) + sequence[sequence.len() - 1]
}

fn extrapolate_back(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|&n| n == 0) {
        return 0;
    }
    let diffs = differences(sequence);
    sequence[0] - extrapolate_back(&diffs)
}

fn differences(sequence: &[i64]) -> Vec<i64> {
    sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}
