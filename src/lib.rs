use std::{error, io, num, result};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn read_stdin() -> result::Result<String, io::Error> {
    io::read_to_string(io::stdin())
}

// TODO: Reuse this on other daily solutions.
pub fn parse_numbers(s: &str) -> result::Result<Vec<u64>, num::ParseIntError> {
    s.split(',').map(str::parse).collect()
}
