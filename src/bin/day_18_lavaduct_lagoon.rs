use itertools::Itertools;
use std::{collections::HashSet, env};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let dig_plan: Vec<_> = input.lines().map(parse_instruction).try_collect()?;
    let (mut x, mut y) = (0, 0);
    let mut trench: HashSet<(i32, i32)> = HashSet::from_iter([(x, y)]);
    let (mut min_x, mut min_y) = (0, 0);
    let (mut max_x, mut max_y) = (0, 0);
    for (dir, count) in dig_plan {
        for _ in 0..count {
            match dir {
                Dir::Up => y -= 1,
                Dir::Down => y += 1,
                Dir::Left => x -= 1,
                Dir::Right => x += 1,
            }
            trench.insert((x, y));
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
    }
    dig_interior(&mut trench);
    println!("{}", trench.len());

    if env::var("DEBUG").is_ok() {
        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                line.push(if (x, y) == (0, 0) {
                    '█'
                } else if trench.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                })
            }
            println!("{line}");
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn dig_interior(trench: &mut HashSet<(i32, i32)>) {
    // TODO: de-hardcode starting interior point.
    let mut to_dig = vec![(0, -1)];
    while let Some((x, y)) = to_dig.pop() {
        trench.insert((x, y));
        let neighbors = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
        for (nx, ny) in neighbors {
            if !trench.contains(&(nx, ny)) {
                to_dig.push((nx, ny))
            }
        }
    }
}

fn parse_instruction(s: &str) -> aoc::Result<(Dir, u32)> {
    let &[dir, count, _hex_color] = &s.split_whitespace().collect_vec()[..].try_into()?;
    let dir = match dir {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => Err(format!("unexpected direction '{dir}'"))?,
    };
    Ok((dir, count.parse()?))
}
