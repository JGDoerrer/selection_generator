use std::env;

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

  // crate::poset::test_enlarge();
}
