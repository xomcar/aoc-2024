use std::error::Error;
use std::fs::{self};
use std::io::{self};

type Report = Vec<i32>;

trait ReportValidator {
    fn is_valid(&self) -> bool;
    fn is_valid_dampened(&self) -> bool;
    fn from_reader<T>(path: T) -> Vec<Report>
    where
        T: io::BufRead;
}

impl ReportValidator for Report {
    fn is_valid(&self) -> bool {
        let is_increasing = self[1] - self[0] > 0;
        if is_increasing {
            for i in 0..self.len() - 1 {
                let diff = self[i + 1] - self[i];
                if diff > 3 || diff < 1 {
                    return false;
                };
            }
        } else {
            for i in 0..self.len() - 1 {
                let diff = self[i] - self[i + 1];
                if diff > 3 || diff < 1 {
                    return false;
                };
            }
        }
        return true;
    }

    fn is_valid_dampened(&self) -> bool {
        if self.is_valid() {
            return true;
        };
        self.clone().iter().any(|&element| {
            let indeces_to_remove: Vec<usize> = self
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if v == element { Some(i) } else { None })
                .collect();
            for index in indeces_to_remove {
                let mut reduced = self.clone();
                reduced.remove(index);
                assert_eq!(self.len(), reduced.len() + 1);
                let is_valid = reduced.is_valid();
                if is_valid {
                    return is_valid;
                }
            }
            false
        })
    }

    fn from_reader<T>(reader: T) -> Vec<Report>
    where
        T: io::BufRead,
    {
        reader
            .lines()
            .filter_map(Result::ok)
            .map(|l| {
                l.split_whitespace()
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Report>()
            })
            .collect()
    }
}

#[test]
fn test_is_valid() {
    let input = r"7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9";
    let reader = io::BufReader::new(input.as_bytes());
    let reports: Vec<Report> = Report::from_reader(reader);
    let n_valid = reports.iter().filter(|v| v.is_valid()).count();
    assert_eq!(2, n_valid);
    assert_eq!((vec![4, 3, 2, 1] as Report).is_valid(), true)
}

#[test]
fn test_is_valid_dampened() {
    let input = r"7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9";
    let reader = io::BufReader::new(input.as_bytes());
    let reports: Vec<Report> = Report::from_reader(reader);
    let n_valid = reports
        .iter()
        .filter(|v| {
            let damp = v.is_valid_dampened();
            damp
        })
        .count();
    assert_eq!(4, n_valid);
}
fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("inputs/02.txt")?;
    let reader = io::BufReader::new(file);
    let reports = Report::from_reader(reader);
    let valid_reports_n = reports.clone().iter().filter(|r| r.is_valid()).count();
    let valid_reports_n_dampened = reports
        .iter()
        .filter(|v| {
            let damp = v.is_valid_dampened();
            println!("dampened result: {}", damp);
            damp
        })
        .count();
    println!("valid reports: {}", valid_reports_n);
    println!("valid_reports_n_dampened: {}", valid_reports_n_dampened);
    Ok(())
}
