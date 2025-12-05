use std::{convert::Infallible, str::FromStr};

struct Grid {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };

        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

impl Grid {
    fn is_roll(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return false;
        }

        self.cells[y as usize][x as usize] == '@'
    }

    fn count_roll_neighbors(&self, x: usize, y: usize) -> usize {
        let dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        dirs.iter()
            .filter(|&&(dx, dy)| self.is_roll(x as isize + dx, y as isize + dy))
            .count()
    }

    fn count_accessible_rolls(&self) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| self.cells[y][x] == '@')
            .filter(|&(x, y)| self.count_roll_neighbors(x, y) < 4)
            .count()
    }

    fn tick(&mut self) -> Option<usize> {
        let to_remove: Vec<(usize, usize)> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| self.cells[y][x] == '@')
            .filter(|&(x, y)| self.count_roll_neighbors(x, y) < 4)
            .collect();

        if to_remove.is_empty() {
            return None;
        }

        let count = to_remove.len();

        for (x, y) in to_remove {
            self.cells[y][x] = '.';
        }

        Some(count)
    }
}

fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();

    let mut res = input.parse::<Grid>().unwrap();

    let sum: usize = std::iter::from_fn(|| res.tick()).sum();
    println!("{}", sum);
}
