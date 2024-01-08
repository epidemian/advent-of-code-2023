use anyhow::Context;
use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let hailstones: Vec<_> = input.lines().map(parse_hailstone).try_collect()?;

    let is_sample = hailstones.len() < 10;
    let area = if is_sample {
        7.0..=27.0
    } else {
        200000000000000.0..=400000000000000.0
    };
    let intersections_count = hailstones
        .iter()
        .tuple_combinations()
        .filter(|(h1, h2)| {
            let ((x1, y1, _z1), (vx1, vy1, _vz1)) = h1;
            let ((x2, y2, _z2), (vx2, vy2, _vz2)) = h2;
            // The equations for the 2D lines of both hailstones are:
            // h1: (x, y) = (x1, y1) + t*(vx1, vy1)
            // h2: (x, y) = (x2, y2) + v*(vx2, vy2)
            //
            // For the intersection point (x, y) contained in both lines, t and u form a system of 2
            // equations:
            // t*vx1 + u*(-vx2) = x2 - x1
            // t*vy1 + u*(-vy2) = y2 - y1
            //
            // Which can be solved using Cramer's rule (https://en.wikipedia.org/wiki/Cramer%27s_rule#Explicit_formulas_for_small_systems)
            //
            // t = ((x1 - x2) * vy2 - vx2 * (y1 - y2)) / (vx2*vy1 - vx1*vy2)
            // u = (vx1 * (y2 - y1) - (x2 - x1) * vy1) / (vx2*vy1 - vx1*vy2)
            //
            // See also: https://en.wikipedia.org/wiki/Intersection_(geometry)#Two_line_segments
            let denom = vx2 * vy1 - vx1 * vy2;
            if denom == 0.0 {
                // Hailstones are parallel
                return false;
            }
            let t = ((x1 - x2) * vy2 - vx2 * (y1 - y2)) / denom;
            let u = (vx1 * (y2 - y1) - (x2 - x1) * vy1) / denom;
            if t < 0.0 || u < 0.0 {
                // Hailstones collide in the past.
                return false;
            }

            let (x, y) = (x1 + t * vx1, y1 + t * vy1);
            area.contains(&x) && area.contains(&y)
        })
        .count();
    println!("{intersections_count}");
    Ok(())
}

type Point = (f64, f64, f64);

fn parse_hailstone(s: &str) -> aoc::Result<(Point, Point)> {
    let nums = aoc::parse_numbers(s)?;
    let (x, y, z, vx, vy, vz) = nums.into_iter().collect_tuple().context("invalid line")?;
    Ok(((x, y, z), (vx, vy, vz)))
}
