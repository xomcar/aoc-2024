#[allow(unused)]
use regex::Regex;
use std::error::Error;
use std::fs::{self};
use std::io::{self};

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("inputs/00.txt")?;
    let _reader = io::BufReader::new(file);

    Ok(())
}
