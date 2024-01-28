use std::collections::HashMap;
use std::sync::RwLock;

use super::poset::Poset;
use super::util::MAX_N;

pub struct CacheSetSingle<const IS_SOLVABLE: bool> {
  cache: [[HashMap<Poset, u8>; MAX_N]; MAX_N],
  mutex: [[RwLock<()>; MAX_N]; MAX_N],
}

impl<const IS_SOLVABLE: bool> CacheSetSingle<IS_SOLVABLE> {
  pub fn new() -> Self {
    CacheSetSingle {
      cache: Default::default(),
      mutex: Default::default(),
    }
  }

  pub fn insert(&mut self, poset: &Poset, remaining_comparisons: u8) {
    let _lock = self.mutex[poset.n() as usize][poset.i() as usize]
      .write()
      .unwrap();

    match self.cache[poset.n() as usize][poset.i() as usize].get_mut(poset) {
      None => {
        self.cache[poset.n() as usize][poset.i() as usize]
          .insert(poset.clone(), remaining_comparisons);
      }
      Some(value) => {
        if (IS_SOLVABLE && remaining_comparisons < *value)
          || (!IS_SOLVABLE && remaining_comparisons > *value)
        {
          *value = remaining_comparisons;
        }
      }
    }
  }

  pub fn check(&self, poset: &Poset, remaining_comparisons: u8) -> bool {
    let _lock = self.mutex[poset.n() as usize][poset.i() as usize]
      .read()
      .unwrap();
    match self.cache[poset.n() as usize][poset.i() as usize].get(poset) {
      Some(value) => {
        (IS_SOLVABLE && remaining_comparisons >= *value)
          || (!IS_SOLVABLE && remaining_comparisons <= *value)
      }
      None => false,
    }
  }

  pub fn size(&self) -> usize {
    let mut sum = 0;
    for n in 0..MAX_N {
      for i in 0..MAX_N {
        let _lock = self.mutex[n][i].read().unwrap();
        sum += self.cache[n][i].len();
      }
    }
    sum
  }
}

pub struct CacheSetDual {
  cache_solvable: CacheSetSingle<true>,
  cache_not_solvable: CacheSetSingle<false>,
}

impl CacheSetDual {
  pub fn new() -> Self {
    Self {
      cache_solvable: CacheSetSingle::new(),
      cache_not_solvable: CacheSetSingle::new(),
    }
  }

  pub fn check_not_solvable(&self, poset: &Poset, remaining_comparisons: u8) -> bool {
    debug_assert!(2 * poset.i() < poset.n());
    self.cache_not_solvable.check(poset, remaining_comparisons)
  }

  pub fn check_solvable(&self, poset: &Poset, remaining_comparisons: u8) -> bool {
    debug_assert!(2 * poset.i() < poset.n());
    self.cache_solvable.check(poset, remaining_comparisons)
  }

  pub fn insert_not_solvable(&mut self, poset: &Poset, remaining_comparisons: u8) {
    debug_assert!(2 * poset.i() < poset.n());
    self.cache_not_solvable.insert(poset, remaining_comparisons);
  }

  pub fn insert_solvable(&mut self, poset: &Poset, remaining_comparisons: u8) {
    debug_assert!(2 * poset.i() < poset.n());
    self.cache_solvable.insert(poset, remaining_comparisons);
  }

  pub fn size(&self) -> usize {
    self.cache_not_solvable.size() + self.cache_solvable.size()
  }
}

// TODO: toString for cache instead of size

#[test]
fn test() {
  let mut poset = Poset::new(10, 2);
  poset.add_less(3, 7);

  let mut poset2 = Poset::new(10, 2);
  poset2.add_less(2, 7);

  let mut cache = CacheSetDual::new();
  cache.insert_solvable(&poset, 2);

  cache.insert_not_solvable(&poset2, 2);

  assert!(!cache.check_solvable(&poset, 1));
  assert!(cache.check_solvable(&poset, 2));
  assert!(cache.check_solvable(&poset, 3));

  assert!(!cache.check_solvable(&poset2, 1));
  assert!(!cache.check_solvable(&poset2, 2));
  assert!(!cache.check_solvable(&poset2, 3));

  assert!(!cache.check_not_solvable(&poset, 1));
  assert!(!cache.check_not_solvable(&poset, 2));
  assert!(!cache.check_not_solvable(&poset, 3));

  assert!(cache.check_not_solvable(&poset2, 1));
  assert!(cache.check_not_solvable(&poset2, 2));
  assert!(!cache.check_not_solvable(&poset2, 3));
}
