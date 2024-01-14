use std::time::Instant;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

use crate::{cache::Cache, poset::Poset};

pub struct Search<'a> {
    n: u8,
    i: u8,
    current_max: u8,
    cache: &'a mut Cache,
    total_posets: u64,
    cache_hits: u64,
    start: Instant,
    progress_bars: MultiProgress,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Cost {
    /// Not solved. Impossible in less than the number of comparisons
    Minimum(u8),
    /// Solved in the number of comparisons
    Solved(u8),
}

impl<'a> Search<'a> {
    pub fn new(n: u8, i: u8, cache: &'a mut Cache) -> Self {
        Search {
            n,
            i,
            current_max: 0,
            cache,
            total_posets: 0,
            cache_hits: 0,
            start: Instant::now(),
            progress_bars: MultiProgress::new(),
        }
    }

    fn search_cache(&mut self, poset: &Poset) -> Option<Cost> {
        self.cache.get_and_do_stuff(poset)
    }

    fn insert_cache(&mut self, poset: Poset, new_cost: Cost) {
        if let Some(cost) = self.cache.get(&poset) {
            let res = match (cost, new_cost) {
                (Cost::Minimum(old_min), Cost::Minimum(new_min)) => {
                    Cost::Minimum(new_min.max(old_min))
                }
                (Cost::Solved(old_solved), Cost::Solved(new_solved)) => {
                    Cost::Solved(new_solved.min(old_solved))
                }
                (Cost::Solved(_), Cost::Minimum(_)) => cost,
                (Cost::Minimum(_), Cost::Solved(_)) => new_cost,
            };

            self.cache.insert(poset, res);
        } else {
            self.cache.insert(poset, new_cost);
        }
    }

    pub fn search(&mut self) -> Cost {
        self.start = Instant::now();

        for max in self.n..u8::MAX {
            self.current_max = max;

            let res = match self.search_rec(Poset::new(self.n, self.i), max, 0) {
                Cost::Solved(solved) => solved,
                Cost::Minimum(_) => {
                    // println!(
                    //     "found no solution for n = {}, i = {}, comparisons = {max}",
                    //     self.n, self.i
                    // );

                    // let duration = Instant::now() - self.start;
                    // let seconds = duration.as_secs_f32() % 60.0;
                    // let minutes = (duration.as_secs() / 60) % 60;
                    // let hours = (duration.as_secs() / (60 * 60)) % 24;
                    // let days = duration.as_secs() / (60 * 60 * 24);
                    // println!(
                    //     "time since start: {}d {}h {}m {}s",
                    //     days, hours, minutes, seconds
                    // );
                    continue;
                }
            };

            println!();
            println!(
                "found solution for n = {}, i = {}: comparisons = {}",
                self.n, self.i, res
            );
            println!("cache entries: {}", self.cache.len());
            println!("cache hits: {}", self.cache_hits);
            println!("posets searched: {}", self.total_posets);
            let duration = Instant::now() - self.start;
            let seconds = duration.as_secs_f32() % 60.0;
            let minutes = (duration.as_secs() / 60) % 60;
            let hours = (duration.as_secs() / (60 * 60)) % 24;
            let days = duration.as_secs() / (60 * 60 * 24);
            println!("time taken: {}d {}h {}m {}s", days, hours, minutes, seconds);

            // println!("poset counts by number of comparisons: ");
            // let counts = self.cache.counts();

            // for (i, count) in counts.iter().enumerate() {
            //     println!("{i:2}: {count:12}");
            // }

            // for entry in self.cache.iter() {
            //     if entry.cost.is_solved()
            //         && (entry.cost.value()
            //             < (entry.poset.compatible_posets().max(1) as f32)
            //                 .log2()
            //                 .floor() as u8)
            //     {
            //         dbg!(
            //             entry.cost,
            //             entry.poset,
            //             entry.poset.compatible_posets(),
            //             (entry.poset.compatible_posets().max(1) as f32)
            //                 .log2()
            //                 .floor()
            //         );
            //     }
            // }

            // assert_eq!(comps, max);
            return Cost::Solved(res);
        }

        unreachable!()
    }

    fn search_rec(&mut self, poset: Poset, max_comparisons: u8, depth: u8) -> Cost {
        if poset.n() == 1 {
            return Cost::Solved(0);
        }

        if max_comparisons == 0 {
            return Cost::Minimum(1);
        }

        if let Some(cost) = self.search_cache(&poset) {
            self.cache_hits += 1;
            match cost {
                Cost::Solved(solved) => {
                    return if solved > max_comparisons {
                        Cost::Minimum(solved)
                    } else {
                        cost
                    }
                }
                Cost::Minimum(min) => {
                    if min > max_comparisons {
                        return cost;
                    }
                }
            }
        }

        if let Some(false) = self.estimate_solvable(poset, max_comparisons, 0, 0, depth) {
            let result = Cost::Minimum(max_comparisons + 1);

            self.insert_cache(poset, result);

            return result;
        }

        let pairs = self.get_comparison_pairs(&poset);

        let progress = if depth + 2 < self.n {
            let progress = ProgressBar::new(pairs.len() as u64)
                .with_style(ProgressStyle::with_template("[{pos:2}/{len:2}] {msg}").unwrap());

            let progress = self.progress_bars.add(progress);
            Some(progress)
        } else {
            None
        };

        let mut result = Cost::Minimum(max_comparisons + 1);

        // search all comparisons
        let mut current_max = max_comparisons;
        for (first, second) in pairs {
            if let Some(progress) = &progress {
                progress.set_message(format!(
                    "max: {current_max:2}, total: {:10}, cache: {:10}",
                    self.total_posets,
                    self.cache.len()
                ));
            }

            // search the first case of the comparison
            let first_result = self.search_rec(first, current_max - 1, depth + 1);

            if !first_result.is_solved() || first_result.value() > current_max - 1 {
                if let Some(progress) = &progress {
                    progress.inc(1);
                }
                continue;
            }

            // search the second case of the comparison
            let second_result = self.search_rec(second, current_max - 1, depth + 1);

            if !second_result.is_solved() || second_result.value() > current_max - 1 {
                if let Some(progress) = &progress {
                    progress.inc(1);
                }
                continue;
            }

            // take the max of the branches of the comparisons
            let new_result = first_result.value().max(second_result.value()) + 1;

            // take the min of all comparisons
            result = Cost::Solved(new_result.min(result.value()));
            current_max = result.value();

            if let Some(progress) = &progress {
                progress.inc(1);
            }
        }

        self.total_posets += 1;

        if let Some(progress) = progress {
            progress.finish_and_clear();
            self.progress_bars.remove(&progress);
        }

        self.insert_cache(poset, result);

        result
    }

    fn get_comparison_pairs(&self, poset: &Poset) -> Vec<(Poset, Poset)> {
        let mut pairs = Vec::with_capacity(poset.n() as usize * (poset.n() as usize - 1) / 2);

        for i in 0..poset.n() {
            for j in (i + 1)..poset.n() {
                if poset.has_order(i, j) {
                    continue;
                }
                let less = poset.with_less(i, j);
                let greater = poset.with_less(j, i);

                let pair = if Self::estimate_hardness(&less) < Self::estimate_hardness(&greater) {
                    (less, greater)
                } else {
                    (greater, less)
                };

                if !pairs.contains(&pair) {
                    pairs.push(pair);
                }
            }
        }

        pairs
    }

    fn estimate_hardness(poset: &Poset) -> u32 {
        let mut hardness = 0;
        let (less, unknown, greater) = poset.calculate_relations();

        for i in 0..poset.n() as usize {
            let d = greater[i].abs_diff(less[i]);
            let u = unknown[i];

            hardness += (d + 2 * u) as u32;
        }

        hardness
    }

    fn estimate_solvable(
        &mut self,
        poset: Poset,
        max_comparisons: u8,
        start_i: u8,
        start_j: u8,
        _depth: u8,
    ) -> Option<bool> {
        if let Some(cost) = self.search_cache(&poset) {
            self.cache_hits += 1;
            match cost {
                Cost::Solved(solved) => {
                    return Some(solved <= max_comparisons);
                }
                Cost::Minimum(min) => {
                    if min > max_comparisons {
                        return Some(false);
                    }
                }
            }
        }

        if !poset.is_solvable_in(max_comparisons) {
            return Some(false);
        }

        if self.current_max - max_comparisons >= poset.n() && poset.n() >= self.n {
            let compatible_posets = poset.num_compatible_posets();
            if compatible_posets == 0 || max_comparisons < compatible_posets.ilog2() as u8 {
                return Some(false);
            }
        }

        let (less, _unknown, greater) = poset.calculate_relations();

        for i in start_i..poset.n() {
            if !(less[i as usize] == 0 && greater[i as usize] >= 2) {
                continue;
            }

            for j in (if i == start_i { start_j } else { 0 })..poset.n() {
                if i == j
                    || !(greater[j as usize] == 0 && less[j as usize] >= 2)
                    || poset.has_order(i, j)
                {
                    continue;
                }

                if let Some(false) = self.estimate_solvable(
                    poset.with_less(i, j),
                    max_comparisons,
                    i,
                    j + 1,
                    _depth + 1,
                ) {
                    return Some(false);
                }
            }
        }

        // if start_i != 0 && start_j != 0 {
        //     let cost = self.search_rec(poset, max_comparisons, _depth);
        //     match cost {
        //         Cost::Solved(solved) => {
        //             if solved <= max_comparisons {
        //                 return Some(true);
        //             } else {
        //                 return Some(false);
        //             }
        //         }
        //         Cost::Minimum(min) => {
        //             if min > max_comparisons {
        //                 return Some(false);
        //             }
        //         }
        //     }
        // }

        None
    }
}

impl Cost {
    pub fn value(&self) -> u8 {
        match self {
            Cost::Minimum(min) => *min,
            Cost::Solved(solved) => *solved,
        }
    }

    pub fn is_solved(&self) -> bool {
        matches!(self, Cost::Solved(_))
    }
}
