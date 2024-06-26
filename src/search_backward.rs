use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use global_counter::primitive::exact::CounterUsize;
use hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::backward_cache::BackwardCache;
use crate::backwards_poset::BackwardsPoset;
use crate::constants::{KNOWN_VALUES, LOWER_BOUNDS, MAX_N, UPPER_BOUNDS};
use crate::utils::format_duration;

pub static COUTNER_USE_NOT_NAUTY: CounterUsize = CounterUsize::new(0);
pub static COUTNER_USE_NAUTY: CounterUsize = CounterUsize::new(0);

pub fn start_search_backward(
    interrupt: &Arc<AtomicBool>,
    backward_search_state_opt: Option<&Arc<RwLock<(HashMap<BackwardsPoset, u8>, i8)>>>,
    n0: u8,
    i0: u8,
    max_comparisons: usize,
) -> Option<(u8, BackwardCache)> {
    let mut table = [[false; MAX_N]; MAX_N];
    BackwardsPoset::rec_temp(&mut table, n0 as usize, i0 as usize);

    let mut cache = BackwardCache::new();
    let mut current_level = HashMap::new();
    for k in 0..=max_comparisons {
        let start = std::time::Instant::now();

        if 0 == k {
            current_level.insert(BackwardsPoset::new(1, 0), (0, 0));
        } else {
            let next_level: Arc<RwLock<HashMap<BackwardsPoset, (u8, u8)>>> =
                Arc::new(RwLock::new(HashMap::new()));
            current_level.par_iter().for_each(|(poset, _)| {
                if !interrupt.load(Ordering::Relaxed) {
                    let result = poset.enlarge_and_remove_less(
                        interrupt,
                        &cache,
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

            current_level = next_level
                .read()
                .expect("cache shouldn't be poisoned")
                .clone();
        };

        cache.add_layer(&current_level);
        if let Some(backward_search_state) = backward_search_state_opt {
            let mut write_lock = backward_search_state
                .write()
                .expect("cache shouldn't be poisoned");
            for (item, _) in &current_level {
                write_lock.0.insert(*item, k as u8);
            }
            write_lock.1 = k as i8;
        }

        println!(
            "# {}: {} in {:.3?}, total: {}",
            k,                   // comparisons done
            current_level.len(), // posets per level
            start.elapsed(),     // time per level
            cache.len(),         // total cached posets
        );

        if current_level.contains_key(&BackwardsPoset::new(n0, i0)) {
            return Some((k as u8, cache));
        } else if current_level.is_empty() || interrupt.load(Ordering::Relaxed) {
            return None;
        }
    }

    None
}

pub fn iterative_deepening_backward(
    interrupt: &Arc<AtomicBool>,
    n: u8,
    i: u8,
) -> (u8, BackwardCache) {
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

        if let Some((comparisons, cache)) = result {
            debug_assert!(comparisons as usize == bound);

            let ratio = 100.0 * COUTNER_USE_NAUTY.get() as f64
                / 1.max(COUTNER_USE_NAUTY.get() + COUTNER_USE_NOT_NAUTY.get()) as f64;
            println!();
            println!("Congratulations. A solution was found!\n\nn: {n}, i: {i}",);
            println!("Comparisons: {comparisons}");
            println!();
            println!("Cache entries: {}", cache.len());
            println!("Cache size: {:.3} Gigabyte", cache.memory_size());
            println!("Nauty Ratio: {ratio:.3?}%");
            println!();
            println!("{}", format_duration(start));

            assert_eq!(
                comparisons as usize, KNOWN_VALUES[n as usize][i as usize],
                "Error: got {}, but expected {}",
                comparisons, KNOWN_VALUES[n as usize][i as usize]
            );
            return (comparisons, cache);
        }
    }
    panic!(
        "Error: got 'nothing' but expected {}",
        KNOWN_VALUES[n as usize][i as usize]
    );
}