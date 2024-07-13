use crate::constants::MAX_N;
use hashbrown::HashMap;
use std::mem::size_of;

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
        hash ^= hash >> 16;
        hash as u16
    }

    pub fn new() -> Self {
        Self {
            buckets: Default::default(),
        }
    }

    fn merge_sorted(
        a: &[(u16, u128, (u8, u8))],
        b: &[(u16, u128, (u8, u8))],
    ) -> Vec<(u16, u128, (u8, u8))> {
        let mut result = Vec::with_capacity(a.len() + b.len());

        let mut it1 = a.iter();
        let mut it2 = b.iter();
        let mut val1 = it1.next();
        let mut val2 = it2.next();

        while let (Some(&v1), Some(&v2)) = (val1, val2) {
            if v1.0.cmp(&v2.0).then(v1.1.cmp(&v2.1)).is_le() {
                result.push(v1);
                val1 = it1.next();
            } else {
                result.push(v2);
                val2 = it2.next();
            }
        }

        if let Some(v1) = val1 {
            result.push(*v1);
            result.extend(it1.copied());
        } else if let Some(v2) = val2 {
            result.push(*v2);
            result.extend(it2.copied());
        }

        result
    }

    pub fn add_layer(&mut self, posets: &HashMap<BackwardsPoset, (u8, u8)>) {
        let mut old_indices: [[usize; MAX_N]; MAX_N] = Default::default();

        for n in 0..MAX_N {
            for i in 0..MAX_N {
                old_indices[n][i] = self.buckets[n][i].data.len();
            }
        }

        for (poset, indices) in posets {
            let packed = poset.pack_poset();
            self.buckets[poset.n() as usize - 1][poset.i() as usize]
                .data
                .push((Self::hash(packed), packed, *indices));
        }

        for n in 0..MAX_N {
            for i in 0..MAX_N {
                let bucket_ni = &mut self.buckets[n][i];
                if bucket_ni.data.is_empty() {
                    continue;
                }

                bucket_ni.data[old_indices[n][i]..].sort_unstable_by(
                    |(hash1, key1, _), (hash2, key2, _)| hash1.cmp(hash2).then(key1.cmp(key2)),
                );
                bucket_ni.data = Self::merge_sorted(
                    &bucket_ni.data[..old_indices[n][i]],
                    &bucket_ni.data[old_indices[n][i]..],
                );
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
