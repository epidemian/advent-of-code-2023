fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let mut grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    tilt_north(&mut grid);
    // for row in grid {
    //     println!("{}", String::from_iter(row));
    // }
    let north_beams_load: usize = grid
        .iter()
        .zip((1..=grid.len()).rev())
        .map(|(row, row_load)| row.iter().filter(|&&tile| tile == 'O').count() * row_load)
        .sum();

    println!("{north_beams_load}");
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
