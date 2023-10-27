use clap::Parser;
use hashbrown::HashMap;
use search::Cost;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    time::Instant,
};

use poset::{Poset, MAX_N};

use crate::search::Search;

mod poset;
mod search;

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

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// The n to start at
    #[arg(short)]
    n: Option<u8>,
    /// The i to start at. Requires n to be set.
    #[arg(short, requires("n"))]
    i: Option<u8>,
    /// Do only a single calculation
    #[arg(short, long, default_value_t = false, requires("i"))]
    single: bool,
    /// The name of the cache to use.
    #[arg(short, long, default_value = "cache.dat", value_hint = clap::ValueHint::FilePath)]
    cache_file: String,
}

fn main() {
    let args = Args::parse();

    let start_n = args.n.unwrap_or(1);

    let mut cache = load_cache().unwrap_or(HashMap::new());

    println!("cache_entries = {}", cache.len());

    for n in start_n..MAX_N as u8 {
        let start_i = if n == start_n { args.i.unwrap_or(0) } else { 0 };

        for i in start_i..(n + 1) / 2 {
            let cost = Search::new(n, i, &mut cache).search();

            if let Cost::Solved(comparisons) = cost {
                if n < KNOWN_VALUES.len() as u8 {
                    assert_eq!(comparisons, KNOWN_VALUES[n as usize - 1][i as usize]);
                }

                save_cache(&cache);
            } else {
                unreachable!()
            }

            if args.single {
                return;
            }
        }
    }
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
