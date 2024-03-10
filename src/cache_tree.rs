use std::collections::HashSet;
use std::sync::RwLock;

use super::poset::Poset;
use super::util::{MAX_COMPARISONS, MAX_N};

// TODO: get_index -> order
// TODO: clean

struct CacheNode<const IS_SOLVABLE: bool> {
  branch_is_less: usize,
  branch_is_not_less: usize,
}

impl<const IS_SOLVABLE: bool> CacheNode<IS_SOLVABLE> {
  fn new() -> Self {
    Self {
      branch_is_less: 0,
      branch_is_not_less: 0,
    }
  }

  fn contains_multi_path(
    &self,
    arena: &Vec<CacheNode<IS_SOLVABLE>>,
    poset: &Poset,
    index: usize,
    took_other_path: bool,
  ) -> bool {
    if index == 0 {
      took_other_path
    } else if poset.is_index(index - 1) {
      0 != self.branch_is_less
        && arena[self.branch_is_less].contains_multi_path(arena, poset, index - 1, took_other_path)
        || (IS_SOLVABLE
          && 0 != self.branch_is_not_less
          && arena[self.branch_is_not_less].contains_multi_path(arena, poset, index - 1, true))
    } else {
      0 != self.branch_is_not_less
        && arena[self.branch_is_not_less].contains_multi_path(
          arena,
          poset,
          index - 1,
          took_other_path,
        )
        || (!IS_SOLVABLE
          && 0 != self.branch_is_less
          && arena[self.branch_is_less].contains_multi_path(arena, poset, index - 1, true))
    }
  }

  fn contains(&self, arena: &Vec<CacheNode<IS_SOLVABLE>>, poset: &Poset, index: usize) -> bool {
    self.contains_multi_path(arena, poset, index, true)
  }

  fn entries(
    &self,
    arena: &Vec<CacheNode<IS_SOLVABLE>>,
    entries: &mut HashSet<Poset>,
    temp: &mut Poset,
    index: usize,
  ) {
    if 0 == index {
      if !arena[0].contains_multi_path(arena, temp, temp.adjacency_size(), false) {
        entries.insert(temp.clone());
      }
    } else if 0 != index {
      if 0 != self.branch_is_less {
        temp.set_index(index - 1, true);
        arena[self.branch_is_less].entries(arena, entries, temp, index - 1);
      }
      if 0 != self.branch_is_not_less {
        temp.set_index(index - 1, false);
        arena[self.branch_is_not_less].entries(arena, entries, temp, index - 1);
      }
    }
  }
}

#[derive(Default)]
pub struct CacheTreeItem<const IS_SOLVABLE: bool> {
  n: u8,
  i: u8,
  arena: Vec<CacheNode<IS_SOLVABLE>>,
  size: usize,
}

impl<const IS_SOLVABLE: bool> CacheTreeItem<IS_SOLVABLE> {
  pub fn new(n: u8, i: u8) -> Self {
    Self {
      n,
      i,
      arena: vec![CacheNode::new()],
      size: 0,
    }
  }

  pub fn reset(&mut self) {
    self.arena = vec![CacheNode::new()];
    self.size = 0;
  }

  pub fn insert(&mut self, poset: &Poset) {
    let mut last_insert = false;
    let mut level = 0;

    if self.arena.capacity() < 100 {
      self.arena.reserve(10000);
    }

    for i in (0..poset.adjacency_size()).rev() {
      // TODO: iterator
      last_insert = false;
      if poset.is_index(i) {
        if 0 == self.arena[level].branch_is_less {
          self.arena.push(CacheNode::new());
          self.arena[level].branch_is_less = self.arena.len() - 1;
          last_insert = true;
        }
        level = self.arena[level].branch_is_less;
      } else {
        if 0 == self.arena[level].branch_is_not_less {
          self.arena.push(CacheNode::new());
          self.arena[level].branch_is_not_less = self.arena.len() - 1;
          last_insert = true;
        }
        level = self.arena[level].branch_is_not_less;
      }
    }

    if last_insert {
      self.size += 1;
    }
  }

  pub fn contains(&self, poset: &Poset) -> bool {
    self.arena[0].contains(&self.arena, poset, poset.adjacency_size())
  }

  pub fn entries(&self) -> HashSet<Poset> {
    // TODO: as iterator
    let mut entries = HashSet::new();
    let mut temp = Poset::new(self.n, self.i);
    let items = temp.adjacency_size();

    self.arena[0].entries(&self.arena, &mut entries, &mut temp, items);
    entries
  }

  pub fn reheap(&mut self) {
    let entries = self.entries();
    self.reset();
    for poset in entries {
      self.insert(&poset);
    }
  }

  pub fn size(&self) -> usize {
    self.size
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

  pub fn reheap(&mut self) {
    for n in 1..MAX_N {
      for i in 0..MAX_N {
        for c in 0..MAX_COMPARISONS {
          let _lock = self.mutex[n][i][c].write().unwrap();
          self.cache[n][i][c].reheap();
        }
      }
    }
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
