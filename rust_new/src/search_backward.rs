use std::collections::HashSet;
use std::time::Duration;

use super::cache_set::CacheSetSolvable;
use super::poset::Poset;
use super::util::{KNOWN_MIN_VALUES, MAX_N};

type CacheSolvable = CacheSetSolvable;

fn start_search_backward(
  poset_cache: &mut CacheSolvable,
  n: u8,
  i0: u8,
  max_comparisons: u8,
) -> (Option<u8>, Duration, Duration) {
  let mut duration_build_posets_total = Duration::from_secs(0);
  let mut duration_test_posets_total = Duration::from_secs(0);
  let mut source = HashSet::new();
  source.insert(Poset::new(1, 0));

  for k in 1..max_comparisons {
    let duration_test_posets;

    let start = std::time::Instant::now();
    let source_new = Poset::enlarge(&source, n, i0);
    let mid = std::time::Instant::now();
    let duration_build_posets = mid - start;
    duration_build_posets_total += duration_build_posets;

    let mut destination: HashSet<Poset> = HashSet::new();
    for item in &source_new {
      for i in 0..n {
        for j in 0..n {
          if item.is_less(i, j) {
            for predecessor in item.remove_less(i, j, |poset| poset_cache.check(poset, k - 1)) {
              if predecessor == Poset::new(n, i0) {
                duration_test_posets = mid.elapsed();
                duration_test_posets_total += duration_test_posets;
                println!(
                  "# {}: {} => {} in {:.3?} ~ {:.3?} | total cached: {} (found solution)",
                  k,
                  source.len(),
                  source_new.len(),
                  duration_build_posets,
                  duration_test_posets,
                  poset_cache.size()
                );
                return (
                  Some(k),
                  duration_build_posets_total,
                  duration_test_posets_total,
                );
              }

              let mut predecessor_normalized = predecessor.clone();
              predecessor_normalized.normalize();
              if poset_cache.check(&predecessor_normalized, k - 1) {
                continue;
              }

              destination.insert(predecessor_normalized.clone());
              poset_cache.insert(&predecessor_normalized, k);
            }
          }
        }
      }
    }
    duration_test_posets = mid.elapsed();
    duration_test_posets_total += duration_test_posets;

    println!(
      "# {}: {} => {} in {:.3?} ~ {:.3?} | total cached: {}",
      k,
      source.len(),
      source_new.len(),
      duration_build_posets,
      duration_test_posets,
      poset_cache.size()
    );

    source = destination;
  }

  (
    None,
    duration_build_posets_total,
    duration_test_posets_total,
  )
}

pub fn main() {
  if false {
    let mut poset_cache = CacheSolvable::new();
    poset_cache.insert(&Poset::new(1, 0), 0);

    let n = 9;
    let i = 4;

    let (comparisons, duration_generate_posets, duration_search) =
      start_search_backward(&mut poset_cache, n, i, n * n);

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
  } else {
    let mut poset_cache = CacheSolvable::new();
    poset_cache.insert(&Poset::new(1, 0), 0);

    for n in 2..MAX_N {
      for i in 0..((n + 1) / 2) {
        let (comparisons, duration_generate_posets, duration_search) =
          start_search_backward(&mut poset_cache, n as u8, i as u8, (n * n) as u8);

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
          if comparisons != KNOWN_MIN_VALUES[n][i] {
            eprintln!(
              "Error: got {}, but expected {}",
              comparisons, KNOWN_MIN_VALUES[n][i]
            );
            std::process::exit(0);
          }
        } else {
          eprintln!(
            "Error: got 'nothing' but expected {}",
            KNOWN_MIN_VALUES[n][i]
          );
          std::process::exit(0);
        }
      }
      println!();
    }
  }
}
