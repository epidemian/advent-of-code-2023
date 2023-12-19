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
type Node = ((usize, usize), (isize, isize), u32);

fn find_min_heat_loss(
    city: &Grid,
    min_straight_len: u32,
    max_straight_len: u32,
) -> aoc::Result<u32> {
    let height = city.len();
    let width = city[0].len();
    let start: Node = ((0, 0), (0, 0), 0);
    let success = |&(pos, _d, straight_len): &Node| {
        pos == (width - 1, height - 1) && straight_len >= min_straight_len
    };
    let successors = |&((x, y), dir, straight_len): &Node| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                let cost = *city.get(ny)?.get(nx)?;
                if dir == (-dx, -dy) {
                    return None;
                }
                if dir != (0, 0) && (dx, dy) != dir && straight_len < min_straight_len {
                    return None;
                }
                let neighbor_straight_len = if (dx, dy) == dir { straight_len + 1 } else { 1 };
                if neighbor_straight_len > max_straight_len {
                    return None;
                }
                Some((((nx, ny), (dx, dy), neighbor_straight_len), cost))
            })
    };

    let (path, min_heat_loss) = dijkstra(&start, successors, success)
        .ok_or("couldn't find a path to the machine parts factory")?;

    debug_print_path(city, &path);

    Ok(min_heat_loss)
}

fn debug_print_path(city: &Grid, path: &[Node]) {
    if env::var("DEBUG").is_err() {
        return;
    }
    for (y, row) in city.iter().enumerate() {
        let mut line = String::new();
        for (x, block) in row.iter().enumerate() {
            if let Some((_, dir, _)) = path.iter().find(|&&(pos, ..)| pos == (x, y)) {
                let ch = match dir {
                    (0, 0) => 'S',
                    (-1, 0) => '<',
                    (1, 0) => '>',
                    (0, -1) => '^',
                    (0, 1) => 'v',
                    _ => unreachable!(),
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
