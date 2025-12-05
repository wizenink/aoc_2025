use std::{
    convert::Infallible,
    ops::{Range, RangeInclusive},
    str::FromStr,
};

struct IngredientsDb {
    ranges: Vec<RangeInclusive<u64>>,
}

impl FromStr for IngredientsDb {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_ranges = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (start, end) = line
                    .split_once('-')
                    .expect("Values should be hypen separated");
                start.parse().unwrap()..=end.parse().unwrap()
            })
            .collect();

        Ok(IngredientsDb {
            ranges: valid_ranges,
        })
    }
}

impl IngredientsDb {
    fn is_fresh(&self, id: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(&id))
    }

    fn total_fresh(&self) -> u64 {
        self.ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }

    fn merge_ranges(&mut self) {
        if self.ranges.is_empty() {
            return;
        }

        self.ranges.sort_by_key(|r| *r.start());

        let mut merged = Vec::new();
        let mut current_range = self.ranges[0].clone();

        for next_range in self.ranges.iter().skip(1) {
            if *next_range.start() <= *current_range.end() + 1 {
                let new_end = std::cmp::max(*current_range.end(), *next_range.end());
                current_range = *current_range.start()..=new_end;
            } else {
                merged.push(current_range);
                current_range = next_range.clone();
            }
        }
        merged.push(current_range);

        self.ranges = merged;
    }
}

fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();

    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let mut inventory: IngredientsDb = ranges_str.parse().unwrap();
    inventory.merge_ranges();

    let count = ingredients_str
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|&id| inventory.is_fresh(id))
        .count();
    println!("{}", count);

    println!("{}", inventory.total_fresh());
}
