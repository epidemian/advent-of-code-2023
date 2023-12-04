use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let engine_grid: Vec<&str> = input.lines().collect();

    let at = |x: usize, y: usize| engine_grid[y].as_bytes()[x];

    let mut sum = 0;
    for (y, line) in engine_grid.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            if at(x, y).is_ascii_digit() {
                let mut num_len = 1;
                while x + num_len < line.len() {
                    if !at(x + num_len, y).is_ascii_digit() {
                        break;
                    }
                    num_len += 1;
                }
                let is_part_number = (x..x + num_len).any(|x| {
                    neighbors(x, y, &engine_grid)
                        .any(|(nx, ny)| !at(nx, ny).is_ascii_digit() && at(nx, ny) != b'.')
                });
                if is_part_number {
                    let num_slice = &line[x..x + num_len];
                    let num: u32 = num_slice.parse()?;
                    sum += num;
                }
                x += num_len;
            } else {
                x += 1;
            }
        }
    }
    println!("{sum}");

    let mut numbers_by_xy = HashMap::new();
    for (num, x, y, num_len) in get_all_numbers(&engine_grid) {
        for x in x..x + num_len {
            numbers_by_xy.insert((x, y), num);
        }
    }
    let mut gear_ratio_sum = 0;
    for (y, line) in engine_grid.iter().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'*' {
                let mut neighbor_nums = HashSet::new();
                for (nx, ny) in neighbors(x, y, &engine_grid) {
                    if let Some(num) = numbers_by_xy.get(&(nx, ny)) {
                        neighbor_nums.insert(*num);
                    }
                }
                if neighbor_nums.len() == 2 {
                    gear_ratio_sum += neighbor_nums.iter().product::<u32>();
                }
            }
        }
    }
    println!("{gear_ratio_sum}");
    Ok(())
}

fn get_all_numbers(engine_grid: &[&str]) -> Vec<(u32, usize, usize, usize)> {
    let mut nums = vec![];
    let at = |x: usize, y: usize| engine_grid[y].as_bytes()[x];
    for (y, line) in engine_grid.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            if at(x, y).is_ascii_digit() {
                let mut num_len = 1;
                while x + num_len < line.len() {
                    if !at(x + num_len, y).is_ascii_digit() {
                        break;
                    }
                    num_len += 1;
                }
                let num_slice = &line[x..x + num_len];
                let num = num_slice.parse().unwrap();
                nums.push((num, x, y, num_len));
                x += num_len;
            } else {
                x += 1;
            }
        }
    }
    nums
}

fn neighbors<'a>(
    x: usize,
    y: usize,
    engine_grid: &'a [&str],
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let (x, y) = (x as isize, y as isize);
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter_map(|(x, y)| {
        if y >= 0
            && y < engine_grid.len() as isize
            && x >= 0
            && x < engine_grid[y as usize].len() as isize
        {
            Some((x as usize, y as usize))
        } else {
            None
        }
    })
}
