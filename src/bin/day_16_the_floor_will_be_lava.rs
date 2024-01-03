use std::iter;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    let initial_energized_tiles = count_energized_tiles(&grid, 0, 0, RIGHT)?;

    let height = grid.len();
    let width = grid[0].len();
    let all_starting_beams = iter::empty()
        .chain((0..height).map(|y| (width - 1, y, LEFT)))
        .chain((0..height).map(|y| (0, y, RIGHT)))
        .chain((0..width).map(|x| (x, height - 1, UP)))
        .chain((0..width).map(|x| (x, 0, DOWN)));
    let mut max_energized_tiles = 0;
    for (x, y, dir) in all_starting_beams {
        let energized_tiles = count_energized_tiles(&grid, x, y, dir)?;
        max_energized_tiles = max_energized_tiles.max(energized_tiles)
    }

    println!("{initial_energized_tiles} {max_energized_tiles}");
    Ok(())
}

const EMPTY: u32 = 0;
const UP: u32 = 1;
const LEFT: u32 = 2;
const DOWN: u32 = 4;
const RIGHT: u32 = 8;

fn count_energized_tiles(
    grid: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    start_dir: u32,
) -> aoc::Result<u32> {
    let mut beams_grid = vec![vec![EMPTY; grid[0].len()]; grid.len()];
    let mut beams = vec![(start_x, start_y, start_dir)];
    while let Some(beam) = beams.pop() {
        let (mut x, mut y, mut dir) = beam;
        loop {
            let Some(&ch) = grid.get(y).and_then(|row| row.get(x)) else {
                // Beam goes out of bounds.
                break;
            };
            if beams_grid[y][x] & dir != 0 {
                break;
            }
            beams_grid[y][x] |= dir;
            match (dir, ch) {
                (_, '.') | (LEFT | RIGHT, '-') | (UP | DOWN, '|') => {}
                // Reflections
                (RIGHT, '/') | (LEFT, '\\') => dir = UP,
                (RIGHT, '\\') | (LEFT, '/') => dir = DOWN,
                (UP, '/') | (DOWN, '\\') => dir = RIGHT,
                (UP, '\\') | (DOWN, '/') => dir = LEFT,
                // Splits
                (LEFT | RIGHT, '|') => {
                    beams.push((x, y.wrapping_sub(1), UP));
                    beams.push((x, y + 1, DOWN));
                    break;
                }
                (UP | DOWN, '-') => {
                    beams.push((x.wrapping_sub(1), y, LEFT));
                    beams.push((x + 1, y, RIGHT));
                    break;
                }
                _ => anyhow::bail!("unexpected tile '{ch}' in direction {dir}"),
            }
            match dir {
                LEFT => x = x.wrapping_sub(1),
                RIGHT => x += 1,
                UP => y = y.wrapping_sub(1),
                DOWN => y += 1,
                _ => unreachable!(),
            }
        }
    }
    let mut energized_tiles_count = 0;
    for row in beams_grid.iter() {
        for &tile_beams in row.iter() {
            if tile_beams != 0 {
                energized_tiles_count += 1;
            }
        }
    }
    Ok(energized_tiles_count)
}
