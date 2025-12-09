use std::cmp::{max, min};
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_s, y_s) = s.split_once(',').unwrap();
        Ok(Point {
            x: x_s.trim().parse().unwrap(),
            y: y_s.trim().parse().unwrap(),
        })
    }
}

struct Edge {
    p1: Point,
    p2: Point,
}

impl Edge {
    fn intersects_box_interior(&self, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
        let ex_min = min(self.p1.x, self.p2.x);
        let ex_max = max(self.p1.x, self.p2.x);
        let ey_min = min(self.p1.y, self.p2.y);
        let ey_max = max(self.p1.y, self.p2.y);

        let x_overlap_start = max(min_x, ex_min);
        let x_overlap_end = min(max_x, ex_max);

        let has_x_overlap = if ex_min == ex_max {
            ex_min > min_x && ex_min < max_x
        } else {
            x_overlap_start < x_overlap_end
        };

        let y_overlap_start = max(min_y, ey_min);
        let y_overlap_end = min(max_y, ey_max);

        let has_y_overlap = if ey_min == ey_max {
            ey_min > min_y && ey_min < max_y
        } else {
            y_overlap_start < y_overlap_end
        };

        has_x_overlap && has_y_overlap
    }
}

struct Polygon {
    vertices: Vec<Point>,
    edges: Vec<Edge>,
}

impl Polygon {
    fn new(vertices: Vec<Point>) -> Self {
        let mut edges = Vec::new();
        if !vertices.is_empty() {
            for i in 0..vertices.len() {
                let p1 = vertices[i];
                let p2 = vertices[(i + 1) % vertices.len()];
                edges.push(Edge { p1, p2 });
            }
        }
        Polygon { vertices, edges }
    }

    fn contains_point(&self, x: f64, y: f64) -> bool {
        let mut inside = false;
        for edge in &self.edges {
            let p1 = edge.p1;
            let p2 = edge.p2;

            let y1 = p1.y as f64;
            let y2 = p2.y as f64;

            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            if y > min_y && y <= max_y {
                let x1 = p1.x as f64;
                let x2 = p2.x as f64;

                let x_intersect = if x1 == x2 {
                    x1
                } else {
                    x1 + (y - y1) / (y2 - y1) * (x2 - x1)
                };

                if x < x_intersect {
                    inside = !inside;
                }
            }
        }
        inside
    }

    fn solve_largest_rect(&self) -> i64 {
        let mut max_area = 0;

        for (i, &p1) in self.vertices.iter().enumerate() {
            for &p2 in self.vertices.iter().skip(i + 1) {
                let min_x = min(p1.x, p2.x);
                let max_x = max(p1.x, p2.x);
                let min_y = min(p1.y, p2.y);
                let max_y = max(p1.y, p2.y);

                let width = max_x - min_x + 1;
                let height = max_y - min_y + 1;
                let area = width * height;
                if area <= max_area {
                    continue;
                }

                let has_intersection = self
                    .edges
                    .iter()
                    .any(|e| e.intersects_box_interior(min_x, max_x, min_y, max_y));

                if has_intersection {
                    continue;
                }

                let center_x = (min_x as f64 + max_x as f64) / 2.0;
                let center_y = (min_y as f64 + max_y as f64) / 2.0;

                if self.contains_point(center_x, center_y) {
                    max_area = area;
                }
            }
        }
        max_area
    }
}

fn solve(input: &str) -> i64 {
    let vertices: Vec<Point> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let poly = Polygon::new(vertices);
    poly.solve_largest_rect()
}
fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();

    let res = solve(&input);
    println!("{}", res);
}
