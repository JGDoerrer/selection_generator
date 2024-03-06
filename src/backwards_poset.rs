use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::mem::swap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt::Debug, os::raw::c_int};

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};

use crate::bitset::BitSet;
use crate::constants::{MAX_N, ONLY_NAUTY_CANONIFY};

pub type PosetIndex = u8;

const fn init_table() -> [([(u8, u8); MAX_N * MAX_N], usize); MAX_N] {
    let mut table1 = [([(0u8, 0u8); MAX_N * MAX_N], 0); MAX_N];
    table1[0] = ([(0, 0); MAX_N * MAX_N], 0);
    table1[1] = ([(0, 0); MAX_N * MAX_N], 1);
    let mut n = 2;
    while n < MAX_N {
        table1[n].1 = (n * n - n) / 2;
        let mut pos = 0;
        while pos < table1[n].1 {
            let mut a = 0;
            let mut k = 0;
            while k < MAX_N {
                if pos < (k * k + k) / 2 {
                    break;
                }
                a = k;
                k += 1;
            }
            let b: usize = pos - ((a * a + a) / 2);
            table1[n].0[pos] = ((a + 1) as u8, b as u8);
            pos += 1;
        }
        n += 1;
    }
    table1
}
const TABLE_ORDER: [([(u8, u8); MAX_N * MAX_N], usize); MAX_N] = init_table();

/// A partially ordered set with <
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackwardsPoset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    adjacency: [BitSet; MAX_N],
}

// impl Hash for Poset {
//   fn hash<H: Hasher>(&self, ra_expand_state: &mut H) {
//     // self.n.hash(ra_expand_state);
//     // self.i.hash(ra_expand_state);
//     self.adjacency.hash(ra_expand_state);
//   }
// }

impl Default for BackwardsPoset {
    fn default() -> Self {
        BackwardsPoset::new(0, 0)
    }
}

impl BackwardsPoset {
    pub fn new(n: u8, i: u8) -> Self {
        debug_assert!(n < MAX_N as u8);

        BackwardsPoset {
            n,
            i,
            adjacency: [BitSet::empty(); MAX_N],
        }
    }

    #[inline]
    pub fn n(&self) -> u8 {
        self.n
    }

    #[inline]
    pub fn i(&self) -> u8 {
        self.i
    }

    #[inline]
    fn set_bit(&mut self, i: PosetIndex, j: PosetIndex) {
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);

        self.adjacency[i as usize].insert(j as usize);
    }

    /// is i < j?
    #[inline]
    pub fn is_less(&self, i: PosetIndex, j: PosetIndex) -> bool {
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);

        self.adjacency[i as usize].contains(j as usize)
    }

    /// returns a bitset of all elements greater than i
    pub fn get_all_greater_than(&self, i: PosetIndex) -> BitSet {
        debug_assert!(i < self.n);
        self.adjacency[i as usize]
    }

    /// returns a bitset of all elements less than i
    pub fn get_all_less_than(&self, i: PosetIndex) -> BitSet {
        debug_assert!(i < self.n);

        let mut less_than_i = 0;

        for j in 0..self.n {
            less_than_i |= (self
                .get_all_greater_than(j)
                .intersect(BitSet::single(i as usize))
                .bits()
                >> i)
                << j;
        }

        less_than_i.into()
    }

    /// is either i < j or j < i?
    #[inline]
    pub fn has_order(&self, i: PosetIndex, j: PosetIndex) -> bool {
        self.is_less(i, j) || self.is_less(j, i)
    }

    /// returns how many elements are less or greater than it
    #[inline]
    pub fn calculate_relations(&self) -> ([u8; MAX_N], [u8; MAX_N]) {
        let mut less = [0u8; MAX_N];
        let mut greater = [0u8; MAX_N];

        for (i, greater) in greater.iter_mut().enumerate().take(self.n as usize) {
            *greater = self.get_all_greater_than(i as u8).len() as u8;
        }

        for (i, less) in less.iter_mut().enumerate().take(self.n as usize) {
            let i_bitset = BitSet::single(i);
            for j in 0..self.n {
                *less += (!self.get_all_greater_than(j).is_disjoint(i_bitset)) as u8;
            }
        }

        (less, greater)
    }

    #[inline]
    fn hash(a: u64, b: u64) -> u64 {
        let mut hash: u64 = 9118271012717746669;

        hash = hash.wrapping_add(a);
        hash = hash.wrapping_mul(5878307119);
        hash ^= hash.wrapping_shl(7);
        hash = hash.wrapping_add(b);
        hash = hash.wrapping_mul(7311227577);
        hash ^= hash.wrapping_shl(9);
        hash = hash.wrapping_add(2072583677);

        hash
    }

    fn canonify(&mut self) {
        self.reduce_elements();
        self.canonify_mapping();
    }

    /// Canonifies the poset and returns a mapping from old to new indices, since they shift around
    fn canonify_mapping(&mut self) -> [PosetIndex; MAX_N] {
        let n = self.n as usize;

        let mut ordered_with_subsets = [BitSet::empty(); MAX_N];

        let mut in_out_degree = [0; MAX_N];

        for i in 0..n {
            let greater = self.get_all_greater_than(i as u8);
            let less = self.get_all_less_than(i as u8);

            ordered_with_subsets[i] = greater.union(less);
            in_out_degree[i] = greater.len() as u64 * MAX_N as u64 + less.len() as u64;
        }

        let mut hash = in_out_degree;

        for _ in 0..2 {
            let mut sum_hash = hash;

            for i in 0..n {
                // sum hashes of neighbours
                for j in ordered_with_subsets[i] {
                    sum_hash[i] = sum_hash[i].wrapping_add(hash[j]);
                }
            }

            // calc new hash based on neighbours hashes
            for i in 0..self.n as usize {
                hash[i] = Self::hash(sum_hash[i], in_out_degree[i]);
            }
        }

        let mut new_indices = [0; MAX_N];
        for i in 0..self.n {
            new_indices[i as usize] = i;
        }

        new_indices[0..n].sort_by(|a, b| {
            in_out_degree[*a as usize]
                .cmp(&in_out_degree[*b as usize])
                .then(hash[*a as usize].cmp(&hash[*b as usize]))
        });

        let mut new = BackwardsPoset::new(self.n, self.i);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize], new_indices[j as usize]) {
                    new.set_bit(i, j);
                }
            }
        }

        // dbg!(&self, &new);
        debug_assert!(new.is_closed(), "{new:?}");
        *self = new;

        new_indices
    }

    /// Removes elements, that are known to be too large/small
    fn reduce_elements(&mut self) -> [PosetIndex; MAX_N] {
        // can the element be ignored, because it is too large/small
        let mut dropped = [false; MAX_N];
        let mut n_less_dropped = 0;

        let (less, greater) = self.calculate_relations();

        for i in 0..self.n as usize {
            if greater[i] > self.i {
                dropped[i] = true;
            } else if less[i] >= self.n - self.i {
                dropped[i] = true;
                n_less_dropped += 1;
            }
        }

        // maps the old indices to the new ones
        let mut new_indices = [0; MAX_N];
        let mut new_n = 0;
        let mut b = self.n as usize - 1;

        for i in 0..self.n {
            if dropped[i as usize] {
                new_indices[b] = i;
                b -= 1;
            } else {
                new_indices[new_n] = i;
                new_n += 1;
            }
        }

        if self.n == new_n as u8 {
            return new_indices;
        }

        let mut new = BackwardsPoset::new(new_n as u8, self.i - n_less_dropped);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize], new_indices[j as usize]) {
                    new.set_bit(i, j);
                }
            }
        }

        if new.i > new.n / 2 {
            new = new.dual();
        }

        // dbg!(&self, &new);
        debug_assert!(new.is_closed(), "{new:?}");
        *self = new;

        new_indices
    }

    fn canonify_nauty(&mut self) {
        let n = self.n as usize;

        let mut options = optionblk {
            getcanon: TRUE,
            defaultptn: FALSE,
            digraph: TRUE,
            ..Default::default()
        };
        let mut stats = statsblk::default();

        let mut labels: [c_int; 64] = (0..64 as c_int).collect::<Vec<_>>().try_into().unwrap();

        let mut ptn = [c_int::from(1); 64];
        ptn[n - 1] = 0;
        let mut zeroes2 = [c_int::from(0); 64];

        // use nauty_Traces_sys::bit as bitmask for the adjacency matrix.
        // E.g. (g[i] & bit[j]) != 0 checks whether there is an edge i -> j.
        let mut dg = [0; 64];
        for (i, mask) in dg.iter_mut().enumerate().take(n) {
            for j in 0..n {
                if self.is_less(i as u8, j as u8) {
                    *mask |= nauty_Traces_sys::bit[j];
                }
            }
        }

        let mut canonical = [0; 64];

        unsafe {
            densenauty(
                dg.as_mut_ptr(),
                labels.as_mut_ptr(),
                ptn.as_mut_ptr(),
                zeroes2.as_mut_ptr(),
                &mut options,
                &mut stats,
                1 as c_int,
                n as c_int,
                canonical.as_mut_ptr(),
            );
        }

        let mut new = BackwardsPoset::new(self.n, self.i);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if canonical[i as usize] & nauty_Traces_sys::bit[j as usize] != 0 {
                    new.set_bit(i, j)
                }
            }
        }

        // dbg!(&self, &new);
        // dbg!(labels);
        *self = new;
    }

    fn canonify_lower_matrix(&self) -> BackwardsPoset {
        let mut new_indices = [0; MAX_N];
        let mut next_free = 0;
        let mut indices_used = BitSet::empty();

        let mut new = BackwardsPoset::new(self.n, self.i);

        while indices_used != BitSet::from_u16(((1u32 << self.n) - 1) as u16) {
            for i in 0..self.n {
                if indices_used.contains(i as usize) {
                    continue;
                }

                let less_than_i = self.get_all_less_than(i);

                if less_than_i.is_empty() || less_than_i.intersect(indices_used) == less_than_i {
                    new_indices[next_free] = i;
                    next_free += 1;
                    indices_used.insert(i as usize);
                }
            }
        }

        new_indices = {
            let mut mapping = [0; MAX_N];
            for i in 0..self.n {
                mapping[new_indices[i as usize] as usize] = i;
            }
            mapping
        };

        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(i, j) {
                    new.set_bit(new_indices[i as usize], new_indices[j as usize])
                }
            }
        }

        debug_assert!(new.is_lower_triangle_matrix());
        new
    }

    /// for debugging
    fn is_lower_triangle_matrix(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) && i > j {
                    return false;
                }
            }
        }
        true
    }

    /// for debugging
    fn is_closed(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    if self.is_less(i, j) && self.is_less(j, k) && !self.is_less(i, k) {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// adds i < j to the poset and normalize
    #[inline]
    pub fn add_less(&mut self, i: PosetIndex, j: PosetIndex) {
        debug_assert!(!self.is_less(i, j));
        debug_assert!(!self.is_less(j, i));

        self.add_and_close(i, j);
        self.canonify();

        debug_assert!(self.is_closed(), "{self:?}");
    }

    /// adds i < j and makes sure, that i < j && j < k => i < k is true
    #[inline]
    pub fn add_and_close(&mut self, i: PosetIndex, j: PosetIndex) {
        let mut stack = Vec::with_capacity(self.n as usize);
        stack.push((i, j));

        while let Some((i, j)) = stack.pop() {
            self.set_bit(i, j);

            for k in self
                .get_all_greater_than(j)
                .intersect(self.get_all_greater_than(i).complement())
            {
                stack.push((i, k as u8));
            }

            for k in self
                .get_all_less_than(i)
                .intersect(self.get_all_less_than(j).complement())
            {
                stack.push((k as u8, j));
            }
        }
    }

    /// returns a clone of the poset, with i < j added
    pub fn with_less(&self, i: PosetIndex, j: PosetIndex) -> Self {
        let mut new = *self;
        new.add_less(i, j);
        new
    }

    /// returns a clone of the poset, with i < j added
    pub fn with_less_mapping(&self, i: PosetIndex, j: PosetIndex) -> (Self, [PosetIndex; MAX_N]) {
        let mut new = *self;

        new.add_and_close(i, j);
        let mapping = new.canonify_mapping();

        (new, mapping)
    }

    /// Assumes self is normalized
    pub fn dual(&self) -> Self {
        let mut dual = BackwardsPoset::new(self.n, self.n - self.i - 1);
        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    dual.set_bit(j, i);
                }
            }
        }

        dual
    }

    pub fn num_compatible_posets(&self) -> usize {
        let canonified = self.canonify_lower_matrix();

        let mut sum = 0;
        for i in 0..canonified.n {
            // assume the ith element is the solution

            let less_than_i = canonified.get_all_less_than(i);
            let greater_than_i = canonified.get_all_greater_than(i);

            let mut less_subsets = Vec::with_capacity(1000);
            less_subsets.push(BitSet::empty());

            for j in 0..canonified.n {
                if j == i || greater_than_i.contains(j as usize) {
                    continue;
                }
                // try adding j to all previous subsets
                let less_than_j = canonified.get_all_less_than(j);

                // try adding j to all previous subsets
                if less_than_i.contains(j as usize) {
                    // all subsets must contain j to be valid
                    for subset in &mut less_subsets {
                        subset.insert(j as usize);
                    }
                } else {
                    for i in 0..less_subsets.len() {
                        let subset = less_subsets[i];

                        // test if adding j would make a valid subset
                        // we know, that there is no k with p[k] > p[j]
                        if less_than_j.intersect(subset) == less_than_j {
                            let mut new_subset = subset;
                            new_subset.insert(j as usize);
                            less_subsets.push(new_subset);
                        }
                    }
                }
            }

            sum += less_subsets
                .into_iter()
                .filter(|s| s.len() == canonified.i as usize)
                .count();
        }

        sum
    }

    pub fn num_compatible_posets_upper_bound(&self) -> usize {
        let canonified = self.canonify_lower_matrix();

        let mut sum = 0;
        for i in 0..canonified.n {
            // assume the ith element is the solution

            let less_than_i = canonified.get_all_less_than(i);
            let greater_than_i = canonified.get_all_greater_than(i);

            let mut counts = [0; MAX_N];
            counts[0] = 1;

            for j in 0..canonified.n {
                if j == i || greater_than_i.contains(j as usize) {
                    continue;
                }

                // try adding j to all previous subsets
                if less_than_i.contains(j as usize) {
                    // all subsets must contain j to be valid
                    for i in (1..=canonified.i as usize).rev() {
                        counts[i] = counts[i - 1];
                    }
                    counts[0] = 0;
                } else {
                    for i in (1..=canonified.i as usize).rev() {
                        counts[i] += counts[i - 1];
                    }
                }
            }

            sum += counts[canonified.i as usize];
        }

        sum
    }

    pub fn set_less(&mut self, i: u8, j: u8, value: bool) {
        if value {
            self.adjacency[i as usize].insert(j as usize);
        } else {
            self.adjacency[i as usize].remove(j as usize);
        }
    }

    pub fn adjacency_size(&self) -> usize {
        TABLE_ORDER[self.n as usize].1
    }

    pub fn is_index(&self, pos: usize) -> bool {
        let item = TABLE_ORDER[self.n as usize].0[pos];
        self.is_less(item.0, item.1)
    }

    pub fn set_index(&mut self, pos: usize, value: bool) {
        let item = TABLE_ORDER[self.n as usize].0[pos];
        self.set_less(item.0, item.1, value);
    }

    // add
    fn add_and_close_recursive(&mut self, i: u8, j: u8) {
        self.set_less(i, j, true);

        for k in 0..self.n {
            if i != k && j != k {
                if self.is_less(j, k) && !self.is_less(i, k) {
                    self.add_and_close_recursive(i, k);
                } else if self.is_less(k, i) && !self.is_less(k, j) {
                    self.add_and_close_recursive(k, j);
                }
            }
        }
    }

    pub fn add_less2(&mut self, i: u8, j: u8) {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed2());
        // TODO: could also add assert !is_less(i, j)

        if !self.is_less(i, j) {
            self.add_and_close_recursive(i, j);
        }

        // postcondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(!self.is_less(j, i));
        debug_assert!(self.is_less(i, j));
        // debug_assert!(self.is_closed2());
    }

    pub fn with_less2(&self, i: u8, j: u8) -> BackwardsPoset {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed2());

        let mut poset = self.clone();
        poset.add_less2(i, j);

        // postcondition
        debug_assert!(poset.i < poset.n);
        debug_assert!((poset.n as usize) < MAX_N);
        debug_assert!(!poset.is_less(j, i));
        debug_assert!(poset.is_less(i, j));
        // debug_assert!(poset.is_closed2());
        debug_assert!(!poset.is_redundant(i, j));

        poset
    }

    pub fn with_less_normalized(&self, i: u8, j: u8) -> BackwardsPoset {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed2());

        let mut poset = self.clone();
        poset.add_less2(i, j);
        poset.normalize();

        // postcondition
        debug_assert!(poset.i < poset.n);
        debug_assert!((poset.n as usize) < MAX_N);
        // debug_assert!(poset.is_closed2());
        // debug_assert!(poset.is_normalized());

        poset
    }

    // reduce
    pub fn calculate_relations2(&self) -> ([u8; MAX_N], [u8; MAX_N]) {
        // TODO: warum so kompliziert in main?
        let mut less = [0u8; MAX_N];
        let mut greater = [0u8; MAX_N];

        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    less[j as usize] += 1;
                    greater[i as usize] += 1;
                }
            }
        }

        (less, greater)
    }

    fn swap_positions(&mut self, i0: u8, j0: u8, i1: u8, j1: u8) {
        let temp = self.is_less(i0, j0);
        self.set_less(i0, j0, self.is_less(i1, j1));
        self.set_less(i1, j1, temp);
    }

    fn swap(&mut self, i: u8, j: u8) {
        for k in 0..self.n {
            if i != k && j != k {
                self.swap_positions(i, k, j, k);
                self.swap_positions(k, i, k, j);
            }
        }
    }

    fn dual2(&mut self) {
        self.i = (self.n - 1) - self.i;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                self.swap_positions(i, j, j, i);
            }
        }
    }

    pub fn reduce_elements2(&mut self) {
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);

        let (less, greater) = self.calculate_relations2(); //TODO: in normal: less and greater swapped

        let mut new_indices = [0u8; MAX_N];
        let mut n_less_dropped = 0;
        let mut new_n = 0u8;
        let mut b = (self.n - 1) as usize;

        for i in 0..self.n {
            if self.i < less[i as usize] {
                new_indices[b] = i;
                b -= 1;
            } else if (self.n - 1) - self.i < greater[i as usize] {
                n_less_dropped += 1;
                new_indices[b] = i;
                b -= 1;
            } else {
                new_indices[new_n as usize] = i;
                new_n += 1;
            }
        }

        if new_n != self.n {
            let mut new_poset = BackwardsPoset::new(new_n, self.i - n_less_dropped);
            for i in 0..new_poset.n {
                for j in 0..new_poset.n {
                    new_poset.set_less(
                        i,
                        j,
                        self.is_less(new_indices[i as usize], new_indices[j as usize]),
                    );
                }
            }
            *self = new_poset;
        }

        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
    }

    // canonify2
    fn canonify_nauty_indicies(&self) -> Vec<u8> {
        let mut options = optionblk {
            getcanon: TRUE,
            defaultptn: FALSE,
            digraph: TRUE,
            ..Default::default()
        };
        let mut stats = statsblk::default();

        let mut labels: [c_int; MAX_N] =
            (0..MAX_N as c_int).collect::<Vec<_>>().try_into().unwrap();

        let mut ptn = [c_int::from(1); MAX_N];
        ptn[self.n as usize - 1] = 0;
        let mut zeroes2 = [c_int::from(0); MAX_N];

        let mut dg = [0; MAX_N];
        for (i, mask) in dg.iter_mut().enumerate().take(self.n as usize) {
            for j in 0..self.n {
                if self.is_less(i as u8, j) {
                    *mask |= nauty_Traces_sys::bit[j as usize];
                }
            }
        }

        let mut canonical = [0; MAX_N];

        unsafe {
            densenauty(
                dg.as_mut_ptr(),
                labels.as_mut_ptr(),
                ptn.as_mut_ptr(),
                zeroes2.as_mut_ptr(),
                &mut options,
                &mut stats,
                1 as c_int,
                self.n as c_int,
                canonical.as_mut_ptr(),
            );
        }

        let mut result = vec![0u8; self.n as usize];
        for i in 0..self.n as usize {
            result[i] = labels[i] as u8;
        }
        result
    }

    pub fn canonify_without_dual(&mut self) {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        // debug_assert!(self.is_closed());

        let old_poset = self.clone();
        let mut new_indices: Vec<u8>;

        if ONLY_NAUTY_CANONIFY {
            new_indices = self.canonify_nauty_indicies();
        } else {
            // TODO: sometimes FALSE
            let (less, greater) = self.calculate_relations2();

            let mut in_out_degree = [0u64; MAX_N];
            for i in 0..self.n as usize {
                in_out_degree[i] = (MAX_N as u64) * (greater[i] as u64) + (less[i] as u64);
            }

            let mut hash = in_out_degree.clone();
            for _ in 0..2 {
                let mut sum_hash = [0; MAX_N];

                for i in 0..self.n {
                    let mut sum = hash[i as usize];

                    for j in 0..self.n {
                        if i != j && (self.is_less(i, j) || self.is_less(j, i)) {
                            sum ^= hash[j as usize]; // TODO: `sum += hash[j as usize]` is faster ...
                        }
                    }

                    sum_hash[i as usize] = sum;
                }

                for i in 0..self.n as usize {
                    hash[i] = sum_hash[i] * (MAX_N as u64 * MAX_N as u64) + in_out_degree[i];
                }
            }

            let cmpr = |&a: &u8, &b: &u8| {
                in_out_degree[a as usize]
                    .cmp(&in_out_degree[b as usize])
                    .then_with(|| hash[a as usize].cmp(&hash[b as usize]))
            };

            new_indices = (0..self.n).collect();
            new_indices.sort_by(cmpr);

            let mut is_unique = true;
            for i in 1..self.n {
                if in_out_degree[new_indices[(i - 1) as usize] as usize]
                    == in_out_degree[new_indices[i as usize] as usize]
                    && hash[new_indices[(i - 1) as usize] as usize]
                        == hash[new_indices[i as usize] as usize]
                {
                    self.swap(new_indices[(i - 1) as usize], new_indices[i as usize]);
                    if *self != old_poset {
                        is_unique = false;
                        break;
                    }
                }
            }

            if !is_unique {
                new_indices = self.canonify_nauty_indicies();
                new_indices.sort_by(cmpr);
            }
        }

        for i in 0..self.n {
            for j in 0..self.n {
                self.set_less(
                    i,
                    j,
                    old_poset.is_less(new_indices[i as usize], new_indices[j as usize]),
                );
            }
        }

        // postcondition
        // debug_assert!(2 * self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        // debug_assert!(self.is_closed2());
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                debug_assert!(!self.is_less(i, j));
            }
        }
    }

    pub fn canonify2(&mut self) {
        if self.n <= 2 * self.i {
            self.dual2();
        }

        self.canonify_without_dual();

        if self.n - 1 == 2 * self.i {
            let mut dualed = self.clone();
            dualed.dual2();
            dualed.canonify_without_dual();
            let mut is_dual = false;
            'break_all: for i in 0..self.n {
                for j in 0..self.n {
                    if self.is_less(i, j) != dualed.is_less(i, j) {
                        is_dual = dualed.is_less(i, j);
                        break 'break_all;
                    }
                }
            }
            if is_dual {
                *self = dualed;
            }
        }
    }

    // normalize
    pub fn normalize(&mut self) {
        self.reduce_elements2();
        self.canonify2();
    }

    // remove less
    fn is_redundant(&self, i: u8, j: u8) -> bool {
        for k in 0..self.n {
            if self.is_less(i, k) && self.is_less(k, j) {
                return true;
            }
        }
        false
    }

    pub fn remove_less(&self, poset_cache: &HashSet<BackwardsPoset>) -> HashSet<BackwardsPoset> {
        // // precondition
        // debug_assert!(self.i < self.n);
        // debug_assert!((self.n as usize) < MAX_N);
        // debug_assert!(self.is_closed2());
        // debug_assert!(self.is_canonified());

        let mut result = HashSet::new();
        for i in 0..self.n {
            for j in 0..self.n {
                if !self.is_less(i, j) || self.is_redundant(i, j) {
                    continue;
                }

                let mut poset_initial = self.clone();
                poset_initial.set_less(i, j, false);

                if !poset_cache.contains(&poset_initial.with_less_normalized(j, i)) {
                    continue;
                }

                result.insert(poset_initial.clone());

                let mut queue = Vec::new();
                queue.push(poset_initial);

                while let Some(poset) = queue.pop() {
                    for i1 in 0..self.n {
                        for j1 in 0..self.n {
                            if !poset.is_less(i1, j1) || poset.is_redundant(i1, j1)
                            // || (j as i32 - i as i32).abs() >= (j1 as i32 - i1 as i32).abs()
                            {
                                continue;
                            }

                            let mut poset_next = poset.clone();
                            poset_next.set_less(i1, j1, false);

                            if result.contains(&poset_next)
                                || *self != poset_next.with_less2(i, j)
                                || !poset_cache.contains(&poset_next.with_less_normalized(j, i))
                            {
                                continue;
                            }

                            result.insert(poset_next.clone());
                            queue.push(poset_next);
                        }
                    }
                }
            }
        }

        let mut cleaned_result = HashSet::new();
        for mut item in result {
            let size = item.n();
            item.reduce_elements2();
            if size != item.n() {
                continue;
            }
            item.canonify2();
            if poset_cache.contains(&item) {
                continue;
            }
            cleaned_result.insert(item);
        }
        cleaned_result
    }

    pub fn enlarge_and_remove_less(
        &self,
        interrupt: &Arc<AtomicBool>,
        poset_cache: &HashSet<BackwardsPoset>,
        table: &[[bool; 15]; 15],
        n: u8,
        i: u8,
    ) -> HashSet<BackwardsPoset> {
        debug_assert!(2 * self.i < self.n);
        debug_assert!(table[self.n as usize][self.i as usize]);

        let mut destination = self.remove_less(poset_cache);

        if self.n == n && self.i == i {
            return destination;
        }

        let mut temp_set_level: [(HashSet<BackwardsPoset>, HashSet<BackwardsPoset>); MAX_N] =
            Default::default();
        let mut temp_set_next: [(HashSet<BackwardsPoset>, HashSet<BackwardsPoset>); MAX_N] =
            Default::default();
        for i0 in 0..MAX_N {
            temp_set_level[i0] = (HashSet::new(), HashSet::new());
            temp_set_next[i0] = (HashSet::new(), HashSet::new());
        }

        temp_set_level[self.i as usize].0.insert(self.clone());

        for n0 in self.n..n {
            for i0 in 0..=i {
                let mut temp_dest = HashSet::new();
                for item in &temp_set_level[i0 as usize].0 {
                    let mut result1 = HashSet::new();
                    if table[n0 as usize + 1][i0 as usize] {
                        item.enlarge_n(interrupt, &mut result1);
                        if interrupt.load(Ordering::Relaxed) {
                            return HashSet::new();
                        }
                    }

                    let condition = 2 * (i0 + 1) < n0 + 1;
                    if (condition && table[n0 as usize + 1][i0 as usize + 1])
                        || (!condition && table[n0 as usize + 1][n0 as usize - i0 as usize - 1])
                    {
                        item.enlarge_nk(interrupt, &mut result1);
                        if interrupt.load(Ordering::Relaxed) {
                            return HashSet::new();
                        }
                    }

                    for it in result1 {
                        debug_assert_eq!(
                            {
                                let mut norm = it.clone();
                                norm.normalize();
                                norm
                            },
                            *self
                        );

                        if temp_set_next[it.i as usize].0.contains(&it)
                            || temp_set_next[it.i as usize].1.contains(&it)
                        {
                            continue;
                        }

                        let mut found = false;
                        for item in it.remove_less(poset_cache) {
                            found |= !destination.contains(&item);
                            temp_dest.insert(item);
                        }
                        if found {
                            temp_set_next[it.i as usize].0.insert(it);
                        } else {
                            temp_set_next[it.i as usize].1.insert(it);
                        }

                        if interrupt.load(Ordering::Relaxed) {
                            return HashSet::new();
                        }
                    }
                }
                for item in temp_dest {
                    destination.insert(item);
                }
            }

            for i0 in 0..MAX_N {
                swap(&mut temp_set_level[i0], &mut temp_set_next[i0]);
                temp_set_next[i0].0.clear();
                temp_set_next[i0].1.clear();
            }
        }

        destination
    }

    fn can_reduce_element_greater(&self, element: u8) -> bool {
        let mut greater = 0u8;
        for k in 0..self.n {
            if self.is_less(k, element) {
                greater += 1;
            }
        }
        self.i < greater
    }

    fn enlarge_n(&self, interrupt: &Arc<AtomicBool>, result: &mut HashSet<BackwardsPoset>) {
        let mut temp = BackwardsPoset::new(self.n + 1, self.i);
        for i in 0..self.n {
            for j in 0..self.n {
                temp.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut unfiltered = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((temp, -1));
        while let Some((poset, number)) = swap_init.pop_back() {
            for k in ((number + 1) as u8)..(poset.n - 1) {
                if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
                    let new_poset = poset.with_less2(k, poset.n - 1);
                    swap_init.push_back((new_poset.clone(), k as i32));
                    if new_poset.can_reduce_element_greater(new_poset.n - 1) {
                        unfiltered.insert(new_poset);
                    }
                }
            }
            if interrupt.load(Ordering::Relaxed) {
                return;
            }
        }

        for mut item in unfiltered {
            item.canonify2();
            result.insert(item);
        }
    }

    fn can_reduce_element_less(&self, element: u8) -> bool {
        let mut less = 0u8;
        for k in 0..self.n {
            if self.is_less(element, k) {
                less += 1;
            }
        }
        (self.n - 1) - self.i < less
    }

    fn enlarge_nk(&self, interrupt: &Arc<AtomicBool>, result: &mut HashSet<BackwardsPoset>) {
        let mut temp = BackwardsPoset::new(self.n + 1, self.i + 1);
        for i in 0..self.n {
            for j in 0..self.n {
                temp.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut unfiltered = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((temp, -1));
        while let Some((poset, number)) = swap_init.pop_back() {
            for k in ((number + 1) as u8)..(poset.n - 1) {
                if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
                    let new_poset = poset.with_less2(poset.n - 1, k);
                    swap_init.push_back((new_poset.clone(), k as i32));
                    if new_poset.can_reduce_element_less(new_poset.n - 1) {
                        unfiltered.insert(new_poset);
                    }
                }
            }
            if interrupt.load(Ordering::Relaxed) {
                return;
            }
        }

        for mut item in unfiltered {
            item.canonify2();
            result.insert(item);
        }
    }

    pub fn rec_temp(table: &mut [[bool; MAX_N]; MAX_N], n: usize, i: usize) {
        table[n][i] = true;
        if 1 <= n && 2 * i < n - 1 {
            Self::rec_temp(table, n - 1, i);
        }
        if 1 <= n && 1 <= i && 2 * (i - 1) < n - 1 {
            Self::rec_temp(table, n - 1, i - 1);
        }
        if 1 <= n && i < n && 2 * (n - i - 1) < n - 1 {
            Self::rec_temp(table, n - 1, n - i - 1);
        }
    }

    fn is_closed2(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                if i == j {
                    if self.is_less(i, j) {
                        eprintln!("on diagonal no '1' allowed:");
                        dbg!(self, i, j);
                        return false;
                    }
                } else {
                    if self.is_less(i, j) && self.is_less(j, i) {
                        eprintln!("it holds i < j and j < i => impossible:");
                        dbg!(self, i, j);
                        return false;
                    }
                    for k in 0..self.n {
                        if self.is_less(i, j) && self.is_less(j, k) && !self.is_less(i, k) {
                            eprintln!("transitive comparison not set:");
                            dbg!(self, i, j, k);
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn is_canonified(&self) -> bool {
        let mut canon = self.clone();
        canon.canonify2();
        if *self != canon {
            dbg!(self);
            dbg!(&canon);
        }
        *self == canon
    }

    fn is_normalized(&self) -> bool {
        let mut canon = self.clone();
        canon.normalize();
        if *self != canon {
            dbg!(self);
            dbg!(&canon);
        }
        *self == canon
    }
}

impl Debug for BackwardsPoset {
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

        f.debug_struct("Poset")
            .field("n", &self.n)
            .field("i", &self.i)
            .field("adjacency", &adjacency)
            .field("all_comparisons", &all_comparisons)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_normalize_1() {
        let n = 10;
        let i = 5;

        let mut hashset = HashSet::new();

        for j in 0..n {
            for k in 0..n {
                if j == k {
                    continue;
                }
                hashset.insert(BackwardsPoset::new(n, i).with_less(j, k));
            }
        }

        // all posets with one comparison should be isomorph
        assert_eq!(hashset.len(), 1);
    }

    #[test]
    fn test_normalize_2() {
        let n = 10;
        let i = 5;

        let mut hashset = HashSet::new();

        for j in 0..n {
            for k in 0..n {
                if j == k {
                    continue;
                }

                for l in 0..n {
                    for m in 0..n {
                        if l == m || (j, k) == (l, m) || (j, k) == (m, l) {
                            continue;
                        }

                        let mut poset = BackwardsPoset::new(n, i);
                        poset.add_and_close(j, k); // just adding without normalizing
                        poset.add_and_close(l, m);
                        poset.canonify();
                        hashset.insert(poset);
                    }
                }
            }
        }

        // all posets with two comparison should be isomorph to one of the four:
        // 1. 0 < 1, 2 < 3
        // 2. 0 < 1, 0 < 2
        // 3. 1 < 0, 2 < 0
        // 4. 0 < 1, 1 < 2
        // This doesn't work yet
        dbg!(&hashset);
        assert_eq!(hashset.len(), 4);
    }

    #[test]
    fn test_reduce() {
        let mut poset = BackwardsPoset::new(4, 0);
        poset.add_and_close(0, 1);
        poset.reduce_elements(); // removes 0
        dbg!(poset);
        poset.add_and_close(0, 2);
        poset.reduce_elements(); // removes 0
        dbg!(poset);
        poset.add_and_close(0, 1);
        poset.reduce_elements(); // removes 0
        dbg!(poset); // gelöst
    }

    #[test]
    fn test_compatible_posets() {
        assert_eq!(BackwardsPoset::new(5, 2).num_compatible_posets(), 30); // 5 * (4 choose 2)
        assert_eq!(BackwardsPoset::new(10, 4).num_compatible_posets(), 1260); // 10 * (9 choose 4)
        let mut poset = BackwardsPoset::new(10, 4);
        poset.add_and_close(0, 1);
        poset.canonify();
        dbg!(poset, poset.num_compatible_posets());
        // assert_eq!(poset.compatible_posets(), 854); // i don't know if this is correct

        let mut poset = BackwardsPoset::new(10, 4);
        poset.add_and_close(0, 1);
        poset.add_and_close(1, 2);
        poset.canonify();
        dbg!(poset, poset.num_compatible_posets());
        // assert_eq!(poset.compatible_posets(), 483); // i don't know if this is correct

        let mut poset = BackwardsPoset::new(6, 1);
        poset.add_and_close(2, 0);
        poset.add_and_close(3, 0);
        poset.add_and_close(4, 1);
        poset.add_and_close(5, 1);
        poset.canonify();
        dbg!(poset, poset.num_compatible_posets());
        // assert_eq!(poset.compatible_posets(), 12); // i don't know if this is correct
    }
}
