use std::fmt::{self, Display};
use std::time::Duration;
use std::time::Instant;
use std::vec;

use super::cache_tree::CacheTreeDual;
use super::poset::Poset;
use super::util::{lower_bound, upper_bound, KNOWN_MIN_VALUES, MAX_N};

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
  cache: &mut CacheTreeDual,
  remaining_comparisons: u8,
  statistics: &mut Statistics,
) -> SearchResult {
  let mut result = SearchResult::NoSolution;

  if cache.check_not_solvable(poset, remaining_comparisons) {
    statistics.hash_match_lower_bound += 1;
    return SearchResult::NoSolution;
  } else if cache.check_solvable(poset, remaining_comparisons) {
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
        cache,
        remaining_comparisons - 1,
        statistics,
      ) == SearchResult::FoundSolution
        && search_recursive(
          &poset.with_less_normalized(j, i),
          cache,
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
    cache.insert_not_solvable(poset, remaining_comparisons);
  } else if result == SearchResult::FoundSolution {
    cache.insert_solvable(poset, remaining_comparisons);
  }

  result
}

fn start_search_now(
  n: u8,
  nth_smallest: u8,
  cache: &mut CacheTreeDual,
  statistics: &mut Statistics,
  pair_wise_optimization: bool,
  max_comparisons: u8,
  time: &mut Duration,
) -> SearchResult {
  let start = Instant::now();
  let mut poset = Poset::new(n, nth_smallest);
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
    cache,
    max_comparisons - comparisons_done,
    statistics,
  );

  if result == SearchResult::FoundSolution {
    print!(" -> FoundSolution");
  } else if result == SearchResult::NoSolution {
    print!(" -> NoSolution");
  }

  let end = Instant::now();
  print!(" in {:?}", end - start);
  *time += end - start;

  result
}

fn start_search(
  n: u8,
  nth_smallest: u8,
  cache: &mut CacheTreeDual,
  statistics: &mut Statistics,
  top_to_bottom: bool,
) -> (Option<u8>, Duration, Duration) {
  let mut search_duration = Duration::from_secs(0);
  let mut validate_duration = Duration::from_secs(0);

  let lower: u8 = lower_bound(n as i32, nth_smallest as i32) as u8;
  let upper: u8 = upper_bound(n as i32, nth_smallest as i32) as u8;

  if lower == upper {
    return (Some(lower), search_duration, validate_duration);
  }

  assert!(2 < n);

  if top_to_bottom {
    // searchRecursive from top
    for i in (lower..upper).rev() {
      if start_search_now(
        n,
        nth_smallest,
        cache,
        statistics,
        true,
        i,
        &mut search_duration,
      ) == SearchResult::NoSolution
        && start_search_now(
          n,
          nth_smallest,
          cache,
          statistics,
          false,
          i,
          &mut validate_duration,
        ) == SearchResult::NoSolution
      {
        return (Some(i + 1), search_duration, validate_duration);
      }
    }

    (Some(lower), search_duration, validate_duration)
  } else {
    // searchRecursive from bottom
    for i in lower..upper {
      if start_search_now(
        n,
        nth_smallest,
        cache,
        statistics,
        true,
        i,
        &mut search_duration,
      ) == SearchResult::FoundSolution
        || start_search_now(
          n,
          nth_smallest,
          cache,
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
  const N_BOUND: usize = 1;

  let mut cache = CacheTreeDual::new();
  cache.insert_solvable(&Poset::new(1, 0), 0);

  for n in 2..MAX_N {
    for nth_smallest in 0..((n + 1) / 2) {
      let mut statistics = Statistics::default();
      let (comparisons, duration_search, duration_validate) = start_search(
        n as u8,
        nth_smallest as u8,
        &mut cache,
        &mut statistics,
        false,
      );

      if let Some(comparisons_value) = comparisons {
        if n >= N_BOUND {
          println!(
            "\rtime '{:?}' + '{:?}' = '{:?}': n = {}, i = {}, {}, cache = {}, comparisons: {}",
            duration_search,
            duration_validate,
            duration_search + duration_validate,
            n,
            nth_smallest,
            statistics,
            cache.size(),
            comparisons_value
          );
        }
        if comparisons_value != KNOWN_MIN_VALUES[n][nth_smallest] {
          eprintln!(
            "Error: got {}, but expected {}",
            comparisons_value, KNOWN_MIN_VALUES[n][nth_smallest]
          );
          std::process::exit(0);
        }
      } else {
        eprintln!("Error, maxComparisons exceeded");
        std::process::exit(0);
      }
      if n >= N_BOUND {
        println!();
      }
    }
  }
}
