use std::mem::size_of;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use global_counter::primitive::exact::CounterUsize;
use hashbrown::{HashMap, HashSet};
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

pub fn start_search_backward(
    interrupt: &Arc<AtomicBool>,
    backward_search_state_opt: Option<&Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>>,
    n0: u8,
    i0: u8,
    max_comparisons: usize,
) -> Option<(u8, usize)> {
    let start = std::time::Instant::now();
    let mut current_level: HashSet<_> = [BackwardsPoset::new(1, 0)].into();

    let mut poset_cache = HashMap::new();
    for item in &current_level {
        poset_cache.insert(*item, 0);
    }
    if let Some(backward_search_state) = backward_search_state_opt {
        let mut write_lock = backward_search_state
            .write()
            .expect("cache shouldn't be poisoned");
        for item in &current_level {
            write_lock.0.insert(*item, 0);
        }
        write_lock.1 = 0;
    }

    println!(
        "# {}: {} in {:.3?}, total: {}",
        0,                   // comparisons done
        current_level.len(), // posets per level
        start.elapsed(),     // time per level
        poset_cache.len(),   // total cached posets
    );

    if current_level.contains(&BackwardsPoset::new(n0, i0)) {
        return Some((0, poset_cache.len()));
    }

    let mut table = [[false; MAX_N]; MAX_N];
    BackwardsPoset::rec_temp(&mut table, n0 as usize, i0 as usize);

    for k in 1..=max_comparisons {
        let start = std::time::Instant::now();

        let next_level: Arc<RwLock<HashSet<BackwardsPoset>>> =
            Arc::new(RwLock::new(HashSet::new()));
        current_level.par_iter().for_each(|poset| {
            if !interrupt.load(Ordering::Relaxed) {
                let result = poset.enlarge_and_remove_less(
                    interrupt,
                    &poset_cache,
                    &table,
                    n0,
                    i0,
                    max_comparisons - k,
                );
                next_level
                    .write()
                    .expect("cache shouldn't be poisoned")
                    .extend(result);
            }
        });

        current_level.clear();
        for item in next_level
            .read()
            .expect("cache shouldn't be poisoned")
            .iter()
        {
            current_level.insert(*item);
        }

        for item in &current_level {
            poset_cache.insert(*item, k as u8);
        }
        if let Some(backward_search_state) = backward_search_state_opt {
            let mut write_lock = backward_search_state
                .write()
                .expect("cache shouldn't be poisoned");
            for item in &current_level {
                write_lock.0.insert(*item, k as u8);
            }
            write_lock.1 = k as i8;
        }

        println!(
            "# {}: {} in {:.3?}, total: {}",
            k,
            current_level.len(),
            start.elapsed(),
            poset_cache.len(),
        );

        if current_level.contains(&BackwardsPoset::new(n0, i0)) {
            return Some((k as u8, poset_cache.len()));
        } else if current_level.is_empty() || interrupt.load(Ordering::Relaxed) {
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
        println!("n: {n}, i: {i} needs at least {bound} comparisons");
        let this_level = std::time::Instant::now();
        let result = start_search_backward(interrupt, None, n, i, bound);
        println!("{}", format_duration(this_level));

        if let Some((comparisons, cache_entries)) = result {
            assert!(comparisons as usize == bound);

            let ratio = 100.0 * COUTNER_USE_NAUTY.get() as f64
                / if 0 == COUTNER_USE_NAUTY.get() + COUTNER_USE_NOT_NAUTY.get() {
                    1
                } else {
                    COUTNER_USE_NAUTY.get() + COUTNER_USE_NOT_NAUTY.get()
                } as f64;
            if USE_LEGACY_OUTPUT {
                let end = start.elapsed();
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
