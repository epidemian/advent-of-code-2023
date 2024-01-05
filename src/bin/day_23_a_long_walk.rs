use itertools::Itertools;
use pathfinding::directed::yen::yen;
use std::env;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = &input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let height = grid.len();
    let width = grid[0].len();

    let k = 1000;
    let k_shortest_paths = yen(
        &(1usize, 0usize),
        |&(x, y)| {
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            dirs.into_iter().filter_map(move |(dx, dy)| {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                let tile = *grid.get(ny)?.get(nx)?;
                match tile {
                    '^' if dy == 1 => return None,
                    'v' if dy == -1 => return None,
                    '<' if dx == 1 => return None,
                    '>' if dx == -1 => return None,
                    '#' => return None,
                    _ => {}
                };
                if (tile == '>' && dx != 1) || (tile == 'v' && dy != 1) {
                    println!("found {tile} going in direction {dx},{dy}");
                }
                Some(((nx, ny), 1))
            })
        },
        |&(x, y)| x == width - 2 && y == height - 1,
        k,
    );
    anyhow::ensure!(k_shortest_paths.len() < k, "Found too many paths");

    let (longest_path, longest_path_length) = &k_shortest_paths
        .iter()
        .max_by_key(|(_path, cost)| cost)
        .unwrap();

    if env::var("DEBUG").is_ok() {
        for (y, row) in grid.iter().enumerate() {
            let mut line = String::new();
            for (x, tile) in row.iter().enumerate() {
                if longest_path.contains(&(x, y)) {
                    line.push_str("\x1b[34;1m");
                    line.push(*tile);
                    line.push_str("\x1b[0m");
                } else {
                    line.push(*tile);
                }
            }
            println!("{line}");
        }
    }

    println!("{longest_path_length}");

    Ok(())
}
