use std::collections::VecDeque;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use global_counter::primitive::exact::CounterUsize;
use hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::backward_cache::BackwardCache;
use crate::backwards_poset::BackwardsPoset;
use crate::constants::{KNOWN_VALUES, LOWER_BOUNDS, MAX_N, UPPER_BOUNDS};
use crate::utils::{format_duration, format_memory, get_memory};

pub static COUTNER_USE_NOT_NAUTY: CounterUsize = CounterUsize::new(0);
pub static COUTNER_USE_NAUTY: CounterUsize = CounterUsize::new(0);

pub fn start_search_backward(
    interrupt: &Arc<AtomicBool>,
    n: u8,
    i: u8,
    max_comparisons: usize,
) -> Option<(u8, BackwardCache)> {
    let mut table = [[false; MAX_N]; MAX_N];
    BackwardsPoset::calculate_poset_table(&mut table, n as usize, i as usize);

    let mut cache = BackwardCache::new();
    let mut current_level = vec![];
    for k in 0..=max_comparisons {
        let start = std::time::Instant::now();

        current_level = if 0 == k {
            vec![(BackwardsPoset::new(1, 0), (0, 0))]
        } else {
            const threshold: usize = 1_000_000;

            let next_level: Arc<RwLock<HashMap<BackwardsPoset, (u8, u8)>>> =
                Arc::new(RwLock::new(HashMap::new()));
            let result_buffer: Arc<RwLock<(usize, VecDeque<HashMap<BackwardsPoset, (u8, u8)>>)>> =
                Arc::new(RwLock::new((0, VecDeque::new())));

            current_level
                .par_iter()
                .for_each(|(poset, _): &(BackwardsPoset, (u8, u8))| {
                    if !interrupt.load(Ordering::Relaxed) {
                        let result =
                            poset.calculate_predecessors(&cache, &table, n, i, max_comparisons - k);

                        let mut writer =
                            result_buffer.write().expect("cache shouldn't be poisoned");
                        writer.0 += result.len();
                        writer.1.push_back(result);

                        if threshold < writer.0 {
                            writer.0 = 0;
                            let mut new_deque = VecDeque::new();
                            mem::swap(&mut writer.1, &mut new_deque);
                            mem::drop(writer);

                            let mut current_level_writer =
                                next_level.write().expect("cache shouldn't be poisoned");
                            for hash_map in new_deque {
                                current_level_writer.extend(hash_map);
                            }
                        }
                    }
                });

            let mut writer = result_buffer.write().expect("cache shouldn't be poisoned");
            let mut new_deque = VecDeque::new();
            mem::swap(&mut writer.1, &mut new_deque);

            let mut current_level_writer = next_level.write().expect("cache shouldn't be poisoned");
            let mut next_level_raw = HashMap::new();
            mem::swap(&mut *current_level_writer, &mut next_level_raw);

            for hash_map in new_deque {
                next_level_raw.extend(hash_map);
            }

            next_level_raw.into_iter().collect()
        };
        current_level.shrink_to_fit();

        let add_level = std::time::Instant::now();
        cache.add_layer(&current_level, k as u8);
        let elapsed = add_level.elapsed();

        println!(
            "# {}: {} in {:.3?}, total: {}, add level: {:.3?}, real memory: {}",
            k,                   // comparisons done
            current_level.len(), // posets per level
            start.elapsed(),     // time per level
            cache.len(),         // total cached posets
            elapsed,
            get_memory()
        );

        if cache.contains(&BackwardsPoset::new(n, i)) {
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
        let result = start_search_backward(interrupt, n, i, bound);
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
            println!(
                "Cache size: {} Gigabyte",
                format_memory(cache.memory_size() as u64),
            );
            println!("Nauty Ratio: {ratio:.3?}%");
            println!();
            println!("{}", format_duration(start));

            if (n as usize) < KNOWN_VALUES.len() && (i as usize) < KNOWN_VALUES[n as usize].len() {
                assert_eq!(
                    comparisons as usize, KNOWN_VALUES[n as usize][i as usize],
                    "Error: got {}, but expected {}",
                    comparisons, KNOWN_VALUES[n as usize][i as usize]
                );
            }

            return (comparisons, cache);
        }
    }
    panic!(
        "Error: got 'nothing' but expected {}",
        KNOWN_VALUES[n as usize][i as usize]
    );
}
