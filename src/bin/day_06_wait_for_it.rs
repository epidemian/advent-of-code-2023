use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let lines: Vec<_> = input.lines().collect();
    let [line_1, line_2] = &lines[..] else {
        Err("expected input to have two lines")?
    };
    let times_s = line_1.strip_prefix("Time:").ok_or("invalid input")?;
    let dists_s = line_2.strip_prefix("Distance:").ok_or("invalid input")?;

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
    (0..race_time)
        .filter(|hold_time| {
            let dist = hold_time * (race_time - hold_time);
            dist > record_dist
        })
        .count()
}