use anyhow::Context;
use itertools::Itertools;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;

    let records: Vec<_> = input.lines().map(parse_record).try_collect()?;
    let ans_1 = possible_arrangements_sum(&records);

    let unfolded_records: Vec<_> = records.into_iter().map(unfold_record).collect();
    let ans_2 = possible_arrangements_sum(&unfolded_records);

    println!("{ans_1} {ans_2}");
    Ok(())
}

type Record = (Vec<u8>, Vec<u64>);

fn possible_arrangements_sum(records: &[Record]) -> u64 {
    records
        .par_iter()
        .map(|(row, groups)| {
            let groups_re = generate_groups_regex(groups);
            count_possible_arrangements(row, &groups_re, &mut HashMap::new())
        })
        .sum()
}

fn count_possible_arrangements(
    row: &[u8],
    groups_re: &Regex,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let Some(unknown_index) = row.iter().position(|&b| b == b'?') else {
        return 1;
    };
    let mut total_count = 0;
    let mut new_row = row.to_vec();

    new_row[unknown_index] = b'#';
    if groups_re.is_match(&new_row) {
        total_count += count_possible_arrangements(&new_row, groups_re, cache);
    }

    new_row[unknown_index] = b'.';
    if groups_re.is_match(&new_row) {
        // How the cache works: we're trying out a '.' here, so to the left of it there are only
        // "fixed" groups of '#'s. We count how many of these groups (which are valid since we've
        // checked the regex already) there are, and if we have already seen this number of groups
        // to the left and in this position (`unknown_index`), then we reuse the number of
        // arrangements stored in the cache and avoid further recursion. Otherwise, we calculate the
        // number of arrangements recursively and store that result on the cache.
        let fixed_groups_count = new_row[0..unknown_index]
            .split(|&b| b == b'.')
            .filter(|s| !s.is_empty())
            .count();
        let cache_key = (unknown_index, fixed_groups_count);
        total_count += if let Some(count) = cache.get(&cache_key) {
            *count
        } else {
            let count = count_possible_arrangements(&new_row, groups_re, cache);
            cache.insert(cache_key, count);
            count
        }
    };

    total_count
}

/// Creates a Regex that can check if a row of springs matches the given group numbers.
///
/// E.g. for the group numbers `[3,2,1]` this will generate the regex
/// `^[.?]*[#?]{3}[.?]+[#?]{2}[.?]+[#?]{1}[.?]*$`, which can check if any row of springs consisting
/// of  `#`, `.` and `?`s can match those group numbers.
fn generate_groups_regex(groups: &[u64]) -> Regex {
    let groups_re = groups.iter().map(|n| format!("[#?]{{{n}}}")).join("[.?]+");
    let full_re = format!("^[.?]*{groups_re}[.?]*$");
    Regex::new(&full_re).expect("generated regex should be valid")
}

fn parse_record(line: &str) -> aoc::Result<Record> {
    let (springs_row, group_numbers) = line.split_once(' ').context("invalid input")?;
    Ok((
        springs_row.as_bytes().to_vec(),
        aoc::parse_numbers(group_numbers)?,
    ))
}

fn unfold_record((springs_row, group_numbers): Record) -> Record {
    let unfolded_row = [springs_row.as_slice()].repeat(5).join(&b'?');
    let unfolded_numbers = group_numbers.repeat(5);
    (unfolded_row, unfolded_numbers)
}
