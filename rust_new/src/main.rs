use std::env;

mod search_backward;
mod search_bidirectional;
mod search_forward;

mod cache_set;
mod cache_tree;
mod poset;
mod util;

enum SearchMode {
  ForwardSearch,
  BackwardSearch,
  BidirectionalSearch,
}

fn main() {
  env::set_var("RUST_BACKTRACE", "1");

  let mut mode = SearchMode::ForwardSearch;
  mode = SearchMode::BackwardSearch;
  // mode = SearchMode::BidirectionalSearch;
  match mode {
    SearchMode::ForwardSearch => search_forward::main(),
    SearchMode::BackwardSearch => search_backward::main(),
    SearchMode::BidirectionalSearch => search_bidirectional::main(),
  }
}
