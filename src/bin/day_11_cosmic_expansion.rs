use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let image = input
        .lines()
        .map(|l| l.chars().map(|ch| ch == '#').collect())
        .collect();

    let ans_1 = galaxy_distances_sum(&image, 2);
    let ans_2 = galaxy_distances_sum(&image, 1_000_000);
    println!("{ans_1} {ans_2}");
    Ok(())
}

type Image = Vec<Vec<bool>>;

fn galaxy_distances_sum(image: &Image, expansion_factor: u64) -> u64 {
    galaxy_positions_after_expansion(image, expansion_factor)
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum()
}

fn galaxy_positions_after_expansion(image: &Image, expansion_factor: u64) -> Vec<(u64, u64)> {
    let mut galaxies = find_galaxies(image);

    let height = image.len();
    for row in (0..height).rev() {
        let row_is_empty = !image[row].iter().contains(&true);
        if row_is_empty {
            for (_, y) in galaxies.iter_mut().filter(|(_, y)| *y > row as u64) {
                *y += expansion_factor - 1;
            }
        }
    }

    let width = image[0].len();
    for col in (0..width).rev() {
        let col_is_empty = !image.iter().map(|row| row[col]).contains(&true);
        if col_is_empty {
            for (x, _) in galaxies.iter_mut().filter(|(x, _)| *x > col as u64) {
                *x += expansion_factor - 1;
            }
        }
    }

    galaxies
}

fn find_galaxies(image: &Image) -> Vec<(u64, u64)> {
    image
        .iter()
        .zip(0..)
        .flat_map(|(row, y)| {
            row.iter()
                .zip(0..)
                .filter(|(tile, _x)| **tile)
                .map(move |(_t, x)| (x, y))
        })
        .collect()
}
