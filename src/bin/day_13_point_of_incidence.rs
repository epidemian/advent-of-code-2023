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
        let rows_above = (0..row).rev();
        let rows_below = row..height;
        let diff_count: usize = rows_above
            .zip(rows_below)
            .map(|(row_above, row_below)| {
                pattern[row_above]
                    .iter()
                    .zip(pattern[row_below].iter())
                    .filter(|(tile_above, tile_below)| tile_above != tile_below)
                    .count()
            })
            .sum();
        if diff_count == expected_diff {
            return row * 100;
        }
    }

    let width = pattern[0].len();
    for col in 1..width {
        let rows_left = (0..col).rev();
        let rows_right = col..width;
        let diff_count: usize = rows_left
            .zip(rows_right)
            .map(|(col_left, col_right)| {
                pattern
                    .iter()
                    .filter(|row| row[col_left] != row[col_right])
                    .count()
            })
            .sum();
        if diff_count == expected_diff {
            return col;
        }
    }

    0
}
