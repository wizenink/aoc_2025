use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

struct TachyonManifold {
    grid: Vec<Vec<char>>,
    width: isize,
    height: usize,
    start: (isize, usize),
}

impl FromStr for TachyonManifold {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();

        let height = grid.len();
        let width = if height > 0 {
            grid[0].len() as isize
        } else {
            0
        };

        let mut start = (0, 0);

        for (y, row) in grid.iter().enumerate() {
            if let Some(x) = row.iter().position(|&c| c == 'S') {
                start = (x as isize, y);
                break;
            }
        }

        Ok(TachyonManifold {
            grid,
            width,
            height,
            start,
        })
    }
}

impl TachyonManifold {
    fn simulate(&self) -> u64 {
        let mut active_paths: HashMap<isize, u64> = HashMap::new();
        active_paths.insert(self.start.0, 1);

        let mut completed_timelines = 0;

        for y in self.start.1..self.height {
            let mut next_paths: HashMap<isize, u64> = HashMap::new();

            for (&x, &count) in &active_paths {
                let current_char = self.grid[y][x as usize];

                match current_char {
                    '^' => {
                        let left = x - 1;
                        if left >= 0 && left < self.width {
                            *next_paths.entry(left).or_insert(0) += count;
                        } else {
                            completed_timelines += count;
                        }

                        let right = x + 1;
                        if right >= 0 && right < self.width {
                            *next_paths.entry(right).or_insert(0) += count;
                        } else {
                            completed_timelines += count;
                        }
                    }
                    _ => {
                        *next_paths.entry(x).or_insert(0) += count;
                    }
                }
            }
            active_paths = next_paths;
        }
        let bottom_exits: u64 = active_paths.values().sum();

        completed_timelines + bottom_exits
    }
}
fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();

    let manifold: TachyonManifold = TachyonManifold::from_str(&input).unwrap();

    let splits = manifold.simulate();

    println!("{}", splits);
}
