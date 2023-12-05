use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (seeds, maps) = parse_input(&input)?;
    let min_location = seeds
        .iter()
        .map(|seed| {
            let mut num = *seed;
            for map in maps.iter() {
                num = map_number(num, map);
            }
            num
        })
        .min()
        .unwrap_or(0);
    println!("{min_location}");
    Ok(())
}

type Map = Vec<RangeMap>;

#[derive(Debug)]
struct RangeMap {
    source_start: u64,
    destination_start: u64,
    length: u64,
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

fn map_number(num: u64, map: &Map) -> u64 {
    for range in map.iter() {
        if let Some(mapped_num) = range.try_map(num) {
            return mapped_num;
        }
    }
    num
}
