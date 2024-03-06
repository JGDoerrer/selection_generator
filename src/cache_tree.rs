use std::sync::RwLock;

use crate::backwards_poset::BackwardsPoset;
use crate::constants::{MAX_COMPARISONS, MAX_N};

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
        poset: &BackwardsPoset,
        index: usize,
        took_other_path: bool,
    ) -> bool {
        if index == 0 {
            took_other_path
        } else if poset.is_index(index - 1) {
            0 != self.branch_is_less
                && arena[self.branch_is_less].contains_multi_path(
                    arena,
                    poset,
                    index - 1,
                    took_other_path,
                )
                || (IS_SOLVABLE
                    && 0 != self.branch_is_not_less
                    && arena[self.branch_is_not_less].contains_multi_path(
                        arena,
                        poset,
                        index - 1,
                        true,
                    ))
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
                    && arena[self.branch_is_less].contains_multi_path(
                        arena,
                        poset,
                        index - 1,
                        true,
                    ))
        }
    }

    fn contains(
        &self,
        arena: &Vec<CacheNode<IS_SOLVABLE>>,
        poset: &BackwardsPoset,
        index: usize,
    ) -> bool {
        self.contains_multi_path(arena, poset, index, true)
    }
}

#[derive(Default)]
pub struct CacheTreeItem<const IS_SOLVABLE: bool> {
    arena: Vec<CacheNode<IS_SOLVABLE>>,
    size: usize,
}

impl<const IS_SOLVABLE: bool> CacheTreeItem<IS_SOLVABLE> {
    pub fn new(n: u8, i: u8) -> Self {
        Self {
            arena: vec![CacheNode::new()],
            size: 0,
        }
    }

    pub fn insert(&mut self, poset: &BackwardsPoset) {
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

    pub fn contains(&self, poset: &BackwardsPoset) -> bool {
        self.arena[0].contains(&self.arena, poset, poset.adjacency_size())
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
                    mutex[n as usize][i as usize][c as usize] =
                        RwLock::new(CacheTreeItem::new(n, i));
                }
            }
        }

        Self { cache, mutex }
    }

    pub fn insert(&mut self, poset: &BackwardsPoset, remaining_comparisons: u8) {
        debug_assert!(2 * poset.i() < poset.n());
        for i in 0..poset.n() {
            for j in i..poset.n() {
                debug_assert!(!poset.is_less(i, j));
            }
        }
        let _lock = self.mutex[poset.n() as usize][poset.i() as usize]
            [remaining_comparisons as usize]
            .write()
            .unwrap();
        self.cache[poset.n() as usize][poset.i() as usize][remaining_comparisons as usize]
            .insert(poset);
    }

    pub fn check(&self, poset: &BackwardsPoset, remaining_comparisons: u8) -> bool {
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
