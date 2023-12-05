use std::{error, io, result};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn read_stdin() -> result::Result<String, io::Error> {
    io::read_to_string(io::stdin())
}
