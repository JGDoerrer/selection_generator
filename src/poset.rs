use std::{fmt::Debug, os::raw::c_int};

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};
use serde::{Deserialize, Serialize};

use crate::{bitset::BitSet, constants::MAX_N};

pub type PosetIndex = u8;

/// A partially ordered set with <
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Poset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    adjacency: [BitSet; MAX_N],
}

impl Default for Poset {
    fn default() -> Self {
        Poset::new(0, 0)
    }
}

impl Poset {
    pub fn new(n: u8, i: u8) -> Self {
        debug_assert!(n < MAX_N as u8);
        debug_assert!(i < n);

        Poset {
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

    fn canonify(&mut self) {
        self.reduce_elements();
        self.canonify_mapping();
    }

    /// Canonifies the poset and returns a mapping from old to new indices, since they shift around
    pub fn canonify_mapping(&mut self) -> [usize; MAX_N] {
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

        for _ in 0..4 {
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
                .then(hash[*a].cmp(&hash[*b]))
        };

        new_indices[0..n].sort_by(comparator);

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

        let mut new = Poset::new(self.n, self.i);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize] as u8, new_indices[j as usize] as u8) {
                    new.set_bit(i, j)
                }
            }
        }

        // assert!(self.is_lower_triangle_matrix());
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
    pub fn reduce_elements(&mut self) -> [PosetIndex; MAX_N] {
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

        let mut new = Poset::new(new_n as u8, self.i - n_less_dropped);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize], new_indices[j as usize]) {
                    new.set_bit(i, j)
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

        let mut new = Poset::new(self.n, self.i);

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

    fn canonify_lower_matrix(&self) -> Poset {
        let mut new_indices = [0; MAX_N];
        let mut next_free = 0;
        let mut indices_used = BitSet::empty();

        let mut new = Poset::new(self.n, self.i);

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
    pub fn with_less_mapping(&self, i: PosetIndex, j: PosetIndex) -> (Self, [usize; MAX_N]) {
        let mut new = *self;

        new.add_and_close(i, j);
        let mapping = new.canonify_mapping();

        (new, mapping)
    }

    /// Assumes self is normalized
    pub fn dual(&self) -> Self {
        let mut dual = Poset::new(self.n, self.n - self.i - 1);
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
}

impl Debug for Poset {
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
                hashset.insert(Poset::new(n, i).with_less(j, k));
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

                        let mut poset = Poset::new(n, i);
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
        let mut poset = Poset::new(4, 0);
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
        assert_eq!(Poset::new(5, 2).num_compatible_posets(), 30); // 5 * (4 choose 2)
        assert_eq!(Poset::new(10, 4).num_compatible_posets(), 1260); // 10 * (9 choose 4)
        let mut poset = Poset::new(10, 4);
        poset.add_and_close(0, 1);
        poset.canonify();
        dbg!(poset, poset.num_compatible_posets());
        // assert_eq!(poset.compatible_posets(), 854); // i don't know if this is correct

        let mut poset = Poset::new(10, 4);
        poset.add_and_close(0, 1);
        poset.add_and_close(1, 2);
        poset.canonify();
        dbg!(poset, poset.num_compatible_posets());
        // assert_eq!(poset.compatible_posets(), 483); // i don't know if this is correct

        let mut poset = Poset::new(6, 1);
        poset.add_and_close(2, 0);
        poset.add_and_close(3, 0);
        poset.add_and_close(4, 1);
        poset.add_and_close(5, 1);
        poset.canonify();
        dbg!(poset, poset.num_compatible_posets());
        // assert_eq!(poset.compatible_posets(), 12); // i don't know if this is correct
    }
}
