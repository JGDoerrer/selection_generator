use crate::constants::MAX_N;
use hashbrown::HashMap;
use std::{cmp::Ordering, mem::size_of};

use crate::backwards_poset::BackwardsPoset;

const HASHTABLE_BITS: usize = 16;
const HASHTABLE_SIZE: usize = 1 << HASHTABLE_BITS;

pub struct CacheBucket {
    hashtable: Vec<usize>,
    data: Vec<(u16, u128, (u8, u8))>,
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
    fn hash(value: u128) -> u16 {
        let mut hash = value as u64;
        hash ^= (value >> 64) as u64;
        hash ^= hash >> 32;
        (hash >> 16) as u16
    }

    pub fn new() -> Self {
        Self {
            buckets: Default::default(),
        }
    }

    pub fn add_layer(&mut self, posets: &HashMap<BackwardsPoset, (u8, u8)>) {
        for (poset, indices) in posets {
            let packed = poset.pack_poset();
            self.buckets[poset.n() as usize - 1][poset.i() as usize]
                .data
                .push((Self::hash(packed), packed, *indices));
        }

        // let mut n = 0;
        // let mut i: i32;
        for bucket_n in &mut self.buckets {
            // n += 1;
            // i = 0;
            for bucket_ni in bucket_n {
                // i += 1;
                if bucket_ni.data.is_empty() {
                    continue;
                }

                bucket_ni
                    .data
                    .sort_by(|(hash1, key1, _), (hash2, key2, _)| {
                        hash1.cmp(hash2).then(key1.cmp(key2))
                    });
                bucket_ni.data.shrink_to_fit();

                // println!("{n} {i}:");
                let mut pos = 0;
                // let mut old = 0;
                for k in 0..HASHTABLE_SIZE {
                    bucket_ni.hashtable[k] = pos;
                    // if k != 0 && k % 256 == 0 {
                    //     println!("{}: {}", k / 256, pos - old);
                    //     old = pos;
                    // }
                    while pos < bucket_ni.data.len() && bucket_ni.data[pos].0 <= k as u16 {
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
        vec: &[(u16, u128, (u8, u8))],
        poset: u128,
    ) -> Option<(u8, u8)> {
        let mut size = right - left;
        while left < right {
            let mid = left + size / 2;
            let cmp = vec[mid].1.cmp(&poset);
            debug_assert!(vec[mid].0.cmp(&Self::hash(poset)).is_eq());

            if cmp == Ordering::Less {
                left = mid + 1;
            } else if cmp == Ordering::Greater {
                right = mid;
            } else {
                return Some(vec[mid].2);
            }

            size = right - left;
        }

        None
    }

    #[inline]
    pub fn get(&self, poset: &BackwardsPoset) -> Option<(u8, u8)> {
        debug_assert_ne!(poset.n(), 0);

        let packed = poset.pack_poset();
        let hash = Self::hash(packed);
        let bucket = &self.buckets[poset.n() as usize - 1][poset.i() as usize];

        let left = bucket.hashtable[hash as usize];
        let right = if hash as usize + 1 == HASHTABLE_SIZE {
            bucket.data.len()
        } else {
            bucket.hashtable[hash as usize + 1]
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
                    + size_of::<Vec<(u16, u128, (u8, u8))>>()
                    + bucket_ni.data.capacity() * size_of::<(u16, u128, (u8, u8))>();
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
