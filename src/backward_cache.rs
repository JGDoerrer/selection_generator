use crate::constants::MAX_N;
use hashbrown::HashMap;
use std::{cmp::Ordering, mem::size_of};

use crate::backwards_poset::BackwardsPoset;

const HASHTABLE_BITS: usize = 16;
const HASHTABLE_SIZE: usize = 1 << HASHTABLE_BITS;
const HASHTABLE_SHIFT: usize = 128 - HASHTABLE_BITS;

pub struct CacheBucket {
    hashtable: Vec<usize>,
    data: Vec<(u128, (u8, u8))>,
}

impl Default for CacheBucket {
    fn default() -> Self {
        let mut result = Self {
            hashtable: vec![0; HASHTABLE_SIZE],
            data: vec![],
        };
        result.hashtable.shrink_to_fit();
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

    pub fn add_layer(&mut self, posets: &HashMap<BackwardsPoset, (u8, u8)>) {
        for (poset, indices) in posets {
            self.buckets[poset.n() as usize - 1][poset.i() as usize]
                .data
                .push((poset.pack_poset(), *indices));
        }

        for bucket_n in &mut self.buckets {
            for bucket_ni in bucket_n {
                if bucket_ni.data.is_empty() {
                    continue;
                }

                bucket_ni
                    .data
                    .sort_by(|(key1, _), (key2, _)| key1.cmp(key2));
                bucket_ni.data.shrink_to_fit();

                let mut pos = 0;
                for k in 0..HASHTABLE_SIZE {
                    bucket_ni.hashtable[k] = pos;
                    while pos < bucket_ni.data.len()
                        && (bucket_ni.data[pos].0 >> HASHTABLE_SHIFT) <= k as u128
                    {
                        pos += 1;
                    }
                }
            }
        }
    }

    #[inline]
    fn binary_search_by(
        mut left: usize,
        mut right: usize,
        vec: &[(u128, (u8, u8))],
        poset: u128,
    ) -> Option<(u8, u8)> {
        let mut size = right - left;
        while left < right {
            let mid = left + size / 2;
            let cmp = vec[mid].0.cmp(&poset);

            if cmp == Ordering::Less {
                left = mid + 1;
            } else if cmp == Ordering::Greater {
                right = mid;
            } else {
                return Some(vec[mid].1);
            }

            size = right - left;
        }

        None
    }

    #[inline]
    pub fn get(&self, poset: &BackwardsPoset) -> Option<(u8, u8)> {
        debug_assert_ne!(poset.n(), 0);

        let packed = poset.pack_poset();
        let bucket = &self.buckets[poset.n() as usize - 1][poset.i() as usize];
        let prefix = packed >> HASHTABLE_SHIFT;

        let left = bucket.hashtable[prefix as usize];
        let right = if prefix as usize + 1 == HASHTABLE_SIZE {
            bucket.data.len()
        } else {
            bucket.hashtable[prefix as usize + 1]
        };

        Self::binary_search_by(left, right, &bucket.data, packed)
    }

    #[inline]
    pub fn contains(&self, poset: &BackwardsPoset) -> bool {
        self.get(poset).is_some()
    }

    pub fn memory_size(&self) -> usize {
        let mut memory_size = 0;
        for bucket_n in &self.buckets {
            for bucket_ni in bucket_n {
                memory_size += size_of::<Vec<usize>>()
                    + bucket_ni.hashtable.len() * size_of::<usize>()
                    + size_of::<Vec<(u128, (u8, u8))>>()
                    + bucket_ni.data.capacity() * size_of::<(u128, (u8, u8))>();
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
