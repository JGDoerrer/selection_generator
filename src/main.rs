use canonified_poset::CanonifiedPoset;
use clap::{
    error::{Error, ErrorKind},
    ArgAction, Parser,
};
use search_backward::single;
use search_forward::Cost;
use std::{
    collections::HashMap,
    fs::{DirBuilder, OpenOptions},
    io::{BufWriter, Write},
    str::FromStr,
    sync::{atomic::AtomicBool, Arc},
};

use crate::{
    cache::Cache,
    constants::{KNOWN_VALUES, MAX_N},
    normal_poset::NormalPoset,
    poset::Poset,
    search_forward::Search,
};

mod algorithm_test;
mod backwards_poset;
mod bitset;
mod cache;
mod cache_tree;
mod canonified_poset;
mod constants;
mod normal_poset;
mod poset;
mod search_backward;
mod search_bidirectional;
mod search_forward;
mod utils;

#[derive(Debug, Clone)]
pub enum SearchMode {
    Forward,
    Backward,
    Bidirectional,
}

impl FromStr for SearchMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(SearchMode::Forward),
            "backward" => Ok(SearchMode::Backward),
            "bidirectional" => Ok(SearchMode::Bidirectional),
            _ => Err(Error::new(ErrorKind::InvalidValue)),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(long)]
    search_mode: SearchMode,
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
    /// Print the algorithm which solves the problem
    #[arg(short, long, default_value_t = false)]
    print_algorithm: bool,
}

fn main() {
    let args = Args::parse();
    match args.search_mode {
        SearchMode::Forward => run_forward(args),
        SearchMode::Backward => run_backward(),
        SearchMode::Bidirectional => search_bidirectional::main(),
    }
}

fn run_forward(args: Args) {
    let start_n = args.n.unwrap_or(1);

    let mut cache = Cache::new(args.max_cache_size);
    let mut algorithm = HashMap::new();

    println!("Cache entries: {}", cache.len());
    println!("Maximum cache entries: {}", cache.max_entries());

    // additional meta information
    if args.verbose != 0 {
        utils::print_git_info();
        utils::print_lscpu();
        utils::print_current_time();
    }

    for n in start_n..=MAX_N as u8 {
        let start_i = if n == start_n { args.i.unwrap_or(0) } else { 0 };

        for i in start_i..(n + 1) / 2 {
            let result = Search::new(n, i, &mut cache, &mut algorithm).search();

            if (n as usize) < KNOWN_VALUES.len() && (i as usize) < KNOWN_VALUES[n as usize].len() {
                assert_eq!(result, KNOWN_VALUES[n as usize][i as usize] as u8);
            }

            if args.print_algorithm {
                let mut mapping = [0; MAX_N];
                mapping
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, elem)| *elem = i as u8);

                DirBuilder::new()
                    .recursive(true)
                    .create("algorithms")
                    .unwrap();

                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(format!("algorithms/{n}_{i}.rs"))
                    .unwrap();

                let mut writer = BufWriter::new(file);

                print_algorithm(
                    CanonifiedPoset::new(n, i),
                    &mut writer,
                    &algorithm,
                    &mut HashMap::new(),
                );
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

fn run_backward() {
    let interrupt = Arc::new(AtomicBool::new(false));

    if false {
        single(&interrupt, 9, 4);
    } else {
        for n in 2..MAX_N as u8 {
            for i in 0..((n + 1) / 2) {
                single(&interrupt, n, i);
            }
            println!();
        }
    }
}

fn print_algorithm<W>(
    poset: CanonifiedPoset,
    writer: &mut BufWriter<W>,
    comparisons: &HashMap<CanonifiedPoset, (u8, u8)>,
    done: &mut HashMap<CanonifiedPoset, usize>,
) -> usize
where
    W: Write,
{
    const VARIABLES: [&str; 16] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
    ];

    if let Some(index) = done.get(&poset) {
        return *index;
    }

    let index = done.len();

    done.insert(poset, index);
    if poset.n() == 1 {
        writeln!(writer, "/// n = 1").unwrap();
        writeln!(writer, "fn select_{index}([a]: [usize; 1]) -> usize {{").unwrap();
        writeln!(writer, "    a").unwrap();
        writeln!(writer, "}}").unwrap();

        return index;
    }

    let comparison = comparisons.get(&poset);
    let (i, j) = if let Some((i, j)) = comparison {
        (*i, *j)
    } else {
        dbg!(poset);
        unreachable!()
    };

    let (less, less_mapping) = poset.to_normal().with_less_mapping(i, j);
    let (greater, greater_mapping) = poset.to_normal().with_less_mapping(j, i);

    let less_index = print_algorithm(less.canonified(), writer, comparisons, done);
    let greater_index = print_algorithm(greater.canonified(), writer, comparisons, done);

    let vars = (0..poset.n() as usize)
        .map(|i| VARIABLES[i].to_string())
        .reduce(|a, b| format!("{a}, {b}"))
        .unwrap();

    let less_vars = less_mapping
        .iter()
        .take(less.n() as usize)
        .map(|i| VARIABLES[*i as usize].to_string())
        .reduce(|a, b| format!("{a}, {b}"))
        .unwrap();

    let greater_vars = greater_mapping
        .iter()
        .take(greater.n() as usize)
        .map(|i| VARIABLES[*i as usize].to_string())
        .reduce(|a, b| format!("{a}, {b}"))
        .unwrap();

    // calculate comparisons
    let mut comparisons = vec![];
    for i in 0..poset.n() {
        'j_loop: for j in i + 1..poset.n() {
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

            if is_less {
                comparisons.push((i, j));
            } else {
                comparisons.push((j, i));
            }
        }
    }

    let comparisons = comparisons
        .into_iter()
        .map(|(i, j)| format!("{} < {}", VARIABLES[i as usize], VARIABLES[j as usize]))
        .reduce(|a, b| format!("{a}, {b}"))
        .map(|s| ", ".to_string() + s.as_str())
        .unwrap_or("".to_string());

    writeln!(
        writer,
        "/// n = {}, i = {}{comparisons}",
        poset.n(),
        poset.i()
    )
    .unwrap();
    writeln!(
        writer,
        "fn select_{index}([{vars}]: [usize; {}]) -> usize {{",
        poset.n()
    )
    .unwrap();
    // i don't know why this is reversed
    writeln!(
        writer,
        "    if {} > {} {{",
        VARIABLES[i as usize], VARIABLES[j as usize]
    )
    .unwrap();
    writeln!(writer, "        select_{less_index}([{less_vars}])").unwrap();
    writeln!(writer, "    }} else {{").unwrap();
    writeln!(writer, "        select_{greater_index}([{greater_vars}])").unwrap();
    writeln!(writer, "    }}").unwrap();
    writeln!(writer, "}}").unwrap();

    index
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
