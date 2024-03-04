use std::{
  collections::HashSet,
  env,
  sync::{atomic::AtomicBool, Arc},
};

use poset::Poset;
use util::MAX_N;

mod search_backward;
mod search_bidirectional;
mod search_forward;

mod cache_set;
mod cache_tree;
mod poset;
mod util;

enum SearchMode {
  Forward,
  Backward,
  Bidirectional,
}

fn main() {
  env::set_var("RUST_BACKTRACE", "1");

  let mut mode;
  // mode = SearchMode::Forward;
  mode = SearchMode::Backward;
  // mode = SearchMode::Bidirectional;

  let args: Vec<String> = env::args().collect();
  if 2 == args.len() {
    if "forward" == args[1] || "0" == args[1] {
      mode = SearchMode::Forward;
    } else if "backward" == args[1] || "1" == args[1] {
      mode = SearchMode::Backward;
    } else if "bidirectional" == args[1] || "2" == args[1] {
      mode = SearchMode::Bidirectional;
    }
  }

  match mode {
    SearchMode::Forward => search_forward::main(),
    SearchMode::Backward => search_backward::main(),
    SearchMode::Bidirectional => search_bidirectional::main(),
  }

  // let interrupt = Arc::new(AtomicBool::new(false));

  // let mut poset_cache = HashSet::new();
  // poset_cache.insert(Poset::new(1, 0));
  // poset_cache.insert(Poset::new(2, 0));
  // poset_cache.insert(Poset::new(3, 0));
  // poset_cache.insert({
  //   let mut a = Poset::new(3, 1);
  //   a.add_less(2, 0);
  //   a
  // });
  // poset_cache.insert({
  //   let mut a = Poset::new(4, 1);
  //   a.add_less(2, 0);
  //   a.add_less(3, 1);
  //   a
  // });

  // let n = 5;
  // let i0 = 2;

  // let mut table = [[false; MAX_N]; MAX_N];
  // Poset::rec_temp(&mut table, n as usize, i0 as usize);

  // let mut item = Poset::new(3, 1);
  // item.add_less(2, 0);

  // // let a = item.enlarge_and_remove_less_old(&interrupt, &poset_cache, &table, n, i0);
  // // let b = item.enlarge_and_remove_less(&interrupt, &poset_cache, &table, n, i0);
  // // if a != b {
  // //   dbg!(item, &poset_cache, a, b);
  // //   panic!();
  // // }

  // let poset = Poset::new(3, 1);

  // let k = 1;
  // let j = 0;

  // let mut result = HashSet::new();
  // poset.super_enlarge_n((k, j), &mut result);
  // poset.super_enlarge_nk((k, j), &mut result);
  // dbg!(result);

  // // let mut poset = Poset::new(4, 1);
  // // poset.add_less(2, 0);
  // // // poset.add_less(0, 1);
  // // // poset.add_less(1, 2);
  // // // poset.add_less(3, 4);
  // // // // poset.add_less(2, 4);
  // // // // poset.add_less(3, 0);

  // // for i in 0..4 {
  // //   for j in 0..4 {
  // //     if i != j && !poset.is_less(i, j) && !poset.is_less(j, i) {
  // //       let mut cloned = poset.clone();
  // //       cloned.add_less(i, j);
  // //       cloned.normalize();
  // //       dbg!(i, j, &cloned);
  // //     }
  // //   }
  // // }
}
