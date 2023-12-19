use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let city = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| ch.to_digit(10).ok_or("unexpected non-digit character"))
                .try_collect()
        })
        .try_collect()?;
    let min_heat_loss_p1 = find_min_heat_loss(&city, 0, 3)?;
    let min_heat_loss_p2 = find_min_heat_loss(&city, 4, 10)?;
    println!("{min_heat_loss_p1} {min_heat_loss_p2}");
    Ok(())
}

fn find_min_heat_loss(
    city: &Vec<Vec<u32>>,
    min_straight_len: u32,
    max_straight_len: u32,
) -> aoc::Result<u32> {
    let height = city.len();
    let width = city[0].len();
    let start = (0_usize, 0_usize, Dir::None, 0);
    let goal = (width - 1, height - 1);
    let (_path, min_heat_loss) = dijkstra(
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
    .ok_or("couldn't find a path to the machine parts factory with ultra crucibles")?;

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
