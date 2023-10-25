use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Read, Write},
    time::Instant,
};

use poset::{Poset, MAX_N};

mod poset;

/// Number of comparisons
pub type Cost = u8;

const KNOWN_VALUES: [&[u8]; 12] = [
    &[0],
    &[1],
    &[2, 3],
    &[3, 4],
    &[4, 6, 6],
    &[5, 7, 8],
    &[6, 8, 10, 10],
    &[7, 9, 11, 12],
    &[8, 11, 12, 14, 14],
    &[9, 12, 14, 15, 16],
    &[10, 13, 15, 17, 18, 18],
    &[11, 14, 17, 18, 19, 20],
];

fn main() {
    let mut cache = load_cache().unwrap_or(HashMap::new());

    println!("cache_entries = {}", cache.len());
    for n in 1..MAX_N as u8 {
        for i in 0..(n + 1) / 2 {
            let start = Instant::now();

            let comparisons = my_search(Poset::new(n, i), Cost::MAX, &mut cache);

            if let Some(comparisons) = comparisons {
                println!("n = {n}, i = {i}, comparisons = {comparisons}");
                println!("cache_entries = {}", cache.len());

                if n < KNOWN_VALUES.len() as u8 {
                    assert_eq!(comparisons, KNOWN_VALUES[n as usize - 1][i as usize]);
                }

                save_cache(&cache);
            } else {
                println!("found no solution for n = {n}, i = {i}");
            }

            println!("time = {}s", (Instant::now() - start).as_secs_f64());
        }
    }
}

fn my_search(
    poset: Poset,
    mut best: Cost,
    cache: &mut HashMap<Poset, Option<Cost>>,
) -> Option<Cost> {
    if let Some(result) = cache.get(&poset) {
        return *result;
    }

    if poset.n() == 1 {
        return Some(0);
    }

    if best == 0 {
        return None;
    }

    if !poset.is_solvable_in(best) {
        return None;
    }

    let mut result = None;

    for i in 0..poset.n() {
        for j in (i + 1)..poset.n() {
            if poset.has_order(i, j) {
                continue;
            }

            let less = poset.with_less(i, j);
            let less_result = my_search(less, best - 1, cache);

            let greater = poset.with_less(j, i);
            let greater_result = my_search(greater, best - 1, cache);

            if let (Some(comps_less), Some(comps_greater)) = (less_result, greater_result) {
                let comps_max = comps_less.max(comps_greater) + 1;
                best = best.min(comps_max);

                if let Some(min) = &mut result {
                    *min = comps_max.min(*min);
                } else {
                    result = Some(comps_max)
                }
            }
        }
    }

    // if result.is_some() {
    cache.insert(poset.dual(), result);
    cache.insert(poset, result);
    // }

    result
}

const CACHE_FILE_PATH: &str = "cache.dat";

fn save_cache(cache: &HashMap<Poset, Option<Cost>>) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(CACHE_FILE_PATH)
        .unwrap();

    let bytes = postcard::to_stdvec(cache).unwrap();

    file.write_all(&bytes).unwrap();
}

fn load_cache() -> Option<HashMap<Poset, Option<Cost>>> {
    let mut file = match OpenOptions::new().read(true).open(CACHE_FILE_PATH) {
        Ok(file) => file,
        Err(err) => {
            dbg!(err);
            return None;
        }
    };

    let mut bytes = vec![];
    match file.read_to_end(&mut bytes) {
        Ok(len) => {
            dbg!(len);
        }
        Err(err) => {
            dbg!(err);
            return None;
        }
    }

    let cache = postcard::from_bytes(&bytes).map_or(None, |c| Some(c));

    cache
}
