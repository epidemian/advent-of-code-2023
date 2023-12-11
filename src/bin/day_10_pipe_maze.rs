fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = find_start_position(&grid).ok_or("start position not found")?;
    let start_dir = [Up, Right, Down, Left]
        .into_iter()
        .find(|d| try_move(start_pos, *d, &grid).is_some())
        .ok_or("no valid direction from start position found")?;

    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut count = 0;
    loop {
        (pos, dir) = try_move(pos, dir, &grid)
            .ok_or_else(|| format!("invalid turn, going {dir:?} from {pos:?}"))?;
        count += 1;
        if pos == start_pos {
            break;
        }
    }
    let half_pipe_length = count / 2;

    println!("{half_pipe_length}");
    Ok(())
}

fn try_move((x, y): Point, dir: Dir, grid: &Grid) -> Option<(Point, Dir)> {
    let new_pos = match dir {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    };
    let ch = at(new_pos, grid)?;
    let new_dir = match (dir, ch) {
        (Left | Right, '-') => dir,
        (Up | Down, '|') => dir,
        (Up, '7') => Left,
        (Up, 'F') => Right,
        (Right, 'J') => Up,
        (Right, '7') => Down,
        (Down, 'J') => Left,
        (Down, 'L') => Right,
        (Left, 'L') => Up,
        (Left, 'F') => Down,
        (_, 'S') => dir,
        _ => return None,
    };
    Some((new_pos, new_dir))
}

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
use Dir::*;

fn at((x, y): Point, grid: &Grid) -> Option<char> {
    let ch = *grid.get(y as usize)?.get(x as usize)?;
    Some(ch)
}

fn find_start_position(grid: &Grid) -> Option<Point> {
    for (row, y) in grid.iter().zip(0..) {
        for (ch, x) in row.iter().zip(0..) {
            if *ch == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}
