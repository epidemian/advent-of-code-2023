use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let (seeds, maps) = parse_input(&input)?;

    let ranges_p1 = seeds.iter().map(|&seed| Range::new(seed, seed + 1));
    let ans_1 = min_location_for_seed_ranges(ranges_p1, &maps);

    let ranges_p2 = seeds
        .iter()
        .tuples()
        .map(|(&start, &length)| Range::new(start, start + length));
    let ans_2 = min_location_for_seed_ranges(ranges_p2, &maps);

    println!("{ans_1} {ans_2}");
    Ok(())
}

type Map = Vec<RangeMap>;

#[derive(Copy, Clone)]
// These are open-ended ranges, so Range::new(3, 6) covers 3, 4, and 5.
struct Range {
    start: u64,
    end: u64,
}

struct RangeMap {
    src: Range,
    dst_start: u64,
}

fn min_location_for_seed_ranges(seed_ranges: impl Iterator<Item = Range>, maps: &[Map]) -> u64 {
    let mut ranges: Vec<_> = seed_ranges.collect();
    for map in maps.iter() {
        let mut mapped_ranges = Vec::with_capacity(ranges.len());
        while let Some(range) = ranges.pop() {
            let mut intersects = false;
            for range_map in map.iter() {
                let intersection = range.intersection(range_map.src);
                if intersection.is_empty() {
                    continue;
                }
                intersects = true;

                // Map the intersection.
                mapped_ranges.push(Range::new(
                    intersection.start - range_map.src.start + range_map.dst_start,
                    intersection.end - range_map.src.start + range_map.dst_start,
                ));

                // Put the rest of the unmapped range back into `ranges` to be mapped afterwards.
                let (left_diff, right_diff) = range.difference(intersection);
                if !left_diff.is_empty() {
                    ranges.push(left_diff)
                }
                if !right_diff.is_empty() {
                    ranges.push(right_diff)
                }

                break;
            }
            if !intersects {
                mapped_ranges.push(range);
            }
        }
        ranges = mapped_ranges;
    }
    ranges.iter().map(|r| r.start).min().unwrap_or(0)
}

impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }

    fn is_empty(&self) -> bool {
        self.end <= self.start
    }

    // The intersection of two ranges A and B might be empty in case they don't overlap.
    fn intersection(&self, other: Range) -> Range {
        Range::new(self.start.max(other.start), self.end.min(other.end))
    }

    // Returns the difference between two ranges.
    // When subtracting a range B from a range A, the result can be up to two ranges, like this:
    // A:   |--------|
    // B:      |-|
    // A-B: |-|   |--|
    // This method returns those two "left" and "right" resulting ranges, which might be empty in
    // the cases where the B range is not within the A range.
    fn difference(&self, other: Range) -> (Range, Range) {
        let left_diff = Range::new(self.start, self.end.min(other.start));
        let right_diff = Range::new(self.start.max(other.end), self.end);
        (left_diff, right_diff)
    }
}

impl RangeMap {
    fn parse(line: &str) -> aoc::Result<RangeMap> {
        let nums: Vec<_> = line.split(' ').map(str::parse).try_collect()?;
        let [dst_start, src_start, length] = nums[..].try_into()?;
        let src = Range::new(src_start, src_start + length);
        Ok(RangeMap { src, dst_start })
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
