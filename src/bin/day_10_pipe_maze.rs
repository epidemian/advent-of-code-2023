use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    let half_pipe_length = measure_pipe_loop(&grid)? / 2;
    let enclosed_count = count_enclosed_tiles(&grid);

    println!("{half_pipe_length} {enclosed_count}");
    Ok(())
}

// Part 1 stuff
type Grid = Vec<Vec<char>>;
type Point = (usize, usize);

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
use Dir::*;

fn measure_pipe_loop(grid: &Grid) -> aoc::Result<u32> {
    let start_pos = find_start_position(grid).ok_or("start position not found")?;
    let start_dir = [Up, Right, Down, Left]
        .into_iter()
        .find(|d| try_move(start_pos, *d, grid).is_some())
        .ok_or("no valid direction from start position found")?;
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut count = 0;
    loop {
        (pos, dir) = try_move(pos, dir, grid)
            .ok_or_else(|| format!("invalid turn, going {dir:?} from {pos:?}"))?;
        count += 1;
        if pos == start_pos {
            break;
        }
    }
    Ok(count)
}

fn try_move(pos: Point, dir: Dir, grid: &Grid) -> Option<(Point, Dir)> {
    let (mut x, mut y) = pos;
    match dir {
        Up => y = y.wrapping_sub(1),
        Down => y += 1,
        Left => x = x.wrapping_sub(1),
        Right => x += 1,
    };
    let ch = grid.get(y)?.get(x)?;
    let new_dir = match (dir, ch) {
        (Left | Right, '-') => dir,
        (Up | Down, '|') => dir,
        (Up, '7') | (Down, 'J') => Left,
        (Up, 'F') | (Down, 'L') => Right,
        (Left, 'L') | (Right, 'J') => Up,
        (Left, 'F') | (Right, '7') => Down,
        (_, 'S') => dir,
        _ => return None,
    };
    Some(((x, y), new_dir))
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

// Part 2 stuff
type FloodFillGrid = Vec<Vec<FloodFillTile>>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FloodFillTile {
    Empty,
    Wall,
    Filled,
}
use FloodFillTile::*;

// This solution to part 2 assumes that there are no other loops of pipe on the grid besides giant
// giant one. And that there's no 'J' or 'S' on the top left corner of the grid, which would mess up
// the flood-filling starting there. If these assumptions would not hold, we'd need to clean up the
// grid of all the junk pipes outside the giant loop before doing the flood-fill.
fn count_enclosed_tiles(grid: &Grid) -> u32 {
    let mut expanded_grid = expand_grid(grid);
    flood_fill(&mut expanded_grid);

    let mut enclosed_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _ch) in row.iter().enumerate() {
            let is_enclosed = (0..3)
                .cartesian_product(0..3)
                .all(|(dx, dy)| expanded_grid[y * 3 + dy][x * 3 + dx] != Filled);
            enclosed_count += is_enclosed as u32;
        }
    }
    enclosed_count
}

fn expand_grid(grid: &Grid) -> FloodFillGrid {
    let height = grid.len();
    let width = grid[0].len();
    let mut expanded_grid = vec![vec![Empty; width * 3]; height * 3];
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            let expanded_tile = expand_tile(*ch);
            for (tx, ty) in (0..3).cartesian_product(0..3) {
                expanded_grid[y * 3 + ty][x * 3 + tx] = expanded_tile[ty][tx];
            }
        }
    }
    expanded_grid
}

fn expand_tile(ch: char) -> [[FloodFillTile; 3]; 3] {
    #[allow(non_snake_case)]
    let W = Wall;
    let e = Empty;
    match ch {
        '|' => [
            [e, W, e], //
            [e, W, e],
            [e, W, e],
        ],
        '-' => [
            [e, e, e], //
            [W, W, W],
            [e, e, e],
        ],
        'L' => [
            [e, W, e], //
            [e, W, W],
            [e, e, e],
        ],
        'J' => [
            [e, W, e], //
            [W, W, e],
            [e, e, e],
        ],
        '7' => [
            [e, e, e], //
            [W, W, e],
            [e, W, e],
        ],
        'F' => [
            [e, e, e], //
            [e, W, W],
            [e, W, e],
        ],
        'S' => [
            [e, W, e], // Leave some gaps in the corner so the flood-fill can get in.
            [W, W, W],
            [e, W, e],
        ],
        _ => [
            [e, e, e], //
            [e, e, e],
            [e, e, e],
        ],
    }
}

fn flood_fill(expanded_grid: &mut FloodFillGrid) {
    let mut unvisited = vec![(0, 0)];
    while let Some((x, y)) = unvisited.pop() {
        expanded_grid[y][x] = Filled;
        let neighbors = [
            (x, y.wrapping_sub(1)),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x + 1, y),
        ];
        for (nx, ny) in neighbors {
            let neighbor_tile = expanded_grid.get(ny).and_then(|row| row.get(nx));
            if let Some(&Empty) = neighbor_tile {
                unvisited.push((nx, ny))
            }
        }
    }
}
