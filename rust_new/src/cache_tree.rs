use std::collections::HashSet;
use std::sync::RwLock;

// #[allow(clippy::wildcard_imports)]
use super::poset::Poset;
use super::util::{MAX_COMPARISONS, MAX_N};

struct CacheNode<const is_solvable: bool> {
  branch_is_less: Option<Box<Self>>,
  branch_is_not_less: Option<Box<Self>>,
}

impl<const is_solvable: bool> CacheNode<is_solvable> {
  fn new() -> Self {
    CacheNode {
      branch_is_less: None,
      branch_is_not_less: None,
    }
  }

  fn contains_multi_path(&self, poset: &Poset, index: u8, took_other_path: bool) -> bool {
    if index == 0 {
      took_other_path
    } else if poset.get_index(index - 1) {
      self.branch_is_less.as_ref().map_or(false, |less| {
        less.contains_multi_path(poset, index - 1, took_other_path)
      }) || (is_solvable
        && self.branch_is_not_less.as_ref().map_or(false, |not_less| {
          not_less.contains_multi_path(poset, index - 1, true)
        }))
    } else {
      (self.branch_is_not_less.as_ref().map_or(false, |not_less| {
        not_less.contains_multi_path(poset, index - 1, took_other_path)
      }))
        || (!is_solvable
          && self.branch_is_less.as_ref().map_or(false, |less| {
            less.contains_multi_path(poset, index - 1, true)
          }))
    }
  }

  fn contains(&self, poset: &Poset, index: u8) -> bool {
    self.contains_multi_path(poset, index, true)
  }

  fn entries(
    &self,
    entries: &mut HashSet<Poset>,
    temp: &mut Poset,
    index: u8,
    root_struct: &Box<Self>,
  ) {
    if 0 == index {
      if !root_struct.as_ref().contains_multi_path(
        temp,
        temp.get_comparison_table_size() as u8,
        false,
      ) {
        entries.insert(temp.clone());
      }
    } else if 0 != index {
      if let Some(branch_is_less) = self.branch_is_less.as_ref() {
        temp.set_index(index - 1, true);
        branch_is_less.entries(entries, temp, index - 1, root_struct);
      }
      if let Some(branch_is_not_less) = self.branch_is_not_less.as_ref() {
        temp.set_index(index - 1, false);
        branch_is_not_less.entries(entries, temp, index - 1, root_struct);
      }
    }
  }
}

#[derive(Default)]
pub struct CacheTreeFixed<const is_solvable: bool> {
  n: u8,
  nth_smallest: u8,
  root: Option<Box<CacheNode<is_solvable>>>,
  size: usize,
}

impl<const is_solvable: bool> CacheTreeFixed<is_solvable> {
  pub fn new(n: u8, nth_smallest: u8) -> Self {
    Self {
      n,
      nth_smallest,
      root: Some(Box::new(CacheNode::new())),
      size: 0,
    }
  }

  pub fn reset(&mut self) {
    self.root = Some(Box::new(CacheNode::new()));
    self.size = 0;
  }

  pub fn insert(&mut self, poset: &Poset) {
    let mut last_insert = false;
    let mut level = self.root.as_mut();

    for i in (0..poset.get_comparison_table_size()).rev() {
      last_insert = false;
      if poset.get_index(i as u8) {
        if level.as_mut().unwrap().branch_is_less.is_none() {
          level.as_mut().unwrap().branch_is_less = Some(Box::new(CacheNode::new()));
          last_insert = true;
        }
        level = level.unwrap().branch_is_less.as_mut();
      } else {
        if level.as_mut().unwrap().branch_is_not_less.is_none() {
          level.as_mut().unwrap().branch_is_not_less = Some(Box::new(CacheNode::new()));
          last_insert = true;
        }
        level = level.unwrap().branch_is_not_less.as_mut();
      }
    }

    if last_insert {
      self.size += 1;
    }
  }

  pub fn contains(&self, poset: &Poset) -> bool {
    self
      .root
      .as_ref()
      .unwrap()
      .contains(poset, poset.get_comparison_table_size() as u8)
  }

  pub fn entries(&mut self) -> HashSet<Poset> {
    let mut entries = HashSet::new();
    if let Some(root) = self.root.as_ref() {
      let mut temp = Poset::new(self.n, self.nth_smallest);

      root.entries(&mut entries, &mut temp, self.n * self.n, root);
    }
    entries
  }

  pub fn size(&self) -> usize {
    self.size
  }
}

struct CacheTreeSingle<const is_solvable: bool> {
  cache: [[[CacheTreeFixed<is_solvable>; MAX_COMPARISONS]; MAX_N]; MAX_N],
  mutex: [[[RwLock<CacheTreeFixed<is_solvable>>; MAX_COMPARISONS]; MAX_N]; MAX_N],
}

impl<const is_solvable: bool> CacheTreeSingle<is_solvable> {
  fn new() -> Self {
    let mut cache: [[[CacheTreeFixed<is_solvable>; MAX_COMPARISONS]; MAX_N]; MAX_N] =
      Default::default();
    let mut mutex: [[[RwLock<CacheTreeFixed<is_solvable>>; MAX_COMPARISONS]; MAX_N]; MAX_N] =
      Default::default();

    for n in 1..MAX_N {
      for k in 0..MAX_N {
        for c in 0..MAX_COMPARISONS {
          cache[n][k][c] = CacheTreeFixed::new(n as u8, k as u8);
          mutex[n][k][c] = RwLock::new(CacheTreeFixed::new(n as u8, k as u8));
        }
      }
    }

    Self { cache, mutex }
  }

  pub fn insert(&mut self, poset: &Poset, remaining_comparisons: u8) {
    assert!(2 * poset.nth_smallest() < poset.n());
    let _lock = self.mutex[poset.n() as usize][poset.nth_smallest() as usize]
      [remaining_comparisons as usize]
      .write()
      .unwrap();
    self.cache[poset.n() as usize][poset.nth_smallest() as usize][remaining_comparisons as usize]
      .insert(poset);
  }

  pub fn check(&self, poset: &Poset, remaining_comparisons: u8) -> bool {
    if is_solvable {
      for c in (0..=remaining_comparisons).rev() {
        let _lock = self.mutex[poset.n() as usize][poset.nth_smallest() as usize][c as usize]
          .read()
          .unwrap();
        if self.cache[poset.n() as usize][poset.nth_smallest() as usize][c as usize].contains(poset)
        {
          return true;
        }
      }
    } else {
      for c in remaining_comparisons..MAX_COMPARISONS as u8 {
        let _lock = self.mutex[poset.n() as usize][poset.nth_smallest() as usize][c as usize]
          .read()
          .unwrap();
        if self.cache[poset.n() as usize][poset.nth_smallest() as usize][c as usize].contains(poset)
        {
          return true;
        }
      }
    }
    false
  }

  pub fn size(&self) -> usize {
    let mut sum = 0;
    for n in 1..MAX_N {
      for i in 0..MAX_N {
        for c in 0..MAX_COMPARISONS {
          let _lock = self.mutex[n][i][c].write().unwrap();
          sum += self.cache[n][i][c].size();
        }
      }
    }
    sum
  }
}

pub struct CacheTreeDual {
  cache_solvable: CacheTreeSingle<true>,
  cache_not_solvable: CacheTreeSingle<false>,
}

impl CacheTreeDual {
  pub fn new() -> Self {
    Self {
      cache_solvable: CacheTreeSingle::new(),
      cache_not_solvable: CacheTreeSingle::new(),
    }
  }

  pub fn check_not_solvable(&self, poset: &Poset, remaining_comparisons: u8) -> bool {
    debug_assert!(2 * poset.nth_smallest() < poset.n());
    self.cache_not_solvable.check(poset, remaining_comparisons)
  }

  pub fn check_solvable(&self, poset: &Poset, remaining_comparisons: u8) -> bool {
    debug_assert!(2 * poset.nth_smallest() < poset.n());
    self.cache_solvable.check(poset, remaining_comparisons)
  }

  pub fn insert_not_solvable(&mut self, poset: &Poset, remaining_comparisons: u8) {
    debug_assert!(2 * poset.nth_smallest() < poset.n());
    self.cache_not_solvable.insert(poset, remaining_comparisons);
  }

  pub fn insert_solvable(&mut self, poset: &Poset, remaining_comparisons: u8) {
    debug_assert!(2 * poset.nth_smallest() < poset.n());
    self.cache_solvable.insert(poset, remaining_comparisons);
  }

  pub fn size(&self) -> usize {
    self.cache_not_solvable.size() + self.cache_solvable.size()
  }

  pub fn test() {
    let mut poset = Poset::new(10, 2);
    poset.add_less(3, 7);

    let mut poset2 = Poset::new(10, 2);
    poset2.add_less(2, 7);

    let mut cache = CacheTreeDual::new();
    cache.insert_solvable(&poset, 2);

    cache.insert_not_solvable(&poset2, 2);

    debug_assert!(!cache.check_solvable(&poset, 1));
    debug_assert!(cache.check_solvable(&poset, 2));
    debug_assert!(cache.check_solvable(&poset, 3));

    debug_assert!(!cache.check_solvable(&poset2, 1));
    debug_assert!(!cache.check_solvable(&poset2, 2));
    debug_assert!(!cache.check_solvable(&poset2, 3));

    debug_assert!(!cache.check_not_solvable(&poset, 1));
    debug_assert!(!cache.check_not_solvable(&poset, 2));
    debug_assert!(!cache.check_not_solvable(&poset, 3));

    debug_assert!(cache.check_not_solvable(&poset2, 1));
    debug_assert!(cache.check_not_solvable(&poset2, 2));
    debug_assert!(!cache.check_not_solvable(&poset2, 3));

    debug_assert!(cache.cache_solvable.cache[10][2][2].entries().len() == 1);

    println!("CacheTreeDual -- success");
  }
}
