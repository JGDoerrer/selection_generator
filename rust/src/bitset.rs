use std::fmt::Debug;

use crate::poset::MAX_N;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BitSet {
    bits: u16,
}

impl BitSet {
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }

    #[inline]
    pub fn from_u16(bits: u16) -> Self {
        BitSet { bits }
    }

    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    #[inline]
    pub fn single(i: usize) -> Self {
        let mut new = Self::empty();
        new.insert(i);
        new
    }

    #[inline]
    pub fn insert(&mut self, index: usize) {
        debug_assert!(index < MAX_N);
        let bit_mask = 1 << index;

        self.bits |= bit_mask;
    }

    #[inline]
    pub fn remove(&mut self, index: usize) {
        debug_assert!(index < MAX_N);
        let bit_mask = 1 << index;

        self.bits &= !bit_mask;
    }

    #[inline]
    pub fn contains(&self, index: usize) -> bool {
        debug_assert!(index < MAX_N);
        let bit_mask = 1 << index;

        (self.bits & bit_mask) != 0
    }

    #[inline]
    pub fn union(self, other: Self) -> Self {
        BitSet {
            bits: self.bits | other.bits,
        }
    }

    #[inline]
    pub fn intersect(self, other: Self) -> Self {
        BitSet {
            bits: self.bits & other.bits,
        }
    }

    #[inline]
    pub fn complement(self) -> Self {
        const MASK: u16 = ((1u32 << (MAX_N + 1)) - 1) as u16;
        BitSet {
            bits: !self.bits & MASK,
        }
    }

    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.bits & other.bits == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    #[inline]
    pub fn iter(&self) -> BitSetIter {
        BitSetIter {
            bitset: *self,
            index: 0,
        }
    }
}

impl IntoIterator for BitSet {
    type IntoIter = BitSetIter;
    type Item = usize;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BitSetIter {
            bitset: self,
            index: 0,
        }
    }
}

impl From<u16> for BitSet {
    #[inline]
    fn from(bits: u16) -> Self {
        Self::from_u16(bits)
    }
}

pub struct BitSetIter {
    bitset: BitSet,
    index: usize,
}

impl Iterator for BitSetIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.bitset.bits.trailing_zeros() as usize;

        if next < MAX_N {
            self.bitset.remove(next);
            Some(next)
        } else {
            None
        }
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
                &format!("{:08b}", self.bits).chars().collect::<String>(),
            )
            .field(
                "set_bits",
                &(0..MAX_N).filter(|i| self.contains(*i)).collect::<Vec<_>>(),
            )
            .finish()
    }
}
