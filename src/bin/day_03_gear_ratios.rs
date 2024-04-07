use itertools::{iproduct, Itertools};
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (grid, width, height) = aoc::parse_char_grid(&input)?;
    let number_spans = get_number_spans(&grid)?;

    // Part 1
    let part_numbers_sum: u32 = number_spans
        .iter()
        .filter(|&&(_n, start_x, end_x, y)| is_part_number(start_x, end_x, y, &grid))
        .map(|(n, ..)| n)
        .sum();

    // Part 2
    let numbers_by_xy: HashMap<(usize, usize), u32> = number_spans
        .into_iter()
        .flat_map(|(num, start_x, end_x, y)| (start_x..end_x).map(move |x| ((x, y), num)))
        .collect();
    let gear_ratios_sum: u32 = iproduct!(0..width, 0..height)
        .filter(|&(x, y)| grid[y][x] == '*')
        .filter_map(|(x, y)| {
            let neighbor_nums = neighbors(x, y, &grid)
                .filter_map(|(nx, ny, _)| numbers_by_xy.get(&(nx, ny)))
                .unique();
            let (a, b) = neighbor_nums.collect_tuple()?;
            Some(a * b)
        })
        .sum();

    println!("{part_numbers_sum} {gear_ratios_sum}");
    Ok(())
}

type Grid = Vec<Vec<char>>;

fn is_part_number(start_x: usize, end_x: usize, y: usize, grid: &Grid) -> bool {
    (start_x..end_x)
        .any(|x| neighbors(x, y, grid).any(|(.., ch)| !ch.is_ascii_digit() && ch != '.'))
}

// Finds all numbers on the grid and returns tuples with (number, start_x, end_x, y) where
// start_x..end_x is the span of the number on the grid.
fn get_number_spans(grid: &Grid) -> aoc::Result<Vec<(u32, usize, usize, usize)>> {
    let mut spans = vec![];
    for (y, line) in grid.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let digits = grid[y][x..].iter().take_while(|ch| ch.is_ascii_digit());
            let s = String::from_iter(digits);
            if !s.is_empty() {
                let num = s.parse()?;
                spans.push((num, x, x + s.len(), y));
            }
            x += s.len().max(1)
        }
    }
    Ok(spans)
}

fn neighbors(x: usize, y: usize, grid: &Grid) -> impl Iterator<Item = (usize, usize, char)> + '_ {
    iproduct!(-1..=1, -1..=1).filter_map(move |(dx, dy)| {
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        let ch = *grid.get(ny)?.get(nx)?;
        Some((nx, ny, ch))
    })
}
