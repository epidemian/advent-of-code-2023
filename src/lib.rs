use itertools::Itertools;
use std::{io, result, str::FromStr};

pub type Result<T> = anyhow::Result<T>;

pub fn read_stdin() -> result::Result<String, io::Error> {
    io::read_to_string(io::stdin())
}

pub fn parse_numbers<T: FromStr>(s: &str) -> result::Result<Vec<T>, T::Err> {
    s.split(|ch: char| !ch.is_ascii_digit() && ch != '-')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

pub fn parse_char_grid(input: &str) -> Result<(Vec<Vec<char>>, usize, usize)> {
    parse_grid(input, Ok)
}

pub fn parse_grid<T>(
    input: &str,
    parse_char: impl Fn(char) -> Result<T>,
) -> Result<(Vec<Vec<T>>, usize, usize)> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(&parse_char).try_collect())
        .try_collect()?;

    let height = grid.len();
    let width = grid.first().map_or(0, |r| r.len());
    for row in grid.iter() {
        anyhow::ensure!(row.len() == width, "rows must be all the same length");
    }

    Ok((grid, width, height))
}
