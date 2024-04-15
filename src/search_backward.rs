use std::collections::{HashMap, HashSet};
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use global_counter::primitive::exact::CounterUsize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::backwards_poset::BackwardsPoset;
use crate::constants::{KNOWN_VALUES, LOWER_BOUNDS, MAX_N, UPPER_BOUNDS};
use crate::utils::format_duration;

pub static COUTNER_USE_NOT_NAUTY: CounterUsize = CounterUsize::new(0);
pub static COUTNER_USE_NAUTY: CounterUsize = CounterUsize::new(0);

const USE_LEGACY_OUTPUT: bool = false;

pub fn cache_size_as_gigabyte(elements: usize) -> f64 {
    (size_of::<HashMap<BackwardsPoset, u8>>() + elements * size_of::<(BackwardsPoset, u8)>()) as f64
        / 1024_f64.powf(3.0)
}

fn start_search_backward(
    interrupt: &Arc<AtomicBool>,
    start_poset: BackwardsPoset,
    n: u8,
    i0: u8,
    max_comparisons: usize,
) -> Option<(u8, usize)> {
    let mut poset_cache = HashMap::new();
    poset_cache.insert(start_poset.clone(), 0);

    let mut source = HashSet::new();
    source.insert(start_poset);

    println!(
        "# {}: {} in {:.3?}, total: {}",
        0,                                   // comparisons done
        1,                                   // posets per level
        std::time::Instant::now().elapsed(), // time per level
        poset_cache.len(),                   // total cached posets
    );

    if source.contains(&BackwardsPoset::new(n, i0)) {
        return Some((0, poset_cache.len()));
    }

    let mut table = [[false; MAX_N]; MAX_N];
    BackwardsPoset::rec_temp(&mut table, n as usize, i0 as usize);

    for k in 1..=max_comparisons {
        let start = std::time::Instant::now();
        let results: Vec<_> = source
            .par_iter()
            .map(|item| {
                if interrupt.load(Ordering::Relaxed) {
                    HashSet::new()
                } else {
                    item.enlarge_and_remove_less(
                        interrupt,
                        &poset_cache,
                        &table,
                        n,
                        i0,
                        max_comparisons - k,
                    )
                }
            })
            .collect();

        let mut destination: HashSet<BackwardsPoset> = HashSet::new();
        for item in results {
            for poset in item {
                assert!(poset.count_min_comparisons() <= max_comparisons - k);
                destination.insert(poset);
            }
        }
        for item in &destination {
            poset_cache.insert(item.clone(), k as u8);
        }
        source = destination;

        println!(
            "# {}: {} in {:.3?}, total: {}",
            k,
            source.len(),
            start.elapsed(),
            poset_cache.len(),
        );

        if source.contains(&BackwardsPoset::new(n, i0)) {
            return Some((k as u8, poset_cache.len()));
        } else if source.is_empty() || interrupt.load(Ordering::Relaxed) {
            return None;
        }
    }

    None
}

pub fn single(interrupt: &Arc<AtomicBool>, n: u8, i: u8) -> u8 {
    COUTNER_USE_NOT_NAUTY.set(0);
    COUTNER_USE_NAUTY.set(0);

    let start = std::time::Instant::now();
    let lower = LOWER_BOUNDS[n as usize][i as usize];
    let upper = UPPER_BOUNDS[n as usize][i as usize];
    for bound in lower..=upper {
        let result = start_search_backward(interrupt, BackwardsPoset::new(1, 0), n, i, bound);

        if let Some((comparisons, cache_entries)) = result {
            assert!(comparisons as usize == bound);

            let end = start.elapsed();
            let ratio = 100.0 * COUTNER_USE_NAUTY.get() as f64
                / if 0 == COUTNER_USE_NAUTY.get() + COUTNER_USE_NOT_NAUTY.get() {
                    1
                } else {
                    COUTNER_USE_NAUTY.get() + COUTNER_USE_NOT_NAUTY.get()
                } as f64;
            if USE_LEGACY_OUTPUT {
                println!("time '{end:.3?}': n = {n}, i = {i}, comparisons: {comparisons}, nauty ratio: {ratio:.3?}%");
            } else {
                println!();
                println!("Congratulations. A solution was found!\n\nn: {n}, i: {i}",);
                println!("Comparisons: {comparisons}");
                println!();
                println!("Cache entries: {cache_entries}");
                println!(
                    "Cache size: {:.3} Gigabyte",
                    cache_size_as_gigabyte(cache_entries)
                );
                println!("Nauty Ratio: {ratio:.3?}%");
                println!();
                println!("{}", format_duration(start));
                println!("==============================================================");
            }
            if comparisons as usize != KNOWN_VALUES[n as usize][i as usize] {
                eprintln!(
                    "Error: got {}, but expected {}",
                    comparisons, KNOWN_VALUES[n as usize][i as usize]
                );
                std::process::exit(0);
            }
            return comparisons;
        }
    }
    eprintln!(
        "Error: got 'nothing' but expected {}",
        KNOWN_VALUES[n as usize][i as usize]
    );
    std::process::exit(0);
}
