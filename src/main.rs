use std::{collections::HashMap, time::Instant};

use poset::{Poset, MAX_N};

mod poset;

fn main() {
    let mut cache = HashMap::new();
    for n in 0..=MAX_N as u8 {
        for i in 0..(n + 1) / 2 {
            let start = Instant::now();

            if let Some(comparisions) = search(Poset::new(n, i), u8::MAX, &mut cache) {
                println!("n = {n}, i = {i}, comparisions = {comparisions}");
                println!("cache_entries = {}", cache.len());
                println!("time = {}s", (Instant::now() - start).as_secs_f64());
            } else {
            }
        }
    }
}

#[derive(Debug, Clone)]
enum DecisionTree {
    Compare(u8, u8, Box<DecisionTree>, Box<DecisionTree>),
    Return(u8),
}

fn search(poset: Poset, mut max_comps: u8, cache: &mut HashMap<Poset, Option<u8>>) -> Option<u8> {
    if let Some(result) = cache.get(&poset) {
        return *result;
    }

    if poset.n() == 1 {
        // cache.insert(poset.clone(), Some(0));

        return Some(0);
    }

    if max_comps == 0 {
        return None;
    }
    let mut result = None;

    for i in 0..poset.n() {
        for j in 0..poset.n() {
            if i == j || poset.has_order(i, j) {
                continue;
            }

            let less = poset.with_less(i, j);
            let less_result = search(less, max_comps - 1, cache);

            let greater = poset.with_less(j, i);
            let greater_result = search(greater, max_comps - 1, cache);

            if let Some(comps_less) = less_result {
                if let Some(comps_greater) = greater_result {
                    let comps_max = comps_less.max(comps_greater) + 1;
                    max_comps = max_comps.min(comps_max);

                    if let Some(min) = &mut result {
                        *min = comps_max.min(*min);
                    } else {
                        result = Some(comps_max)
                    }
                }
            }
        }
    }

    cache.insert(poset, result);

    result
}
