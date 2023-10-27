use std::time::{Duration, Instant};

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::poset::{Poset, MAX_N};

pub struct Search<'a> {
    n: u8,
    i: u8,
    cache: &'a mut HashMap<Poset, Cost>,
    total_nodes: u64,
    start: Instant,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Cost {
    /// Not solved. Impossible in less than the number of comparisons
    Minimum(u8),
    /// Solved in the number of comparisons
    Solved(u8),
}

impl<'a> Search<'a> {
    pub fn new(n: u8, i: u8, cache: &'a mut HashMap<Poset, Cost>) -> Self {
        Search {
            n,
            i,
            cache,
            total_nodes: 0,
            start: Instant::now(),
        }
    }

    pub fn search(&mut self) -> Cost {
        self.start = Instant::now();

        for max in 1..u8::MAX {
            match self.search_rec(Poset::new(self.n, self.i), max, 0) {
                cost @ Cost::Solved(comps) => {
                    println!(
                        "found solution for n = {}, i = {}, comparisons = {comps}",
                        self.n, self.i
                    );
                    println!("time = {:?}", Instant::now() - self.start);

                    return cost;
                }
                _ => {
                    println!(
                        "found no solution for n = {}, i = {}, comparisons = {max}",
                        self.n, self.i
                    );
                }
            }
        }

        unreachable!()
    }

    fn search_rec(&mut self, poset: Poset, mut max_comparisons: u8, depth: u8) -> Cost {
        if let Some(cost) = self.cache.get(&poset) {
            match cost {
                Cost::Solved(_) => {
                    // if *solved > max_comparisons {
                    //     return Cost::Minimum(*solved);
                    // } else {
                    //     return Cost::Solved(*solved);
                    // }
                    return *cost;
                }
                Cost::Minimum(min) => {
                    if *min > max_comparisons {
                        return *cost;
                    }
                }
            }
        }

        if poset.n() == 1 {
            return Cost::Solved(0);
        }

        if max_comparisons == 0 {
            self.cache.insert(poset, Cost::Minimum(1));
            return Cost::Minimum(1);
        }

        if !poset.is_solvable_in(max_comparisons) {
            self.cache.insert(poset, Cost::Minimum(max_comparisons + 1));
            return Cost::Minimum(max_comparisons + 1);
        }

        let mut pairs = Vec::with_capacity((poset.n() * (poset.n() - 1) / 2).into());

        for i in 0..poset.n() {
            for j in (i + 1)..poset.n() {
                if poset.has_order(i, j) {
                    continue;
                }

                let less = poset.with_less(i, j);
                let greater = poset.with_less(j, i);

                let pair = if Self::get_hardness(&less) > Self::get_hardness(&greater) {
                    (less, greater)
                } else {
                    (greater, less)
                };

                if !pairs.contains(&pair) {
                    pairs.push(pair);
                }
            }
        }

        let mut result = Cost::Minimum(max_comparisons + 1);

        for (first, second) in pairs {
            self.total_nodes += 1;
            if self.total_nodes % 1000000 == 0 {
                println!("nodes = {}", self.total_nodes);
                println!("time = {:?}", Instant::now() - self.start);
            }

            let first_result = self.search_rec(first, max_comparisons - 1, depth + 1);

            if first_result.value() > max_comparisons - 1 {
                continue;
            }

            self.total_nodes += 1;
            if self.total_nodes % 1000000 == 0 {
                println!("nodes = {}", self.total_nodes);
                println!("time = {:?}", Instant::now() - self.start);
            }

            let second_result = self.search_rec(second, max_comparisons - 1, depth + 1);

            if second_result.value() > max_comparisons - 1 {
                continue;
            }

            let new_result = match first_result.max(second_result) {
                Cost::Minimum(_) => continue,
                Cost::Solved(solved) => Cost::Solved(solved + 1),
            };

            result = result.min(new_result);

            max_comparisons = if let Cost::Solved(comps) = result {
                comps.min(max_comparisons)
            } else {
                max_comparisons
            };
        }

        self.cache.insert(poset.dual(), result);
        self.cache.insert(poset, result);

        result
    }

    fn get_hardness(poset: &Poset) -> u8 {
        let mut hardness = 0;

        let mut less = [0u8; MAX_N];
        let mut greater = [0u8; MAX_N];
        let mut unknown = [0u8; MAX_N];

        for i in 0..poset.n() {
            for j in (i + 1)..poset.n() {
                if poset.is_less(i, j) {
                    less[j as usize] += 1;
                    greater[i as usize] += 1;
                } else if poset.is_less(j, i) {
                    less[i as usize] += 1;
                    greater[j as usize] += 1;
                } else {
                    unknown[i as usize] += 1;
                    unknown[j as usize] += 1;
                }
            }
        }

        for i in 0..poset.n() as usize {
            let d = greater[i].abs_diff(less[i]);
            let u = unknown[i];

            hardness += d + 2 * u;
        }

        hardness
    }
}

impl Cost {
    pub fn min(self, other: Self) -> Self {
        match self {
            Cost::Minimum(min) => match other {
                Cost::Minimum(min2) => Cost::Minimum(min.min(min2)),
                Cost::Solved(solved) => {
                    if min < solved {
                        Cost::Minimum(min)
                    } else {
                        Cost::Solved(solved)
                    }
                }
            },
            Cost::Solved(solved) => match other {
                Cost::Minimum(min) => {
                    if min < solved {
                        Cost::Minimum(min)
                    } else {
                        Cost::Solved(solved)
                    }
                }
                Cost::Solved(solved2) => Cost::Solved(solved.min(solved2)),
            },
        }
    }

    pub fn max(self, other: Self) -> Self {
        match self {
            Cost::Minimum(min) => match other {
                Cost::Minimum(min2) => Cost::Minimum(min.max(min2)),
                Cost::Solved(solved) => Cost::Minimum(solved.max(min)),
            },
            Cost::Solved(solved) => match other {
                Cost::Minimum(min) => Cost::Minimum(solved.max(min)),
                Cost::Solved(solved2) => Cost::Solved(solved.max(solved2)),
            },
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Cost::Minimum(min) => *min,
            Cost::Solved(solved) => *solved,
        }
    }
}
