use std::time::Instant;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

use crate::{
    cache::Cache,
    constants::{LOWER_BOUNDS, MAX_N, UPPER_BOUNDS},
    poset::Poset,
};

pub struct Search<'a> {
    n: u8,
    i: u8,
    current_max: u8,
    cache: &'a mut Cache,
    analytics: Analytics,
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

pub struct Analytics {
    total_posets: u64,
    cache_hits: u64,
    cache_misses: u64,
    cache_replaced: u64,
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

impl<'a> Search<'a> {
    pub fn new(n: u8, i: u8, cache: &'a mut Cache) -> Self {
        Search {
            n,
            i,
            current_max: 0,
            cache,
            analytics: Analytics {
                total_posets: 0,
                cache_hits: 0,
                cache_misses: 0,
                cache_replaced: 0,
            },
            start: Instant::now(),
            progress_bars: MultiProgress::new(),
        }
    }

    fn search_cache(&mut self, poset: &Poset) -> Option<Cost> {
        let result = self.cache.get_and_do_stuff(poset);
        if result.is_some() {
            self.analytics.cache_hits += 1;
        } else {
            self.analytics.cache_misses += 1;
        }
        result
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

            let replaced = self.cache.insert(poset, res);
            if replaced {
                self.analytics.cache_replaced += 1;
            }
        } else {
            let replaced = self.cache.insert(poset, new_cost);
            if replaced {
                self.analytics.cache_replaced += 1;
            }
        }
    }

    pub fn search(&mut self) -> u8 {
        self.start = Instant::now();

        let min = LOWER_BOUNDS[self.n as usize][self.i as usize];
        let max = UPPER_BOUNDS[self.n as usize][self.i as usize];

        let mut result = max as u8;

        for current in min..max {
            let current = current as u8;
            self.current_max = current;

            result = match self.search_rec(Poset::new(self.n, self.i), current, 0) {
                Cost::Solved(solved) => solved,
                Cost::Minimum(_) => {
                    continue;
                }
            };
            break;
        }

        // Print the found solution
        println!();
        println!(
            "Congratulations. A solution was found!\n\nn: {}, i: {}",
            self.n, self.i
        );
        println!("Comparisons: {}", result);
        println!();

        self.print_cache();
        self.print_duration();

        result as u8
    }

    fn search_rec(&mut self, poset: Poset, max_comparisons: u8, depth: u8) -> Cost {
        if poset.n() == 1 {
            return Cost::Solved(0);
        }

        if max_comparisons == 0 {
            return Cost::Minimum(1);
        }

        if let Some(cost) = self.search_cache(&poset) {
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
                    self.analytics.total_posets,
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
            current_max = result.value() - 1;

            if let Some(progress) = &progress {
                progress.inc(1);
            }
        }

        self.analytics.total_posets += 1;

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

                let hardness_less = Self::estimate_hardness(&less);
                let hardness_greater = Self::estimate_hardness(&greater);

                let pair = if hardness_less < hardness_greater {
                    (less, greater, hardness_greater)
                } else {
                    (greater, less, hardness_less)
                };

                if !pairs.contains(&pair) {
                    pairs.push(pair);
                }
            }
        }

        pairs.sort_by_key(|pair| pair.2);

        pairs.into_iter().map(|(a, b, _)| (a, b)).collect()
    }

    fn estimate_hardness(poset: &Poset) -> u32 {
        let (less, greater) = poset.calculate_relations();

        let mut counts = [0; MAX_N];

        for i in 0..poset.n() as usize {
            counts[(poset.i() - greater[i]) as usize] += 1;
            counts[(poset.n() - poset.i() - 1 - less[i]) as usize] += 1;
        }
        counts
            .into_iter()
            .enumerate()
            .map(|(i, c)| ((MAX_N - i) as u32).pow(2) * c)
            .sum::<u32>()
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

        if self.current_max - max_comparisons >= poset.n() && poset.n() + 1 >= self.n {
            let compatible_posets = poset.num_compatible_posets();
            if compatible_posets == 0 || max_comparisons < compatible_posets.ilog2() as u8 {
                return Some(false);
            }
        }

        let (less, greater) = poset.calculate_relations();

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

        if start_i != 0 && start_j != 0 {
            let cost = self.search_rec(poset, max_comparisons, _depth);
            match cost {
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

        None
    }

    /// Print out a human readable duration in the format:
    /// days, hours, minutes, seconds
    pub fn print_duration(&self) {
        // Calculate the values for a human readable duration
        println!();

        let duration = Instant::now() - self.start;
        let seconds = duration.as_secs_f32() % 60.0;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / (60 * 60)) % 24;
        let days = duration.as_secs() / (60 * 60 * 24);

        println!("Duration: {}d {}h {}m {}s", days, hours, minutes, seconds);
        println!();
    }

    /// Print information out the cache, e.g. cache entries, hits, misses etc.
    pub fn print_cache(&self) {
        // Print information about the cache
        println!("Cache entries: {}", self.cache.len());
        println!("Cache hits: {}", self.analytics.cache_hits);
        println!("Cache misses: {}", self.analytics.cache_misses);
        println!("Cache replaced: {}", self.analytics.cache_replaced);
        println!();
        println!("Posets searched: {}", self.analytics.total_posets);
    }
}
