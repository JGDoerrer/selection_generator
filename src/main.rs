use clap::Parser;
use hashbrown::HashMap;
use search::Cost;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
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
    &[11, 14, 17, 18, 19, 20], // why is this wrong??
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
    #[arg(long, default_value = "cache.dat", value_hint = clap::ValueHint::FilePath)]
    cache_load_file: String,
    #[arg(long, default_value = "cache.dat", value_hint = clap::ValueHint::FilePath)]
    cache_save_file: String,
    #[arg(long, default_value_t = false)]
    no_cache: bool,
    #[arg(short, long, default_value_t = false)]
    explore: bool,
}

fn main() {
    let args = Args::parse();

    let start_n = args.n.unwrap_or(1);

    let mut cache = if args.no_cache {
        HashMap::new()
    } else {
        load_cache(&args.cache_load_file).unwrap_or(HashMap::new())
    };

    println!("cache_entries = {}", cache.len());

    for n in start_n..MAX_N as u8 {
        let start_i = if n == start_n { args.i.unwrap_or(0) } else { 0 };

        for i in start_i..(n + 1) / 2 {
            let cost = Search::new(n, i, &mut cache).search();

            if let Cost::Solved(_comparisons) = cost {
                if n < KNOWN_VALUES.len() as u8 {
                    // assert_eq!(comparisons, KNOWN_VALUES[n as usize - 1][i as usize]);
                }

                if !args.no_cache {
                    save_cache(&args.cache_save_file, &cache);
                }

                if args.explore {
                    explore(Poset::new(n, i), &cache);
                    return;
                }
            } else {
                unreachable!()
            }

            if args.single {
                return;
            }
        }
    }
}

fn explore(poset: Poset, cache: &HashMap<Poset, Cost>) {
    dbg!(&poset);

    print!("     |");
    for i in 0..poset.n() {
        print!("  {i}");
    }
    println!();

    println!("-----+{}", "---".repeat(poset.n().into()));
    for i in 0..poset.n() {
        print!(" {i} < | ");

        for j in 0..poset.n() {
            if i == j || poset.has_order(i, j) {
                print!("   ");
                continue;
            }

            let less = poset.with_less(i, j);
            match cache.get(&less) {
                Some(cost) => print!("{:2} ", cost.value()),
                None => print!("?? "),
            }
        }

        println!()
    }

    let mut input = String::new();

    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim().split_once('<') {
        Some((left, right)) => match (left.trim().parse::<u8>(), right.trim().parse::<u8>()) {
            (Ok(left), Ok(right)) => {
                explore(poset.with_less(left, right), cache);
            }
            _ => {}
        },
        None => {}
    }
}

fn save_cache(path: &String, cache: &HashMap<Poset, Cost>) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let bytes = postcard::to_stdvec(cache).unwrap();

    file.write_all(&bytes).unwrap();
}

fn load_cache(path: &String) -> Option<HashMap<Poset, Cost>> {
    let mut file = match OpenOptions::new().read(true).open(path) {
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
