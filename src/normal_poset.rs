use core::fmt::Debug;
use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};
use serde::{Deserialize, Serialize};
use std::os::raw::c_int;

use crate::{bitset::BitSet, canonified_poset::CanonifiedPoset, constants::MAX_N, poset::Poset};

/// A partially ordered set with <
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NormalPoset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    adjacency: [BitSet; MAX_N],
}

impl Default for NormalPoset {
    fn default() -> Self {
        NormalPoset::new(0, 0)
    }
}

impl Poset for NormalPoset {
    fn new(n: u8, i: u8) -> Self {
        debug_assert!(n < MAX_N as u8);
        debug_assert!(i < n);

        NormalPoset {
            n,
            i,
            adjacency: [BitSet::empty(); MAX_N],
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

    /// is i < j?
    #[inline]
    fn is_less(&self, i: u8, j: u8) -> bool {
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);

        self.adjacency[i as usize].contains(j as usize)
    }

    /// returns a bitset of all elements greater than i
    #[inline]
    fn get_all_greater_than(&self, i: u8) -> BitSet {
        debug_assert!(i < self.n);
        self.adjacency[i as usize]
    }

    /// returns a bitset of all elements less than i
    #[inline]
    fn get_all_less_than(&self, i: u8) -> BitSet {
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
}

impl NormalPoset {
    #[inline]
    pub fn set_is_less(&mut self, i: u8, j: u8) {
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);

        self.adjacency[i as usize].insert(j as usize);
    }

    pub fn set_all_greater_than(&mut self, i: usize, greater: BitSet) {
        debug_assert!(i < self.n as usize);

        self.adjacency[i] = greater;
    }

    /// adds i < j and makes sure, that i < j && j < k => i < k is true
    #[inline]
    pub fn add_and_close(&mut self, i: u8, j: u8) {
        let mut stack = Vec::with_capacity(self.n() as usize);
        stack.push((i, j));

        while let Some((i, j)) = stack.pop() {
            self.set_is_less(i, j);

            for k in self
                .get_all_greater_than(j)
                .intersect(self.get_all_greater_than(i).complement())
            {
                stack.push((i, k as u8));
            }

            for k in 0..self.n {
                if self.is_less(k, i) && !self.is_less(k, j) {
                    stack.push((k as u8, j));
                }
            }
        }
        debug_assert!(self.is_closed(), "{self:?}");
    }

    #[inline]
    pub fn canonified(&self) -> CanonifiedPoset {
        let mut copy = *self;
        copy.reduce_elements();
        let mapping = copy.get_canonification_mapping();

        let mut canonified = CanonifiedPoset::new(copy.n, copy.i);

        for i in 0..canonified.n() {
            let mapped_i = mapping[i as usize] as u8;
            for j in (i + 1)..canonified.n() {
                if copy.is_less(mapped_i, mapping[j as usize] as u8) {
                    canonified.set_is_less(i, j)
                }
            }
        }

        canonified
    }

    #[inline]
    pub fn canonify(&mut self) {
        self.reduce_elements();
        self.canonify_mapping();
    }

    /// Canonifies the poset and returns a mapping from old to new indices, since they shift around
    #[inline]
    fn get_canonification_mapping(&self) -> [usize; MAX_N] {
        let n = self.n as usize;

        let mut ordered_with_subsets = [BitSet::empty(); MAX_N];

        let mut in_out_degree = [0; MAX_N];

        for i in 0..n {
            let greater = self.get_all_greater_than(i as u8);
            let less = self.get_all_less_than(i as u8);

            ordered_with_subsets[i] = greater.union(less);
            in_out_degree[i] = greater.len() as u64 * MAX_N as u64 + less.len() as u64;
        }

        let in_out_degree = in_out_degree;

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
                let new_hash = sum_hash[i]
                    .wrapping_mul(MAX_N.pow(2) as u64)
                    .wrapping_add(in_out_degree[i]);

                hash[i] = new_hash;
            }
        }

        let hash = hash;

        let mut new_indices = [0; MAX_N];
        new_indices
            .iter_mut()
            .enumerate()
            .take(self.n as usize)
            .for_each(|(i, index)| *index = i);

        let comparator = |a: &usize, b: &usize| {
            in_out_degree[*a]
                .cmp(&in_out_degree[*b])
                .then_with(|| hash[*a].cmp(&hash[*b]))
                .reverse()
        };

        new_indices[0..n].sort_unstable_by(comparator);

        // let mut is_unique = true;
        // for i in 1..self.n as usize {
        //     let i0 = new_indices[i];
        //     let i1 = new_indices[i - 1];

        //     let degree = in_out_degree[i0];
        //     if degree == 0 || degree == 1 || degree == MAX_N as u64 {
        //         continue;
        //     }

        //     if in_out_degree[i1] == in_out_degree[i0] && hash[i1] == hash[i0] {
        //         let mut new = *self;
        //         new.swap(i1 as u8, i0 as u8);
        //         if new != *self {
        //             is_unique = false;
        //             break;
        //         }
        //     }
        // }

        // if !is_unique {
        //     new_indices = self.canonify_nauty_indicies();

        //     new_indices[0..n].sort_by(comparator);
        // }

        new_indices
    }

    fn canonify_mapping(&mut self) -> [usize; MAX_N] {
        let new_indices = self.get_canonification_mapping();

        let mut new = NormalPoset::new(self.n, self.i);

        // make the new poset
        for i in 0..new.n {
            for j in (i + 1)..new.n {
                if self.is_less(new_indices[i as usize] as u8, new_indices[j as usize] as u8) {
                    new.set_is_less(i, j)
                }
            }
        }

        debug_assert!(new.is_lower_triangle_matrix(), "{new:?}");
        debug_assert!(new.is_closed(), "{new:?}");

        *self = new;

        new_indices
    }

    pub fn set_less(&mut self, i: u8, j: u8, value: bool) {
        if value {
            self.adjacency[i as usize].insert(j as usize);
        } else {
            self.adjacency[i as usize].remove(j as usize);
        }
    }

    fn swap_positions(&mut self, i0: u8, j0: u8, i1: u8, j1: u8) {
        let temp = self.is_less(i0, j0);
        self.set_less(i0, j0, self.is_less(i1, j1));
        self.set_less(i1, j1, temp);
    }

    fn swap(&mut self, i: u8, j: u8) {
        for k in 0..self.n {
            if !(i != k && j != k) {
                continue;
            }
            self.swap_positions(i, k, j, k);
            self.swap_positions(k, i, k, j);
        }
    }

    /// Removes elements, that are known to be too large/small
    #[inline]
    pub fn reduce_elements(&mut self) -> [u8; MAX_N] {
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
            if !dropped[i as usize] {
                new_indices[new_n] = i;
                new_n += 1;
            } else {
                new_indices[b] = i;
                b -= 1;
            }
        }

        if self.n == new_n as u8 {
            return new_indices;
        }

        let mut new = NormalPoset::new(new_n as u8, self.i - n_less_dropped);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize], new_indices[j as usize]) {
                    new.set_is_less(i, j)
                }
            }
        }

        if new.i > (new.n + 1) / 2 {
            new = new.dual();
        }

        // dbg!(&self, &new);
        debug_assert!(new.is_closed(), "{new:?}");
        *self = new;

        new_indices
    }

    fn canonify_nauty_indicies(&self) -> [usize; MAX_N] {
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

        let mut result = [0; MAX_N];
        for i in 0..self.n as usize {
            result[i] = labels[i] as usize;
        }
        result
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

        let mut new = NormalPoset::new(self.n, self.i);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if canonical[i as usize] & nauty_Traces_sys::bit[j as usize] != 0 {
                    new.set_is_less(i, j)
                }
            }
        }

        // dbg!(&self, &new);
        // dbg!(labels);
        *self = new;
    }

    /// adds i < j to the poset and normalize
    #[inline]
    pub fn add_less(&mut self, i: u8, j: u8) {
        debug_assert!(!self.is_less(i, j));
        debug_assert!(!self.is_less(j, i));

        self.add_and_close(i, j);
        self.canonify();
    }

    /// returns a clone of the poset, with i < j added
    pub fn with_less(&self, i: u8, j: u8) -> CanonifiedPoset {
        let mut new = *self;
        new.add_and_close(i, j);
        new.canonified()
    }

    /// returns a clone of the poset, with i < j added
    pub fn with_less_mapping(&self, i: u8, j: u8) -> (Self, [usize; MAX_N]) {
        let mut new = *self;

        new.add_and_close(i, j);
        let mapping = new.canonify_mapping();

        (new, mapping)
    }

    /// Assumes self is normalized
    pub fn dual(&self) -> Self {
        let mut dual = NormalPoset::new(self.n, self.n - self.i - 1);
        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    dual.set_is_less(j, i);
                }
            }
        }

        dual
    }

    pub fn num_compatible_posets_upper_bound(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.n {
            // assume the ith element is the solution

            let less_than_i = self.get_all_less_than(i);
            let greater_than_i = self.get_all_greater_than(i);

            let mut counts = [0; MAX_N];
            counts[0] = 1;

            for j in 0..self.n {
                if j == i || greater_than_i.contains(j as usize) {
                    continue;
                }

                // try adding j to all previous subsets
                if less_than_i.contains(j as usize) {
                    // all subsets must contain j to be valid
                    for i in (1..=self.i as usize).rev() {
                        counts[i] = counts[i - 1];
                    }
                    counts[0] = 0;
                } else {
                    for i in (1..=self.i as usize).rev() {
                        counts[i] += counts[i - 1];
                    }
                }
            }

            sum += counts[self.i as usize];
        }

        sum
    }
}

impl Debug for NormalPoset {
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

        f.debug_struct("NormalPoset")
            .field("n", &self.n)
            .field("i", &self.i)
            .field("adjacency", &adjacency)
            .field("all_comparisons", &all_comparisons)
            .finish()
    }
}
