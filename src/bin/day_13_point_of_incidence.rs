fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let patterns: Vec<_> = input
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let ans_1: usize = patterns.iter().map(|p| summarize_pattern(p, 0)).sum();
    let ans_2: usize = patterns.iter().map(|p| summarize_pattern(p, 1)).sum();
    println!("{ans_1} {ans_2}");

    Ok(())
}

fn summarize_pattern(pattern: &Vec<Vec<char>>, expected_diff: usize) -> usize {
    let height = pattern.len();
    for row in 1..height {
        let count_row_diffs = |(row_a, row_b): (usize, usize)| {
            pattern[row_a]
                .iter()
                .zip(pattern[row_b].iter())
                .filter(|(tile_a, tile_b)| tile_a != tile_b)
                .count()
        };
        let diff_count: usize = (0..row).rev().zip(row..height).map(count_row_diffs).sum();
        if diff_count == expected_diff {
            return row * 100;
        }
    }

    let width = pattern[0].len();
    for col in 1..width {
        let count_col_diffs = |(col_a, col_b)| {
            pattern
                .iter()
                .filter(|row| row[col_a] != row[col_b])
                .count()
        };
        let diff_count: usize = (0..col).rev().zip(col..width).map(count_col_diffs).sum();
        if diff_count == expected_diff {
            return col;
        }
    }

    0
}
