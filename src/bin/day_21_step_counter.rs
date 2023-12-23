use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_reach;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let size = grid.len() as i64;
    let start = (size / 2, size / 2);
    if grid[start.1 as usize][start.0 as usize] != 'S' {
        Err("expected to start in the middle of the grid")?
    }

    let ans_1 = count_reachable_tiles(&grid, start, 64);

    let part_2_step_count = 26501365;
    assert_eq!(part_2_step_count % size, size / 2);

    // For part 2, we do a quadratic extrapolation of this function.
    // TODO: explain why we're assuming that this is quadratic.
    // f(x) = ax² + bx + c
    let f = |x: i64| count_reachable_tiles(&grid, start, (size / 2 + size * x) as u64) as i64;
    let f_0 = f(0);
    let f_1 = f(1);
    let f_2 = f(2);

    // Find coefficients a, b, c
    // f(0) = c
    // f(1) = a + b + c
    // f(2) = 4a + 2b + c
    // c = f(0)
    let c = f_0;
    // f(2) - 2*f(1) = 2a - c
    // a = (f(2) - 2*f(1) + c) / 2
    let a = (f_2 - 2 * f_1 + c) / 2;
    // b = f(1) - a - c
    let b = f_1 - a - c;
    let f_quadratic = |x: i64| a * x * x + b * x + c;

    assert_eq!(f_0, f_quadratic(0));
    assert_eq!(f_1, f_quadratic(1));
    assert_eq!(f_2, f_quadratic(2));

    let ans_2 = f_quadratic(part_2_step_count / size);
    println!("{ans_1} {ans_2}");

    Ok(())
}

type Grid = Vec<Vec<char>>;

fn count_reachable_tiles(grid: &Grid, start_pos: (i64, i64), steps_count: u64) -> u64 {
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
    let reachable_tiles_count = reachable_tiles
        .filter(|item| item.total_cost % 2 == evenness)
        .count() as u64;
    reachable_tiles_count
}
