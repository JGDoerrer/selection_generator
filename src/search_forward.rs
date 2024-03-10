use std::{
    collections::HashMap,
    ops::Deref,
    sync::{
        atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering},
        Arc, Mutex, RwLock,
    },
    thread::{self, spawn},
    time::{Duration, Instant},
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use priority_queue::PriorityQueue;

use crate::{
    cache::Cache,
    canonified_poset::CanonifiedPoset,
    constants::{LOWER_BOUNDS, UPPER_BOUNDS},
    poset::Poset,
    tree::{OtherState, Parent, Priority, SearchState, Task},
   
    utils::format_duration,
};

type TaskQueue = PriorityQueue<Task, Priority>;
type CallbackStash = HashMap<CanonifiedPoset, Vec<Task>>;
type Algorithm = HashMap<CanonifiedPoset, (u8, u8)>;

pub struct Search {
    n: u8,
    i: u8,
    current_max: AtomicU8,
    cache: Arc<Cache>,
    task_queue: Mutex<TaskQueue>,
    analytics: Analytics,
    active_posets: RwLock<CallbackStash>,
    threads: AtomicUsize,
    comparisons: Arc<Mutex<Algorithm>>,
}

struct Worker {
    search: Arc<Search>,
    private_queue: Vec<(Task, bool)>,
}

impl Deref for Worker {
    type Target = Search;
    fn deref(&self) -> &Self::Target {
        self.search.as_ref()
    }
}

#[derive(Debug, Clone, Copy)]
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
    stashed: AtomicU64,
    cache_duplicates: AtomicU64,
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
    const MIN_THREAD_COMPARISONS: u8 = 12;
    pub fn new(n: u8, i: u8, cache: Arc<Cache>, comparisons: Arc<Mutex<Algorithm>>) -> Self {
        let min = LOWER_BOUNDS[n as usize][i as usize];
        let max = UPPER_BOUNDS[n as usize][i as usize];
        Search {
            n,
            i,
            current_max: 0.into(),
            cache,
            task_queue: PriorityQueue::new().into(),
            active_posets: HashMap::new().into(),
            analytics: Analytics::new(((max + min) as u8 / 4).max(2)),
            threads: AtomicUsize::new(1),
            comparisons,
        }
    }

    pub fn search(self: Arc<Search>) -> u8 {
        let start = Instant::now();

        let min = LOWER_BOUNDS[self.n as usize][self.i as usize];
        let max = UPPER_BOUNDS[self.n as usize][self.i as usize];

        let mut result = max as u8;

        for current in min..max {
            let current = current as u8;
            self.current_max.store(current, Ordering::Relaxed);

            let search_result = self.search_threaded(CanonifiedPoset::new(self.n, self.i), current);
            result = match search_result {
                Cost::Solved(solved) => solved,
                Cost::Minimum(min) => {
                    self.analytics.multiprogress.clear().unwrap();
                    println!(
                        "n: {}, i: {} needs at least {} comparisons",
                        self.n, self.i, min
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
        println!("Comparisons: {}", result);
        println!();

        self.print_cache();
        println!("{}", format_duration(start));
        println!();

        result
    }

    fn search_threaded(self: &Arc<Search>, poset: CanonifiedPoset, max_comparisons: u8) -> Cost {
        let worker = Worker::new(self.clone());
        if max_comparisons < Search::MIN_THREAD_COMPARISONS || (self.i == 5 && max_comparisons == 24) {
            return worker.search_rec(poset, max_comparisons, 0);
        }

        let thread_count = if let Ok(count) = thread::available_parallelism() {
            count.get()
        } else {
            return worker.search_rec(poset, max_comparisons, 0);
        };

        let (root, children) = Task {
            poset,
            parent: Parent::Root(max_comparisons),
            other: OtherState::Solved(0),
            depth: 0,
            comparison: (0, 0),
        }
        .expand();

        self.task_queue.lock().unwrap().extend(children.map(|t| {
            let priority = t.priority();
            (t, priority)
        }));
        self.analytics.inc_length(root.depth, root.total_children);

        for _ in 0..max_comparisons / 2 {
            let task = if let Some(t) = worker.fetch_task() {
                t
            } else {
                return root.current_best();
            };
            let r = worker.check_heuristics(&task);
            if let Some(cost) = r {
                worker.apply_heuristic_result(&task, cost);
            } else {
                worker.expand_task(task);
            }
        }

        let mut threads = Vec::new();
        self.threads.store(thread_count, Ordering::Relaxed);

        for _ in 0..thread_count {
            let worker = Worker::new(self.clone());
            let root = root.clone();
            threads.push(spawn(move || {
                // let mut i: u64 = 0;
                while root.open_children.load(Ordering::Relaxed) > 0 {
                    while let Some(task) = worker.fetch_task() {
                        // i = i.wrapping_add(1);
                        // // update max comparisons
                        // if i % (1 << 20) == 0 {
                        //     let mut queue = worker.task_queue.lock().unwrap();
                        //     queue.iter_mut().for_each(|(t, p)| p.max_comparisons = t.max_comparisons());
                        // }
                        worker.search_tree(task);
                    }
                    thread::sleep(Duration::from_nanos(500));
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

    fn update_stats(&self, depth: u8) {
        self.analytics.update_stats(
            depth,
            self.current_max.load(Ordering::Relaxed),
            self.cache.len(),
            self.cache.max_entries(),
            self.active_posets.read().unwrap().len(),
            self.threads.load(Ordering::Relaxed),
        );
    }

    /// Print information out the cache, e.g. cache entries, hits, misses etc.
    pub fn print_cache(&self) {
        // Print information about the cache
        println!("Cache entries: {}", self.cache.len());
        println!("Cache size: {} Gigabyte", self.cache.size_as_gigabyte());
        println!("Cache hits: {}", self.analytics.cache_hits());
        println!("Cache misses: {}", self.analytics.cache_misses());
        println!("Cache replaced: {}", self.analytics.cache_replaced());
        println!("Duplicate work: {}", self.analytics.cache_duplicates());
        println!();
        println!("Posets searched: {}", self.analytics.total_posets());
    }
}

impl Worker {
    fn new(search: Arc<Search>) -> Self {
        Worker {
            search,
            private_queue: Vec::new(),
        }
    }

    fn search_tree(&self, task: Task) {
        // self.private_queue.push((start, false));

        // while let Some(task) = self.fetch_private_task() {
        if task.max_comparisons() < self.n - 1 {
            let cost = self.search_rec(task.poset, task.max_comparisons(), task.depth);
            self.apply_heuristic_result(&task, cost);
        } else {
            let mut current = task;
            loop {
                let r = self.check_heuristics(&current);
                if let Some(cost) = r {
                    self.apply_heuristic_result(&current, cost);
                    break;
                } else if let Some(comparison) = current.poset.find_useful_comparison() {
                    current = current.to_estimate_solvable_task(comparison);
                } else {
                    self.expand_task(current);
                    break;
                }
            }
        };
        // }

        // assert!(self.private_queue.is_empty());
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

    fn fetch_task(&self) -> Option<Task> {
        let mut active_posets = self.search.active_posets.write().unwrap();
        let mut task_queue = self.task_queue.lock().unwrap();

        while let Some((task, _)) = task_queue.pop() {
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

    fn queue_global(&self, task: Task) {
        let priority = task.priority();
        self.task_queue.lock().unwrap().push(task, priority);
    }

    fn propagate_done(&self, parent: &Parent) {
        if let Parent::Parent(state) = parent {
            self.analytics.inc(state.depth, 1);
            let open = state.open_children.fetch_sub(1, Ordering::Relaxed) - 1;

            if open == 0 {
                let result = state.current_best();
                self.insert_cache(state.poset, result);
                self.analytics
                    .inc_complete(state.depth, state.total_children);
                self.update_stats(state.depth);
                self.apply_result(state, result);
            }
        }
    }

    fn propagate_solved(&self, parent: &Parent, cost: u8, comparison: (u8, u8)) {
        match parent {
            Parent::Parent(p) => {
                let mut current_best = p.current_best.write().unwrap();
                // stop if there's no improvement
                if current_best.value() > cost {
                    self.comparisons.lock().unwrap().insert(p.poset, comparison);
                    *current_best = Cost::Solved(cost)
                } else {
                    return;
                }
                // or if the parent is only on the first poset
                if let OtherState::Solved(other) = p.other {
                    self.propagate_solved(&p.parent, cost.max(other) + 1, p.comparison);
                }
            }
            _ => {}
        }
    }

    fn apply_result(&self, state: &Arc<SearchState>, cost: Cost) {
        assert!(cost.is_solved() || cost.value() > state.max_comparisons());

        if let Some(task) = state.make_second_task(cost) {
            self.queue_global(task);
        } else if let Parent::Estimate(estimated) = &state.parent {
            let max_comparisons = estimated.max_comparisons();
            if let Cost::Solved(solved) = cost {
                if solved <= max_comparisons {
                    self.expand_task(*estimated.to_owned());
                } else {
                    self.apply_heuristic_result(&estimated, Cost::Minimum(max_comparisons + 1))
                }
            } else {
                self.apply_heuristic_result(&estimated, Cost::Minimum(max_comparisons + 1))
            }
        } else {
            self.propagate_done(&state.parent);
        }
        self.notify_stashed(&state.poset, cost);
    }

    fn apply_heuristic_result(&self, task: &Task, cost: Cost) {
        assert!(cost.is_solved() || cost.value() > task.max_comparisons());
        if let Cost::Solved(solved) = cost {
            if let OtherState::Solved(other) = task.other {
                self.propagate_solved(&task.parent, solved.max(other) + 1, task.comparison.clone())
            }
        }

        if let Some(task) = task.make_second_task(cost) {
            self.queue_global(task);
        } else if let Parent::Estimate(estimated) = &task.parent {
            let max_comparisons = estimated.max_comparisons();
            if let Cost::Solved(solved) = cost {
                if solved <= max_comparisons {
                    self.expand_task(*estimated.to_owned());
                } else {
                    self.apply_heuristic_result(&estimated, Cost::Minimum(max_comparisons + 1))
                }
            } else {
                self.apply_heuristic_result(&estimated, Cost::Minimum(max_comparisons + 1))
            }
        } else {
            self.propagate_done(&task.parent);
        }
        self.notify_stashed(&task.poset, cost);
    }

    fn notify_stashed(&self, poset: &CanonifiedPoset, cost: Cost) {
        let waiters = self.active_posets.write().unwrap().remove(poset);
        if let Some(waiters) = waiters {
            self.analytics
                .stashed
                .fetch_sub(waiters.len() as u64, Ordering::Relaxed);
            for task in waiters {
                if answers(&task, cost) {
                    self.apply_heuristic_result(&task, cost);
                } else {
                    self.queue_global(task);
                }
            }
        }
    }

    fn check_heuristics(&self, task: &Task) -> Option<Cost> {
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

        let mut comparisons = 0;
        for i in 0..task.poset.n() {
            'j_loop: for j in task.poset.get_all_greater_than(i) {
                let j = j as u8;
                for k in (i + 1)..j {
                    if task.poset.is_less(i, k) && task.poset.is_less(k, j) {
                        continue 'j_loop;
                    }
                }

                comparisons += 1;
            }
        }
        if comparisons + max_comparisons < task.poset.n() {
            return Some(Cost::Minimum(max_comparisons + 1));
        }

        None
    }

    fn expand_task(&self, task: Task) -> Arc<SearchState> {
        let (state, children) = task.expand();

        self.analytics.inc_length(state.depth, state.total_children);

        children.for_each(|task| self.queue_global(task));

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

        if let Some(false) = self.estimate_solvable(poset, max_comparisons, depth) {
            let result = Cost::Minimum(max_comparisons + 1);

            self.insert_cache(poset, result);

            return result;
        }

        let mut pairs = poset.get_comparison_pairs();
        pairs.sort_by_key(|pair| pair.4);
        let n_pairs = pairs.len() as u64;

        self.analytics.inc_length(depth, n_pairs);

        // search all comparisons
        let mut best_comparison = (0, 0);
        let mut current_best = max_comparisons + 1;
        for (first, second, i, j, _) in pairs {
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
            best_comparison = (i, j);

            current_best = first_result.value().max(second_result.value()) + 1;

            self.analytics.inc(depth, 1);
        }

        let result = if current_best <= max_comparisons {
            self.comparisons
                .lock()
                .unwrap()
                .insert(poset, best_comparison);
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
        &self,
        poset: CanonifiedPoset,
        max_comparisons: u8,
        depth: u8,
    ) -> Option<bool> {
        let compatible_posets = poset.num_compatible_posets();
        if compatible_posets == 0 || (max_comparisons as u32) < compatible_posets.ilog2() {
            return Some(false);
        }

        let (less, greater) = poset.calculate_relations();

        for i in 0..poset.n() {
            if !(less[i as usize] == 0 && greater[i as usize] >= 2) {
                continue;
            }

            for j in i..poset.n() {
                if !(greater[j as usize] == 0 && less[j as usize] >= 2) || poset.has_order(i, j) {
                    continue;
                }

                let cost = self.search_rec(poset.with_less(i, j), max_comparisons, depth + 1);
                match cost {
                    Cost::Solved(solved) => {
                        return Some(solved <= max_comparisons);
                    }
                    Cost::Minimum(_) => {
                        return Some(false);
                    }
                }
            }
        }

        None
    }
}

fn answers(task: &Task, cost: Cost) -> bool {
    match cost {
        Cost::Minimum(min) => min > task.max_comparisons(),
        Cost::Solved(_) => true,
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
        cache_max: usize,
        nodes: usize,
        threads: usize,
    ) {
        if depth >= self.max_progress_depth {
            return;
        }
        self.progress_bars[0].0.set_message(format!(
            "limit: {:3} total: {:10}, cache: {:10} ({:.3}%), duplicates: {:10}",
            current_max,
            self.total_posets.load(Ordering::Relaxed),
            cache_entries,
            cache_entries as f32 / cache_max as f32 * 100.0,
            self.cache_duplicates(),
        ));
        self.progress_bars[1].0.set_message(format!(
            "active nodes: {:10}, stashed: {:7}, threads: {:3}",
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
