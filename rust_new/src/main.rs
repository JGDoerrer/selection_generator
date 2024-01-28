use std::env;

mod search_backward;
mod search_bidirectional;
mod search_forward;

mod cache_set;
mod cache_tree;
mod poset;
mod util;

fn main() {
  env::set_var("RUST_BACKTRACE", "1");

  search_forward::main();
  // search_backward::main();
  // search_bidirectional::main();
}
