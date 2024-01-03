use anyhow::Context;
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let bricks = input.lines().map(parse_brick).try_collect()?;
    let (supports, supported_by) = fall_bricks(bricks);

    let fall_counts = supports
        .iter()
        .enumerate()
        .map(|(brick_id, _supported_bricks)| {
            count_falls_if_disintegrated(brick_id, &supports, &supported_by)
        })
        .collect_vec();

    let safe_disintegration_count = fall_counts.iter().filter(|&&count| count == 0).count();
    let total_fall_sum: usize = fall_counts.iter().sum();

    println!("{safe_disintegration_count} {total_fall_sum}");

    Ok(())
}

type Point = (u32, u32, u32);

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

fn count_falls_if_disintegrated(
    brick_id: usize,
    supports: &[HashSet<usize>],
    supported_by: &[HashSet<usize>],
) -> usize {
    let mut falling_bricks: HashSet<_> = HashSet::from_iter([brick_id]);

    let mut to_fall = vec![brick_id];
    while let Some(brick_id) = to_fall.pop() {
        let unsupported_bricks = supports[brick_id]
            .iter()
            .copied()
            .filter(|&supported_brick_id| {
                supported_by[supported_brick_id]
                    .iter()
                    .all(|other_brick_id| falling_bricks.contains(other_brick_id))
            })
            .collect_vec();

        falling_bricks.extend(unsupported_bricks.iter());
        to_fall.extend(unsupported_bricks);
    }

    // Subtract 1 because we don't want to count the initial brick itself.
    falling_bricks.len() - 1
}

fn parse_brick(line: &str) -> aoc::Result<Vec<Point>> {
    let (x1, y1, z1, x2, y2, z2) = aoc::parse_numbers(line)?
        .into_iter()
        .collect_tuple()
        .context("invalid brick line")?;
    anyhow::ensure!(x1 <= x2 && y1 <= y2 && z1 <= z2);

    let points = iproduct!(x1..=x2, y1..=y2, z1..=z2).collect();
    Ok(points)
}
