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

const KNOWN_MIN_VALUES: [&[u8]; 15] = [
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
    &[13, 16, 19, 21, 23, 24, 24],
    &[14, 17, 20, 23, 25, 25, 23, 24],
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

    for n in start_n..=MAX_N as u8 {
        let start_i = if n == start_n { args.i.unwrap_or(0) } else { 0 };

        for i in start_i..(n + 1) / 2 {
            let old_cache_len = cache.len();
            let cost = Search::new(n, i, &mut cache).search();

            if let Cost::Solved(_comparisons) = cost {
                if n < KNOWN_MIN_VALUES.len() as u8 {
                    assert_eq!(_comparisons, KNOWN_MIN_VALUES[n as usize - 1][i as usize]);
                }

                println!("cache_entries = {}", cache.len());

                if !args.no_cache && cache.len() != old_cache_len {
                    save_cache(&args.cache_save_file, &cache);
                }

                if args.explore {
                    let mapping = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
                    explore(Poset::new(n, i), mapping, &cache);
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

fn explore(poset: Poset, mapping: [u8; MAX_N], cache: &HashMap<Poset, Cost>) {
    loop {
        let old_mapping = {
            let mut old = [0; MAX_N];
            for i in 0..poset.n() {
                old[mapping[i as usize] as usize] = i;
            }
            old
        };

        print!("     |");
        for i in 0..poset.n() {
            print!(" {i} |");
        }
        println!();

        println!("-----+{}", "---+".repeat(poset.n().into()));
        for i in 0..poset.n() {
            let mapped_i = old_mapping[i as usize];

            print!("{i:2} < |");

            for j in 0..poset.n() {
                let mapped_j = old_mapping[j as usize];

                if mapped_i == mapped_j || poset.has_order(mapped_i, mapped_j) {
                    if poset.is_less(mapped_i, mapped_j) {
                        print!("   |");
                    } else {
                        print!("   |");
                    }
                    continue;
                }

                let less = poset.with_less(mapped_i, mapped_j);
                match cache.get(&less) {
                    Some(Cost::Solved(cost)) => print!(" {:<2}|", cost),
                    Some(Cost::Minimum(cost)) => print!(">{:2}|", cost - 1),
                    None => print!(" ? |"),
                }
            }

            println!();
            println!("-----+{}", "---+".repeat(poset.n().into()));
        }

        let mut input = String::new();

        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "" | "q" => break,
            _ => match input.trim().split_once('<') {
                Some((left, right)) => {
                    match (left.trim().parse::<u8>(), right.trim().parse::<u8>()) {
                        (Ok(i), Ok(j)) => {
                            let mapped_i = old_mapping[i as usize];
                            let mapped_j = old_mapping[j as usize];

                            let (next, new_mapping) = poset.with_less_mapping(mapped_i, mapped_j);

                            let next_mapping = {
                                let mut new = [0; MAX_N];
                                for i in 0..poset.n() {
                                    new[i as usize] = mapping[new_mapping[i as usize] as usize];
                                }
                                new
                            };

                            explore(next, next_mapping, cache);
                        }
                        _ => {}
                    }
                }
                None => {}
            },
        }
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
