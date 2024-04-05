use itertools::{iproduct, Itertools};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let image = aoc::parse_grid(&input, |ch| Ok(ch == '#'))?;

    let ans_1 = galaxy_distances_sum(&image, 2);
    let ans_2 = galaxy_distances_sum(&image, 1_000_000);
    println!("{ans_1} {ans_2}");
    Ok(())
}

type Image = (Vec<Vec<bool>>, usize, usize);

fn galaxy_distances_sum(image: &Image, expansion_factor: usize) -> usize {
    galaxy_positions_after_expansion(image, expansion_factor)
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum()
}

fn galaxy_positions_after_expansion(image: &Image, expansion_factor: usize) -> Vec<(usize, usize)> {
    let (ref pixels, width, height) = *image;

    let mut galaxies: Vec<_> = iproduct!(0..width, 0..height)
        .filter(|&(x, y)| pixels[y][x])
        .collect();

    for row in (0..height).rev() {
        let row_is_empty = !pixels[row].iter().contains(&true);
        if row_is_empty {
            for (_, y) in galaxies.iter_mut().filter(|(_, y)| *y > row) {
                *y += expansion_factor - 1;
            }
        }
    }

    for col in (0..width).rev() {
        let col_is_empty = !pixels.iter().map(|row| row[col]).contains(&true);
        if col_is_empty {
            for (x, _) in galaxies.iter_mut().filter(|(x, _)| *x > col) {
                *x += expansion_factor - 1;
            }
        }
    }

    galaxies
}
