use std::fmt::{self, Display};
use std::time::Duration;
use std::time::Instant;
use std::vec;

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
  statistics: &mut Statistics,
) -> SearchResult {
  let mut result = SearchResult::NoSolution;

  if cache_not_solvable.check(poset, remaining_comparisons) {
    statistics.hash_match_lower_bound += 1;
    return SearchResult::NoSolution;
  } else if cache_solvable.check(poset, remaining_comparisons) {
    statistics.hash_match_upper_bound += 1;
    return SearchResult::FoundSolution;
  } else if 0 == remaining_comparisons {
    result = SearchResult::NoSolution;
    statistics.no_solution += 1;
  } else {
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
        &poset.with_less_normalized(i, j),
        cache_solvable,
        cache_not_solvable,
        remaining_comparisons - 1,
        statistics,
      ) == SearchResult::FoundSolution
        && search_recursive(
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
  }

  if result == SearchResult::NoSolution {
    cache_not_solvable.insert(poset, remaining_comparisons);
  } else if result == SearchResult::FoundSolution {
    cache_solvable.insert(poset, remaining_comparisons);
  }

  result
}

fn start_search_now(
  n: u8,
  i: u8,
  cache_solvable: &mut CacheSolvable,
  cache_not_solvable: &mut CacheNotSolvable,
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
  n: u8,
  i: u8,
  cache_solvable: &mut CacheSolvable,
  cache_not_solvable: &mut CacheNotSolvable,
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
        return (Some(max_comparisons + 1), search_duration, validate_duration);
      }
    }

    (Some(lower), search_duration, validate_duration)
  } else {
    // searchRecursive from bottom
    for i in lower..upper {
      if start_search_now(
        n,
        i,
        cache_solvable,
        cache_not_solvable,
        statistics,
        true,
        i,
        &mut search_duration,
      ) == SearchResult::FoundSolution
        || start_search_now(
          n,
          i,
          cache_solvable,
          cache_not_solvable,
          statistics,
          false,
          i,
          &mut validate_duration,
        ) == SearchResult::FoundSolution
      {
        return (Some(i), search_duration, validate_duration);
      }
    }

    (Some(upper), search_duration, validate_duration)
  }
}

pub fn main() {
  let mut cache_solvable = CacheSolvable::new();
  let mut cache_not_solvable = CacheNotSolvable::new();
  cache_solvable.insert(&Poset::new(1, 0), 0);

  for n in 2..MAX_N as u8 {
    for i in 0..(((n + 1) / 2) as u8) {
      let mut statistics = Statistics::default();
      let (comparisons, duration_search, duration_validate) = start_search(
        n,
        i,
        &mut cache_solvable,
        &mut cache_not_solvable,
        &mut statistics,
        true,
      );

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
