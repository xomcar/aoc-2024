use std::error::Error;
use std::fs::{self};
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("inputs/01.txt")?;
    let reader = io::BufReader::new(file);
    let (mut sx, mut dx): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let sx: i32 = tokens.next().unwrap().parse().unwrap();
            let dx: i32 = tokens.next().unwrap().parse().unwrap();
            (sx, dx)
        })
        .unzip();
    sx.sort();
    dx.sort();

    let sum = std::iter::zip(&sx, &dx).fold(0, |sum, (a, b)| sum + (b - a).abs());
    println!("total distance: {}", sum);

    let mut score: i32 = 0;
    for i in sx {
        let count = dx.iter().filter(|&e| *e == i).count();
        score += count as i32 * i;
    }
    println!("similarity score: {}", score);
    Ok(())
}
