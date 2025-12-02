use std::{num::ParseIntError, str::FromStr};

struct IdRange {
    start: u64,
    stop: u64,
}

impl FromStr for IdRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once('-').expect("Must contain a hypen separator");
        Ok(IdRange {
            start: start_str.parse()?,
            stop: end_str.parse()?,
        })
    }
}

fn is_valid_for_pattern_len(n: u64, total_digits: u32, k: u32) -> bool {
    let repeats = total_digits / k;
    let pattern = n / 10_u64.pow(total_digits - k);

    let mut reconstructed = 0;
    for _ in 0..repeats {
        reconstructed = reconstructed * 10_u64.pow(k) + pattern;
    }

    reconstructed == n
}

fn is_repeated_sequence(n: u64) -> bool {
    if n < 10 {
        return false;
    }

    let digit_count = n.ilog10() + 1;

    for k in 1..=(digit_count / 2) {
        if digit_count % k == 0 {
            if is_valid_for_pattern_len(n, digit_count, k) {
                return true;
            }
        }
    }
    false
}
fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();

    let res: u64 = input
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<IdRange>().expect("Invalid range format"))
        .flat_map(|r| r.start..=r.stop)
        .filter(|&id| is_repeated_sequence(id))
        .sum();

    println!("{}", res);
}
