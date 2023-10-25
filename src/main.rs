use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    time::Instant,
};

use poset::{Poset, MAX_N};

mod poset;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Cost {
    /// Not solved. Impossible in less than the number of comparisons
    Minimum(u8),
    /// Solved in the number of comparisons
    Solved(u8),
}

impl Cost {
    pub fn min(self, other: Self) -> Self {
        match self {
            Cost::Minimum(min) => match other {
                Cost::Minimum(min2) => Cost::Minimum(min.min(min2)),
                Cost::Solved(solved) => {
                    if min < solved {
                        Cost::Minimum(min)
                    } else {
                        Cost::Solved(solved)
                    }
                }
            },
            Cost::Solved(solved) => match other {
                Cost::Minimum(min) => {
                    if min < solved {
                        Cost::Minimum(min)
                    } else {
                        Cost::Solved(solved)
                    }
                }
                Cost::Solved(solved2) => Cost::Solved(solved.min(solved2)),
            },
        }
    }

    pub fn max(self, other: Self) -> Self {
        match self {
            Cost::Minimum(min) => match other {
                Cost::Minimum(min2) => Cost::Minimum(min.max(min2)),
                Cost::Solved(solved) => Cost::Minimum(solved.max(min)),
            },
            Cost::Solved(solved) => match other {
                Cost::Minimum(min) => Cost::Minimum(solved.max(min)),
                Cost::Solved(solved2) => Cost::Solved(solved.max(solved2)),
            },
        }
    }
}

const KNOWN_VALUES: [&[u8]; 13] = [
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
    &[12, 15, 18, 20, 21, 22, 23],
];

fn main() {
    let mut cache = load_cache().unwrap_or(HashMap::new());

    println!("cache_entries = {}", cache.len());

    for n in 1..MAX_N as u8 {
        for i in 0..(n + 1) / 2 {
            let start = Instant::now();

            for k in 1..u8::max_value() {
                let mut nodes = 0;
                let cost = new_search(Poset::new(n, i), k, &mut nodes, &mut cache);

                if let Cost::Solved(comparisons) = cost {
                    println!("n = {n}, i = {i}, comparisons = {comparisons}");
                    println!("cache_entries = {}", cache.len());
                    println!("nodes = {}", nodes);

                    if n < KNOWN_VALUES.len() as u8 {
                        assert_eq!(comparisons, KNOWN_VALUES[n as usize - 1][i as usize]);
                    }

                    save_cache(&cache);

                    break;
                } else {
                    println!("found no solution for n = {n}, i = {i}, comparisons = {k}");
                }
            }

            println!("time = {}s", (Instant::now() - start).as_secs_f64());
        }
    }
}

fn new_search(
    poset: Poset,
    mut max_comparisons: u8,
    nodes: &mut u64,
    cache: &mut HashMap<Poset, Cost>,
) -> Cost {
    if let Some(cost) = cache.get(&poset) {
        match cost {
            Cost::Solved(_) => {
                // if *solved > max_comparisons {
                //     return Cost::Minimum(*solved);
                // } else {
                //     return Cost::Solved(*solved);
                // }
                return *cost;
            }
            Cost::Minimum(min) => {
                if *min > max_comparisons {
                    return *cost;
                }
            }
        }
    }

    if poset.n() == 1 {
        return Cost::Solved(0);
    }

    if max_comparisons == 0 {
        cache.insert(poset, Cost::Minimum(1));
        return Cost::Minimum(1);
    }

    if !poset.is_solvable_in(max_comparisons) {
        cache.insert(poset, Cost::Minimum(max_comparisons + 1));
        return Cost::Minimum(max_comparisons + 1);
    }

    let mut result = Cost::Minimum(max_comparisons + 1);

    for i in 0..poset.n() {
        for j in (i + 1)..poset.n() {
            if poset.has_order(i, j) {
                continue;
            }

            let less = poset.with_less(i, j);
            let greater = poset.with_less(j, i);

            let less_result = new_search(less, max_comparisons - 1, nodes, cache);
            let greater_result = new_search(greater, max_comparisons - 1, nodes, cache);

            let new_result = match less_result.max(greater_result) {
                Cost::Minimum(_) => continue,
                Cost::Solved(solved) => Cost::Solved(solved + 1),
            };

            result = result.min(new_result);

            max_comparisons = if let Cost::Solved(comps) = result {
                comps.min(max_comparisons)
            } else {
                max_comparisons
            };
        }
    }

    *nodes += 1;
    if *nodes % 10000 == 0 {
        dbg!(cache.len());
    }

    cache.insert(poset.dual(), result);
    cache.insert(poset, result);

    result
}

const CACHE_FILE_PATH: &str = "cache.dat";

fn save_cache(cache: &HashMap<Poset, Cost>) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(CACHE_FILE_PATH)
        .unwrap();

    let bytes = postcard::to_stdvec(cache).unwrap();

    file.write_all(&bytes).unwrap();
}

fn load_cache() -> Option<HashMap<Poset, Cost>> {
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
