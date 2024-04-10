use std::fmt::{self, Display};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::vec;

use global_counter::primitive::exact::CounterUsize;
use hashbrown::HashSet;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::util::upper_bound;

use super::cache_tree::{CacheTreeNotSolvable, CacheTreeSolvable};
use super::poset::Poset;
use super::util::{KNOWN_MIN_VALUES, MAX_N};

type CacheSolvable = CacheTreeSolvable;
type CacheNotSolvable = CacheTreeNotSolvable;

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
  poset: &Poset,
  cache_solvable: &mut CacheSolvable,
  cache_not_solvable: &mut CacheNotSolvable,
  remaining_comparisons: u8,
) -> SearchResult {
  if cache_not_solvable.check(poset, remaining_comparisons) {
    return SearchResult::NoSolution;
  } else if cache_solvable.check(poset, remaining_comparisons) {
    return SearchResult::FoundSolution;
  } else if 0 == remaining_comparisons {
    return SearchResult::NoSolution;
  }

  let mut result = SearchResult::NoSolution;

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
      &poset.with_less_normalized(i, j),
      cache_solvable,
      cache_not_solvable,
      remaining_comparisons - 1,
    ) == SearchResult::FoundSolution
      && search_recursive(
        &poset.with_less_normalized(j, i),
        cache_solvable,
        cache_not_solvable,
        remaining_comparisons - 1,
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

const USE_CHECKS: bool = true;

pub static COUTNER_USE_NOT_NAUTY: CounterUsize = CounterUsize::new(0);
pub static COUTNER_USE_NAUTY: CounterUsize = CounterUsize::new(0);

#[allow(clippy::too_many_lines)]
fn start_search_backward(
  interrupt: &Arc<AtomicBool>,
  cache_solvable: &Arc<RwLock<CacheSolvable>>,
  cache_not_solvable: &Arc<RwLock<CacheNotSolvable>>,
  start_poset: Poset,
  n: u8,
  i0: u8,
  max_comparisons: u8,
) -> (Option<u8>, Duration) {
  let mut source = HashSet::new();
  source.insert(start_poset);

  let mut poset_cache = source.clone();

  let mut table = [[false; MAX_N]; MAX_N];
  Poset::rec_temp(&mut table, n as usize, i0 as usize);

  let mut check_time = Duration::from_millis(0);
  for k in 1..max_comparisons {
    let start = std::time::Instant::now();
    let results: Vec<_> = source
      .clone()
      .into_iter()
      .par_bridge()
      .map(|item| {
        if interrupt.load(Ordering::Relaxed) {
          HashSet::new()
        } else {
          item.enlarge_and_remove_less(interrupt, &poset_cache, &table, n, i0)
        }
      })
      .collect();

    let mut destination: HashSet<Poset> = HashSet::new();
    for item in results {
      for poset in item {
        if k as usize + poset.count_min_comparisons()
          <= upper_bound(n as i32, i0 as i32) as usize
      {
        destination.insert(poset);
      }
    }
  }
    poset_cache.extend(destination.clone());
    let search_duration = start.elapsed();

    if USE_CHECKS {
      let start_check = std::time::Instant::now();
      let _: Vec<()> = destination
        .iter()
        .map(|predecessor| {
          assert_eq!(
            SearchResult::NoSolution,
            search_recursive(
              predecessor,
              &mut cache_solvable.write().expect(""),
              &mut cache_not_solvable.write().expect(""),
              k - 1,
            ),
            "predecessor: {predecessor}, k - 1: {}",
            k - 1
          );
        })
        .collect();
      let _: Vec<()> = destination
        .iter()
        .map(|predecessor| {
          assert_eq!(
            SearchResult::FoundSolution,
            search_recursive(
              predecessor,
              &mut cache_solvable.write().expect(""),
              &mut cache_not_solvable.write().expect(""),
              k,
            ),
            "predecessor: {predecessor}, k: {k}",
          );
        })
        .collect();
      check_time += start_check.elapsed();
    }

    println!(
      "# {k}: {} => {} in {:.3?} | total cached: {}",
      source.len(),
      destination.len(),
      search_duration,
      poset_cache.len()
    );
    // let ratio = 100.0 * COUTNER_USE_NAUTY.get() as f64
    //   / (COUTNER_USE_NAUTY.get() as f64 + COUTNER_USE_NOT_NAUTY.get() as f64);
    // println!("nauty ratio: {ratio:.3?}%");

    if destination.contains(&Poset::new(n, i0)) {
      // let mut count = [0; MAX_N];
      // for item in &poset_cache {
      //   let mut pairs = 0;
      //   for i in 0..item.n() {
      //     for j in 0..item.n() {
      //       if item.is_less(i, j) {
      //         let mut is_pair = true;
      //         for k in 0..item.n() {
      //           if (k != j && item.is_less(i, k)) || (k != i && item.is_less(k, j)) {
      //             is_pair = false;
      //           }
      //         }
      //         if is_pair {
      //           pairs += 1;
      //         }
      //       }
      //     }
      //   }
      //   count[pairs] += 1;
      // }
      // let mut total = 0;
      // for i in 0..MAX_N {
      //   if 0 == count[i] {
      //     println!("remaining pairs: {}", poset_cache.len() - total);
      //     break;
      //   }
      //   println!("{i} pairs: {}", count[i]);
      //   total += count[i];
      // }

      return (Some(k), check_time);
    }

    source = destination;

    if interrupt.load(Ordering::Relaxed) {
      return (None, check_time);
    }
  }

  (None, check_time)
}

fn single(
  interrupt: &Arc<AtomicBool>,
  cache_solvable: &Arc<RwLock<CacheSolvable>>,
  cache_not_solvable: &Arc<RwLock<CacheNotSolvable>>,
  n: u8,
  i: u8,
) {
  COUTNER_USE_NOT_NAUTY.set(0);
  COUTNER_USE_NAUTY.set(0);
  let start = std::time::Instant::now();
  let (comparisons2, check_time) = start_search_backward(
    interrupt,
    cache_solvable,
    cache_not_solvable,
    Poset::new(1, 0),
    n,
    i,
    n * n,
  );
  let end = start.elapsed() - check_time;
  let ratio = 100.0 * COUTNER_USE_NAUTY.get() as f64
    / (COUTNER_USE_NAUTY.get() as f64 + COUTNER_USE_NOT_NAUTY.get() as f64);

    if let Some(comparisons) = comparisons2 {
    if USE_CHECKS {
      println!("time '{end:.3?}' (check-time: {check_time:.3?}): n = {n}, i = {i}, comparisons: {comparisons}, nauty ratio: {ratio:.3?}%");
    } else {
      println!("time '{end:.3?}': n = {n}, i = {i}, comparisons: {comparisons}, nauty ratio: {ratio:.3?}%");
    }
    assert_eq!(
      comparisons, KNOWN_MIN_VALUES[n as usize][i as usize],
      "Error: got {comparisons}, but expected {}",
      KNOWN_MIN_VALUES[n as usize][i as usize]
    );
  } else {
    panic!(
      "Error: got 'nothing' but expected {}",
      KNOWN_MIN_VALUES[n as usize][i as usize]
    );
  }
}

pub fn main() {
  let interrupt = Arc::new(AtomicBool::new(false));
  let cache_solvable = Arc::new(RwLock::new(CacheSolvable::new()));
  let cache_not_solvable = Arc::new(RwLock::new(CacheNotSolvable::new()));
  cache_solvable
    .write()
    .expect("")
    .insert(&Poset::new(1, 0), 0);

  if false {
    single(&interrupt, &cache_solvable, &cache_not_solvable, 10, 4);
  } else if true {
    for n in 2..MAX_N as u8 {
      for i in 0..((n + 1) / 2) {
        single(&interrupt, &cache_solvable, &cache_not_solvable, n, i);
      }
      println!();
    }
  }
}
