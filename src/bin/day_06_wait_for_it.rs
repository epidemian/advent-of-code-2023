use anyhow::Context;
use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (line_1, line_2) = input
        .lines()
        .collect_tuple()
        .context("expected input to have two lines")?;
    let times_s = line_1.strip_prefix("Time:").context("invalid input")?;
    let dists_s = line_2.strip_prefix("Distance:").context("invalid input")?;

    let times: Vec<u64> = times_s.split_whitespace().map(str::parse).try_collect()?;
    let distances: Vec<u64> = dists_s.split_whitespace().map(str::parse).try_collect()?;
    let ans_1: usize = times
        .into_iter()
        .zip(distances)
        .map(|(time, record_dist)| ways_to_beat_record(time, record_dist))
        .product();

    let time: u64 = times_s.replace(' ', "").parse()?;
    let record_dist: u64 = dists_s.replace(' ', "").parse()?;
    let ans_2 = ways_to_beat_record(time, record_dist);

    println!("{ans_1} {ans_2}");
    Ok(())
}

fn ways_to_beat_record(race_time: u64, record_dist: u64) -> usize {
    // Note: this is doing a linear algorithm when it could use some sort of binary search to find
    // the lowest and highest button-holding times that beats record, and return their difference.
    (0..race_time)
        .filter(|hold_time| {
            let dist = hold_time * (race_time - hold_time);
            dist > record_dist
        })
        .count()
}
