use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_reach;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = &input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let start_pos = find_start_position(grid).ok_or("starting position not found")?;

    let reachable_tiles = dijkstra_reach(&start_pos, |&(x, y), cost| {
        let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        dirs.into_iter().filter_map(move |(dx, dy)| {
            if cost >= 64 {
                return None;
            }
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            let tile = *grid.get(ny)?.get(nx)?;
            if tile == '#' {
                return None;
            }
            Some(((nx, ny), 1))
        })
    });
    let reachable_tiles_count = reachable_tiles
        .filter(|item| item.total_cost % 2 == 0)
        .count();
    println!("{reachable_tiles_count}");

    Ok(())
}

fn find_start_position(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}
