use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (start_grid, width, height) = aoc::parse_char_grid(&input)?;
    anyhow::ensure!(width == height, "grid must be square");

    // Part 1
    let mut grid = start_grid.clone();
    tilt_platform(&mut grid, NORTH);
    let north_beams_load_p1 = get_north_beams_load(&grid);

    // Part 2
    let mut grid = start_grid.clone();
    let mut remaining_spins = 1_000_000_000;
    let mut grids_memo = HashMap::new();
    while remaining_spins > 0 {
        for dir in [NORTH, WEST, SOUTH, EAST] {
            tilt_platform(&mut grid, dir);
        }
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
type Pos = (usize, usize);
type Dir = (isize, isize);

const NORTH: Dir = (0, -1);
const WEST: Dir = (-1, 0);
const SOUTH: Dir = (0, 1);
const EAST: Dir = (1, 0);

fn tilt_platform(grid: &mut Grid, (dx, dy): Dir) {
    let size = grid.len() as isize;
    let neg_dir = (-dx, -dy);
    let mut line_start = (0, 0);
    for _ in 0..size {
        let off_x = if dx == 1 { size - 1 } else { 0 };
        let off_y = if dy == 1 { size - 1 } else { 0 };
        let mut pos = add(line_start, (off_x, off_y));
        let mut last_empty = pos;
        // We move on a vertical or horizontal line, going in the opposite of the given direction,
        // and "rolling" the round rocks in the given direction "down" to `last_empty`, updating the
        // latter as we go.
        for _ in 0..size {
            match at(grid, pos) {
                '#' => last_empty = add(pos, neg_dir),
                'O' => {
                    *at(grid, pos) = '.';
                    *at(grid, last_empty) = 'O';
                    last_empty = add(last_empty, neg_dir)
                }
                _ => {}
            }
            pos = add(pos, neg_dir);
        }
        // Move start down if we're tilting horizontally, and right if tiling vertically.
        line_start = add(line_start, (dy.abs(), dx.abs()));
    }
}

fn at(grid: &mut Grid, (x, y): Pos) -> &mut char {
    &mut grid[y][x]
}

fn add((x, y): Pos, (dx, dy): Dir) -> Pos {
    (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))
}

fn get_north_beams_load(grid: &Grid) -> usize {
    grid.iter()
        .zip((1..=grid.len()).rev())
        .map(|(row, row_load)| row.iter().filter(|&&tile| tile == 'O').count() * row_load)
        .sum()
}
