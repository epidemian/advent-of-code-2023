use std::collections::{HashMap, HashSet};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (grid, ..) = aoc::parse_char_grid(&input)?;
    let number_spans = get_number_spans(&grid)?;

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
        for (x, &ch) in line.iter().enumerate() {
            if ch == '*' {
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

type Grid = Vec<Vec<char>>;

fn is_part_number(start_x: usize, end_x: usize, y: usize, grid: &Grid) -> bool {
    (start_x..=end_x)
        .any(|x| neighbors(x, y, grid).any(|(.., ch)| !ch.is_ascii_digit() && ch != '.'))
}

// Finds all numbers on the grid and returns tuples with (number, start_x, end_x, y) where
// start_x..=end_x is the span of the number on the grid.
fn get_number_spans(grid: &Grid) -> aoc::Result<Vec<(u32, usize, usize, usize)>> {
    let mut spans = vec![];
    for (y, line) in grid.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            if grid[y][x].is_ascii_digit() {
                let mut end_x = x;
                while end_x + 1 < line.len() && grid[y][end_x + 1].is_ascii_digit() {
                    end_x += 1;
                }
                let num = String::from_iter(&line[x..=end_x]).parse()?;
                spans.push((num, x, end_x, y));
                x = end_x + 1;
            } else {
                x += 1;
            }
        }
    }
    Ok(spans)
}

fn neighbors(x: usize, y: usize, grid: &Grid) -> impl Iterator<Item = (usize, usize, char)> + '_ {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter_map(move |(dx, dy)| {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            let ch = *grid.get(ny)?.get(nx)?;
            Some((nx, ny, ch))
        })
}
