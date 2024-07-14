use crate::constants::MAX_N;
use hashbrown::HashMap;
use rayon::slice::ParallelSliceMut;
use std::{cmp::Ordering, mem::size_of};

use crate::backwards_poset::BackwardsPoset;

fn extend_sorted<T, F>(a: &mut Vec<T>, b: &[T], mut compare: F)
where
    T: Copy + Clone + Default,
    F: FnMut(&T, &T) -> Ordering,
{
    if a.is_empty() {
        a.extend(b);
        return;
    }

    let mut a_pos: i32 = a.len() as i32 - 1;

    let mut b_iter = b.iter().rev();
    let mut b_value = b_iter.next();

    a.resize(a.len() + b.len(), Default::default());

    let mut i = a.len();
    while 0 <= a_pos && b_value.is_some() {
        debug_assert!(0 < i);
        i -= 1;

        let b_val = b_value.unwrap();
        let a_val = &a[a_pos as usize];

        a[i] = if compare(a_val, b_val).is_le() {
            b_value = b_iter.next();
            *b_val
        } else {
            a_pos -= 1;
            *a_val
        };
    }

    while let Some(b_value_next) = b_value {
        debug_assert!(0 < i);
        i -= 1;

        a[i] = *b_value_next;
        b_value = b_iter.next();
    }
}

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
        hash ^= hash >> 16;
        hash as u16
    }

    pub fn new() -> Self {
        Self {
            buckets: Default::default(),
        }
    }

    pub fn add_layer(&mut self, posets: &HashMap<BackwardsPoset, (u8, u8)>) {
        let mut new_items: [[CacheBucket; MAX_N]; MAX_N] = Default::default();

        for (poset, indices) in posets {
            let packed = poset.pack_poset();
            new_items[poset.n() as usize - 1][poset.i() as usize]
                .data
                .push((Self::hash(packed), packed, *indices));
        }

        let comparator = |(hash1, key1, _): &(u16, u128, (u8, u8)),
                          (hash2, key2, _): &(u16, u128, (u8, u8))| {
            hash1.cmp(hash2).then(key1.cmp(key2))
        };

        for n in 0..MAX_N {
            for i in 0..MAX_N {
                if new_items.is_empty() {
                    continue;
                }

                new_items[n][i].data.par_sort_unstable_by(comparator);

                let bucket_ni = &mut self.buckets[n][i];
                extend_sorted(&mut bucket_ni.data, &new_items[n][i].data, comparator);
                bucket_ni.data.shrink_to_fit();

                let mut pos = 0;
                for k in 0..HASHTABLE_SIZE {
                    bucket_ni.hashtable[k] = pos;
                    while pos < bucket_ni.data.len() && bucket_ni.data[pos].0 <= k as u16 {
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
        let hash = Self::hash(packed);
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
