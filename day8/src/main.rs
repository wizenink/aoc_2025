use std::{convert::Infallible, str::FromStr};

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<i64> = s.split(',').map(|v| v.parse().unwrap()).collect();

        Ok(Point {
            x: v[0],
            y: v[1],
            z: v[2],
        })
    }
}

impl Point {
    fn dist_sq(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }
}

struct Edge {
    u: usize,
    v: usize,
    dist_sq: i64,
}

struct UnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            count: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.count -= 1;
            true
        } else {
            false
        }
    }
}

fn solve(input: &str) -> i64 {
    let points: Vec<Point> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                u: i,
                v: j,
                dist_sq: points[i].dist_sq(&points[j]),
            });
        }
    }

    edges.sort_by(|a, b| a.dist_sq.cmp(&b.dist_sq));

    let mut dsu = UnionFind::new(n);
    let limit = 1000.min(edges.len());

    for edge in edges {
        if dsu.union(edge.u, edge.v) {
            if dsu.count == 1 {
                let p1 = &points[edge.u];
                let p2 = &points[edge.v];
                return p1.x * p2.x;
            }
        }
    }

    0
}
fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();
    let res = solve(&input);

    println!("{}", res);
}
