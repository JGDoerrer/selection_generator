use std::fmt::Debug;

use crate::poset::MAX_N;

// Sadly i can't calculate the number of bytes needed at compile time
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitSet {
    bytes: [u8; Self::BYTES],
}

impl Default for BitSet {
    fn default() -> Self {
        BitSet {
            bytes: [0; Self::BYTES],
        }
    }
}

impl BitSet {
    const BYTES: usize = (MAX_N + 7) / 8;

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn single(i: usize) -> Self {
        let mut new = Self::empty();
        new.insert(i);
        new
    }

    pub fn insert(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_mask = 1 << (index % 8);

        self.bytes[byte_index] |= bit_mask;
    }

    pub fn contains(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_mask = 1 << (index % 8);

        (self.bytes[byte_index] & bit_mask) > 0
    }

    pub fn union(&self, other: Self) -> Self {
        let mut bytes = self.bytes.clone();
        bytes
            .iter_mut()
            .zip(other.bytes.iter())
            .for_each(|(a, b)| *a |= b);
        BitSet { bytes }
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.bytes
            .iter()
            .zip(other.bytes.iter())
            .all(|(a, b)| a & b == 0)
    }

    pub fn iter(&self) -> BitSetIter {
        BitSetIter {
            bitset: self.clone(),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.iter().map(|b| b.count_ones() as usize).sum()
    }
}
pub struct BitSetIter {
    bitset: BitSet,
    index: usize,
}

impl Iterator for BitSetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index..MAX_N {
            if self.bitset.contains(i) {
                self.index = i + 1;
                return Some(i);
            }
        }
        self.index = MAX_N;

        None
    }

    fn all<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        for i in self.index..MAX_N {
            if self.bitset.contains(i) && !f(i) {
                return false;
            }
        }
        true
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        (self.index..MAX_N)
            .filter(|i| self.bitset.contains(*i))
            .count()
    }
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitSet")
            .field(
                "bits",
                &self
                    .bytes
                    .iter()
                    .map(|byte| format!("{byte:08b}").chars().collect::<String>())
                    .collect::<String>(),
            )
            .field(
                "set_bits",
                &(0..MAX_N).filter(|i| self.contains(*i)).collect::<Vec<_>>(),
            )
            .finish()
    }
}
