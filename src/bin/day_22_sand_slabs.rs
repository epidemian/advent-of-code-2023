use itertools::Itertools;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let mut bricks: Vec<_> = input.lines().map(parse_brick).try_collect()?;
    bricks.sort_by_key(|(start, _end)| start.2);

    let mut grid = HashMap::new();
    let mut supports = vec![vec![]; bricks.len()];

    for (brick_id, brick) in bricks.iter_mut().enumerate() {
        loop {
            let new_brick = fall_1(*brick);
            if new_brick.0 .2 == 0 {
                for point in brick_points(*brick) {
                    grid.insert(point, brick_id);
                }
                break;
            }

            let mut collides = false;
            for new_point in brick_points(new_brick) {
                if let Some(&other_brick_id) = grid.get(&new_point) {
                    if !supports[other_brick_id].contains(&brick_id) {
                        supports[other_brick_id].push(brick_id);
                    }
                    collides = true;
                };
            }
            if collides {
                for point in brick_points(*brick) {
                    grid.insert(point, brick_id);
                }
                break;
            }

            *brick = new_brick;
        }
    }

    let safe_disintegration_count = supports
        .iter()
        .enumerate()
        .filter(|&(brick_id, supported_bricks)| {
            supported_bricks.iter().all(|supported_brick_id| {
                supports
                    .iter()
                    .enumerate()
                    .any(|(other_brick_id, other_supported_bricks)| {
                        other_brick_id != brick_id
                            && other_supported_bricks.contains(supported_brick_id)
                    })
            })
        })
        .count();

    println!("{safe_disintegration_count}");

    Ok(())
}

type Point = (u32, u32, u32);
type Brick = (Point, Point);

fn fall_1(((x1, y1, z1), (x2, y2, z2)): Brick) -> Brick {
    ((x1, y1, z1 - 1), (x2, y2, z2 - 1))
}

// TODO: Remove dynamic dispatch
fn brick_points(((x1, y1, z1), (x2, y2, z2)): Brick) -> Box<dyn Iterator<Item = Point>> {
    if x1 != x2 {
        return Box::new((x1..=x2).map(move |x| (x, y1, z1)));
    }
    if y1 != y2 {
        return Box::new((y1..=y2).map(move |y| (x1, y, z1)));
    }
    Box::new((z1..=z2).map(move |z| (x1, y1, z)))
}

fn parse_brick(line: &str) -> aoc::Result<Brick> {
    let (x1, y1, z1, x2, y2, z2) = aoc::parse_numbers(line)?
        .into_iter()
        .collect_tuple()
        .ok_or("invalid brick line")?;
    assert!(x1 <= x2);
    assert!(y1 <= y2);
    assert!(z1 <= z2);

    Ok(((x1, y1, z1), (x2, y2, z2)))
}
