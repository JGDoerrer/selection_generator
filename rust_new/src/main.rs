use std::time::Duration;
use std::{collections::HashSet, env};

mod cache_set;
mod cache_tree;
mod poset;
mod util;

use cache_set::CacheSetSingle;
use cache_tree::CacheTreeDual;
use poset::Poset;
use util::{KNOWN_MIN_VALUES, MAX_N};

enum Mode {
  Test,
  SingleRun,
  MultiRun,
}

fn start_search_backward(
  poset_cache: &mut CacheSetSingle<true>,
  n: u8,
  nth_smallest: u8,
  max_comparisons: u8,
) -> (Option<u8>, Duration, Duration) {
  let mut duration_build_posets_total = Duration::from_secs(0);
  let mut duration_test_posets_total = Duration::from_secs(0);
  let mut source = HashSet::new();
  source.insert(Poset::new(1, 0));

  for k in 1..max_comparisons {
    let duration_test_posets;

    let start = std::time::Instant::now();
    let source_new = Poset::enlarge(&source, n, nth_smallest);
    let mid = std::time::Instant::now();
    let duration_build_posets = mid - start;
    duration_build_posets_total += duration_build_posets;

    let mut destination: HashSet<Poset> = HashSet::new();
    for item in &source_new {
      for i in 0..n {
        for j in 0..n {
          if item.is_less(i, j) {
            for predecessor in item.remove_less(i, j, |poset| poset_cache.check(poset, k - 1)) {
              if predecessor == Poset::new(n, nth_smallest) {
                duration_test_posets = mid.elapsed();
                duration_test_posets_total += duration_test_posets;
                println!(
                  "# {}: {} => {} in {:.3}s ~ {:.3}s | total cached: {} (found solution)",
                  k,
                  source.len(),
                  source_new.len(),
                  duration_build_posets.as_secs_f64(),
                  duration_test_posets.as_secs_f64(),
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
      "# {}: {} => {} in {:.3}s ~ {:.3}s | total cached: {}",
      k,
      source.len(),
      source_new.len(),
      duration_build_posets.as_secs_f64(),
      duration_test_posets.as_secs_f64(),
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

fn main() {
  env::set_var("RUST_BACKTRACE", "1");

  // use indextree::Arena;

  // // Create a new arena
  // let arena = &mut Arena::new();

  // // Add some new nodes to the arena
  // let a = arena.new_node(1);
  // let b = arena.new_node(2);

  // // Append b to a
  // a.append(b, arena);
  // assert_eq!(b.ancestors(arena).into_iter().count(), 2);
  // exit(0);
  let mode = Mode::MultiRun;
  match mode {
    Mode::Test => {
      // CacheSetDual::test();
      // CacheSetSingle::test();

      CacheTreeDual::test();
      // CacheTreeSingle::test();
      // CacheTreeFixed::test();

      Poset::test();
    }
    Mode::SingleRun => {
      let mut poset_cache: CacheSetSingle<true> = CacheSetSingle::new();
      poset_cache.insert(&Poset::new(1, 0), 0);

      let n = 9;
      let nth_smallest = 4;

      let (comparisons, duration_generate_posets, duration_search) =
        start_search_backward(&mut poset_cache, n, nth_smallest, n * n);

      if let Some(comparisons) = comparisons {
        println!(
          "time '{:.3}s + {:.3}s = {:.3}s': n = {}, i = {}, comparisons: {}",
          duration_generate_posets.as_secs_f64(),
          duration_search.as_secs_f64(),
          (duration_generate_posets + duration_search).as_secs_f64(),
          n,
          nth_smallest,
          comparisons
        );

        if comparisons != KNOWN_MIN_VALUES[n as usize][nth_smallest as usize] {
          eprintln!(
            "Error: got {}, but expected {}",
            comparisons, KNOWN_MIN_VALUES[n as usize][nth_smallest as usize]
          );
          std::process::exit(0);
        }
      } else {
        eprintln!(
          "Error: got 'nothing' but expected {}",
          KNOWN_MIN_VALUES[n as usize][nth_smallest as usize]
        );
        std::process::exit(0);
      }
    }
    Mode::MultiRun => {
      const N_BOUND: usize = 1;

      let mut poset_cache: CacheSetSingle<true> = CacheSetSingle::new();
      poset_cache.insert(&Poset::new(1, 0), 0);

      for n in 2..MAX_N {
        for nth_smallest in 0..((n + 1) / 2) {
          let (comparisons, duration_generate_posets, duration_search) =
            start_search_backward(&mut poset_cache, n as u8, nth_smallest as u8, (n * n) as u8);

          if let Some(comparisons) = comparisons {
            if n >= N_BOUND {
              println!(
                "time '{:.3}s + {:.3}s = {:.3}s': n = {}, i = {}, comparisons: {}",
                duration_generate_posets.as_secs_f64(),
                duration_search.as_secs_f64(),
                (duration_generate_posets + duration_search).as_secs_f64(),
                n,
                nth_smallest,
                comparisons
              );
            }
            if comparisons != KNOWN_MIN_VALUES[n][nth_smallest] {
              eprintln!(
                "Error: got {}, but expected {}",
                comparisons, KNOWN_MIN_VALUES[n][nth_smallest]
              );
              std::process::exit(0);
            }
          } else {
            eprintln!(
              "Error: got 'nothing' but expected {}",
              KNOWN_MIN_VALUES[n][nth_smallest]
            );
            std::process::exit(0);
          }
        }
        if n >= N_BOUND {
          println!();
        }
      }
    }
  }

  dbg!("success");
}
