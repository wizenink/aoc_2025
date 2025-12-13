use std::cmp::Reverse;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Shape {
    // Coordinates relative to top-left (0,0)
    points: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Shape {
    fn area(&self) -> usize {
        self.points.len()
    }

    fn normalize(&self) -> Shape {
        if self.points.is_empty() {
            return self.clone();
        }
        let min_r = self.points.iter().map(|p| p.0).min().unwrap();
        let min_c = self.points.iter().map(|p| p.1).min().unwrap();

        let new_points: Vec<_> = self
            .points
            .iter()
            .map(|(r, c)| (r - min_r, c - min_c))
            .collect();

        let height = new_points.iter().map(|p| p.0).max().unwrap() + 1;
        let width = new_points.iter().map(|p| p.1).max().unwrap() + 1;

        // Sort points to ensure canonical representation for deduplication
        let mut sorted_points = new_points;
        sorted_points.sort();

        Shape {
            points: sorted_points,
            width,
            height,
        }
    }

    /// Generate unique variations (rotations + flips)
    fn generate_variants(&self) -> Vec<Shape> {
        let mut unique = HashSet::new();
        let mut variants = Vec::new();

        let mut current = self.clone();

        for _ in 0..4 {
            let norm = current.normalize();
            if unique.insert(norm.clone()) {
                variants.push(norm);
            }

            let mut flipped = current.clone();
            flipped.points = flipped
                .points
                .iter()
                .map(|&(r, c)| (r, current.width - 1 - c))
                .collect();
            let norm_flip = flipped.normalize();
            if unique.insert(norm_flip.clone()) {
                variants.push(norm_flip);
            }

            // Rotate 90 deg clockwise
            let new_points: Vec<_> = current
                .points
                .iter()
                .map(|&(r, c)| (c, current.height - 1 - r))
                .collect();
            current = Shape {
                points: new_points,
                width: current.height,
                height: current.width,
            }
            .normalize();
        }

        variants
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    #[inline]
    fn get_idx(&self, r: usize, c: usize) -> usize {
        r * self.width + c
    }

    fn can_fit(&self, r: usize, c: usize, shape: &Shape) -> bool {
        if r + shape.height > self.height || c + shape.width > self.width {
            return false;
        }
        for &(pr, pc) in &shape.points {
            if self.cells[self.get_idx(r + pr, c + pc)] {
                return false;
            }
        }
        true
    }

    fn toggle(&mut self, r: usize, c: usize, shape: &Shape) {
        for &(pr, pc) in &shape.points {
            let idx = self.get_idx(r + pr, c + pc);
            self.cells[idx] = !self.cells[idx];
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>) {
    let mut base_shapes = Vec::new();
    let mut queries = Vec::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.peek() {
        let line = line.trim();
        if line.is_empty() {
            lines.next();
            continue;
        }

        if let Some(colon_idx) = line.find(':') {
            if line[..colon_idx].chars().all(|c| c.is_digit(10)) {
                lines.next(); // consume ID line
                let mut points = Vec::new();
                let mut r = 0;
                let mut w = 0;

                while let Some(shape_line) = lines.peek() {
                    let shape_line = shape_line.trim();
                    if shape_line.is_empty() || shape_line.contains(':') {
                        break;
                    }
                    w = shape_line.len();
                    for (c, ch) in shape_line.chars().enumerate() {
                        if ch == '#' {
                            points.push((r, c));
                        }
                    }
                    r += 1;
                    lines.next();
                }

                let shape = Shape {
                    points,
                    width: w,
                    height: r,
                }
                .normalize();
                // Pre-compute all variants for this shape ID
                base_shapes.push(shape.generate_variants());
                continue;
            }
        }

        if let Some((dims, counts)) = line.split_once(':') {
            lines.next(); // consume line
            let (w_str, h_str) = dims.trim().split_once('x').unwrap();
            let w: usize = w_str.parse().unwrap();
            let h: usize = h_str.parse().unwrap();

            let reqs: Vec<usize> = counts
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            queries.push((w, h, reqs));
            continue;
        }

        lines.next();
    }

    (base_shapes, queries)
}

fn solve_recursive(
    grid: &mut Grid,
    presents: &[usize],
    variants_lookup: &[Vec<Shape>],
    present_idx: usize,
) -> bool {
    if present_idx >= presents.len() {
        return true;
    }

    let shape_id = presents[present_idx];
    let possible_shapes = &variants_lookup[shape_id];

    for r in 0..grid.height {
        for c in 0..grid.width {
            for shape_variant in possible_shapes {
                if grid.can_fit(r, c, shape_variant) {
                    grid.toggle(r, c, shape_variant);

                    if solve_recursive(grid, presents, variants_lookup, present_idx + 1) {
                        return true;
                    }

                    grid.toggle(r, c, shape_variant);
                }
            }
        }
    }

    false
}

fn solve(input: &str) -> usize {
    let (base_shapes, queries) = parse_input(input);
    let mut solvable_count = 0;

    for (w, h, counts) in queries {
        // Expand counts into a flat list of shape indices
        let mut presents_to_fit = Vec::new();
        let mut total_area = 0;

        for (shape_id, &count) in counts.iter().enumerate() {
            if shape_id < base_shapes.len() {
                let area = base_shapes[shape_id][0].area();
                for _ in 0..count {
                    presents_to_fit.push((area, shape_id));
                    total_area += area;
                }
            }
        }

        if total_area > w * h {
            continue;
        }

        presents_to_fit.sort_by_key(|k| Reverse(k.0));
        let sorted_indices: Vec<usize> = presents_to_fit.iter().map(|p| p.1).collect();

        let mut grid = Grid::new(w, h);
        if solve_recursive(&mut grid, &sorted_indices, &base_shapes, 0) {
            solvable_count += 1;
        }
    }

    solvable_count
}

fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();
    let res = solve(&input);
    println!("{}", res);
}
