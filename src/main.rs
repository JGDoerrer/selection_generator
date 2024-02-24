use clap::{ArgAction, Parser};
use search::Cost;
use std::io::Write;

use crate::{
    cache::Cache,
    constants::{KNOWN_VALUES, MAX_N},
    normal_poset::NormalPoset,
    poset::Poset,
    search::Search,
};

mod bitset;
mod cache;
mod canonified_poset;
mod constants;
mod normal_poset;
mod poset;
mod search;
mod utils;

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
    /// Explore the cache interactively
    #[arg(short, long, default_value_t = false)]
    explore: bool,
    /// The max amount of bytes of the cache
    #[arg(long, default_value_t = 1 << 33)]
    max_cache_size: usize,
    /// Increase verbosity level
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();

    let start_n = args.n.unwrap_or(1);

    let mut cache = Cache::new(args.max_cache_size);

    println!("Cache entries: {}", cache.len());
    println!("Maximum cache entries: {}", cache.max_entries());

    // additional meta information
    if args.verbose != 0 {
        utils::print_git_info();
        utils::print_lscpu();
    }

    for n in start_n..=MAX_N as u8 {
        let start_i = if n == start_n { args.i.unwrap_or(0) } else { 0 };

        for i in start_i..(n + 1) / 2 {
            let result = Search::new(n, i, &mut cache).search();

            if n < KNOWN_VALUES.len() as u8 {
                assert_eq!(result, KNOWN_VALUES[n as usize][i as usize] as u8);
            }

            if args.explore {
                let mapping = {
                    let mut mapping = [0; MAX_N];
                    mapping
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, elem)| *elem = i as u8);
                    mapping
                };

                explore(NormalPoset::new(n, i), mapping, &cache);
                return;
            }

            if args.single {
                return;
            }
        }
    }
}

fn explore(poset: NormalPoset, mapping: [u8; MAX_N], cache: &Cache) {
    loop {
        let old_mapping = {
            let mut old = [0; MAX_N];
            for i in 0..poset.n() {
                old[mapping[i as usize] as usize] = i;
            }
            old
        };

        let mut best_comp = (0, 0);
        let mut best_cost = u8::MAX;

        // print comparisons
        let mut first = true;
        for i in 0..poset.n() {
            let normal_i = i;
            let i = old_mapping[i as usize];

            'j_loop: for j in normal_i + 1..poset.n() {
                let normal_j = j;
                let j = old_mapping[j as usize];

                if !poset.has_order(i, j) {
                    continue;
                }

                let is_less = poset.is_less(i, j);

                for k in 0..poset.n() {
                    if k != i
                        && k != j
                        && poset.is_less(i, k) == is_less
                        && poset.is_less(k, j) == is_less
                    {
                        continue 'j_loop;
                    }
                }

                if first {
                    first = false
                } else {
                    print!(", ");
                }
                if is_less {
                    print!("{normal_i} < {normal_j}");
                } else {
                    print!("{normal_j} < {normal_i}");
                }
            }
        }
        println!();

        // print matrix
        print!("     |");
        for i in 0..poset.n() {
            print!(" {i:2}|");
        }
        println!();

        println!("-----+{}", "---+".repeat(poset.n().into()));
        for i in 0..poset.n() {
            let mapped_i = old_mapping[i as usize];

            print!("{i:2} < |");

            for j in 0..poset.n() {
                let mapped_j = old_mapping[j as usize];

                if mapped_i == mapped_j || poset.has_order(mapped_i, mapped_j) {
                    print!("   |");
                    continue;
                }

                let less = poset.with_less(mapped_i, mapped_j);
                let greater = poset.with_less(mapped_j, mapped_i);
                match cache.get(&less) {
                    Some(Cost::Solved(cost)) => {
                        print!(" {:<2}|", cost);

                        if let Some(Cost::Solved(other_cost)) = cache.get(&greater) {
                            let max_cost = cost.max(other_cost);
                            if max_cost < best_cost {
                                best_cost = max_cost;
                                best_comp = (i, j);
                            }
                        }
                    }
                    Some(Cost::Minimum(cost)) => print!(">{:2}|", cost - 1),
                    None => print!(" ? |"),
                }
            }

            println!();
            println!("-----+{}", "---+".repeat(poset.n().into()));
        }

        let mut input = String::new();

        println!(
            "best: {}, {}, cost: {}",
            best_comp.0, best_comp.1, best_cost
        );

        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "" | "q" => break,
            _ => {
                if let Some((left, right)) = input.trim().split_once('<') {
                    if let (Ok(i), Ok(j)) = (left.trim().parse::<u8>(), right.trim().parse::<u8>())
                    {
                        let mapped_i = old_mapping[i as usize];
                        let mapped_j = old_mapping[j as usize];

                        let (next, new_mapping) = poset.with_less_mapping(mapped_i, mapped_j);

                        let next_mapping = {
                            let mut new = [0; MAX_N];
                            for i in 0..poset.n() {
                                new[i as usize] = mapping[new_mapping[i as usize]];
                            }
                            new
                        };

                        explore(next, next_mapping, cache);
                    }
                }
            }
        }
    }
}
