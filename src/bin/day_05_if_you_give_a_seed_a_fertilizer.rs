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
    seeds
        .iter()
        .map(|seed| map_seed_to_location(*seed, maps))
        .min()
        .unwrap_or(0)
}

fn min_location_part_2(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .iter()
        .tuples()
        .flat_map(|(&start, &length)| {
            (start..start + length).map(|seed| map_seed_to_location(seed, maps))
        })
        .min()
        .unwrap_or(0)
}

fn map_seed_to_location(seed: u64, maps: &[Map]) -> u64 {
    let mut num = seed;
    for map in maps.iter() {
        num = map_number(num, map);
    }
    num
}

fn map_number(num: u64, map: &Map) -> u64 {
    for range in map.iter() {
        if let Some(mapped_num) = range.try_map(num) {
            return mapped_num;
        }
    }
    num
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

    fn try_map(&self, num: u64) -> Option<u64> {
        if self.source_start <= num && num < self.source_start + self.length {
            Some(num - self.source_start + self.destination_start)
        } else {
            None
        }
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
