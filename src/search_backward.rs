use std::collections::HashSet;
use std::fmt::{self, Display};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::vec;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

fn start_search_backward(
  interrupt: &Arc<AtomicBool>,
  start_poset: Poset,
  n: u8,
  i0: u8,
  max_comparisons: u8,
) -> (Option<u8>, Duration) {
  let cache_solvable = Arc::new(RwLock::new(CacheSolvable::new()));
  let cache_not_solvable = Arc::new(RwLock::new(CacheNotSolvable::new()));
  cache_solvable
    .write()
    .expect("")
    .insert(&Poset::new(1, 0), 0);

  let mut poset_cache = HashSet::new();
  poset_cache.insert(start_poset.clone());

  let mut source = HashSet::new();
  source.insert(start_poset);

  let mut table = [[false; MAX_N]; MAX_N];
  Poset::rec_temp(&mut table, n as usize, i0 as usize);

  let mut check_time = Duration::from_millis(0);
  for k in 1..max_comparisons {
    let start = std::time::Instant::now();
    let results: Vec<_> = source
      .par_iter()
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
      destination.extend(item);
    }
    poset_cache.extend(destination.clone());
    let search_duration = start.elapsed();

    if USE_CHECKS {
      let start_check = std::time::Instant::now();
      let _: Vec<()> = destination
        .iter()
        .map(|predecessor| {
          if SearchResult::NoSolution
            != search_recursive(
              predecessor,
              &mut cache_solvable.write().expect(""),
              &mut cache_not_solvable.write().expect(""),
              k - 1,
            )
          {
            dbg!(predecessor, k - 1);
            panic!();
          }
          if SearchResult::FoundSolution
            != search_recursive(
              predecessor,
              &mut cache_solvable.write().expect(""),
              &mut cache_not_solvable.write().expect(""),
              k,
            )
          {
            dbg!(predecessor, k);
            panic!();
          }
        })
        .collect();
      check_time += start_check.elapsed();
    }
    // dbg!(&source, &destination);

    println!(
      "# {k}: {} => {} in {:.3?} | total cached: {}",
      source.len(),
      destination.len(),
      search_duration,
      poset_cache.len()
    );

    if destination.contains(&Poset::new(n, i0)) {
      return (Some(k), check_time);
    }

    source = destination;

    if interrupt.load(Ordering::Relaxed) {
      return (None, check_time);
    }
  }

  (None, check_time)
}

fn single(interrupt: &Arc<AtomicBool>, n: u8, i: u8) {
  let start = std::time::Instant::now();
  let (comparisons, check_time) = start_search_backward(interrupt, Poset::new(1, 0), n, i, n * n);
  let end = start.elapsed() - check_time;

  if let Some(comparisons) = comparisons {
    if USE_CHECKS {
      println!("time '{end:.3?}' (check-time: {check_time:.3?}): n = {n}, i = {i}, comparisons: {comparisons}");
    } else {
      println!("time '{end:.3?}': n = {n}, i = {i}, comparisons: {comparisons}");
    }
    if comparisons != KNOWN_MIN_VALUES[n as usize][i as usize] {
      eprintln!(
        "Error: got {}, but expected {}",
        comparisons, KNOWN_MIN_VALUES[n as usize][i as usize]
      );
      std::process::exit(0);
    }
  } else {
    eprintln!(
      "Error: got 'nothing' but expected {}",
      KNOWN_MIN_VALUES[n as usize][i as usize]
    );
    std::process::exit(0);
  }
}

pub fn main() {
  let interrupt = Arc::new(AtomicBool::new(false));

  if false {
    single(&interrupt, 9, 4);
  } else if true {
    for n in 2..MAX_N as u8 {
      for i in 0..((n + 1) / 2) {
        single(&interrupt, n, i);
      }
      println!();
    }
  } else {
    let mut a = Poset::new(6, 2);
    a.add_less(3, 0);
    a.add_less(4, 1);
    a.add_less(5, 0);
    a.add_less(5, 1);
    a.add_less(5, 2);

    let mut b = a.clone();
    b.add_less(4, 2);

    dbg!(&a, &b);
    assert!(a.subset_of(&b));

    // dbg!(a.remove_less(|_| true, false));
    // dbg!(b.remove_less(|_| true, false));
  }
}
