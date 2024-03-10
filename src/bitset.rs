use std::fmt::Debug;

use crate::constants::MAX_N;

/// A bitset to store up to MAX_N bits
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BitSet {
    bits: u16,
}

impl BitSet {
    #[inline]
    pub const fn empty() -> Self {
        BitSet { bits: 0 }
    }

    #[inline]
    pub const fn from_u16(bits: u16) -> Self {
        BitSet { bits }
    }

    #[inline]
    pub const fn bits(&self) -> u16 {
        self.bits
    }

    #[allow(unused)]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits == 0
    }

    #[inline]
    pub const fn single(i: usize) -> Self {
        BitSet { bits: 1 << i }
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
    pub const fn contains(&self, index: usize) -> bool {
        debug_assert!(index < MAX_N);
        let bit_mask = 1 << index;

        (self.bits & bit_mask) != 0
    }

    #[inline]
    pub const fn union(&self, other: Self) -> Self {
        BitSet {
            bits: self.bits | other.bits,
        }
    }

    #[inline]
    pub const fn intersect(&self, other: Self) -> Self {
        BitSet {
            bits: self.bits & other.bits,
        }
    }

    #[inline]
    pub const fn complement(&self) -> Self {
        const MASK: u16 = ((1u32 << (MAX_N + 1)) - 1) as u16;
        BitSet {
            bits: !self.bits & MASK,
        }
    }

    #[inline]
    pub const fn is_disjoint(&self, other: Self) -> bool {
        self.bits & other.bits == 0
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }
}

impl IntoIterator for BitSet {
    type IntoIter = BitSetIter;
    type Item = usize;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BitSetIter { bitset: self }
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
}

impl Iterator for BitSetIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.bitset.bits.trailing_zeros() as usize;

        if next < MAX_N {
            // remove first set bit
            self.bitset.bits = (self.bitset.bits - 1) & self.bitset.bits;
            Some(next)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.bitset.len(), Some(self.bitset.len()))
    }
}

impl ExactSizeIterator for BitSetIter {
    #[inline]
    fn len(&self) -> usize {
        self.bitset.len()
    }
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitSet")
            .field(
                "bits",
                &format!("{:016b}", self.bits).chars().collect::<String>(),
            )
            .field(
                "set_bits",
                &(0..MAX_N).filter(|i| self.contains(*i)).collect::<Vec<_>>(),
            )
            .finish()
    }
}
