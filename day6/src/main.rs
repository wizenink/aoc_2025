use std::{convert::Infallible, str::FromStr};
enum Operator {
    Add,
    Multiply,
}
struct Worksheet {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl FromStr for Worksheet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().filter(|l| !l.is_empty()).collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let grid = lines
            .into_iter()
            .map(|line| {
                let mut bytes = line.as_bytes().to_vec();
                bytes.resize(width, b' ');
                bytes
            })
            .collect();

        Ok(Worksheet {
            grid,
            width,
            height,
        })
    }
}

impl Worksheet {
    fn solver(&self) -> Solver {
        Solver {
            worksheet: self,
            current_col: 0,
        }
    }
}
struct Solver<'a> {
    worksheet: &'a Worksheet,
    current_col: usize,
}

impl<'a> Iterator for Solver<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_col >= self.worksheet.width {
            return None;
        }

        let mut start = self.current_col;

        while start < self.worksheet.width {
            let is_sep =
                (0..self.worksheet.height).all(|row| self.worksheet.grid[row][start] == b' ');
            if !is_sep {
                break;
            }
            start += 1;
        }

        if start >= self.worksheet.width {
            self.current_col = start;
            return None;
        }

        let mut end = start;
        while end < self.worksheet.width {
            let is_sep = (0..self.worksheet.height).all(|r| self.worksheet.grid[r][end] == b' ');
            if is_sep {
                break;
            }
            end += 1;
        }

        self.current_col = end + 1;
        let mut numbers = Vec::new();
        let mut operation = Operator::Add; // Default
        for c in start..end {
            for r in 0..self.worksheet.height {
                match self.worksheet.grid[r][c] {
                    b'+' => operation = Operator::Add,
                    b'*' => operation = Operator::Multiply,
                    _ => {}
                }
            }
        }

        for c in (start..end).rev() {
            let mut digit_str = String::new();

            for r in 0..self.worksheet.height {
                let byte = self.worksheet.grid[r][c];
                if byte.is_ascii_digit() {
                    digit_str.push(byte as char);
                }
            }

            if !digit_str.is_empty() {
                if let Ok(num) = digit_str.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }

        let res = match operation {
            Operator::Add => numbers.iter().sum(),
            Operator::Multiply => numbers.iter().product(),
        };
        Some(res)
    }
}

fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();

    let worksheet: Worksheet = Worksheet::from_str(&input).unwrap();

    let mut solver = worksheet.solver();

    let sum: u64 = solver.sum();

    println!("{}", sum);
}
