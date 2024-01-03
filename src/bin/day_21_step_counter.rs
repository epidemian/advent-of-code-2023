use anyhow::Context;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_reach;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let start = find_start_position(&grid).context("starting position not found")?;

    let ans_1 = count_reachable_tiles(&grid, start, 64);
    let ans_2 = extrapolate_reachable_tiles(&grid, start, 26501365);
    println!("{ans_1} {ans_2}");

    Ok(())
}

type Grid = Vec<Vec<char>>;
type Point = (i64, i64);

fn count_reachable_tiles(grid: &Grid, start_pos: Point, steps_count: u64) -> u64 {
    let size = grid.len() as i64;
    let reachable_tiles = dijkstra_reach(&start_pos, |&(x, y), cost| {
        let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        dirs.into_iter().filter_map(move |(dx, dy)| {
            if cost >= steps_count {
                return None;
            }
            let (nx, ny) = (x + dx, y + dy);
            let tile = grid[ny.rem_euclid(size) as usize][nx.rem_euclid(size) as usize];
            if tile == '#' {
                return None;
            }
            Some(((nx, ny), 1))
        })
    });
    let evenness = steps_count % 2;
    reachable_tiles
        .filter(|item| item.total_cost % 2 == evenness)
        .count() as u64
}

fn extrapolate_reachable_tiles(grid: &Grid, start: Point, steps_count: i64) -> i64 {
    let size = grid.len() as i64;

    // For part 2, we do a quadratic extrapolation of this function.
    let f = |x: i64| count_reachable_tiles(grid, start, (size / 2 + size * x) as u64) as i64;

    // In case of the actual input, this function is:
    // f(x) = #reachable_tiles(x * 131 + 65)
    //
    // And we're assuming this behaves quadratically because the input map has the start position in
    // the middle and allows the elf to walk to any of its border tiles in a "straight line" (i.e.,
    // moving straight left/right and then up/down. This is because all tiles up, down, left or
    // right from the start are clear, as well as all the tiles in the border. So all of the grid
    // copies in the infinite 2D plane will be reachable for walking in the same way.
    //
    // If f is quadratic, it can be expressed as:
    // f(x) = ax² + bx + c
    // So we need to find the coefficients a, b, c. We can do so by evaluating the function a couple
    // of times and getting the coefficients from there:
    // f(0) = c
    // f(1) = a + b + c
    // f(2) = 4a + 2b + c
    let f_0 = f(0);
    let f_1 = f(1);
    let f_2 = f(2);
    // c = f(0)
    let c = f_0;
    // f(2) - 2*f(1) = 2a - c
    // a = (f(2) - 2*f(1) + c) / 2
    let a = (f_2 - 2 * f_1 + c) / 2;
    // b = f(1) - a - c
    let b = f_1 - a - c;

    let f_quadratic = |x| a * x * x + b * x + c;

    assert_eq!(f_0, f_quadratic(0));
    assert_eq!(f_1, f_quadratic(1));
    assert_eq!(f_2, f_quadratic(2));

    f_quadratic(steps_count / size)
}

fn find_start_position(grid: &Grid) -> Option<Point> {
    for (row, y) in grid.iter().zip(0..) {
        for (&tile, x) in row.iter().zip(0..) {
            if tile == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}
