use std::{
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc, Mutex, RwLock,
    },
    thread::{self, spawn},
    time::Instant,
};

use hashbrown::HashMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::{
    cache::Cache,
    canonified_poset::CanonifiedPoset,
    constants::{LOWER_BOUNDS, MAX_N, UPPER_BOUNDS},
    poset::Poset, tree::{OtherState, Parent, SearchState, Task},
};

type MutArc<T> = Arc<Mutex<T>>;
type RwArc<T> = Arc<RwLock<T>>;

type TaskQueue = Vec<Task>;
type CallbackStash = HashMap<CanonifiedPoset, Vec<Task>>;

#[derive(Clone)]
pub struct Search {
    n: u8,
    i: u8,
    current_max: u8,
    cache: Arc<Cache>,
    task_queue: MutArc<TaskQueue>,
    analytics: Arc<Analytics>,
    start: Instant,
    active_posets: RwArc<CallbackStash>,
    threads: Arc<AtomicUsize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cost {
    /// Not solved. Impossible in less than the number of comparisons
    Minimum(u8),
    /// Solved in the number of comparisons
    Solved(u8),
}

pub struct Analytics {
    total_posets: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    cache_replaced: AtomicU64,
    cache_duplicates: AtomicU64,
    stashed: AtomicU64,
    max_progress_depth: u8,
    multiprogress: MultiProgress,
    progress_bars: Vec<(ProgressBar, AtomicU64)>,
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

impl Search {
    pub fn new(n: u8, i: u8, cache: Arc<Cache>) -> Self {
        Search {
            n,
            i,
            current_max: 0,
            cache,
            task_queue: Arc::new(Mutex::new(Vec::new())),
            active_posets: Arc::new(RwLock::new(HashMap::new())),
            analytics: Arc::new(Analytics::new(0)),
            start: Instant::now(),
            threads: AtomicUsize::new(1).into(),
        }
    }

    fn update_stats(&self, depth: u8) {
        self.analytics.update_stats(
            depth,
            self.current_max,
            self.cache.len(),
            self.active_posets.read().unwrap().len(),
            self.threads.load(Ordering::Relaxed),
        );
    }

    fn search_cache(&self, poset: &CanonifiedPoset) -> Option<Cost> {
        let result = self.cache.get_mut(poset);
        if result.is_some() {
            self.analytics.record_hit();
        } else {
            self.analytics.record_miss();
        }
        result
    }

    fn insert_cache(&self, poset: CanonifiedPoset, new_cost: Cost) {
        if let Some(cost) = self.cache.get(&poset) {
            let res = match (cost, new_cost) {
                (Cost::Minimum(old_min), Cost::Minimum(new_min)) => {
                    Cost::Minimum(new_min.max(old_min))
                }
                (Cost::Solved(old_solved), Cost::Solved(new_solved)) => {
                    self.analytics.record_duplicate();
                    Cost::Solved(new_solved.min(old_solved))
                }
                (Cost::Solved(_), Cost::Minimum(_)) => {
                    self.analytics.record_duplicate();
                    cost
                }
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
        self.start = Instant::now();

        let min = LOWER_BOUNDS[self.n as usize][self.i as usize];
        let max = UPPER_BOUNDS[self.n as usize][self.i as usize];

        let mut result = max as u8;

        for current in min..max {
            let current = current as u8;
            self.current_max = current;
            self.analytics = Arc::new(self.analytics.with_max_depth(current / 2));

            let search_result = self.search_tree(CanonifiedPoset::new(self.n, self.i), current);
            result = match search_result {
                Cost::Solved(solved) => solved,
                Cost::Minimum(min) => {
                    self.analytics.multiprogress.clear().unwrap();
                    println!(
                        "n: {}, i: {} needs at least {} comparisons",
                        self.n, self.i, min
                    );
                    println!("{}", self.format_duration());

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
        println!("Comparisons: {}", result);
        println!();

        self.print_cache();
        println!("{}", self.format_duration());
        println!();

        result
    }

    fn search_tree(&self, poset: CanonifiedPoset, max_comparisons: u8) -> Cost {
        const THRESHOLD: u8 = 3;

        if max_comparisons < THRESHOLD {
            return self.search_rec(poset, max_comparisons, 0);
        }

        let mut threads = Vec::new();

        let root = self.expand_task(Task {
            poset,
            parent: Parent::Root(max_comparisons),
            other: OtherState::Solved(0),
            depth: 0,
        });

        // start single-threaded to build a starting tree
        for _ in 0..max_comparisons / 2 {
            let task = self.fetch_task().unwrap();
            let r = self.check_task(&task);
            if let Some(cost) = r {
                self.apply_heuristic_result(&task, cost);
            } else {
                self.expand_task(task);
            }
        }

        let thread_count = if let Ok(count) = thread::available_parallelism() {
            count.get()
        } else {
            return self.search_rec(poset, max_comparisons, 0);
        };

        self.threads.store(thread_count, Ordering::Relaxed);

        for _ in 0..thread_count {
            let worker = self.clone();
            threads.push(spawn(move || {
                while let Some(task) = worker.fetch_task() {

                    if task.max_comparisons() < THRESHOLD {
                        let cost =
                            worker.search_rec(task.poset, task.max_comparisons(), task.depth);
                        worker.apply_heuristic_result(&task, cost);
                    } else {
                        let r = worker.check_task(&task);
                        if let Some(cost) = r {
                            worker.apply_heuristic_result(&task, cost);
                        } else {
                            worker.expand_task(task);
                        }
                    };
                }
                worker.threads.fetch_sub(1, Ordering::Relaxed);
            }))
        }

        let mut panicking = false;

        for handle in threads {
            match handle.join() {
                Ok(()) => {}
                Err(_) => {
                    panicking = true;
                }
            }
        }

        assert_eq!(self.active_posets.read().unwrap().len(), 0);

        if panicking {
            panic!("Worker panicked")
        }

        root.current_best()
    }

    fn fetch_task(&self) -> Option<Task> {
        let mut active_posets = self.active_posets.write().unwrap();
        let mut task_queue = self.task_queue.lock().unwrap();

        while let Some(task) = task_queue.pop() {
            let entry = active_posets.get_mut(&task.poset);

            match entry {
                Some(waiting) => {
                    self.analytics.stashed.fetch_add(1, Ordering::Relaxed);
                    waiting.push(task);
                }
                None => {
                    active_posets.insert(task.poset, Vec::new());
                    return Some(task);
                }
            }
        }
        None
    }

    fn propagate_done(&self, parent: &Parent, cost: Cost) {
        match parent {
            Parent::Parent(parent) => {
                if cost.is_solved() {
                    let mut current_best = parent.current_best.write().unwrap();
                    *current_best = Cost::Solved(current_best.value().min(cost.value()));
                }
                let open = parent.open_children.fetch_sub(1, Ordering::Relaxed) - 1;

                if open == 0 {
                    let result = parent.current_best();
                    self.insert_cache(parent.poset, result);
                    self.analytics
                        .inc_complete(parent.depth, parent.total_children);
                    self.update_stats(parent.depth);
                    self.apply_result(parent, result);
                }
            }
            Parent::Root(_) => (),
        }
    }

    fn apply_result(&self, state: &Arc<SearchState>, cost: Cost) {
        assert!(cost.is_solved() || cost.value() > state.max_comparisons());
        self.apply_inner(&state.poset, &state.other, &state.parent, state.max_comparisons(), state.depth, cost)
    }

    fn apply_heuristic_result(&self, task: &Task, cost: Cost) {
        assert!(cost.is_solved() || cost.value() > task.max_comparisons());
        self.apply_inner(&task.poset, &task.other, &task.parent, task.max_comparisons(), task.depth, cost)
    }

    fn apply_inner(
        &self,
        poset: &CanonifiedPoset,
        other: &OtherState,
        parent: &Parent,
        max_comparisons: u8,
        depth: u8,
        cost: Cost,
    ) {
        self.analytics.inc(depth - 1, 1);
        match cost {
            Cost::Solved(solved) => match other {
                OtherState::Solved(other) => {
                    self.propagate_done(parent, Cost::Solved(solved.max(*other) + 1));
                }
                OtherState::Open(poset) => {
                    if solved <= max_comparisons {
                        self.task_queue.lock().unwrap().push(Task {
                            poset: *poset,
                            parent: parent.clone(),
                            other: OtherState::Solved(solved),
                            depth,
                        });
                    } else {
                        self.propagate_done(parent, Cost::Minimum(solved));
                    }
                }
            },
            Cost::Minimum(min) => self.propagate_done(parent, Cost::Minimum(min + 1)),
        }
        let waiters = self.active_posets.write().unwrap().remove(poset);
        if let Some(waiters) = waiters {
            self.analytics.stashed.fetch_sub(waiters.len() as u64, Ordering::Relaxed);
            for task in waiters {
                match cost {
                    Cost::Minimum(min) => {
                        if min > task.max_comparisons() {
                            self.apply_heuristic_result(&task, cost);
                        } else {
                            self.task_queue.lock().unwrap().push(task);
                        }
                    }
                    Cost::Solved(_) => self.apply_heuristic_result(&task, cost),
                }
            }
        }
    }

    fn check_task(&self, task: &Task) -> Option<Cost> {
        if task.poset.n() == 1 {
            return Some(Cost::Solved(0));
        }

        let max_comparisons = task.max_comparisons();
        if max_comparisons == 0 {
            return Some(Cost::Minimum(1));
        }

        if let Some(cost) = self.search_cache(&task.poset) {
            match cost {
                Cost::Solved(solved) => {
                    return if solved > max_comparisons {
                        Some(Cost::Minimum(solved))
                    } else {
                        Some(cost)
                    }
                }
                Cost::Minimum(min) => {
                    if min > max_comparisons {
                        return Some(cost);
                    }
                }
            }
        }

        if let Some(false) = self.estimate_solvable(task.poset, max_comparisons, 0, 0, task.depth) {
            let result = Cost::Minimum(max_comparisons + 1);

            self.insert_cache(task.poset, result);

            return Some(result);
        }
        None
    }

    fn expand_task(&self, task: Task) -> Arc<SearchState> {
        let pairs = self.get_comparison_pairs(&task.poset);
        let n_pairs = pairs.len() as u64;

        let state = Arc::new(SearchState {
            current_best: Cost::Minimum(task.max_comparisons() + 1).into(),
            poset: task.poset,
            parent: task.parent,
            other: task.other,
            total_children: n_pairs,
            open_children: n_pairs.into(),
            depth: task.depth,
        });

        self.analytics.inc_length(task.depth, n_pairs);

        for (first, second) in pairs.into_iter().rev() {
            let current = Task {
                poset: first,
                parent: Parent::Parent(state.clone()),
                other: OtherState::Open(second),
                depth: state.depth + 1,
            };
            self.task_queue.lock().unwrap().push(current);
        }

        state
    }

    fn search_rec(&self, poset: CanonifiedPoset, max_comparisons: u8, depth: u8) -> Cost {
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
        let n_pairs = pairs.len() as u64;

        self.analytics.inc_length(depth, n_pairs);

        // search all comparisons
        let mut current_best = max_comparisons + 1;
        for (first, second) in pairs {
            self.update_stats(depth);

            // search the first case of the comparison
            let first_result = self.search_rec(first, current_best - 2, depth + 1);

            if !first_result.is_solved() || first_result.value() > current_best - 2 {
                self.analytics.inc(depth, 1);
                continue;
            }

            // search the second case of the comparison
            let second_result = self.search_rec(second, current_best - 2, depth + 1);

            if !second_result.is_solved() || second_result.value() > current_best - 2 {
                self.analytics.inc(depth, 1);
                continue;
            }

            // take the max of the branches of the comparisons
            // if the current pair maximum was worse, the
            // continues above never let this be reached
            current_best = first_result.value().max(second_result.value()) + 1;

            self.analytics.inc(depth, 1);
        }

        let result = if current_best <= max_comparisons {
            Cost::Solved(current_best)
        } else {
            Cost::Minimum(max_comparisons + 1)
        };

        self.analytics.inc_complete(depth, n_pairs);

        self.analytics.record_poset();

        self.insert_cache(poset, result);

        result
    }

    fn get_comparison_pairs(
        &self,
        poset: &CanonifiedPoset,
    ) -> Vec<(CanonifiedPoset, CanonifiedPoset)> {
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

    fn estimate_hardness(poset: &CanonifiedPoset) -> u32 {
        let (less, greater) = poset.calculate_relations();

        let mut sum = 0;

        for i in 0..poset.n() as usize {
            sum += (MAX_N as u32 - (poset.i() - greater[i]) as u32).pow(2);
            sum += (MAX_N as u32 - (poset.n() - poset.i() - 1 - less[i]) as u32).pow(2);
        }

        sum
    }

    fn estimate_solvable(
        &self,
        poset: CanonifiedPoset,
        max_comparisons: u8,
        start_i: u8,
        start_j: u8,
        depth: u8,
    ) -> Option<bool> {
        if start_i != 0 || start_j != 0 {
            match self.search_cache(&poset) {
                Some(Cost::Solved(solved)) => {
                    return Some(solved <= max_comparisons);
                }
                Some(Cost::Minimum(min)) => {
                    if min > max_comparisons {
                        return Some(false);
                    }
                }
                _ => (),
            }
        }

        let compatible_posets = poset.num_compatible_posets();
        if compatible_posets == 0 || (max_comparisons as u32) < compatible_posets.ilog2() {
            return Some(false);
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

                if let Some(false) =
                    self.estimate_solvable(poset.with_less(i, j), max_comparisons, i, j + 1, depth)
                {
                    return Some(false);
                }
            }
        }

        if start_i != 0 || start_j != 0 {
            let cost = self.search_rec(poset, max_comparisons, depth + 1);
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

        None
    }

    /// Print out a human readable duration in the format:
    /// days, hours, minutes, seconds
    pub fn format_duration(&self) -> String {
        // Calculate the values for a human readable duration

        let duration = Instant::now() - self.start;
        let seconds = duration.as_secs_f32() % 60.0;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / (60 * 60)) % 24;
        let days = duration.as_secs() / (60 * 60 * 24);

        format!("Duration: {}d {}h {}m {}s", days, hours, minutes, seconds)
    }

    /// Print information out the cache, e.g. cache entries, hits, misses etc.
    pub fn print_cache(&self) {
        // Print information about the cache
        println!("Cache entries: {}", self.cache.len());
        println!("Cache hits: {}", self.analytics.cache_hits());
        println!("Cache misses: {}", self.analytics.cache_misses());
        println!("Cache replaced: {}", self.analytics.cache_replaced());
        println!("Duplicate work: {}", self.analytics.cache_duplicates());
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
                .with_style(ProgressStyle::with_template("[{pos:4}/{len:4}] {msg}").unwrap());
            let pb = multiprogress.add(pb);
            progress_bars.push((pb, AtomicU64::new(0)));
        }
        Analytics {
            total_posets: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            cache_replaced: AtomicU64::new(0),
            stashed: AtomicU64::new(0),
            max_progress_depth,
            multiprogress,
            progress_bars,
            cache_duplicates: 0.into(),
        }
    }

    fn with_max_depth(&self, new_depth: u8) -> Self {
        let new = Self::new(new_depth);
        new.total_posets
            .store(self.total_posets(), Ordering::Relaxed);
        new.cache_hits.store(self.cache_hits(), Ordering::Relaxed);
        new.cache_misses
            .store(self.cache_misses(), Ordering::Relaxed);
        new.cache_duplicates
            .store(self.cache_duplicates(), Ordering::Relaxed);
        new
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
    fn update_stats(
        &self,
        depth: u8,
        current_max: u8,
        cache_entries: usize,
        nodes: usize,
        threads: usize,
    ) {
        if depth >= self.max_progress_depth {
            return;
        }
        self.progress_bars[0].0.set_message(format!(
            "limit: {:3} total: {:10}, cache: {:10}",
            current_max,
            self.total_posets.load(Ordering::Relaxed),
            cache_entries,
        ));
        self.progress_bars[1].0.set_message(format!(
            "duplicates: {:10}, active nodes: {:10}, stashed: {:7} threads: {:3}",
            self.cache_duplicates(),
            nodes,
            self.stashed.load(Ordering::Relaxed),
            threads,
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
    fn record_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_replace(&self) {
        self.cache_replaced.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_poset(&self) {
        self.total_posets.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn record_duplicate(&self) {
        self.cache_duplicates.fetch_add(1, Ordering::Relaxed);
    }

    fn cache_hits(&self) -> u64 {
        self.cache_hits.load(Ordering::Relaxed)
    }

    fn cache_misses(&self) -> u64 {
        self.cache_misses.load(Ordering::Relaxed)
    }

    fn cache_replaced(&self) -> u64 {
        self.cache_replaced.load(Ordering::Relaxed)
    }

    fn total_posets(&self) -> u64 {
        self.total_posets.load(Ordering::Relaxed)
    }

    fn cache_duplicates(&self) -> u64 {
        self.cache_duplicates.load(Ordering::Relaxed)
    }
}

impl Drop for Analytics {
    fn drop(&mut self) {
        self.complete_all();
    }
}
