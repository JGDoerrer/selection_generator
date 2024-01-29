use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::cache_set::CacheSetSolvable;
use super::poset::Poset;
use super::util::{KNOWN_MIN_VALUES, MAX_N};

type CacheSolvable = CacheSetSolvable;

fn start_search_backward(
  poset_cache: Arc<RwLock<CacheSolvable>>,
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

    let atomic_break = Arc::new(AtomicBool::new(false));

    let n_source_new = source_new.len();

    let results: Vec<HashSet<Poset>> = source_new.par_iter().map(|item| {
      let mut destination: HashSet<Poset> = HashSet::new();
      for i in 0..n {
        for j in 0..n {
          if item.is_less(i, j) {
            for mut predecessor in item.remove_less(i, j, |poset| {
              poset_cache.read().expect("cache shouldn't be poisoned").check(poset, k - 1)
            }) {
              if predecessor == Poset::new(n, i0) {
                atomic_break.store(true, Ordering::Relaxed);
              }
              predecessor.normalize();
              if poset_cache.read().expect("cache shouldn't be poisoned").check(&predecessor, k - 1) {
                continue;
              }
              poset_cache.write().expect("cache shouldn't be poisoned").insert(&predecessor, k);
              destination.insert(predecessor);

              if atomic_break.load(Ordering::Relaxed) {
                return destination;
              }
            }
          }
        }
      }
      destination
    }).collect();

    let mut destination: HashSet<Poset> = HashSet::new();
    for item in results {
      for poset in item {
        destination.insert(poset);
      }
    }

    if atomic_break.load(Ordering::Acquire) {
      duration_test_posets = mid.elapsed();
      duration_test_posets_total += duration_test_posets;
      println!(
        "# {}: {} => {} in {:.3?} ~ {:.3?} | total cached: {} (found solution)",
        k,
        source.len(),
        n_source_new,
        duration_build_posets,
        duration_test_posets,
        poset_cache.read().expect("cache shouldn't be poisoned").size()
      );
      return (
        Some(k),
        duration_build_posets_total,
        duration_test_posets_total,
      );
    }

    duration_test_posets = mid.elapsed();
    duration_test_posets_total += duration_test_posets;

    println!(
      "# {}: {} => {} in {:.3?} ~ {:.3?} | total cached: {}",
      k,
      source.len(),
      n_source_new,
      duration_build_posets,
      duration_test_posets,
      poset_cache.read().expect("cache shouldn't be poisoned").size()
    );

    source = destination;
  }

  (
    None,
    duration_build_posets_total,
    duration_test_posets_total,
  )
}

fn single(poset_cache: Arc<RwLock<CacheSolvable>>, n: usize, i: usize) {
  let (comparisons, duration_generate_posets, duration_search) =
    start_search_backward(poset_cache, n as u8, i as u8, (n * n) as u8);

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
  let poset_cache = Arc::new(RwLock::new(CacheSolvable::new()));
  poset_cache.write().expect("cache shouldn't be poisoned").insert(&Poset::new(1, 0), 0);

  if false {
    single(poset_cache, 9, 4);
  } else {
    for n in 2..MAX_N {
      for i in 0..((n + 1) / 2) {
        single(poset_cache.clone(), n, i);
      }
      println!();
    }
  }
}
