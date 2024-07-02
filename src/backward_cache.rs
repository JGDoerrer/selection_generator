use crate::constants::MAX_N;
use hashbrown::HashMap;
use std::mem::size_of;

use crate::backwards_poset::BackwardsPoset;

pub struct BackwardCache {
    buckets: Vec<[[HashMap<u128, (u8, u8)>; MAX_N]; MAX_N]>,
}

impl BackwardCache {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, posets: &HashMap<BackwardsPoset, (u8, u8)>) {
        let mut new_bucket: [[HashMap<u128, (u8, u8)>; MAX_N]; MAX_N] = Default::default();
        for (poset, indices) in posets {
            debug_assert_ne!(poset.n(), 0);
            new_bucket[poset.n() as usize - 1][poset.i() as usize]
                .insert(poset.pack_poset(), *indices);
        }
        for n in 0..new_bucket.len() {
            for i in 0..new_bucket[n].len() {
                new_bucket[n][i].shrink_to_fit();
            }
        }
        self.buckets.push(new_bucket);
    }

    pub fn contains(&self, poset: &BackwardsPoset) -> bool {
        debug_assert_ne!(poset.n(), 0);
        let packed = poset.pack_poset();
        self.buckets
            .iter()
            .rev()
            .any(|bucket| bucket[poset.n() as usize - 1][poset.i() as usize].contains_key(&packed))
    }

    pub fn get(&self, poset: &BackwardsPoset) -> (u8, u8) {
        debug_assert_ne!(poset.n(), 0);
        let packed = poset.pack_poset();
        for bucket in self.buckets.iter().rev() {
            if let Some(comparison_pair) =
                bucket[poset.n() as usize - 1][poset.i() as usize].get(&packed)
            {
                return *comparison_pair;
            }
        }
        panic!();
    }

    pub fn memory_size(&self) -> u64 {
        let mut memory_size = 0;
        for k in 0..self.buckets.len() {
            for n in 0..self.buckets[k].len() {
                for i in 0..self.buckets[k][n].len() {
                    memory_size += size_of::<HashMap<u128, (u8, u8)>>()
                        + self.buckets[k][n][i].capacity() * size_of::<(u128, (u8, u8))>();
                }
            }
        }
        memory_size as u64
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        for bucket in &self.buckets {
            for n in 0..bucket.len() {
                for i in 0..bucket[n].len() {
                    length += bucket[n][i].len();
                }
            }
        }
        length
    }
}
