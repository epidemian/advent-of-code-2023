use std::{io, result, str::FromStr};

pub type Result<T> = anyhow::Result<T>;

pub fn read_stdin() -> result::Result<String, io::Error> {
    io::read_to_string(io::stdin())
}

// TODO: Reuse this on other daily solutions.
pub fn parse_numbers<T: FromStr>(s: &str) -> result::Result<Vec<T>, T::Err> {
    s.split(|ch: char| !ch.is_ascii_digit() && ch != '-')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}
