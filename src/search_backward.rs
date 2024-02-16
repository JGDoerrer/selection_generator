use core::panic;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::Instant;
use std::vec;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::cache_set::CacheSetSolvable;
use super::cache_tree::{CacheTreeNotSolvable, CacheTreeSolvable};
use super::poset::Poset;
use super::util::{lower_bound, upper_bound, KNOWN_MIN_VALUES, MAX_N};

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

#[allow(clippy::too_many_lines)]
fn start_search_backward(
  interrupt: &Arc<AtomicBool>,
  n: u8,
  i0: u8,
  max_comparisons: u8,
) -> (Option<u8>, Duration, Duration) {
  let poset_cache = Arc::new(RwLock::new(CacheSetSolvable::new()));
  poset_cache
    .write()
    .expect("cache shouldn't be poisoned")
    .insert(&Poset::new(1, 0), 0);

  let poset_cache2 = Arc::new(RwLock::new(CacheTreeSolvable::new()));

  let mut duration_build_posets_total = Duration::from_secs(0);
  let mut duration_test_posets_total = Duration::from_secs(0);
  let mut source = HashSet::new();
  source.insert(Poset::new(1, 0));

  let mut test123: [[HashSet<Poset>; 20]; 20] = Default::default();
  for n0 in 0..=n as usize {
    for i1 in 0..(n0 + 1) / 2 {
      test123[n0][i1] = crate::poset::iterate_all_closed_canonified(n0 as u8, i1 as u8);
    }
  }

  for k in 1..max_comparisons {
    let start = std::time::Instant::now();
    let mut source_new = Poset::enlarge_bruteForce(
      &test123[n as usize][i0 as usize],
      &interrupt, &source, n, i0,
    );
    // es ex. item `in` source_new : item `subset_of` X
    let mut filtered = HashSet::new();//Poset::filter(interrupt, &source_new.clone());
    for item in &source_new {
      filtered.insert(item);
    }
    dbg!(filtered.len());
    let mid = std::time::Instant::now();
    let duration_build_posets = mid - start;
    duration_build_posets_total += duration_build_posets;

    let atomic_break = Arc::new(AtomicBool::new(false));
    let interrupt_local = interrupt.clone();
    let results: Vec<HashSet<Poset>> = filtered
      .iter()
      .map(|item| {
        let mut destination: HashSet<Poset> = HashSet::new();
        for predecessor in item.remove_less_bruteForce(&test123[item.n() as usize][item.i() as usize],|poset| {
          poset_cache
            .read()
            .expect("cache shouldn't be poisoned")
            .check(poset, k - 1)
        }) {
          // ang. es ex item1, item2 :
          // - beide in source_new
          // - item1 subset of item2
          // Kann ich Aussage über `predecessor of item1` treffen, wenn
          //   `predessescor von item2` bekannt?

          // - führe Berechnung nur für item2 aus
          // (item, predecessor)

          // predecessor sollte in k Schritten lösbar sein
          // continue, wenn in WENIGER als k Schritten lösbar

          // Aussage:
          // item1 subset item2 => A subset B and every Element in a is an Element from B (subset)

          // if poset_cache2
          //   .read()
          //   .expect("cache shouldn't be poisoned")
          //   .check(&predecessor, 0)
          // {
          //   continue;
          // }
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

          if predecessor == Poset::new(n, i0) {
            atomic_break.store(true, Ordering::Relaxed);
          }
          if atomic_break.load(Ordering::Relaxed) || interrupt_local.load(Ordering::Relaxed) {
            break;
          }

          {
            let mut cache_solvable = CacheSolvable::new();
            let mut cache_not_solvable = CacheNotSolvable::new();
            cache_solvable.insert(&Poset::new(1, 0), 0);

            if SearchResult::NoSolution
              != search_recursive(
                &predecessor,
                &mut cache_solvable,
                &mut cache_not_solvable,
                k - 1,
              )
            {
              dbg!(predecessor, k - 1);
              panic!();
            }
            if SearchResult::FoundSolution
              != search_recursive(
                &predecessor,
                &mut cache_solvable,
                &mut cache_not_solvable,
                k,
              )
            {
              dbg!(predecessor, k);
              panic!();
            }
          }

          destination.insert(predecessor);
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

    for mut item in source_new {
      item.normalize();
      poset_cache2.write().expect("msg").insert(&item, 0);
    }

    source = destination;
  }

  (
    None,
    duration_build_posets_total,
    duration_test_posets_total,
  )
}

fn single(interrupt: &Arc<AtomicBool>, n: u8, i: u8) {
  let (comparisons, duration_generate_posets, duration_search) =
    start_search_backward(interrupt, n, i, n * n);

  if let Some(comparisons) = comparisons {
    println!(
      "time '{:.3?} + {:.3?} = {:.3?}': n = {}, i = {}, comparisons: {}",
      duration_generate_posets,
      duration_search,
      (duration_generate_posets + duration_search),
      n,
      i,
      comparisons
    );
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

    dbg!(a.remove_less(|poset| true));
    dbg!(b.remove_less(|poset| true));
  }
}
