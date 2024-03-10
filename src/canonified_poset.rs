use core::fmt::Debug;

use crate::{bitset::BitSet, constants::MAX_N, normal_poset::NormalPoset, poset::Poset};

/// A partially ordered set with <
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonifiedPoset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    adjacency: [BitSet; (MAX_N + 1) / 2],
}

impl Poset for CanonifiedPoset {
    fn new(n: u8, i: u8) -> Self {
        CanonifiedPoset {
            n,
            i,
            adjacency: [BitSet::empty(); (MAX_N + 1) / 2],
        }
    }

    #[inline]
    fn n(&self) -> u8 {
        self.n
    }

    #[inline]
    fn i(&self) -> u8 {
        self.i
    }

    #[inline]
    fn is_less(&self, i: u8, j: u8) -> bool {
        let (row, column) = Self::get_index(i, j);
        self.adjacency[row].contains(column)
    }

    #[inline]
    fn has_order(&self, i: u8, j: u8) -> bool {
        self.is_less(i, j)
    }

    #[inline]
    fn get_all_greater_than(&self, i: u8) -> BitSet {
        const MASKS: [BitSet; MAX_N] = {
            let mut masks = [BitSet::empty(); MAX_N];

            let mut i = 0;
            while i < MAX_N {
                masks[i] = if i < (MAX_N + 1) / 2 {
                    BitSet::from_u16((((1u32 << (MAX_N - i + 1)) - 1) as u16) << (i + 1))
                } else {
                    BitSet::from_u16(((1u32 << (MAX_N - i + 1)) - 1) as u16)
                };
                i += 1;
            }

            masks
        };

        let i = i as usize;
        let mask = MASKS[i];

        if i < (MAX_N + 1) / 2 {
            let row = i;
            self.adjacency[row].intersect(mask)
        } else {
            let row = MAX_N - i;
            (self.adjacency[row].intersect(mask).bits() << (i + 1)).into()
        }
    }

    fn is_lower_triangle_matrix(&self) -> bool {
        true
    }

    /// returns how many elements are less or greater than it
    #[inline]
    fn calculate_relations(&self) -> ([u8; MAX_N], [u8; MAX_N]) {
        let mut less = [0u8; MAX_N];
        let mut greater = [0u8; MAX_N];

        let all_greater_than = {
            let mut bitsets = [BitSet::empty(); MAX_N];
            bitsets
                .iter_mut()
                .take(self.n as usize)
                .enumerate()
                .for_each(|(i, bs)| *bs = self.get_all_greater_than(i as u8));
            bitsets
        };

        for (i, greater) in greater.iter_mut().enumerate().take(self.n() as usize) {
            *greater = all_greater_than[i].len() as u8;
        }

        for (i, less) in less.iter_mut().enumerate().take(self.n() as usize) {
            let i_bitset = BitSet::single(i);
            for greater_than_j in all_greater_than.into_iter().take(i) {
                *less += !greater_than_j.is_disjoint(i_bitset) as u8;
            }
        }

        (less, greater)
    }

    /// returns a bitset of all elements less than i
    #[inline]
    fn get_all_less_than(&self, i: u8) -> BitSet {
        debug_assert!(i < self.n());

        let mut less_than_i = 0;

        let i_bitset = BitSet::single(i as usize);

        for j in 0..i {
            less_than_i |= self.get_all_greater_than(j).intersect(i_bitset).bits() >> (i - j);
        }

        less_than_i.into()
    }
}

impl CanonifiedPoset {
    #[inline]
    const fn get_index(i: u8, j: u8) -> (usize, usize) {
        debug_assert!(i < j);
        let i = i as usize;
        let j = j as usize;

        let row = if i < (MAX_N + 1) / 2 { i } else { MAX_N - i };
        let column = if i < (MAX_N + 1) / 2 { j } else { j - i - 1 };

        (row, column)
    }

    #[inline]
    pub fn set_is_less(&mut self, i: u8, j: u8) {
        let (row, column) = Self::get_index(i, j);
        self.adjacency[row].insert(column);
    }

    /// returns a clone of the poset, with i < j added
    pub fn with_less(&self, i: u8, j: u8) -> CanonifiedPoset {
        let mut new: NormalPoset = (*self).into();
        new.add_and_close(i, j);
        new.canonified()
    }

    pub fn to_normal(self) -> NormalPoset {
        self.into()
    }

    pub fn get_comparison_pairs(&self) -> Vec<(CanonifiedPoset, CanonifiedPoset, u8, u8)> {
        let mut pairs = Vec::with_capacity(self.n() as usize * (self.n() as usize - 1) / 2);

        for i in 0..self.n() {
            for j in (i + 1)..self.n() {
                if self.has_order(i, j) {
                    continue;
                }

                let less = self.with_less(i, j);
                let greater = self.with_less(j, i);

                let hardness_less = less.estimate_hardness();
                let hardness_greater = greater.estimate_hardness();

                let pair = if hardness_less < hardness_greater {
                    (less, greater, i, j, hardness_greater)
                } else {
                    (greater, less, i, j, hardness_less)
                };

                if pairs
                    .iter()
                    .find(|e: &&(CanonifiedPoset, CanonifiedPoset, u8, u8, u32)| {
                        e.0 == pair.0 && e.1 == pair.1
                    })
                    .is_none()
                {
                    pairs.push(pair);
                }
            }
        }

        pairs.sort_by_key(|pair| pair.4);

        pairs
            .into_iter()
            .map(|(a, b, c, d, _)| (a, b, c, d))
            .collect()
    }
}

impl From<CanonifiedPoset> for NormalPoset {
    fn from(value: CanonifiedPoset) -> Self {
        let mut new_poset = NormalPoset::new(value.n, value.i);
        for i in 0..value.n {
            new_poset.set_all_greater_than(i as usize, value.get_all_greater_than(i));
        }
        new_poset
    }
}

impl Debug for CanonifiedPoset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // nicer debug output
        let adjacency: Vec<String> = (0..self.n)
            .map(|i| {
                (0..self.n)
                    .map(|j| if self.is_less(i, j) { '1' } else { '0' })
                    .collect()
            })
            .collect();

        let all_comparisons: Vec<String> = (0..self.n)
            .flat_map(|i| {
                (0..self.n).flat_map(move |j| {
                    if self.is_less(i, j) {
                        vec![format!("{i} < {j}")]
                    } else {
                        vec![]
                    }
                })
            })
            .collect();

        f.debug_struct("CanonifiedPoset")
            .field("n", &self.n)
            .field("i", &self.i)
            .field("adjacency", &adjacency)
            .field("all_comparisons", &all_comparisons)
            .finish()
    }
}
