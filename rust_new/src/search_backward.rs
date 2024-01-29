use std::collections::HashSet;
use std::mem::swap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use super::cache_set::CacheSetSolvable;
use super::poset::Poset;
use super::util::{KNOWN_MIN_VALUES, MAX_N};

type CacheSolvable = CacheSetSolvable;

fn start_search_backward(
  threadpool: &mut ThreadPool,
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

    let atomic_break_global = Arc::new(Mutex::new(false));
    let mut poset_cache2 = CacheSolvable::new();
    swap(poset_cache, &mut poset_cache2); // ist das effizient?
    let poset_cache_global = Arc::new(Mutex::new(poset_cache2));

    let (tx, rx) = channel();
    let n_source_new = source_new.len();
    for item in &source_new {
      let tx = tx.clone();
      let item2 = item.clone();
      let atomic_break_local = atomic_break_global.clone();
      let poset_cache_local = poset_cache_global.clone();
      threadpool.execute(move || {
        let mut destination: HashSet<Poset> = HashSet::new();
        for i in 0..n {
          for j in 0..n {
            if item2.is_less(i, j) {
              for predecessor in item2.remove_less(i, j, |poset| {
                (*poset_cache_local.lock().unwrap()).check(poset, k - 1)
              }) {
                if predecessor == Poset::new(n, i0) {
                  let mut atomic_break = atomic_break_local.lock().unwrap();
                  *atomic_break = true;
                }

                let mut predecessor_normalized = predecessor.clone();
                predecessor_normalized.normalize();
                if (*poset_cache_local.lock().unwrap()).check(&predecessor_normalized, k - 1) {
                  continue;
                }

                destination.insert(predecessor_normalized.clone());
                (*poset_cache_local.lock().unwrap()).insert(&predecessor_normalized, k);

                let atomic_break = atomic_break_local.lock().unwrap();
                if *atomic_break {
                  tx.send(destination)
                    .expect("channel will be there waiting for the pool");
                  return;
                }
              }
            }
          }
        }
        tx.send(destination)
          .expect("channel will be there waiting for the pool");
      });
    }

    let mut destination: HashSet<Poset> = HashSet::new();
    for item in rx.iter().take(n_source_new) {
      for poset in item {
        destination.insert(poset);
      }
    }
    swap(poset_cache, &mut *poset_cache_global.lock().unwrap());

    if destination.contains(&Poset::new(n, i0)) {
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

fn single(threadpool: &mut ThreadPool, poset_cache: &mut CacheSolvable, n: usize, i: usize) {
  let (comparisons, duration_generate_posets, duration_search) =
    start_search_backward(threadpool, poset_cache, n as u8, i as u8, (n * n) as u8);

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
  let mut poset_cache = CacheSolvable::new();
  poset_cache.insert(&Poset::new(1, 0), 0);

  let mut pool = ThreadPool::new(20);

  if false {
    single(&mut pool, &mut poset_cache, 9, 4);
  } else {
    for n in 2..MAX_N {
      for i in 0..((n + 1) / 2) {
        single(&mut pool, &mut poset_cache, n, i);
      }
      println!();
    }
  }
}
