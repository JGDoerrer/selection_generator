use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::BitXor;

use super::poset::*;

#[derive(Clone)]
pub struct PosetSet<const subset_direction: bool> {
  n: u8,
  nth_smallest: u8,
}

impl<const subset_direction: bool> PosetSet<subset_direction> {
  pub fn new(n: u8, nth_smallest: u8) -> PosetSet<subset_direction> {
    PosetSet {
      n: n,
      nth_smallest: nth_smallest,
    }
  }

  pub fn reset(&self) {}

  pub fn insert(&self, poset: Poset) {}

  pub fn entries(&self) -> HashSet<Poset> {
    let result: HashSet<Poset> = HashSet::new();
    result
  }
}