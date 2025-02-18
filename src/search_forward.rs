use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, RwLock,
    },
    thread,
    time::Instant,
};

use hashbrown::HashMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::{backwards_poset::BackwardsPoset, cache::Cache, constants::{LOWER_BOUNDS, UPPER_BOUNDS}, free_poset::FreePoset, poset::Poset, pseudo_canonified_poset::PseudoCanonifiedPoset, search_backward::start_search_backward, utils::format_duration, WeightFunction};

pub struct Search<'a> {
    n: u8,
    i: u8,
    current_max: u8,
    cache: &'a mut Cache,
    analytics: Analytics,
    comparisons: &'a mut HashMap<PseudoCanonifiedPoset, (u8, u8)>,
    use_bidirectional_search: bool,
    weight_function: WeightFunction,
}

#[derive(Debug, Clone, Copy)]
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
    max_progress_depth: u8,
    multiprogress: MultiProgress,
    progress_bars: Vec<(ProgressBar, AtomicU64)>,
}

impl Cost {
    pub fn value(self) -> u8 {
        match self {
            Cost::Minimum(min) => min,
            Cost::Solved(solved) => solved,
        }
    }

    pub fn is_solved(self) -> bool {
        matches!(self, Cost::Solved(_))
    }
}

impl<'a> Search<'a> {
    pub fn new(
        n: u8,
        i: u8,
        cache: &'a mut Cache,
        comparisons: &'a mut HashMap<PseudoCanonifiedPoset, (u8, u8)>,
        use_bidirectional_search: bool,
        weight_function: WeightFunction,
    ) -> Self {
        Search {
            n,
            i,
            current_max: 0,
            cache,
            analytics: Analytics::new(n.max(4) - 3),
            comparisons,
            use_bidirectional_search,
            weight_function,
        }
    }

    fn search_cache(&mut self, poset: &PseudoCanonifiedPoset) -> Option<Cost> {
        let result = self.cache.get_mut(poset);
        if result.is_some() {
            self.analytics.record_hit();
        } else {
            self.analytics.record_miss();
        }
        result
    }

    fn insert_cache(&mut self, poset: PseudoCanonifiedPoset, new_cost: Cost) {
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
                self.analytics.record_replace();
            }
        } else {
            let replaced = self.cache.insert(poset, new_cost);
            if replaced {
                self.analytics.record_replace();
            }
        }
    }

    pub fn search(&mut self) -> u8 {
        const PAIR_WISE_OPTIMIZATION: bool = false;

        let start = Instant::now();

        let min = LOWER_BOUNDS[self.n as usize][self.i as usize];
        let max = UPPER_BOUNDS[self.n as usize][self.i as usize];

        let mut result = max as u8;

        for current in min.. {
            let backward_search_state = Arc::new(RwLock::new((HashMap::new(), -1)));
            let interrupt = Arc::new(AtomicBool::new(false));
            let handle = if self.use_bidirectional_search {
                let n_local = self.n;
                let i_local = self.i;
                let interrupt_local = interrupt.clone();
                let backward_search_state_local = backward_search_state.clone();
                Some(thread::spawn(move || {
                    start_search_backward(
                        &interrupt_local,
                        Some(&backward_search_state_local),
                        n_local,
                        i_local,
                        current,
                    );
                }))
            } else {
                None
            };

            let mut poset = FreePoset::new(self.n, self.i);
            let mut comparisons_done = 0u8;
            if PAIR_WISE_OPTIMIZATION {
                println!("Attention: searching with pairwise-optimisation");
                for k in (0..self.n - 1).step_by(2) {
                    comparisons_done += 1;
                    poset.add_and_close(k, k + 1);
                }
            }

            let current = current as u8 - comparisons_done;
            self.current_max = current;
            self.analytics.set_max_depth(current / 2);

            let search_result =
                self.search_rec(&backward_search_state, poset.canonified(), current, 0);

            if let Some(handle) = handle {
                interrupt.store(true, Ordering::Relaxed);
                handle.join().unwrap();
            }

            result = match search_result {
                Cost::Solved(solved) => solved + comparisons_done,
                Cost::Minimum(min) => {
                    self.analytics.multiprogress.clear().unwrap();
                    println!(
                        "n: {}, i: {} needs at least {} comparisons",
                        self.n,
                        self.i,
                        min + comparisons_done
                    );
                    println!("{}", format_duration(start));

                    continue;
                }
            };
            break;
        }

        self.analytics.complete_all();

        // Print the found solution
        println!();
        println!(
            "Congratulations. A solution was found!\n\nn: {}, i: {}",
            self.n, self.i
        );
        println!("Comparisons: {result}");
        println!();

        self.print_cache();
        println!("{}", format_duration(start));
        println!();

        result
    }

    #[allow(clippy::too_many_lines)]
    fn search_rec(
        &mut self,
        backward_search_state: &Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>,
        poset: PseudoCanonifiedPoset,
        max_comparisons: u8,
        depth: u8,
    ) -> Cost {
        if poset.n() == 1 {
            return Cost::Solved(0);
        }

        if max_comparisons == 0 {
            return Cost::Minimum(1);
        }

        if let Some(cost) = self.search_cache(&poset) {
            match cost {
                Cost::Solved(_) => {
                    return cost;
                }
                Cost::Minimum(min) => {
                    if min > max_comparisons {
                        return cost;
                    }
                }
            }
        }

        if self.use_bidirectional_search {
            let read_lock = backward_search_state
                .read()
                .expect("cache shouldn't be poisoned");
            if max_comparisons as i8 + 1 <= read_lock.1 {
                // TODO: idk, ob das passt; oder ohne '+1'?
                return if let Some(&value) = read_lock.0.get(&poset.to_backward()) {
                    Cost::Solved(value)
                } else {
                    Cost::Minimum(max_comparisons + 1)
                };
            }
        }

        if let Some(false) =
            self.estimate_solvable(backward_search_state, poset, max_comparisons, depth)
        {
            let result = Cost::Minimum(max_comparisons + 1);

            self.insert_cache(poset, result);

            return result;
        }

        let pairs = poset.get_comparison_pairs();
        let n_pairs = pairs.len() as u64;

        self.analytics.inc_length(depth, n_pairs);

        // search all comparisons
        let mut best_comparison = (0, 0);
        let mut current_best = max_comparisons + 1;
        for (first, second, i, j) in pairs {
            self.analytics.update_stats(
                depth,
                self.current_max,
                self.cache.len(),
                self.cache.max_entries(),
            );

            // search the first case of the comparison
            let first_result =
                self.search_rec(backward_search_state, first, current_best - 2, depth + 1);

            if !first_result.is_solved() || first_result.value() > current_best - 2 {
                self.analytics.inc(depth, 1);
                continue;
            }

            // search the second case of the comparison
            let second_result =
                self.search_rec(backward_search_state, second, current_best - 2, depth + 1);

            if !second_result.is_solved() || second_result.value() > current_best - 2 {
                self.analytics.inc(depth, 1);
                continue;
            }

            // take the max of the branches of the comparisons
            // if the current pair maximum was worse, the
            // continues above never let this be reached
            best_comparison = (i, j);

            current_best = first_result.value().max(second_result.value()) + 1;

            self.analytics.inc(depth, 1);
        }

        let result = if current_best <= max_comparisons {
            self.comparisons.insert(poset, best_comparison);
            Cost::Solved(current_best)
        } else {
            Cost::Minimum(max_comparisons + 1)
        };

        self.analytics.inc_complete(depth, n_pairs);

        self.analytics.record_poset();

        self.insert_cache(poset, result);

        result
    }

    fn estimate_solvable(
        &mut self,
        backward_search_state: &Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>,
        poset: PseudoCanonifiedPoset,
        max_comparisons: u8,
        depth: u8,
    ) -> Option<bool> {
        
        match self.weight_function {
            WeightFunction::CompatibleSolutions => {
                let compatible_posets = poset.num_compatible_posets();
                if compatible_posets == 0 || (max_comparisons as u32) < (compatible_posets - 1).ilog2() + 1 {
                    return Some(false);
                }
            }
            WeightFunction::Weight0 => {
                let weight = poset.weight0();
                // ilog2 is rounded down -> (weight - 1).ilog2() + 1 is rounded up
                if weight <= 1 || (max_comparisons as u32) <= (weight - 1).ilog2() {
                    return Some(false);
                }
            }
            WeightFunction::Weight => {
                let scale = (1..=self.n).map(|k| k as u128).product();
                let weight = poset.weight(depth as usize + max_comparisons as usize, scale);
                if weight <= scale || (max_comparisons as u32) <= ((weight / scale) - 1).ilog2() {
                    return Some(false);
                }
            }
            WeightFunction::None => {}
        }

        

        let (less, greater) = poset.calculate_relations();

        let mut best = (0, 0);
        let mut best_count = 0;

        for i in 0..poset.n() {
            if !(less[i as usize] == 0 && greater[i as usize] >= 2) {
                continue;
            }

            for j in i..poset.n() {
                if !(greater[j as usize] == 0 && less[j as usize] >= 2) || poset.has_order(i, j) {
                    continue;
                }

                let count = greater[i as usize] + less[j as usize];

                if count > best_count {
                    best = (i, j);
                    best_count = count;
                }
            }
        }

        if best_count > 0 {
            let cost = self.search_rec(
                backward_search_state,
                poset.with_less(best.0, best.1),
                max_comparisons,
                depth + 1,
            );
            match cost {
                Cost::Solved(solved) => {
                    return Some(solved <= max_comparisons);
                }
                Cost::Minimum(_) => {
                    return Some(false);
                }
            }
        }

        None
    }

    /// Print information out the cache, e.g. cache entries, hits, misses etc.
    pub fn print_cache(&self) {
        // Print information about the cache
        println!("Cache entries: {}", self.cache.len());
        println!("Cache size: {:.3} Gigabyte", self.cache.size_as_gigabyte());
        println!("Cache hits: {}", self.analytics.cache_hits());
        println!("Cache misses: {}", self.analytics.cache_misses());
        println!("Cache replaced: {}", self.analytics.cache_replaced());
        println!();
        println!("Posets searched: {}", self.analytics.total_posets());
    }
}

impl Analytics {
    fn new(max_progress_depth: u8) -> Analytics {
        let multiprogress = MultiProgress::new();

        let mut progress_bars = Vec::with_capacity(max_progress_depth as usize);
        for _ in 0..max_progress_depth {
            let pb = ProgressBar::new(0)
                .with_style(ProgressStyle::with_template("[{pos:2}/{len:2}] {msg}").unwrap());
            let pb = multiprogress.add(pb);
            progress_bars.push((pb, AtomicU64::new(0)));
        }
        Analytics {
            total_posets: 0,
            cache_hits: 0,
            cache_misses: 0,
            cache_replaced: 0,
            max_progress_depth,
            multiprogress,
            progress_bars,
        }
    }

    fn set_max_depth(&mut self, new_depth: u8) {
        if new_depth > self.max_progress_depth {
            for _ in self.max_progress_depth..new_depth {
                let pb = ProgressBar::new(0)
                    .with_style(ProgressStyle::with_template("[{pos:2}/{len:2}] {msg}").unwrap());
                let pb = self.multiprogress.add(pb);
                self.progress_bars.push((pb, AtomicU64::new(0)));
            }
        } else {
            for _ in new_depth..self.max_progress_depth {
                let (pb, _) = self.progress_bars.pop().unwrap();
                pb.finish_and_clear();
                self.multiprogress.remove(&pb);
            }
        }
        self.max_progress_depth = new_depth;
    }

    #[inline]
    fn inc_length(&self, depth: u8, count: u64) {
        if depth >= self.max_progress_depth {
            return;
        }
        self.progress_bars[depth as usize].0.inc_length(count);
        self.progress_bars[depth as usize]
            .1
            .fetch_add(count, Ordering::Relaxed);
    }

    #[inline]
    fn inc(&self, depth: u8, amount: u64) {
        if depth >= self.max_progress_depth {
            return;
        }
        self.progress_bars[depth as usize].0.inc(amount);
    }

    #[inline]
    fn inc_complete(&self, depth: u8, count: u64) {
        if depth >= self.max_progress_depth {
            return;
        }
        let (pb, len) = &self.progress_bars[depth as usize];

        pb.inc(count.wrapping_neg());
        pb.set_length(len.fetch_sub(count, Ordering::Release) - count);
    }

    #[inline]
    fn update_stats(&self, depth: u8, current_max: u8, cache_entries: usize, max_entries: usize) {
        if depth >= self.max_progress_depth {
            return;
        }

        let cache_percentage = cache_entries as f64 / max_entries as f64 * 100.0;
        self.progress_bars[0].0.set_message(format!(
            "limit: {:3} total: {:10}, cache: {:10} ({:2.2} %)",
            current_max, self.total_posets, cache_entries, cache_percentage
        ));
    }

    fn complete_all(&self) {
        for i in 0..self.max_progress_depth as usize {
            let (pb, _) = &self.progress_bars[i];
            pb.finish_and_clear();
            self.multiprogress.remove(pb);
        }
    }

    #[inline]
    fn record_hit(&mut self) {
        self.cache_hits += 1;
    }

    #[inline]
    fn record_miss(&mut self) {
        self.cache_misses += 1;
    }

    #[inline]
    fn record_replace(&mut self) {
        self.cache_replaced += 1;
    }

    #[inline]
    fn record_poset(&mut self) {
        self.total_posets += 1;
    }

    fn cache_hits(&self) -> u64 {
        self.cache_hits
    }

    fn cache_misses(&self) -> u64 {
        self.cache_misses
    }

    fn cache_replaced(&self) -> u64 {
        self.cache_replaced
    }

    fn total_posets(&self) -> u64 {
        self.total_posets
    }
}

impl Drop for Analytics {
    fn drop(&mut self) {
        self.complete_all();
    }
}
