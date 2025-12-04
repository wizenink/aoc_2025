use std::{convert::Infallible, num::ParseIntError, str::FromStr};

struct BatteryBank {
    batteries: Vec<u8>,
}

impl FromStr for BatteryBank {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.trim().bytes().map(|b| b - b'0').collect();
        Ok(BatteryBank { batteries: digits })
    }
}

impl BatteryBank {
    fn max_joltage(&self, k: usize) -> u64 {
        let mut cursor = 0;
        let mut result: u64 = 0;
        let n = self.batteries.len();

        for remaining in (1..=k).rev() {
            let limit = n - remaining;
            let window = &self.batteries[cursor..=limit];

            let (offset, &digit) = window
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, val)| val)
                .expect("Window should never be empty");

            result = result * 10 + (digit as u64);

            cursor += offset + 1;
        }

        result
    }
}

fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();

    let res: u64 = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.parse::<BatteryBank>()
                .expect("Parsing should never fail")
        })
        .map(|bank| bank.max_joltage(12))
        .sum();

    println!("{}", res);
}
