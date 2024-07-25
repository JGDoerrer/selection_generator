use crate::{
    constants::MAX_N,
    utils::{extend_sorted, hash},
};
use rayon::slice::ParallelSliceMut;
use std::mem::size_of_val;

use crate::backwards_poset::BackwardsPoset;

const HASHTABLE_BITS: usize = 16;
const HASHTABLE_SIZE: usize = 1 << HASHTABLE_BITS;

pub struct CacheBucket {
    hashtable: Vec<usize>,
    data: Vec<(u16, u128, (u8, u8), u8)>, // (hash from encoded poset, encoded poset, indices, comparisons)
}

impl Default for CacheBucket {
    fn default() -> Self {
        let mut result = Self {
            hashtable: vec![0; HASHTABLE_SIZE],
            data: vec![],
        };
        result.hashtable.shrink_to_fit();
        result.data.shrink_to_fit();
        result
    }
}

pub struct BackwardCache {
    buckets: [[CacheBucket; MAX_N]; MAX_N],
}

impl BackwardCache {
    pub fn new() -> Self {
        Self {
            buckets: Default::default(),
        }
    }

    pub fn add_layer(&mut self, posets: &Vec<(BackwardsPoset, (u8, u8))>, comparisons: u8) {
        let mut new_items: [[Vec<(u16, u128, (u8, u8), u8)>; MAX_N]; MAX_N] = Default::default();

        for (poset, indices) in posets {
            let packed = poset.pack_poset();
            new_items[poset.n() as usize - 1][poset.i() as usize].push((
                hash(packed),
                packed,
                *indices,
                comparisons,
            ));
        }

        let comparator =
            |(hash1, key1, _, _): &(u16, u128, (u8, u8), u8),
             (hash2, key2, _, _): &(u16, u128, (u8, u8), u8)| {
                hash1.cmp(hash2).then(key1.cmp(key2))
            };

        for n in 0..MAX_N {
            for i in 0..MAX_N {
                if new_items.is_empty() {
                    continue;
                }

                new_items[n][i].par_sort_unstable_by(comparator);

                extend_sorted(&mut self.buckets[n][i].data, &new_items[n][i], comparator);

                let mut pos = 0;
                for k in 0..HASHTABLE_SIZE {
                    self.buckets[n][i].hashtable[k] = pos;
                    while pos < self.buckets[n][i].data.len()
                        && self.buckets[n][i].data[pos].0 <= k as u16
                    {
                        pos += 1;
                    }
                }
            }
        }
    }

    #[inline]
    pub fn get(&self, poset: &BackwardsPoset) -> Option<(u8, u8)> {
        debug_assert_ne!(poset.n(), 0);

        let packed = poset.pack_poset();
        let hash = hash(packed);
        let bucket = &self.buckets[poset.n() as usize - 1][poset.i() as usize];

        let left = bucket.hashtable[hash as usize];
        let right = if hash as usize + 1 == HASHTABLE_SIZE {
            bucket.data.len()
        } else {
            bucket.hashtable[hash as usize + 1]
        };

        if let Ok(index) = bucket.data[left..right].binary_search_by(|poset| poset.1.cmp(&packed)) {
            Some(bucket.data[left + index].2)
        } else {
            None
        }
    }

    #[inline]
    pub fn contains(&self, poset: &BackwardsPoset) -> bool {
        self.get(poset).is_some()
    }

    pub fn memory_size(&self) -> usize {
        let mut memory_size = 0;
        for bucket_n in &self.buckets {
            for bucket_ni in bucket_n {
                memory_size += size_of_val(&bucket_ni.hashtable) + size_of_val(&bucket_ni.data);
            }
        }
        memory_size
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        for bucket_n in &self.buckets {
            for bucket_ni in bucket_n {
                length += bucket_ni.data.len();
            }
        }
        length
    }
}
