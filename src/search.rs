use std::time::Instant;

use hashbrown::HashMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

use crate::poset::{Poset, MAX_N};

pub struct Search<'a> {
    n: u8,
    i: u8,
    current_max: u8,
    cache: &'a mut HashMap<Poset, Cost>,
    total_nodes: u64,
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
    pub fn new(n: u8, i: u8, cache: &'a mut HashMap<Poset, Cost>) -> Self {
        Search {
            n,
            i,
            current_max: 0,
            cache,
            total_nodes: 0,
            start: Instant::now(),
            progress_bars: MultiProgress::new(),
        }
    }

    pub fn search(&mut self) -> Cost {
        self.start = Instant::now();

        for max in 0..u8::MAX {
            self.current_max = max;
            match self.search_rec(Poset::new(self.n, self.i), max, 0) {
                cost @ Cost::Solved(comps) => {
                    // self.print_info();
                    println!(
                        "found solution for n = {}, i = {}, comparisons = {comps}",
                        self.n, self.i
                    );
                    println!("cache entries: {}", self.cache.len());
                    let duration = Instant::now() - self.start;
                    let seconds = duration.as_secs_f32() % 60.0;
                    let minutes = (duration.as_secs() / 60) % 60;
                    let hours = (duration.as_secs() / (60 * 60)) % 24;
                    let days = duration.as_secs() / (60 * 60 * 24);
                    println!(
                        "time since start: {}d {}h {}m {}s",
                        days, hours, minutes, seconds
                    );

                    // assert_eq!(comps, max);
                    return cost;
                }
                _cost => {
                    println!(
                        "found no solution for n = {}, i = {}, comparisons = {max}",
                        self.n, self.i
                    );

                    let duration = Instant::now() - self.start;
                    let seconds = duration.as_secs_f32() % 60.0;
                    let minutes = (duration.as_secs() / 60) % 60;
                    let hours = (duration.as_secs() / (60 * 60)) % 24;
                    let days = duration.as_secs() / (60 * 60 * 24);
                    println!(
                        "time since start: {}d {}h {}m {}s",
                        days, hours, minutes, seconds
                    );
                }
            }
        }

        unreachable!()
    }

    fn search_rec(&mut self, poset: Poset, mut max_comparisons: u8, depth: u8) -> Cost {
        if let Some(cost) = self.cache.get(&poset) {
            match cost {
                Cost::Solved(_solved) => {
                    // if *solved > max_comparisons {
                    //     return Cost::Minimum(max_comparisons + 1);
                    // } else {
                    //     return Cost::Solved(*solved);
                    // }
                    return *cost;
                }
                Cost::Minimum(min) => {
                    if *min > max_comparisons {
                        return *cost;
                    }
                    //  else {
                    //     // if *min < max_comparisons {
                    //     // result = Cost::Minimum(*min)
                    //     // }
                    // }
                }
            }
        }

        if poset.n() == 1 {
            return Cost::Solved(0);
        }

        if max_comparisons == 0 {
            // self.cache.insert(poset, Cost::Minimum(1));
            return Cost::Minimum(1);
        }

        if let Some(false) = self.estimate_solvable(poset.clone(), max_comparisons, 0, 0) {
            let result = Cost::Minimum(max_comparisons + 1);

            if let Some(cost) = self.cache.get(&poset) {
                let res = match (cost, result) {
                    (Cost::Minimum(old_min), Cost::Minimum(new_min)) => {
                        Cost::Minimum(new_min.max(*old_min))
                    }
                    (Cost::Solved(old_solved), Cost::Solved(new_solved)) => {
                        Cost::Solved(new_solved.min(*old_solved))
                    }
                    (Cost::Solved(_), Cost::Minimum(_)) => *cost,
                    (Cost::Minimum(_), Cost::Solved(_)) => result,
                };

                self.cache.insert(poset.clone(), res);
            } else {
                self.cache.insert(poset.clone(), result);
            }

            return result;
        }

        let pairs = self.get_comparison_pairs(&poset);

        let progress = if depth <= self.n * 2 / 3 {
            let progress = ProgressBar::new(pairs.len() as u64).with_style(
                ProgressStyle::with_template("[{pos:2}/{len:2}] {msg} {wide_bar}").unwrap(),
            );

            let progress = self.progress_bars.add(progress.clone());
            Some(progress)
        } else {
            None
        };

        let mut result = Cost::Minimum(max_comparisons + 1);

        for (first, second) in pairs {
            if let Some(progress) = &progress {
                progress.set_message(format!(
                    "max: {max_comparisons:2}, nodes: {:10}, cache: {:10}",
                    self.total_nodes,
                    self.cache.len()
                ));
            }

            let first_result = self.search_rec(first, max_comparisons - 1, depth + 1);

            if !first_result.is_solved() || first_result.value() > max_comparisons - 1 {
                if let Some(progress) = &progress {
                    progress.inc(1);
                }
                continue;
            }

            let second_result = self.search_rec(second, max_comparisons - 1, depth + 1);

            if !second_result.is_solved() || second_result.value() > max_comparisons - 1 {
                if let Some(progress) = &progress {
                    progress.inc(1);
                }
                continue;
            }

            let new_result = first_result.value().max(second_result.value()) + 1;

            // result = result.min(new_result);

            if !result.is_solved() || new_result <= result.value() {
                result = Cost::Solved(new_result);
                max_comparisons = new_result;
            }

            if let Some(progress) = &progress {
                progress.inc(1);
            }
        }

        self.total_nodes += 1;
        // if self.total_nodes % 10000 == 0 {
        //     // dbg!(&poset, max_comparisons);
        //     self.print_info();
        // }

        if let Some(progress) = progress {
            progress.finish_and_clear();
            self.progress_bars.remove(&progress);
        }

        // if !result.is_solved() {
        //     result = Cost::Minimum(max_comparisons + 1);
        // }

        // if let Some(cost) = self.cache.get(&poset) {
        //     let res = match (cost, result) {
        //         (Cost::Minimum(old_min), Cost::Minimum(new_min)) => {
        //             Cost::Minimum(new_min.max(*old_min))
        //         }
        //         (Cost::Solved(old_solved), Cost::Solved(new_solved)) => {
        //             Cost::Solved(new_solved.min(*old_solved))
        //         }
        //         (Cost::Solved(_), Cost::Minimum(_)) => *cost,
        //         (Cost::Minimum(_), Cost::Solved(_)) => result,
        //     };

        //     self.cache.insert(poset.clone(), res);
        // } else {
        self.cache.insert(poset.clone(), result);
        // }

        // if let Some(cost) = self.cache.get(&poset.dual()) {
        //     if !cost.is_solved() {
        //         self.cache.insert(poset.dual(), result);
        //     }
        // } else {
        //     self.cache.insert(poset.dual(), result);
        // }

        // self.cache.insert(poset.dual(), result);

        result
    }

    fn get_comparison_pairs(&self, poset: &Poset) -> Vec<(Poset, Poset)> {
        let mut comparisons = Vec::with_capacity(poset.n() as usize * poset.n() as usize);

        for i in 0..poset.n() {
            for j in (i + 1)..poset.n() {
                if poset.has_order(i, j) {
                    continue;
                }

                comparisons.push((i, j));
            }
        }

        comparisons.sort_by_cached_key(|(i, j)| Self::get_priority(poset, *i, *j));
        comparisons.reverse();

        let mut pairs = Vec::with_capacity(comparisons.len());

        for (i, j) in comparisons {
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
        pairs
    }

    fn get_priority(poset: &Poset, i: u8, j: u8) -> u8 {
        let mut compares = 0;

        for k in 0..poset.n() {
            if k == i || k == j {
                continue;
            }

            if poset.has_order(i, k) {
                compares += 1;
            }
            if poset.has_order(j, k) {
                compares += 1;
            }
        }

        let mut priority = 0;

        if compares == 0 {
            priority = 250;
        } else {
            if poset.greater()[i as usize] >= 2 {
                priority += 50;
            }
            if poset.greater()[j as usize] >= 2 {
                priority += 50;
            }
            if poset.less()[i as usize] >= 2 {
                priority += 50;
            }
            if poset.less()[j as usize] >= 2 {
                priority += 50;
            }
        }

        priority += compares;

        priority
    }

    fn estimate_hardness(poset: &Poset) -> u8 {
        let mut hardness = 0;

        let less = poset.less();
        let greater = poset.greater();
        let mut unknown = [0u8; MAX_N];

        for i in 0..poset.n() {
            for j in (i + 1)..poset.n() {
                if !poset.has_order(i, j) {
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

    fn estimate_solvable(
        &mut self,
        poset: Poset,
        max_comparisons: u8,
        start_i: u8,
        start_j: u8,
    ) -> Option<bool> {
        if let Some(cost) = self.cache.get(&poset) {
            match *cost {
                Cost::Solved(solved) => {
                    if solved <= max_comparisons {
                        return Some(true);
                    } else {
                        return Some(false);
                    }
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

        let less = poset.less();
        let greater = poset.greater();

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

                if let Some(false) =
                    self.estimate_solvable(poset.with_less(i, j), max_comparisons, i, j + 1)
                {
                    return Some(false);
                }
            }
        }

        // if start_i != 0 && start_j != 0 {
        //     let cost = self.search_rec(poset.clone(), max_comparisons, depth);
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
