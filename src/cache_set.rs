use std::sync::RwLock;

use hashbrown::HashMap;

use super::poset::Poset;
use super::util::MAX_N;

pub struct CacheSet<const IS_SOLVABLE: bool> {
  cache: [[HashMap<Poset, u8>; MAX_N]; MAX_N],
  mutex: [[RwLock<()>; MAX_N]; MAX_N],
}

impl<const IS_SOLVABLE: bool> CacheSet<IS_SOLVABLE> {
  pub fn new() -> Self {
    CacheSet {
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

pub type CacheSetSolvable = CacheSet<true>;

pub type CacheSetNotSolvable = CacheSet<false>;

#[test]
fn test1() {
  let mut poset = Poset::new(10, 2);
  poset.add_less(3, 7);
  poset.normalize();

  let mut cache_solvable = CacheSetSolvable::new();
  cache_solvable.insert(&poset, 2);

  assert!(!cache_solvable.check(&poset, 1));
  assert!(cache_solvable.check(&poset, 2));
  assert!(cache_solvable.check(&poset, 3));
}

#[test]
fn test2() {
  let mut poset = Poset::new(10, 2);
  poset.add_less(2, 7);
  poset.normalize();

  let mut cache_not_solvable = CacheSetNotSolvable::new();
  cache_not_solvable.insert(&poset, 2);

  assert!(cache_not_solvable.check(&poset, 1));
  assert!(cache_not_solvable.check(&poset, 2));
  assert!(!cache_not_solvable.check(&poset, 3));
}
