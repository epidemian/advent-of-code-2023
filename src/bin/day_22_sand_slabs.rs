use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let bricks = input.lines().map(parse_brick).try_collect()?;
    let (supports, supported_by) = fall_bricks(bricks);

    let safe_disintegration_count = supports
        .iter()
        .enumerate()
        .filter(|&(brick_id, supported_bricks)| {
            supported_bricks.iter().all(|&supported_brick_id| {
                supported_by[supported_brick_id]
                    .iter()
                    .any(|&other_brick_id| other_brick_id != brick_id)
            })
        })
        .count();

    let total_fall_sum: usize = supports
        .iter()
        .enumerate()
        .map(|(brick_id, _supported_bricks)| {
            let mut falling_bricks = HashSet::from_iter([brick_id]);
            count_falls_if_disintegrated(brick_id, &supports, &supported_by, &mut falling_bricks);
            falling_bricks.len() - 1
        })
        .sum();

    println!("{safe_disintegration_count} {total_fall_sum}");

    Ok(())
}

fn fall_bricks(mut bricks: Vec<Vec<Point>>) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    bricks.sort_by_key(|brick| brick[0].2);

    let mut grid = HashMap::<Point, usize>::new();
    let mut supports = vec![HashSet::new(); bricks.len()];
    let mut supported_by = vec![HashSet::new(); bricks.len()];

    for (brick_id, brick) in bricks.iter_mut().enumerate() {
        loop {
            let supporting_bricks: HashSet<_> = brick
                .iter()
                .filter_map(|&(x, y, z)| grid.get(&(x, y, z - 1)).copied())
                .collect();
            for &supporting_brick in supporting_bricks.iter() {
                supports[supporting_brick].insert(brick_id);
            }
            let collides = !supporting_bricks.is_empty();
            supported_by[brick_id] = supporting_bricks;

            if collides || brick[0].2 == 1 {
                for &point in brick.iter() {
                    grid.insert(point, brick_id);
                }
                break;
            }

            for (_x, _y, z) in brick.iter_mut() {
                *z -= 1;
            }
        }
    }
    (supports, supported_by)
}

type Point = (u32, u32, u32);

fn count_falls_if_disintegrated(
    brick_id: usize,
    supports: &[HashSet<usize>],
    supported_by: &[HashSet<usize>],
    falling_bricks: &mut HashSet<usize>,
) {
    let unsupported_bricks: Vec<_> = supports[brick_id]
        .iter()
        .copied()
        .filter(|&supported_brick_id| {
            !supported_by[supported_brick_id]
                .iter()
                .any(|other_brick_id| !falling_bricks.contains(other_brick_id))
        })
        .collect();

    falling_bricks.extend(unsupported_bricks.iter());
    for &unsupported_brick_id in unsupported_bricks.iter() {
        count_falls_if_disintegrated(unsupported_brick_id, supports, supported_by, falling_bricks)
    }
}

fn parse_brick(line: &str) -> aoc::Result<Vec<Point>> {
    let (x1, y1, z1, x2, y2, z2) = aoc::parse_numbers(line)?
        .into_iter()
        .collect_tuple()
        .ok_or("invalid brick line")?;
    assert!(x1 <= x2);
    assert!(y1 <= y2);
    assert!(z1 <= z2);

    let points = if x1 != x2 {
        (x1..=x2).map(|x| (x, y1, z1)).collect()
    } else if y1 != y2 {
        (y1..=y2).map(|y| (x1, y, z1)).collect()
    } else {
        (z1..=z2).map(|z| (x1, y1, z)).collect()
    };

    Ok(points)
}
