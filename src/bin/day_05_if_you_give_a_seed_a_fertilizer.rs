use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (seeds, maps) = parse_input(&input)?;

    let ans_1 = min_location_part_1(&seeds, &maps);
    let ans_2 = min_location_part_2(&seeds, &maps);
    println!("{ans_1} {ans_2}");
    Ok(())
}

type Map = Vec<RangeMap>;

struct RangeMap {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

fn min_location_part_1(seeds: &[u64], maps: &[Map]) -> u64 {
    let flattened_ranges: Vec<_> = seeds.iter().flat_map(|seed| [*seed, 1]).collect();
    min_location_part_2(&flattened_ranges, maps)
}

fn min_location_part_2(seeds: &[u64], maps: &[Map]) -> u64 {
    let mut ranges: Vec<(u64, u64)> = seeds.iter().cloned().tuples().collect();
    for map in maps.iter() {
        let mut mapped_ranges = Vec::with_capacity(ranges.len());
        while let Some((start, length)) = ranges.pop() {
            let (mut start, mut length) = (start, length);
            for range_map in map.iter() {
                if start + length - 1 < range_map.source_start
                    || start > range_map.source_start + range_map.length - 1
                {
                    // Ranges don't overlap
                    continue;
                } else if range_map.source_start <= start
                    && start + length <= range_map.source_start + range_map.length
                {
                    // (start, length) is entirely within range_map source range
                    let mapped_start = start - range_map.source_start + range_map.destination_start;
                    mapped_ranges.push((mapped_start, length));
                    length = 0;
                    break;
                }
                if start < range_map.source_start
                    && range_map.source_start + range_map.length < start + length
                {
                    mapped_ranges.push((range_map.destination_start, range_map.length));
                    ranges.push((
                        range_map.source_start + range_map.length,
                        start + length - range_map.source_start - range_map.length,
                    ));
                    length = range_map.source_start - start;
                } else if start < range_map.source_start {
                    mapped_ranges.push((
                        range_map.destination_start,
                        length + start - range_map.source_start,
                    ));
                    length = range_map.source_start - start;
                } else if start + length > range_map.source_start + range_map.length {
                    mapped_ranges.push((
                        start - range_map.source_start + range_map.destination_start,
                        range_map.source_start + range_map.length - start,
                    ));
                    length = start + length - range_map.source_start - range_map.length;
                    start = range_map.source_start + range_map.length;
                } else {
                    unreachable!();
                }
            }
            if length != 0 {
                mapped_ranges.push((start, length));
            }
        }
        ranges = mapped_ranges;
    }
    ranges.iter().map(|(start, _l)| *start).min().unwrap_or(0)
}

impl RangeMap {
    fn parse(line: &str) -> aoc::Result<RangeMap> {
        let nums: Vec<_> = line.split(' ').map(str::parse).try_collect()?;
        let [destination_start, source_start, length] = nums[..].try_into()?;
        Ok(RangeMap {
            source_start,
            destination_start,
            length,
        })
    }
}

fn parse_input(input: &str) -> aoc::Result<(Vec<u64>, Vec<Map>)> {
    let (seeds_part, rest) = input.split_once("\n\n").ok_or("invalid input")?;
    let seeds = &seeds_part.strip_prefix("seeds: ").ok_or("invalid input")?;
    let seeds = seeds.split(' ').map(str::parse).try_collect()?;
    let maps = rest.split("\n\n").map(parse_map).try_collect()?;
    Ok((seeds, maps))
}

fn parse_map(block: &str) -> aoc::Result<Vec<RangeMap>> {
    block.lines().skip(1).map(RangeMap::parse).collect()
}
