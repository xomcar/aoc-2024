use regex::Regex;
use std::error::Error;
use std::fs::{self};

fn main() -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string("inputs/03.txt")?;
    let result1 = parse_input(&string)?;
    let result2 = parse_with_dos(&string)?;
    println!("part 1: {}", result1);
    println!("part 2: {}", result2);
    Ok(())
}

#[test]
fn test_parse_mul() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(161, parse_input(input).unwrap());
}

#[test]
fn test_parse_mul_dos() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, parse_with_dos(input).unwrap());
}

fn parse_input(input: &str) -> Result<i32, Box<dyn Error>> {
    let mul = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let value: i32 = mul.captures_iter(input).fold(0, |acc, captures| {
        acc + captures[1].parse::<i32>().unwrap() * captures[2].parse::<i32>().unwrap()
    });
    Ok(value)
}

fn parse_with_dos(input: &str) -> Result<i32, Box<dyn Error>> {
    let mul = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))")?;
    let mut mul_val = 1;
    let value: i32 = mul.captures_iter(input).fold(0, |acc, captures| {
        if let Some(_) = captures.get(1) {
            return acc
                + mul_val
                    * captures[1].parse::<i32>().unwrap()
                    * captures[2].parse::<i32>().unwrap();
        } else if let Some(_) = captures.get(3) {
            mul_val = 1;
        } else if let Some(_) = captures.get(4) {
            mul_val = 0;
        }
        acc
    });
    Ok(value)
}
