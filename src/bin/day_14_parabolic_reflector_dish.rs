use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let start_grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    // Part 1
    let mut grid = start_grid.clone();
    tilt_north(&mut grid);
    let north_beams_load_p1 = get_north_beams_load(&grid);

    // Part 2
    let mut grid = start_grid.clone();
    let mut remaining_spins = 1_000_000_000;
    let mut grids_memo = HashMap::new();
    while remaining_spins > 0 {
        spin_cycle(&mut grid);
        remaining_spins -= 1;
        if let Some(prev_remaining_spins) = grids_memo.get(&grid) {
            let cycle_size = prev_remaining_spins - remaining_spins;
            remaining_spins %= cycle_size;
        } else {
            grids_memo.insert(grid.clone(), remaining_spins);
        }
    }
    let north_beams_load_p2 = get_north_beams_load(&grid);

    println!("{north_beams_load_p1} {north_beams_load_p2}");
    Ok(())
}

type Grid = Vec<Vec<char>>;

fn tilt_north(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();
    for col in 0..width {
        for i in 0..height {
            for j in 1..height - i {
                if grid[j][col] == 'O' && grid[j - 1][col] == '.' {
                    grid[j - 1][col] = 'O';
                    grid[j][col] = '.';
                }
            }
        }
    }
}

fn spin_cycle(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();

    let mut roll = |x1: usize, y1: usize, x2: usize, y2: usize| {
        let can_roll = grid[y1][x1] == 'O' && grid[y2][x2] == '.';
        if can_roll {
            grid[y2][x2] = 'O';
            grid[y1][x1] = '.';
        }
        can_roll
    };

    // Tilt north
    for x in 0..width {
        for i in 0..height {
            let mut rolled = false;
            for y in 1..height - i {
                rolled |= roll(x, y, x, y - 1);
            }
            if !rolled {
                break;
            }
        }
    }

    // Tilt west
    for y in 0..height {
        for i in 0..width {
            let mut rolled = false;
            for x in 1..width - i {
                rolled |= roll(x, y, x - 1, y);
            }
            if !rolled {
                break;
            }
        }
    }

    // Tilt south
    for x in 0..width {
        for i in 0..height {
            let mut rolled = false;
            for y in (i..height - 1).rev() {
                rolled |= roll(x, y, x, y + 1);
            }
            if !rolled {
                break;
            }
        }
    }

    // Tilt east
    for y in 0..height {
        for i in 0..width {
            let mut rolled = false;
            for x in (i..width - 1).rev() {
                rolled |= roll(x, y, x + 1, y);
            }
            if !rolled {
                break;
            }
        }
    }
}

fn get_north_beams_load(grid: &Grid) -> usize {
    grid.iter()
        .zip((1..=grid.len()).rev())
        .map(|(row, row_load)| row.iter().filter(|&&tile| tile == 'O').count() * row_load)
        .sum()
}
