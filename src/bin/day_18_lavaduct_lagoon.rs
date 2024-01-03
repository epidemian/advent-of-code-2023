use anyhow::{bail, Context};
use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let dig_plan_p1: Vec<_> = input.lines().map(parse_instruction_p1).try_collect()?;
    let dig_plan_p2: Vec<_> = input.lines().map(parse_instruction_p2).try_collect()?;
    let lagoon_area_p1 = get_lagoon_area(&dig_plan_p1);
    let lagoon_area_p2 = get_lagoon_area(&dig_plan_p2);
    println!("{lagoon_area_p1} {lagoon_area_p2}");
    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn get_lagoon_area(dig_plan: &[(Dir, i64)]) -> i64 {
    let polygon = build_polygon(dig_plan);
    let polygon_area = get_polygon_area(&polygon);
    let polygon_perimeter: i64 = dig_plan.iter().map(|(_, c)| *c).sum();
    // The polygon area doesn't take into account the "thickness" of the trench, which adds this
    // extra area based on the polygon perimeter. Check sample_square.txt file for an easy to
    // understand example of this.
    polygon_area + polygon_perimeter / 2 + 1
}

fn build_polygon(dig_plan: &[(Dir, i64)]) -> Vec<(i64, i64)> {
    let (mut x, mut y) = (0, 0);
    let mut polygon = vec![(x, y)];
    for &(dir, count) in dig_plan.iter() {
        match dir {
            Dir::Up => y -= count,
            Dir::Down => y += count,
            Dir::Left => x -= count,
            Dir::Right => x += count,
        }
        polygon.push((x, y))
    }
    polygon
}

/// Calculates the area of a polygon using the Shoelace formula.
/// See https://en.wikipedia.org/wiki/Shoelace_formula
fn get_polygon_area(polygon: &[(i64, i64)]) -> i64 {
    let twice_area: i64 = polygon
        .iter()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum();
    twice_area / 2
}

fn parse_instruction_p1(s: &str) -> aoc::Result<(Dir, i64)> {
    let (dir, count, _hex) = s.split_whitespace().collect_tuple().context("bad input")?;
    let dir = match dir {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => bail!("unexpected direction '{dir}'"),
    };
    Ok((dir, count.parse()?))
}

fn parse_instruction_p2(s: &str) -> aoc::Result<(Dir, i64)> {
    let (_dir, _count, hex) = s.split_whitespace().collect_tuple().context("bad input")?;
    let hex = hex.get(2..8).context("invalid hex number")?;
    let count = i64::from_str_radix(&hex[0..5], 16)?;
    let dir = match &hex[5..] {
        "0" => Dir::Right,
        "1" => Dir::Down,
        "2" => Dir::Left,
        "3" => Dir::Up,
        d => bail!("unexpected direction '{d}'"),
    };
    Ok((dir, count))
}
