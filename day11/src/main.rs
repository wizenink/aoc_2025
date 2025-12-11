use std::{collections::HashMap, convert::Infallible, str::FromStr};

struct ReactorNetwork {
    adj: HashMap<String, Vec<String>>,
}

impl FromStr for ReactorNetwork {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adj = HashMap::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some((src, dests_str)) = line.split_once(':') {
                let src = src.trim().to_string();
                let dests: Vec<String> = dests_str
                    .trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

                adj.insert(src, dests);
            }
        }
        Ok(ReactorNetwork { adj })
    }
}

impl ReactorNetwork {
    fn count_paths(&self, start: &str, end: &str) -> u64 {
        let mut memo = HashMap::new();
        self.dfs(start, end, &mut memo)
    }

    fn dfs(&self, current: &str, target: &str, memo: &mut HashMap<String, u64>) -> u64 {
        if let Some(&count) = memo.get(current) {
            return count;
        }

        if current == target {
            return 1;
        }

        let mut total_paths = 0;

        if let Some(neighbors) = self.adj.get(current) {
            for neighbor in neighbors {
                total_paths += self.dfs(neighbor, target, memo);
            }
        }

        memo.insert(current.to_string(), total_paths);
        total_paths
    }

    fn solve_visits(&self) -> u64 {
        // Scenario 1: svr -> dac -> fft -> out
        let svr_dac = self.count_paths("svr", "dac");
        let dac_fft = self.count_paths("dac", "fft");
        let fft_out = self.count_paths("fft", "out");

        let path1_count = svr_dac * dac_fft * fft_out;

        // Scenario 2: svr -> fft -> dac -> out
        let svr_fft = self.count_paths("svr", "fft");
        let fft_dac = self.count_paths("fft", "dac");
        let dac_out = self.count_paths("dac", "out");
        let path2_count = svr_fft * fft_dac * dac_out;

        path1_count + path2_count
    }
}

fn solve(input: &str) -> u64 {
    let net: ReactorNetwork = input.parse().unwrap();
    //net.count_paths("you", "out")
    net.solve_visits()
}
fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();
    let res = solve(&input);
    println!("{}", res);
}
