use std::collections::HashSet;
use std::sync::RwLock;

// #[allow(clippy::wildcard_imports)]
use super::poset::Poset;
use super::util::{MAX_COMPARISONS, MAX_N};

// TODO: ARENA!!!
// TODO: get_index -> order
// TODO: clean

// Idee: 0. Platz "leer"
// anstatt Option<Box<Self>> => u32

struct CacheNode<const IS_SOLVABLE: bool> {
  branch_is_less: Option<Box<Self>>,
  branch_is_not_less: Option<Box<Self>>,
}

impl<const IS_SOLVABLE: bool> CacheNode<IS_SOLVABLE> {
  fn new() -> Self {
    Self {
      branch_is_less: None,
      branch_is_not_less: None,
    }
  }

  fn contains_multi_path(&self, poset: &Poset, index: usize, took_other_path: bool) -> bool {
    if index == 0 {
      took_other_path
    } else if poset.is_index(index - 1) {
      self.branch_is_less.as_ref().map_or(false, |less| {
        less.contains_multi_path(poset, index - 1, took_other_path)
      }) || (IS_SOLVABLE
        && self.branch_is_not_less.as_ref().map_or(false, |not_less| {
          not_less.contains_multi_path(poset, index - 1, true)
        }))
    } else {
      (self.branch_is_not_less.as_ref().map_or(false, |not_less| {
        not_less.contains_multi_path(poset, index - 1, took_other_path)
      }))
        || (!IS_SOLVABLE
          && self.branch_is_less.as_ref().map_or(false, |less| {
            less.contains_multi_path(poset, index - 1, true)
          }))
    }
  }

  fn contains(&self, poset: &Poset, index: usize) -> bool {
    self.contains_multi_path(poset, index, true)
  }

  fn entries(
    &self,
    entries: &mut HashSet<Poset>,
    temp: &mut Poset,
    index: usize,
    root_struct: &Self,
  ) {
    if 0 == index {
      if !root_struct.contains_multi_path(temp, temp.adjacency_size(), false) {
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
pub struct CacheTreeItem<const IS_SOLVABLE: bool> {
  n: u8,
  i: u8,
  root: Option<Box<CacheNode<IS_SOLVABLE>>>,
  size: usize,
}

impl<const IS_SOLVABLE: bool> CacheTreeItem<IS_SOLVABLE> {
  pub fn new(n: u8, i: u8) -> Self {
    Self {
      n,
      i,
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
    let mut level = self.root.as_mut().unwrap();

    for i in (0..poset.adjacency_size()).rev() {
      // TODO: iterator
      last_insert = false;
      if poset.is_index(i) {
        if level.branch_is_less.is_none() {
          level.branch_is_less = Some(Box::new(CacheNode::new()));
          last_insert = true;
        }
        level = level.branch_is_less.as_mut().unwrap();
      } else {
        if level.branch_is_not_less.is_none() {
          level.branch_is_not_less = Some(Box::new(CacheNode::new()));
          last_insert = true;
        }
        level = level.branch_is_not_less.as_mut().unwrap();
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
      .contains(poset, poset.adjacency_size())
  }

  pub fn entries(&self) -> HashSet<Poset> {
    let mut entries = HashSet::new();
    if let Some(root) = self.root.as_ref() {
      let mut temp = Poset::new(self.n, self.i);
      let items = temp.adjacency_size();

      root.entries(&mut entries, &mut temp, items, root);
    }
    entries
  }

  pub fn size(&self) -> usize {
    self.size // TODO: eigentlich falsch => clean & count & entries???
  }
}

pub struct CacheTree<const IS_SOLVABLE: bool> {
  cache: [[[CacheTreeItem<IS_SOLVABLE>; MAX_COMPARISONS]; MAX_N]; MAX_N],
  mutex: [[[RwLock<CacheTreeItem<IS_SOLVABLE>>; MAX_COMPARISONS]; MAX_N]; MAX_N],
}

impl<const IS_SOLVABLE: bool> CacheTree<IS_SOLVABLE> {
  pub fn new() -> Self {
    let mut cache: [[[CacheTreeItem<IS_SOLVABLE>; MAX_COMPARISONS]; MAX_N]; MAX_N] =
      Default::default();
    let mut mutex: [[[RwLock<CacheTreeItem<IS_SOLVABLE>>; MAX_COMPARISONS]; MAX_N]; MAX_N] =
      Default::default();

    for n in 1..MAX_N as u8 {
      for i in 0..MAX_N as u8 {
        for c in 0..MAX_COMPARISONS as u8 {
          cache[n as usize][i as usize][c as usize] = CacheTreeItem::new(n, i);
          mutex[n as usize][i as usize][c as usize] = RwLock::new(CacheTreeItem::new(n, i));
        }
      }
    }

    Self { cache, mutex }
  }

  pub fn insert(&mut self, poset: &Poset, remaining_comparisons: u8) {
    debug_assert!(2 * poset.i() < poset.n());
    for i in 0..poset.n() {
      for j in i..poset.n() {
        debug_assert!(!poset.is_less(i, j));
      }
    }
    let _lock = self.mutex[poset.n() as usize][poset.i() as usize][remaining_comparisons as usize]
      .write()
      .unwrap();
    self.cache[poset.n() as usize][poset.i() as usize][remaining_comparisons as usize]
      .insert(poset);
  }

  pub fn check(&self, poset: &Poset, remaining_comparisons: u8) -> bool {
    debug_assert!(2 * poset.i() < poset.n());
    for i in 0..poset.n() {
      for j in i..poset.n() {
        debug_assert!(!poset.is_less(i, j));
      }
    }
    if IS_SOLVABLE {
      for c in (0..=remaining_comparisons).rev() {
        let _lock = self.mutex[poset.n() as usize][poset.i() as usize][c as usize]
          .read()
          .unwrap();
        if self.cache[poset.n() as usize][poset.i() as usize][c as usize].contains(poset) {
          return true;
        }
      }
    } else {
      for c in remaining_comparisons..MAX_COMPARISONS as u8 {
        let _lock = self.mutex[poset.n() as usize][poset.i() as usize][c as usize]
          .read()
          .unwrap();
        if self.cache[poset.n() as usize][poset.i() as usize][c as usize].contains(poset) {
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

pub type CacheTreeSolvable = CacheTree<true>;

pub type CacheTreeNotSolvable = CacheTree<false>;

#[test]
fn test1() {
  let mut poset = Poset::new(10, 2);
  poset.add_less(3, 7);
  poset.normalize();

  let mut cache_solvable = CacheTreeSolvable::new();
  cache_solvable.insert(&poset, 2);

  assert!(!cache_solvable.check(&poset, 1));
  assert!(cache_solvable.check(&poset, 2));
  assert!(cache_solvable.check(&poset, 3));

  assert!(cache_solvable.cache[10][2][2].entries().len() == 1);
}

#[test]
fn test2() {
  let mut poset = Poset::new(10, 2);
  poset.add_less(2, 7);
  poset.normalize();

  let mut cache_not_solvable = CacheTreeNotSolvable::new();
  cache_not_solvable.insert(&poset, 2);

  assert!(cache_not_solvable.check(&poset, 1));
  assert!(cache_not_solvable.check(&poset, 2));
  assert!(!cache_not_solvable.check(&poset, 3));
}
