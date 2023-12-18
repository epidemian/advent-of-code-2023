use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let city: &Vec<Vec<u32>> = &input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| ch.to_digit(10).ok_or("unexpected non-digit character"))
                .try_collect()
        })
        .try_collect()?;
    let height = city.len();
    let width = city[0].len();
    let goal = (width - 1, height - 1);
    let start = (0usize, 0usize, Dir::None, 0);

    let (_path, min_heat_loss_p1) = dijkstra(
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
                let neighbor_straight_len = if nd == dir { straight_len + 1 } else { 1 };
                if neighbor_straight_len >= 4 {
                    return None;
                }
                Some(((nx, ny, nd, neighbor_straight_len), cost))
            })
        },
        |&(x, y, ..)| (x, y) == goal,
    )
    .ok_or("couldn't find a path to the machine parts factory")?;

    let (_path, min_heat_loss_p2) = dijkstra(
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
                if dir != Dir::None && nd != dir && straight_len < 4 {
                    return None;
                }
                let neighbor_straight_len = if nd == dir { straight_len + 1 } else { 1 };
                if neighbor_straight_len >= 11 {
                    return None;
                }
                Some(((nx, ny, nd, neighbor_straight_len), cost))
            })
        },
        |&(x, y, _d, straight_len)| (x, y) == goal && straight_len >= 4,
    )
    .ok_or("couldn't find a path to the machine parts factory with ultra crucibles")?;

    println!("{min_heat_loss_p1} {min_heat_loss_p2}");

    Ok(())
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
