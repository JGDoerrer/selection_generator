use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::sync::atomic::{AtomicBool, AtomicI8, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::Instant;
use std::{thread, vec};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::backwards_poset::BackwardsPoset;
use crate::cache_tree::{CacheTreeNotSolvable, CacheTreeSolvable};
use crate::constants::{lower_bound, upper_bound, KNOWN_VALUES, MAX_N, UPPER_BOUNDS};

fn start_search_backward(
    interrupt: &Arc<AtomicBool>,
    backward_search_state: &Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>,
    start_poset: BackwardsPoset,
    n: u8,
    i0: u8,
    max_comparisons: u8,
) -> Option<u8> {
    {
        let mut write_lock = backward_search_state
            .write()
            .expect("cache shouldn't be poisoned");
        write_lock.0.insert(start_poset.clone(), 0);
        write_lock.1 = 0;
    }

    let mut source = HashSet::new();
    source.insert(start_poset);

    let mut table = [[false; MAX_N]; MAX_N];
    BackwardsPoset::rec_temp(&mut table, n as usize, i0 as usize);

    for k in 1..max_comparisons {
        let start = std::time::Instant::now();
        let results: Vec<_> = source
            .par_iter()
            .map(|item| {
                if interrupt.load(Ordering::Relaxed) {
                    HashSet::new()
                } else {
                    item.enlarge_and_remove_less(
                        interrupt,
                        &backward_search_state
                            .read()
                            .expect("cache shouldn't be poisoned")
                            .0,
                        &table,
                        n,
                        i0,
                        k,
                    )
                }
            })
            .collect();

        let mut destination: HashSet<BackwardsPoset> = HashSet::new();
        for item in results {
            for poset in item {
                debug_assert!(
                    poset.count_min_comparisons()
                        <= UPPER_BOUNDS[n as usize][i0 as usize] - k as usize
                );
                destination.insert(poset);
            }
        }
        {
            let mut write_lock = backward_search_state
                .write()
                .expect("cache shouldn't be poisoned");
            for item in &destination {
                write_lock.0.insert(item.clone(), k);
            }
            write_lock.1 = k as i8;
        }

        println!(
            "# {k}: {} => {} in {:.3?} | total cached: {}",
            source.len(),
            destination.len(),
            start.elapsed(),
            backward_search_state
                .read()
                .expect("cache shouldn't be poisoned")
                .0
                .len()
        );

        if destination.contains(&BackwardsPoset::new(n, i0)) {
            return Some(k);
        }

        source = destination;

        if interrupt.load(Ordering::Relaxed) {
            return None;
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SearchResult {
    FoundSolution,
    NoSolution,
}

#[derive(Debug, Default)]
struct Statistics {
    hash_match_lower_bound: usize,
    hash_match_upper_bound: usize,
    no_solution: usize,
    brute_force: usize,
}

impl Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(cache_l: {}, cache_u: {}, noSol: {}, bruteForce: {})",
            self.hash_match_lower_bound,
            self.hash_match_upper_bound,
            self.no_solution,
            self.brute_force
        )
    }
}

fn search_recursive(
    backward_search_state: &Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>,
    poset: &BackwardsPoset,
    cache_solvable: &mut CacheTreeSolvable,
    cache_not_solvable: &mut CacheTreeNotSolvable,
    remaining_comparisons: u8,
    statistics: &mut Statistics,
) -> SearchResult {
    {
        let read_lock = backward_search_state
            .read()
            .expect("cache shouldn't be poisoned");
        if remaining_comparisons as i8 == read_lock.1 {
            return if read_lock.0.contains_key(poset) {
                SearchResult::FoundSolution
            } else {
                SearchResult::NoSolution
            };
        }
    }

    if cache_not_solvable.check(poset, remaining_comparisons) {
        statistics.hash_match_lower_bound += 1;
        return SearchResult::NoSolution;
    } else if cache_solvable.check(poset, remaining_comparisons) {
        statistics.hash_match_upper_bound += 1;
        return SearchResult::FoundSolution;
    } else if 0 == remaining_comparisons {
        statistics.no_solution += 1;
        return SearchResult::NoSolution;
    }

    let mut result = SearchResult::NoSolution;
    statistics.brute_force += 1;

    let (less, greater) = poset.calculate_relations();

    let cmp = |a: &(u8, u8), b: &(u8, u8)| {
        (greater[b.0 as usize] + less[b.1 as usize])
            .cmp(&(greater[a.0 as usize] + less[a.1 as usize]))
    };

    let mut temp: Vec<(u8, u8)> = vec![];
    for i in 0..poset.n() {
        for j in i + 1..poset.n() {
            if !poset.is_less(i, j) && !poset.is_less(j, i) {
                if cmp(&(j, i), &(i, j)).is_le() {
                    temp.push((i, j));
                } else {
                    temp.push((j, i));
                }
            }
        }
    }

    temp.sort_by(cmp);

    for &(i, j) in &temp {
        if search_recursive(
            backward_search_state,
            &poset.with_less_normalized(i, j),
            cache_solvable,
            cache_not_solvable,
            remaining_comparisons - 1,
            statistics,
        ) == SearchResult::FoundSolution
            && search_recursive(
                backward_search_state,
                &poset.with_less_normalized(j, i),
                cache_solvable,
                cache_not_solvable,
                remaining_comparisons - 1,
                statistics,
            ) == SearchResult::FoundSolution
        {
            result = SearchResult::FoundSolution;
            break;
        }
    }

    if result == SearchResult::NoSolution {
        cache_not_solvable.insert(poset, remaining_comparisons);
    } else if result == SearchResult::FoundSolution {
        cache_solvable.insert(poset, remaining_comparisons);
    }

    result
}

fn start_search_now(
    backward_search_state: &Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>,
    n: u8,
    i: u8,
    cache_solvable: &mut CacheTreeSolvable,
    cache_not_solvable: &mut CacheTreeNotSolvable,
    statistics: &mut Statistics,
    pair_wise_optimization: bool,
    max_comparisons: u8,
    time: &mut Duration,
) -> SearchResult {
    let start = Instant::now();
    let mut poset = BackwardsPoset::new(n, i);
    let mut comparisons_done = 0u8;

    if pair_wise_optimization {
        print!("# search with Pair-Optimisation & maxComparisons = {max_comparisons}");
        for k in (0..n - 1).step_by(2) {
            comparisons_done += 1;
            poset.add_less(k, k + 1);
        }
    } else {
        print!("# search with maxComparisons = {max_comparisons}");
        comparisons_done += 1;
        poset.add_less(0, 1);
    }
    poset.normalize();

    let result = search_recursive(
        backward_search_state,
        &poset,
        cache_solvable,
        cache_not_solvable,
        max_comparisons - comparisons_done,
        statistics,
    );

    if result == SearchResult::FoundSolution {
        print!(" -> FoundSolution");
    } else if result == SearchResult::NoSolution {
        print!(" -> NoSolution");
    }

    let end = Instant::now();
    println!(" in {:.3?}", end - start);
    *time += end - start;

    result
}

fn start_search(
    backward_search_state: &Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>,
    n: u8,
    i: u8,
    cache_solvable: &mut CacheTreeSolvable,
    cache_not_solvable: &mut CacheTreeNotSolvable,
    statistics: &mut Statistics,
    top_to_bottom: bool,
) -> (Option<u8>, Duration, Duration) {
    let mut search_duration = Duration::from_secs(0);
    let mut validate_duration = Duration::from_secs(0);

    let lower: u8 = lower_bound(n as usize, i as usize) as u8;
    let upper: u8 = upper_bound(n as usize, i as usize) as u8;

    if lower == upper {
        return (Some(lower), search_duration, validate_duration);
    }

    assert!(2 < n);

    if top_to_bottom {
        // searchRecursive from top
        for max_comparisons in (lower..upper).rev() {
            if start_search_now(
                backward_search_state,
                n,
                i,
                cache_solvable,
                cache_not_solvable,
                statistics,
                true,
                max_comparisons,
                &mut search_duration,
            ) == SearchResult::NoSolution
                && start_search_now(
                    backward_search_state,
                    n,
                    i,
                    cache_solvable,
                    cache_not_solvable,
                    statistics,
                    false,
                    max_comparisons,
                    &mut validate_duration,
                ) == SearchResult::NoSolution
            {
                return (
                    Some(max_comparisons + 1),
                    search_duration,
                    validate_duration,
                );
            }
        }

        (Some(lower), search_duration, validate_duration)
    } else {
        // searchRecursive from bottom
        for max_comparisons in lower..upper {
            if start_search_now(
                backward_search_state,
                n,
                i,
                cache_solvable,
                cache_not_solvable,
                statistics,
                true,
                max_comparisons,
                &mut search_duration,
            ) == SearchResult::FoundSolution
                || start_search_now(
                    backward_search_state,
                    n,
                    i,
                    cache_solvable,
                    cache_not_solvable,
                    statistics,
                    false,
                    max_comparisons,
                    &mut validate_duration,
                ) == SearchResult::FoundSolution
            {
                return (Some(max_comparisons), search_duration, validate_duration);
            }
        }

        (Some(upper), search_duration, validate_duration)
    }
}

pub fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(5)
        .build_global()
        .unwrap();

    let mut cache_solvable = CacheTreeSolvable::new();
    let mut cache_not_solvable = CacheTreeNotSolvable::new();
    cache_solvable.insert(&BackwardsPoset::new(1, 0), 0);

    for n in 2..MAX_N as u8 {
        for i in 0..((n + 1) / 2) {
            let mut statistics = Statistics::default();

            let backward_search_state = Arc::new(RwLock::new((HashMap::new(), -1)));
            let interrupt = Arc::new(AtomicBool::new(false));
            let dyn_level = Arc::new(AtomicI8::new(-1));
            let handle = {
                let backward_search_state_local = backward_search_state.clone();
                let interrupt_local = interrupt.clone();
                #[allow(unused)]
                let dyn_level_local = dyn_level.clone();
                thread::spawn(move || {
                    start_search_backward(
                        &interrupt_local,
                        &backward_search_state_local,
                        BackwardsPoset::new(1, 0),
                        n,
                        i,
                        n * n,
                    );
                })
            };

            let (comparisons, duration_search, duration_validate) = start_search(
                &backward_search_state,
                n,
                i,
                &mut cache_solvable,
                &mut cache_not_solvable,
                &mut statistics,
                true,
            );

            interrupt.store(true, Ordering::Relaxed);
            handle.join().unwrap();

            if let Some(comparisons_value) = comparisons {
                println!(
            "\rtime '{:.3?}' + '{:.3?}' = '{:.3?}': n = {}, i = {}, {}, cache = {} + {} = {}, comparisons: {}",
            duration_search,
            duration_validate,
            duration_search + duration_validate,
            n,
            i,
            statistics,
            cache_solvable.size(),
            cache_not_solvable.size(),
            cache_solvable.size() + cache_not_solvable.size(),
            comparisons_value
          );
                if comparisons_value as usize != KNOWN_VALUES[n as usize][i as usize] {
                    eprintln!(
                        "Error: got {}, but expected {}",
                        comparisons_value, KNOWN_VALUES[n as usize][i as usize]
                    );
                    std::process::exit(0);
                }
            } else {
                eprintln!("Error, maxComparisons exceeded");
                std::process::exit(0);
            }
        }
        println!();
    }
}
