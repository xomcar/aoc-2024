#[allow(unused)]
use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let text = read_to_string("inputs/04.txt")?;
    let matrix = Matrix::from_string(&text);
    let mut total_simple = 0;

    for cell in matrix.clone() {
        if cell.value == 'X' {
            total_simple += matrix.match_word("MAS", cell);
        }
    }

    println!("{}", total_simple);
    Ok(())
}

#[derive(Clone, Debug)]
struct Matrix {
    grid: Vec<Vec<char>>,
    index: usize,
}

struct Cell {
    value: char,
    row: usize,
    col: usize,
}

impl Iterator for Matrix {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        if self.index >= rows * cols {
            return None;
        }
        let row = self.index / rows;
        let col = self.index % cols;
        self.index += 1;
        return Some(Cell {
            value: self.grid[row][col],
            row,
            col,
        });
    }
}

impl Matrix {
    fn from_string(input: &str) -> Matrix {
        let mut grid = <Vec<Vec<char>>>::new();
        for line in input.lines() {
            let mut row = <Vec<char>>::new();
            for char in line.chars() {
                row.push(char);
            }
            grid.push(row);
        }
        Matrix { grid, index: 0 }
    }

    fn match_word(&self, target_str: &str, cell: Cell) -> u32 {
        let start_row = cell.row;
        let start_col = cell.col;
        let directions = [
            (-1, 0),  // up
            (1, 0),   // down
            (0, -1),  // left
            (0, 1),   // right
            (-1, -1), // top-left
            (-1, 1),  // top-right
            (1, -1),  // bottom-left
            (1, 1),   // bottom-right
        ];

        let mut matches = 0;
        for &(dx, dy) in &directions {
            let mut curr_row = start_row as isize;
            let mut curr_col = start_col as isize;
            for target in target_str.chars() {
                let new_row = curr_row as isize + dx;
                let new_col = curr_col as isize + dy;

                if new_row >= 0 && new_col >= 0 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if new_row < self.grid.len() && new_col < self.grid[0].len() {
                        if self.grid[new_row][new_col] == target {
                            curr_col += dy;
                            curr_row += dx;
                            if target == target_str.chars().last().unwrap() {
                                matches += 1
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        matches
    }
}
