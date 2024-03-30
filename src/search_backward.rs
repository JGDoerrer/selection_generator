use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::backwards_poset::BackwardsPoset;
use crate::constants::{KNOWN_VALUES, MAX_N};

fn start_search_backward(
    interrupt: &Arc<AtomicBool>,
    start_poset: BackwardsPoset,
    n: u8,
    i0: u8,
    max_comparisons: u8,
) -> Option<u8> {
    let mut poset_cache = HashMap::new();
    poset_cache.insert(start_poset.clone(), 0);

    let mut source = HashSet::new();
    source.insert(start_poset);

    let mut table = [[false; MAX_N]; MAX_N];
    BackwardsPoset::rec_temp(&mut table, n as usize, i0 as usize);

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

        let mut destination: HashSet<BackwardsPoset> = HashSet::new();
        for item in results {
            destination.extend(item);
        }
        for item in &destination {
            poset_cache.insert(item.clone(), k);
        }

        println!(
            "# {k}: {} => {} in {:.3?} | total cached: {}",
            source.len(),
            destination.len(),
            start.elapsed(),
            poset_cache.len()
        );

        if destination.contains(&BackwardsPoset::new(n, i0)) {
            return Some(k);
        }

        source = destination;

        if interrupt.load(Ordering::Relaxed) {
            return None;
        }
    }

    None
}

pub fn single(interrupt: &Arc<AtomicBool>, n: u8, i: u8) {
    let start = std::time::Instant::now();
    let comparisons = start_search_backward(interrupt, BackwardsPoset::new(1, 0), n, i, n * n);
    let end = start.elapsed();

    if let Some(comparisons) = comparisons {
        println!("time '{end:.3?}': n = {n}, i = {i}, comparisons: {comparisons}");
        if comparisons as usize != KNOWN_VALUES[n as usize][i as usize] {
            eprintln!(
                "Error: got {}, but expected {}",
                comparisons, KNOWN_VALUES[n as usize][i as usize]
            );
            std::process::exit(0);
        }
    } else {
        eprintln!(
            "Error: got 'nothing' but expected {}",
            KNOWN_VALUES[n as usize][i as usize]
        );
        std::process::exit(0);
    }
}
