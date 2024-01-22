use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};

const GLOBAL_MAX_N: usize = 15;

struct PosetCacheSet<N: usize, C: usize> {
  cache: [[HashMap<Poset<N>, u8>; GLOBAL_MAX_N]; GLOBAL_MAX_N],
  mutex_cache: [[RwLock<()>; GLOBAL_MAX_N]; GLOBAL_MAX_N],
}

impl<N: usize, C: usize> PosetCacheSet<N, C> {
  fn new() -> Self {
    PosetCacheSet {
      cache: Default::default(),
      mutex_cache: Default::default(),
    }
  }

  fn insert_not_solvable(&self, poset: &Poset<N>, remaining_comparisons: u8) {
    let _lock = self.mutex_cache[poset.size()][poset.nth()].write().unwrap();
    let temp = self.cache[poset.size()][poset.nth()].get_mut(poset);
    match temp {
      Some(value) => {
        if remaining_comparisons > *value {
          *value = remaining_comparisons;
        }
      }
      None => {
        self.cache[poset.size()][poset.nth()].insert(poset.clone(), remaining_comparisons);
      }
    }
  }

  fn insert_solvable(&self, poset: &Poset<N>, remaining_comparisons: u8) {
    let _lock = self.mutex_cache[poset.size()][poset.nth()].write().unwrap();
    let temp = self.cache[poset.size()][poset.nth()].get_mut(poset);
    match temp {
      Some(value) => {
        if remaining_comparisons < *value {
          *value = remaining_comparisons;
        }
      }
      None => {
        self.cache[poset.size()][poset.nth()].insert(poset.clone(), remaining_comparisons);
      }
    }
  }

  fn check_not_solvable(
    &self,
    poset: &Poset<N>,
    remaining_comparisons: u8,
    _special: bool,
  ) -> bool {
    let _lock = self.mutex_cache[poset.size()][poset.nth()].read().unwrap();
    if let Some(value) = self.cache[poset.size()][poset.nth()].get(poset) {
      return remaining_comparisons <= *value;
    }
    false
  }

  fn check_solvable(&self, poset: &Poset<N>, remaining_comparisons: u8, _special: bool) -> bool {
    let _lock = self.mutex_cache[poset.size()][poset.nth()].read().unwrap();
    if let Some(value) = self.cache[poset.size()][poset.nth()].get(poset) {
      return remaining_comparisons >= *value;
    }
    false
  }

  fn clean(&self, is_not_solvable: bool) {
    // Implement cleaning logic if needed
    // ...
  }

  fn size(&self) -> usize {
    let mut sum = 0;
    for n in 0..GLOBAL_MAX_N {
      for i in 0..GLOBAL_MAX_N {
        let _lock = self.mutex_cache[n][i].read().unwrap();
        sum += self.cache[n][i].len();
      }
    }
    sum
  }
}

struct CacheSet<N: usize, C: usize> {
  cache_not_solvable: PosetCacheSet<N, C>,
  cache_solvable: PosetCacheSet<N, C>,
}

impl<N: usize, C: usize> CacheSet<N, C> {
  fn new() -> Self {
    CacheSet {
      cache_not_solvable: PosetCacheSet::new(),
      cache_solvable: PosetCacheSet::new(),
    }
  }

  fn check_not_solvable(
    &self,
    poset: &Poset<N>,
    remaining_comparisons: u8,
    _special: bool,
  ) -> bool {
    assert!(2 * poset.nth() < poset.size());
    self
      .cache_not_solvable
      .check_not_solvable(poset, remaining_comparisons, _special)
  }

  fn check_solvable(&self, poset: &Poset<N>, remaining_comparisons: u8, _special: bool) -> bool {
    assert!(2 * poset.nth() < poset.size());
    self
      .cache_solvable
      .check_solvable(poset, remaining_comparisons, _special)
  }

  fn insert_not_solvable(&self, poset: &Poset<N>, remaining_comparisons: u8) {
    assert!(2 * poset.nth() < poset.size());
    self
      .cache_not_solvable
      .insert_not_solvable(poset, remaining_comparisons);
  }

  fn insert_solvable(&self, poset: &Poset<N>, remaining_comparisons: u8) {
    assert!(2 * poset.nth() < poset.size());
    self
      .cache_solvable
      .insert_solvable(poset, remaining_comparisons);
  }

  fn clean(&self) {
    self.cache_not_solvable.clean(true);
    self.cache_solvable.clean(false);
  }

  fn size(&self) -> usize {
    self.cache_not_solvable.size() + self.cache_solvable.size()
  }
}
