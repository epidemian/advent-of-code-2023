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

#[derive(Copy, Clone, Debug)]
struct Range {
    start: u64,
    length: u64,
}

struct RangeMap {
    src: Range,
    dst_start: u64,
}

fn min_location_part_1(seeds: &[u64], maps: &[Map]) -> u64 {
    let flattened_ranges: Vec<_> = seeds.iter().flat_map(|seed| [*seed, 1]).collect();
    min_location_part_2(&flattened_ranges, maps)
}

fn min_location_part_2(seeds: &[u64], maps: &[Map]) -> u64 {
    let mut ranges: Vec<Range> = seeds
        .iter()
        .cloned()
        .tuples()
        .map(|(start, length)| Range { start, length })
        .collect();
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
                    intersection.length,
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
    fn new(start: u64, length: u64) -> Range {
        Range { start, length }
    }

    fn end(&self) -> u64 {
        self.start + self.length - 1
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    // The intersection of two ranges might be empty in case they don't overlap.
    fn intersection(&self, other: Range) -> Range {
        let intersection_start = self.start.max(other.start);
        let intersection_end = self.end().min(other.end());
        let intersection_length = if intersection_start <= intersection_end {
            intersection_end - intersection_start + 1
        } else {
            0
        };
        Range::new(intersection_start, intersection_length)
    }

    // Returns the difference between two ranges.
    // When subtracting a range B from a range A, the result can be up to two ranges, like this:
    // A:   |--------|
    // B:      |-|
    // A-B: |-|   |--|
    // This method returns those two "left" and "right" resulting ranges, which might be empty in
    // the cases where the B range is not within the A range.
    fn difference(&self, other: Range) -> (Range, Range) {
        let left_diff = Range::new(
            self.start,
            other.start.saturating_sub(self.start).min(self.length),
        );
        let right_diff = Range::new(
            self.start.max(other.end() + 1),
            self.end().saturating_sub(other.end()).min(self.length),
        );
        (left_diff, right_diff)
    }
}

impl RangeMap {
    fn parse(line: &str) -> aoc::Result<RangeMap> {
        let nums: Vec<_> = line.split(' ').map(str::parse).try_collect()?;
        let [dst_start, src_start, length] = nums[..].try_into()?;
        let src = Range::new(src_start, length);
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
