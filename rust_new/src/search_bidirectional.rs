use std::collections::HashSet;
use std::fmt::{self, Display};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::Instant;
use std::{thread, vec};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::cache_set::{CacheSetNotSolvable, CacheSetSolvable};
use super::cache_tree::{CacheTreeNotSolvable, CacheTreeSolvable};
use super::poset::Poset;
use super::util::{lower_bound, upper_bound, KNOWN_MIN_VALUES, MAX_N};

fn start_search_backward(
  interrupt: &Arc<AtomicBool>,
  dyn_level: &Arc<AtomicU8>,
  poset_cache: &Arc<RwLock<CacheSetSolvable>>,
  n: u8,
  i0: u8,
  max_comparisons: u8,
) -> (Option<u8>, Duration, Duration) {
  let mut duration_build_posets_total = Duration::from_secs(0);
  let mut duration_test_posets_total = Duration::from_secs(0);
  let mut source = HashSet::new();
  source.insert(Poset::new(1, 0));

  for k in 1..max_comparisons {
    let start = std::time::Instant::now();
    let source_new = Poset::enlarge(interrupt, &source, n, i0);
    let mid = std::time::Instant::now();
    let duration_build_posets = mid - start;
    duration_build_posets_total += duration_build_posets;

    let atomic_break = Arc::new(AtomicBool::new(false));
    let interrupt_local = interrupt.clone();
    let results: Vec<HashSet<Poset>> = source_new
      .par_iter()
      .map(|item| {
        let mut destination: HashSet<Poset> = HashSet::new();
        for i in 0..n {
          for j in 0..n {
            if item.is_less(i, j) {
              for mut predecessor in item.remove_less(i, j, |poset| {
                poset_cache
                  .read()
                  .expect("cache shouldn't be poisoned")
                  .check(poset, k - 1)
              }) {
                if predecessor == Poset::new(n, i0) {
                  atomic_break.store(true, Ordering::Relaxed);
                }
                predecessor.normalize();
                if poset_cache
                  .read()
                  .expect("cache shouldn't be poisoned")
                  .check(&predecessor, k - 1)
                {
                  continue;
                }
                poset_cache
                  .write()
                  .expect("cache shouldn't be poisoned")
                  .insert(&predecessor, k);
                destination.insert(predecessor);

                if atomic_break.load(Ordering::Relaxed) || interrupt_local.load(Ordering::Relaxed) {
                  return destination;
                }
              }
            }
          }
        }
        destination
      })
      .collect();

    if interrupt.load(Ordering::Relaxed) {
      return (
        None,
        duration_build_posets_total,
        duration_test_posets_total,
      );
    }

    let mut destination: HashSet<Poset> = HashSet::new();
    for item in results {
      for poset in item {
        destination.insert(poset);
      }
    }
    dyn_level.store(k, Ordering::Relaxed);

    let duration_test_posets = mid.elapsed();
    duration_test_posets_total += duration_test_posets;

    print!(
      "# {}: {} => {} in {:.3?} ~ {:.3?} | total cached: {}",
      k,
      source.len(),
      source_new.len(),
      duration_build_posets,
      duration_test_posets,
      poset_cache
        .read()
        .expect("cache shouldn't be poisoned")
        .size()
    );
    if atomic_break.load(Ordering::Acquire) {
      println!(" (found solution)");
      return (
        Some(k),
        duration_build_posets_total,
        duration_test_posets_total,
      );
    }
    println!();

    source = destination;
  }

  (
    None,
    duration_build_posets_total,
    duration_test_posets_total,
  )
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
      self.hash_match_lower_bound, self.hash_match_upper_bound, self.no_solution, self.brute_force
    )
  }
}

fn search_recursive(
  poset_cache: &Arc<RwLock<CacheSetSolvable>>,
  dyn_level: &Arc<AtomicU8>,
  poset: &Poset,
  cache_solvable: &mut CacheTreeSolvable,
  cache_not_solvable: &mut CacheTreeNotSolvable,
  remaining_comparisons: u8,
  statistics: &mut Statistics,
) -> SearchResult {
  if remaining_comparisons <= dyn_level.load(Ordering::Relaxed) + 1 {
    if poset_cache
      .read()
      .unwrap()
      .check(poset, remaining_comparisons)
    {
      return SearchResult::FoundSolution;
    } else if remaining_comparisons <= dyn_level.load(Ordering::Relaxed) {
      return SearchResult::NoSolution;
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
    (greater[b.0 as usize] + less[b.1 as usize]).cmp(&(greater[a.0 as usize] + less[a.1 as usize]))
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
      poset_cache,
      dyn_level,
      &poset.with_less_normalized(i, j),
      cache_solvable,
      cache_not_solvable,
      remaining_comparisons - 1,
      statistics,
    ) == SearchResult::FoundSolution
      && search_recursive(
        poset_cache,
        dyn_level,
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
  poset_cache: &Arc<RwLock<CacheSetSolvable>>,
  dyn_level: &Arc<AtomicU8>,
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
  let mut poset = Poset::new(n, i);
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
    poset_cache,
    dyn_level,
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
  poset_cache: &Arc<RwLock<CacheSetSolvable>>,
  dyn_level: &Arc<AtomicU8>,
  n: u8,
  i: u8,
  cache_solvable: &mut CacheTreeSolvable,
  cache_not_solvable: &mut CacheTreeNotSolvable,
  statistics: &mut Statistics,
  top_to_bottom: bool,
) -> (Option<u8>, Duration, Duration) {
  let mut search_duration = Duration::from_secs(0);
  let mut validate_duration = Duration::from_secs(0);

  let lower: u8 = lower_bound(n as i32, i as i32) as u8;
  let upper: u8 = upper_bound(n as i32, i as i32) as u8;

  if lower == upper {
    return (Some(lower), search_duration, validate_duration);
  }

  assert!(2 < n);

  if top_to_bottom {
    // searchRecursive from top
    for max_comparisons in (lower..upper).rev() {
      if start_search_now(
        poset_cache,
        dyn_level,
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
          poset_cache,
          dyn_level,
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
        poset_cache,
        dyn_level,
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
          poset_cache,
          dyn_level,
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
  let poset_cache = Arc::new(RwLock::new(CacheSetSolvable::new()));
  poset_cache
    .write()
    .expect("cache shouldn't be poisoned")
    .insert(&Poset::new(1, 0), 0);

  let mut cache_solvable = CacheTreeSolvable::new();
  let mut cache_not_solvable = CacheTreeNotSolvable::new();
  cache_solvable.insert(&Poset::new(1, 0), 0);

  for n in 2..MAX_N as u8 {
    for i in 0..((n + 1) / 2) {
      let mut statistics = Statistics::default();

      let poset_cache_global = poset_cache.clone();

      let interrupt = Arc::new(AtomicBool::new(false));
      let dyn_level = Arc::new(AtomicU8::new(0));
      let handle = {
        let interrupt_local = interrupt.clone();
        let dyn_level_local = dyn_level.clone();
        thread::spawn(move || {
          start_search_backward(
            &interrupt_local,
            &dyn_level_local,
            &poset_cache_global,
            n,
            i,
            n * n,
          );
        })
      };

      let (comparisons, duration_search, duration_validate) = start_search(
        &poset_cache.clone(),
        &dyn_level.clone(),
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
        if comparisons_value != KNOWN_MIN_VALUES[n as usize][i as usize] {
          eprintln!(
            "Error: got {}, but expected {}",
            comparisons_value, KNOWN_MIN_VALUES[n as usize][i as usize]
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
