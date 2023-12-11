fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let start_pos = find_start_position(&grid).ok_or("start position not found")?;
    let start_dir = [Up, Right, Down, Left]
        .into_iter()
        .find(|d| try_move(start_pos, *d, &grid).is_some())
        .ok_or("no valid direction from start position found")?;

    // Part 1
    let mut is_loop_grid = vec![vec![false; width]; height];
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut count = 0;
    loop {
        (pos, dir) = try_move(pos, dir, &grid)
            .ok_or_else(|| format!("invalid turn, going {dir:?} from {pos:?}"))?;
        count += 1;
        is_loop_grid[pos.1 as usize][pos.0 as usize] = true;

        if pos == start_pos {
            break;
        }
    }
    let half_pipe_length = count / 2;

    // Part 2
    let mut expanded_grid = expand_grid(&grid)?;
    flood_fill(&mut expanded_grid);
    let enclosed_count = count_enclosed_tiles(&grid, &expanded_grid);

    println!("{half_pipe_length} {enclosed_count}");
    Ok(())
}

fn try_move((x, y): Point, dir: Dir, grid: &Grid) -> Option<(Point, Dir)> {
    let new_pos = match dir {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    };
    let ch = at(new_pos, grid)?;
    let new_dir = match (dir, ch) {
        (Left | Right, '-') => dir,
        (Up | Down, '|') => dir,
        (Up, '7') => Left,
        (Up, 'F') => Right,
        (Right, 'J') => Up,
        (Right, '7') => Down,
        (Down, 'J') => Left,
        (Down, 'L') => Right,
        (Left, 'L') => Up,
        (Left, 'F') => Down,
        (_, 'S') => dir,
        _ => return None,
    };
    Some((new_pos, new_dir))
}

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
use itertools::Itertools;
use Dir::*;

fn at((x, y): Point, grid: &Grid) -> Option<char> {
    let ch = *grid.get(y as usize)?.get(x as usize)?;
    Some(ch)
}

fn find_start_position(grid: &Grid) -> Option<Point> {
    for (row, y) in grid.iter().zip(0..) {
        for (ch, x) in row.iter().zip(0..) {
            if *ch == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn expand_grid(grid: &Grid) -> aoc::Result<Vec<Vec<u8>>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut expanded_grid = vec![vec![0; width * 3]; height * 3];
    for (row, y) in grid.iter().zip(0..) {
        for (ch, x) in row.iter().zip(0..) {
            let expanded_tile = expand_tile(ch)?;
            for tile_y in 0..3 {
                for tile_x in 0..3 {
                    expanded_grid[y * 3 + tile_y][x * 3 + tile_x] = expanded_tile[tile_y][tile_x];
                }
            }
        }
    }
    Ok(expanded_grid)
}

const EMPTY: u8 = 0;
const WALL: u8 = 1;
const FILLED: u8 = 2;

fn flood_fill(expanded_grid: &mut [Vec<u8>]) {
    let mut unvisited = vec![(0, 0)];
    while let Some((x, y)) = unvisited.pop() {
        if expanded_grid[y][x] != EMPTY {
            continue;
        }
        expanded_grid[y][x] = FILLED;
        let neighbors = [
            (x, y.wrapping_sub(1)),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x + 1, y),
        ];
        for (nx, ny) in neighbors {
            let neighbor_tile = expanded_grid.get(ny).and_then(|row| row.get(nx));
            if let Some(&EMPTY) = neighbor_tile {
                unvisited.push((nx, ny))
            }
        }
    }
}

fn count_enclosed_tiles(grid: &Grid, expanded_grid: &[Vec<u8>]) -> u32 {
    let mut enclosed_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _ch) in row.iter().enumerate() {
            let is_enclosed = (0..3)
                .cartesian_product(0..3)
                .all(|(dx, dy)| expanded_grid[y * 3 + dy][x * 3 + dx] != FILLED);
            enclosed_count += is_enclosed as u32;
        }
    }
    enclosed_count
}

#[allow(non_snake_case)]
fn expand_tile(ch: &char) -> aoc::Result<[[u8; 3]; 3]> {
    let W = WALL;
    let O = EMPTY;
    Ok(match *ch {
        '|' => [
            [O, W, O], //
            [O, W, O],
            [O, W, O],
        ],
        '-' => [
            [O, O, O], //
            [W, W, W],
            [O, O, O],
        ],
        'L' => [
            [O, W, O], //
            [O, W, W],
            [O, O, O],
        ],
        'J' => [
            [O, W, O], //
            [W, W, O],
            [O, O, O],
        ],
        '7' => [
            [O, O, O], //
            [W, W, O],
            [O, W, O],
        ],
        'F' => [
            [O, O, O], //
            [O, W, W],
            [O, W, O],
        ],
        '.' => [
            [O, O, O], //
            [O, O, O],
            [O, O, O],
        ],
        'S' => [
            [O, W, O], // Leave some gaps in the corner so the flood-fill can get in.
            [W, W, W],
            [O, W, O],
        ],
        _ => Err(format!("unexpected character {ch}"))?,
    })
}
