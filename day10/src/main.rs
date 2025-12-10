use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<f64>>,
    target: Vec<f64>,
    num_counters: usize,
    num_buttons: usize,
}

impl FromStr for Machine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_brace = s.find('{').unwrap();
        let end_brace = s.find('}').unwrap();
        let target: Vec<f64> = s[start_brace + 1..end_brace]
            .split(',')
            .map(|n| n.trim().parse::<f64>().unwrap())
            .collect();
        let num_counters = target.len();

        let start_buttons = s.find(']').unwrap() + 1;
        let button_section = &s[start_buttons..start_brace];
        let mut buttons = Vec::new();
        for part in button_section.split(')') {
            if let Some(start) = part.find('(') {
                let mut vec = vec![0.0; num_counters];
                for n in part[start + 1..].split(',') {
                    if let Ok(idx) = n.trim().parse::<usize>() {
                        if idx < num_counters {
                            vec[idx] = 1.0;
                        }
                    }
                }
                buttons.push(vec);
            }
        }
        let len = buttons.len();
        Ok(Machine {
            buttons,
            target,
            num_counters,
            num_buttons: len,
        })
    }
}

impl Machine {
    fn solve(&self) -> Option<usize> {
        // Build Augmented Matrix
        let mut m = vec![vec![0.0; self.num_buttons + 1]; self.num_counters];
        for r in 0..self.num_counters {
            for c in 0..self.num_buttons {
                m[r][c] = self.buttons[c][r];
            }
            m[r][self.num_buttons] = self.target[r];
        }

        // Gaussian Elimination
        let mut pivot_row = 0;
        let mut col_to_pivot_row = vec![None; self.num_buttons];
        let mut free_vars = Vec::new();

        for c in 0..self.num_buttons {
            if pivot_row >= self.num_counters {
                free_vars.push(c);
                continue;
            }

            let mut best_r = None;
            for r in pivot_row..self.num_counters {
                if m[r][c].abs() > 1e-5 {
                    best_r = Some(r);
                    break;
                }
            }

            if let Some(r) = best_r {
                m.swap(pivot_row, r);
                let pivot_val = m[pivot_row][c];
                // Normalize row
                for k in c..=self.num_buttons {
                    m[pivot_row][k] /= pivot_val;
                }
                // Eliminate column
                for other_r in 0..self.num_counters {
                    if other_r != pivot_row {
                        let factor = m[other_r][c];
                        if factor.abs() > 1e-9 {
                            for k in c..=self.num_buttons {
                                m[other_r][k] -= factor * m[pivot_row][k];
                            }
                        }
                    }
                }
                col_to_pivot_row[c] = Some(pivot_row);
                pivot_row += 1;
            } else {
                free_vars.push(c);
            }
        }

        for r in pivot_row..self.num_counters {
            if m[r][self.num_buttons].abs() > 1e-4 {
                return None;
            }
        }

        if free_vars.is_empty() {
            let mut total = 0;
            for c in 0..self.num_buttons {
                if let Some(r) = col_to_pivot_row[c] {
                    let val = m[r][self.num_buttons];
                    if val < -1e-4 {
                        return None;
                    }
                    let round = val.round();
                    if (val - round).abs() > 1e-4 {
                        return None;
                    }
                    total += round as usize;
                }
            }
            return Some(total);
        }

        // Search for minimum solution using branch and bound
        let mut best_solution = None;

        fn search_min(
            machine: &Machine,
            idx: usize,
            free_vars: &[usize],
            free_vals: &mut [usize],
            m: &Vec<Vec<f64>>,
            col_to_pivot_row: &Vec<Option<usize>>,
            best_so_far: &mut Option<usize>,
        ) {
            // Only sum the free variables we've set so far (0..idx)
            let current_sum: usize = free_vals[0..idx].iter().sum();

            // Prune if we already exceeded the best solution
            if let Some(best) = *best_so_far {
                if current_sum >= best {
                    return;
                }
            }

            if idx == free_vars.len() {
                let total_free: usize = free_vals.iter().sum();
                let mut total = total_free;

                for c in 0..machine.num_buttons {
                    if let Some(r) = col_to_pivot_row[c] {
                        let mut val = m[r][machine.num_buttons];
                        for (i, &fv) in free_vars.iter().enumerate() {
                            val -= m[r][fv] * (free_vals[i] as f64);
                        }

                        if val < -1e-4 {
                            return;
                        }
                        let round = val.round();
                        if (val - round).abs() > 1e-4 {
                            return;
                        }
                        total += round as usize;
                    }
                }

                if best_so_far.is_none() || total < best_so_far.unwrap() {
                    eprintln!(
                        "Found: free_vals={:?}, total={}, current_sum={}",
                        free_vals, total, current_sum
                    );
                    *best_so_far = Some(total);
                }
                return;
            }

            let max_val = machine.target.iter().map(|&x| x as usize).sum::<usize>();

            for val in 0..=max_val {
                free_vals[idx] = val;

                if idx == 0 && val <= 5 && free_vars.len() == 2 {
                    eprintln!(
                        "Trying idx={}, val={}, free_vals={:?}",
                        idx,
                        val,
                        &free_vals[..free_vars.len()]
                    );
                }

                let mut valid = true;
                for c in 0..machine.num_buttons {
                    if let Some(r) = col_to_pivot_row[c] {
                        let mut depends_on_future = false;
                        for future_idx in (idx + 1)..free_vars.len() {
                            if m[r][free_vars[future_idx]].abs() > 1e-9 {
                                depends_on_future = true;
                                break;
                            }
                        }

                        if !depends_on_future {
                            let mut val = m[r][machine.num_buttons];
                            for i in 0..=idx {
                                let fv = free_vars[i];
                                val -= m[r][fv] * (free_vals[i] as f64);
                            }

                            if val < -1e-4 {
                                if idx < 2 && free_vars.len() <= 2 {
                                    eprintln!(
                                        "Pruned: idx={}, free_vals={:?}, pivot_col={}, val={} < 0",
                                        idx,
                                        &free_vals[..=idx],
                                        c,
                                        val
                                    );
                                }
                                valid = false;
                                break;
                            }
                            let round = val.round();
                            if (val - round).abs() > 1e-4 {
                                if idx < 2 && free_vars.len() <= 2 {
                                    eprintln!(
                                        "Pruned: idx={}, free_vals={:?}, pivot_col={}, val={} non-integer",
                                        idx,
                                        &free_vals[..=idx],
                                        c,
                                        val
                                    );
                                }
                                valid = false;
                                break;
                            }
                        }
                    }
                }

                if valid {
                    search_min(
                        machine,
                        idx + 1,
                        free_vars,
                        free_vals,
                        m,
                        col_to_pivot_row,
                        best_so_far,
                    );
                }
            }
        }

        let mut free_vals = vec![0; free_vars.len()];
        search_min(
            self,
            0,
            &free_vars,
            &mut free_vals,
            &m,
            &col_to_pivot_row,
            &mut best_solution,
        );

        best_solution
    }
}

fn solve_all(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Machine>().unwrap())
        .enumerate() // Keep index for debugging
        .map(|(i, m)| {
            m.solve()
                .expect(&format!("Machine {} is truly unsolvable.", i))
        })
        .sum()
}

fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned())).unwrap();
    let res = solve_all(&input);
    println!("{}", res);
}
