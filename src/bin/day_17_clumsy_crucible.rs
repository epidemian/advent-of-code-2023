use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::env;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let parse_digit = |ch: char| ch.to_digit(10).ok_or("unexpected non-digit character");
    let city = input
        .lines()
        .map(|l| l.chars().map(parse_digit).try_collect())
        .try_collect()?;

    let min_heat_loss_p1 = find_min_heat_loss(&city, 0, 3)?;
    let min_heat_loss_p2 = find_min_heat_loss(&city, 4, 10)?;
    println!("{min_heat_loss_p1} {min_heat_loss_p2}");
    Ok(())
}

type Grid = Vec<Vec<u32>>;

fn find_min_heat_loss(
    city: &Grid,
    min_straight_len: u32,
    max_straight_len: u32,
) -> aoc::Result<u32> {
    let height = city.len();
    let width = city[0].len();
    let start = (0_usize, 0_usize, Dir::None, 0);
    let goal = (width - 1, height - 1);
    let (path, min_heat_loss) = dijkstra(
        &start,
        |&(x, y, dir, straight_len)| {
            [
                (x + 1, y, Dir::Right),
                (x, y + 1, Dir::Down),
                (x.wrapping_sub(1), y, Dir::Left),
                (x, y.wrapping_sub(1), Dir::Up),
            ]
            .into_iter()
            .filter_map(move |(nx, ny, nd)| {
                let cost = *city.get(ny)?.get(nx)?;
                if nd == dir.opposite() {
                    return None;
                }
                if dir != Dir::None && nd != dir && straight_len < min_straight_len {
                    return None;
                }
                let neighbor_straight_len = if nd == dir { straight_len + 1 } else { 1 };
                if neighbor_straight_len > max_straight_len {
                    return None;
                }
                Some(((nx, ny, nd, neighbor_straight_len), cost))
            })
        },
        |&(x, y, _d, straight_len)| (x, y) == goal && straight_len >= min_straight_len,
    )
    .ok_or("couldn't find a path to the machine parts factory")?;

    debug_print_path(city, path);

    Ok(min_heat_loss)
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    None,
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::None => Dir::None,
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }
}

fn debug_print_path(city: &Grid, path: Vec<(usize, usize, Dir, u32)>) {
    if env::var("DEBUG").is_err() {
        return;
    }
    for (y, row) in city.iter().enumerate() {
        let mut line = String::new();
        for (x, block) in row.iter().enumerate() {
            if let Some((_, _, d, _)) = path.iter().find(|&&(px, py, ..)| (px, py) == (x, y)) {
                let ch = match d {
                    Dir::None => 'S',
                    Dir::Up => '^',
                    Dir::Right => '>',
                    Dir::Down => 'v',
                    Dir::Left => '<',
                };
                line.push_str(&format!("\x1b[34;1m{ch}\x1b[0m"));
            } else {
                line.push_str(&block.to_string());
            }
        }
        println!("{line}");
    }
    println!();
}
