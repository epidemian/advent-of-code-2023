use std::collections::{HashMap, HashSet};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid: Vec<&str> = input.lines().collect();
    let number_spans = get_number_spans(&grid);

    // Part 1
    let part_numbers_sum: u32 = number_spans
        .iter()
        .filter(|&&(_n, start_x, end_x, y)| is_part_number(start_x, end_x, y, &grid))
        .map(|(n, ..)| n)
        .sum();

    // Part 2
    let mut numbers_by_xy = HashMap::new();
    for (num, start_x, end_x, y) in number_spans {
        for x in start_x..=end_x {
            numbers_by_xy.insert((x, y), num);
        }
    }
    let mut gear_ratios_sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'*' {
                let mut neighbor_nums = HashSet::new();
                for (nx, ny, _b) in neighbors(x, y, &grid) {
                    if let Some(num) = numbers_by_xy.get(&(nx, ny)) {
                        neighbor_nums.insert(*num);
                    }
                }
                if neighbor_nums.len() == 2 {
                    gear_ratios_sum += neighbor_nums.iter().product::<u32>();
                }
            }
        }
    }

    println!("{part_numbers_sum} {gear_ratios_sum}");
    Ok(())
}

type Grid<'a> = [&'a str];

fn is_part_number(start_x: usize, end_x: usize, y: usize, grid: &Grid) -> bool {
    (start_x..=end_x)
        .any(|x| neighbors(x, y, grid).any(|(.., byte)| !byte.is_ascii_digit() && byte != b'.'))
}

fn at(x: usize, y: usize, grid: &Grid) -> u8 {
    grid[y].as_bytes()[x]
}

// Finds all numbers on the grid and returns tuples with (number, start_x, end_x, y) where
// start_x..=end_x is the span of the number on the grid.
fn get_number_spans(grid: &Grid) -> Vec<(u32, usize, usize, usize)> {
    let mut spans = vec![];
    for (y, line) in grid.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            if at(x, y, grid).is_ascii_digit() {
                let mut end_x = x;
                while end_x + 1 < line.len() && at(end_x + 1, y, grid).is_ascii_digit() {
                    end_x += 1;
                }
                // Note: this parse() should not fail as the slice should contain only ASCII digits.
                let num = line[x..=end_x].parse().unwrap();
                spans.push((num, x, end_x, y));
                x = end_x + 1;
            } else {
                x += 1;
            }
        }
    }
    spans
}

fn neighbors<'a>(
    x: usize,
    y: usize,
    grid: &'a Grid,
) -> impl Iterator<Item = (usize, usize, u8)> + 'a {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(dx, dy)| {
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        let byte = *grid.get(ny)?.as_bytes().get(nx)?;
        Some((nx, ny, byte))
    })
}
