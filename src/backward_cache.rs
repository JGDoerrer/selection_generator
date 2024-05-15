use crate::{pseudo_canonified_poset::PseudoCanonifiedPoset, constants::MAX_N, poset::Poset};
use hashbrown::HashMap;
use std::mem::size_of;

use crate::backwards_poset::BackwardsPoset;

pub struct BackwardCache {
    buckets: Vec<[[HashMap<PseudoCanonifiedPoset, (u8, u8)>; MAX_N]; MAX_N]>, // TODO: CanonifiedPoset
}

impl BackwardCache {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }

    fn to_canonified(poset: &BackwardsPoset) -> PseudoCanonifiedPoset {
        let mut canonified = PseudoCanonifiedPoset::new(poset.n(), poset.i());
        for i in 0..poset.n() {
            for j in 0..poset.n() {
                if poset.is_less(i, j) {
                    canonified.set_is_less(j, i);
                }
            }
        }
        canonified
    }

    pub fn add_layer(&mut self, posets: &HashMap<BackwardsPoset, (u8, u8)>) {
        if !self.buckets.is_empty() {
            let k = self.buckets.len() - 1;
            for n in 0..self.buckets[k].len() {
                for i in 0..self.buckets[k][n].len() {
                    self.buckets[k][n][i].shrink_to_fit();
                }
            }
        }
        let mut new_bucket: [[HashMap<PseudoCanonifiedPoset, (u8, u8)>; MAX_N]; MAX_N] =
            Default::default();
        for (poset, indices) in posets {
            new_bucket[poset.n() as usize][poset.i() as usize]
                .insert(Self::to_canonified(poset), *indices);
        }
        self.buckets.push(new_bucket);
    }

    pub fn contains(&self, poset: &BackwardsPoset) -> bool {
        let canonified = Self::to_canonified(poset);
        self.buckets
            .iter()
            .rev()
            .any(|bucket| bucket[poset.n() as usize][poset.i() as usize].contains_key(&canonified))
    }

    pub fn get(&self, poset: &BackwardsPoset) -> (u8, u8) {
        let canonified = Self::to_canonified(poset);
        for bucket in self.buckets.iter().rev() {
            if let Some(comparison_pair) =
                bucket[poset.n() as usize][poset.i() as usize].get(&canonified)
            {
                return *comparison_pair;
            }
        }
        panic!();
    }

    pub fn memory_size(&self) -> f64 {
        let mut memory_size = 0;
        for k in 0..self.buckets.len() {
            for n in 0..self.buckets[k].len() {
                for i in 0..self.buckets[k][n].len() {
                    memory_size += size_of::<HashMap<PseudoCanonifiedPoset, (u8, u8)>>()
                        + self.buckets[k][n][i].capacity()
                            * size_of::<(PseudoCanonifiedPoset, (u8, u8))>();
                }
            }
        }
        memory_size as f64 / ((10usize).pow(9) as f64)
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
