use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let mut grid = input
        .lines()
        .map(|l| l.chars().map(|ch| ch == '#').collect())
        .collect();
    expand_universe(&mut grid);

    let distance_sum: usize = galaxy_positions(&grid)
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum();

    println!("{distance_sum}");

    Ok(())
}

type Grid = Vec<Vec<bool>>;

fn expand_universe(grid: &mut Grid) {
    let height = grid.len();
    let width = grid[0].len();

    for y in (0..height).rev() {
        let row_is_empty = !grid[y].iter().contains(&true);
        if row_is_empty {
            grid.insert(y, vec![false; width]);
        }
    }

    for x in (0..width).rev() {
        let col_is_empty = !grid.iter().map(|row| row[x]).contains(&true);
        if col_is_empty {
            for row in grid.iter_mut() {
                row.insert(x, false);
            }
        }
    }
}

fn galaxy_positions(grid: &Grid) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_x, tile)| **tile)
                .map(move |(x, _t)| (x, y))
        })
        .collect()
}
