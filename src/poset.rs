use std::{fmt::Debug, os::raw::c_int};

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};
use serde::{Deserialize, Serialize};

use crate::{bitset::BitSet, KNOWN_MIN_VALUES};

pub const MAX_N: usize = 15;

pub type PosetIndex = u8;

/// A partially ordered set with <
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Poset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    adjacency: [u16; MAX_N],
}

impl Default for Poset {
    fn default() -> Self {
        Poset::new(0, 0)
    }
}

impl Poset {
    pub fn new(n: u8, i: u8) -> Self {
        debug_assert!(n < MAX_N as u8);

        Poset {
            n,
            i,
            adjacency: [0; MAX_N],
        }
    }

    pub fn n(&self) -> u8 {
        self.n
    }

    pub fn i(&self) -> u8 {
        self.i
    }

    #[inline]
    fn set_bit(&mut self, i: PosetIndex, j: PosetIndex) {
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);

        let mask = 1 << j;

        self.adjacency[i as usize] |= mask;
    }

    /// is i < j?
    #[inline]
    pub fn is_less(&self, i: PosetIndex, j: PosetIndex) -> bool {
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);

        let mask = 1 << j;

        (self.adjacency[i as usize] & mask) != 0
    }

    /// returns a bitset of all elements greater than i
    pub fn get_all_greater_than(&self, i: PosetIndex) -> BitSet {
        debug_assert!(i < self.n);
        self.adjacency[i as usize].into()
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

    /// returns how many elements are less, unknown or greater than it
    #[inline]
    pub fn calculate_relations(&self) -> ([u8; MAX_N], [u8; MAX_N], [u8; MAX_N]) {
        let mut less = [0u8; MAX_N];
        let mut greater = [0u8; MAX_N];
        let mut unknown = [0u8; MAX_N];

        for i in 0..self.n as usize {
            greater[i] = self.get_all_greater_than(i as u8).len() as u8;

            let i_bitset = BitSet::single(i as usize);
            for j in 0..self.n {
                less[i] += (self.get_all_greater_than(j).intersect(i_bitset).bits() >> i) as u8;
            }

            unknown[i] = self.n - greater[i] - less[i];
        }

        (less, unknown, greater)
    }

    #[inline]
    fn hash(a: u64, b: u64) -> u64 {
        let mut hash: u64 = 9118271012717746669;

        hash = hash.wrapping_mul(a);
        hash = hash.wrapping_shl(7);
        hash = hash.wrapping_add(3032928155878307119);
        hash = hash.wrapping_mul(b);
        hash = hash.wrapping_shr(9);
        hash = hash.wrapping_add(16728691407311227577);
        hash = hash.wrapping_shl(11);
        hash = hash.wrapping_mul(1536811303);
        hash = hash.wrapping_shr(15);
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

        let (less, _unknown, greater) = self.calculate_relations();

        let mut in_out_degree = [0; MAX_N];

        for i in 0..n {
            in_out_degree[i] = greater[i] as u64 * MAX_N as u64 + less[i] as u64;
        }

        let mut hash = in_out_degree;

        for _ in 0..2 {
            let mut sum_hash = [0; MAX_N];

            for i in 0..self.n {
                let mut sum = hash[i as usize];

                // sum hashes of neighbours
                for j in 0..self.n {
                    if i == j {
                        continue;
                    }

                    if self.has_order(i, j) {
                        sum = sum.wrapping_add(hash[j as usize]);
                    }
                }

                sum_hash[i as usize] = sum;
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

        // let mut i = 1;

        // i += new_indices
        //     .iter()
        //     .take(n)
        //     .take_while(|i| in_out_degree[**i as usize] == 0)
        //     .count()
        //     + 1;

        // while i < n {
        //     // search for elements with same hashes

        //     let index = new_indices[i] as usize;
        //     let next_index = new_indices[i - 1] as usize;

        //     if !(in_out_degree[index] == in_out_degree[next_index]
        //         && hash[index] == hash[next_index])
        //     {
        //         i += 1;
        //         continue;
        //     }

        //     while i < n
        //         && (in_out_degree[index] == in_out_degree[next_index]
        //             && hash[index] == hash[next_index])
        //     {
        //         i += 1;
        //     }

        //     if i == n {
        //         continue;
        //     }

        //     unreachable!();

        //     for _ in 0..3 {
        //         let mut sum_hash = [0; MAX_N];

        //         for i in i..n {
        //             let mapped_i = new_indices[i];
        //             let mut sum = hash[mapped_i as usize];

        //             // sum hashes of neighbours
        //             for &mapped_j in new_indices.iter().take(i) {
        //                 if mapped_i == mapped_j {
        //                     continue;
        //                 }

        //                 if self.has_order(mapped_i, mapped_j) {
        //                     sum = sum.wrapping_add(hash[mapped_j as usize]);
        //                 }
        //             }

        //             sum_hash[mapped_i as usize] = sum;
        //         }

        //         // calc new hash based on neighbours hashes
        //         for &i in new_indices.iter().take(n).skip(i) {
        //             let i = i as usize;
        //             hash[i] = Self::hash(sum_hash[i], in_out_degree[i]);
        //         }
        //     }

        //     new_indices[i..n].sort_by(|a, b| {
        //         in_out_degree[*a as usize]
        //             .cmp(&in_out_degree[*b as usize])
        //             .then(hash[*a as usize].cmp(&hash[*b as usize]))
        //     });
        // }

        let mut new = Poset::new(self.n, self.i);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize], new_indices[j as usize]) {
                    new.set_bit(i, j)
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

    /// Removes elements, that are known to be too large/small
    fn reduce_elements(&mut self) -> [PosetIndex; MAX_N] {
        // can the element be ignored, because it is too large/small
        let mut dropped = [false; MAX_N];
        let mut n_less_dropped = 0;

        let mut less = [0; MAX_N];
        for i in 0..self.n {
            for j in 0..self.n {
                less[i as usize] += (self
                    .get_all_greater_than(j)
                    .intersect(BitSet::single(i as usize))
                    .bits()
                    >> i) as u8;
            }
        }

        for i in 0..self.n as usize {
            if self.get_all_greater_than(i as u8).len() > self.i.into() {
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

        // dbg!(&self, &new);
        debug_assert!(new.is_closed(), "{new:?}");
        *self = new;

        new_indices
    }

    fn remove_elements(&mut self, to_remove: Vec<PosetIndex>) {
        if to_remove.is_empty() {
            return;
        }

        // maps the old indices to the new ones
        let mut new_indices = [0; MAX_N];
        let mut new_n = 0;
        let mut b = self.n as usize;

        for i in 0..self.n {
            if !to_remove.contains(&i) {
                new_indices[new_n] = i;
                new_n += 1;
            } else {
                b -= 1;
                new_indices[b] = i;
            }
        }

        if self.n == new_n as u8 {
            return;
        }

        let mut new = Poset::new(new_n as u8, self.i.min(new_n as u8));

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize], new_indices[j as usize]) {
                    new.set_bit(i, j)
                }
            }
        }

        // dbg!(&self, &new);
        debug_assert!(new.is_closed(), "{new:?}");
        *self = new;
    }

    /// Removes elements, that are in relation with i
    fn remove_elements_ordered_with(&mut self, i: PosetIndex) -> (u8, u8) {
        let mut dropped = [false; MAX_N];
        let mut n_less_dropped = 0;
        let mut n_greater_dropped = 0;

        for j in 0..self.n {
            if self.is_less(i, j) || j == i {
                dropped[j as usize] = true;
                n_greater_dropped += 1;
            } else if self.is_less(j, i) {
                dropped[j as usize] = true;
                n_less_dropped += 1;
            }
        }

        // maps the old indices to the new ones
        let mut new_indices = [0; MAX_N];
        let mut new_n = 0;
        let mut b = self.n as usize;

        for j in 0..self.n {
            if dropped[j as usize] {
                b -= 1;
                new_indices[b] = j;
            } else {
                new_indices[new_n] = j;
                new_n += 1;
            }
        }

        let mut new = Poset::new(new_n as u8, self.i);

        // make the new poset
        for i in 0..new.n {
            for j in 0..new.n {
                if self.is_less(new_indices[i as usize], new_indices[j as usize]) {
                    new.set_bit(i, j)
                }
            }
        }

        // dbg!(&self, &new);
        debug_assert!(new.is_closed(), "{new:?}");
        *self = new;
        (n_less_dropped, n_greater_dropped)
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

                if less_than_i.len() == 0 || less_than_i.intersect(indices_used) == less_than_i {
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
    pub fn add_and_close(&mut self, i: PosetIndex, j: PosetIndex) {
        self.set_bit(i, j);
        for k in 0..self.n {
            if i != k && j != k {
                if self.is_less(k, i) && !self.is_less(k, j) {
                    self.add_and_close(k, j)
                }
                if self.is_less(j, k) && !self.is_less(i, k) {
                    self.add_and_close(i, k)
                }
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

    /// adapted from [https://www.cs.hut.fi/~cessu/selection/selgen.c.html]
    pub fn is_solvable_in(&self, max_comparisons: u8) -> bool {
        if self.i == 0 || self.i == self.n - 1 {
            max_comparisons >= self.n - 1
        } else if self.i == 1 {
            let (less, _unknown, greater) = self.calculate_relations();

            let mut num_groups = 0;
            let mut s = 0u32;

            for i in 0..self.n as usize {
                if greater[i] == 0 {
                    num_groups += 1;
                    s += 1 << less[i];
                }
            }

            max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 3
        } else if self.i == self.n - 2 {
            let (less, _unknown, greater) = self.calculate_relations();

            let mut num_groups = 0;
            let mut s = 0u32;

            for i in 0..self.n as usize {
                if less[i] == 0 {
                    num_groups += 1;
                    s += 1 << greater[i];
                }
            }

            max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 3
        } else if self.n - 1 < KNOWN_MIN_VALUES.len() as u8 {
            let (less, _unknown, greater) = self.calculate_relations();

            let mut comps = KNOWN_MIN_VALUES[self.n as usize - 1]
                [(self.i as usize).min((self.n - self.i - 1) as usize)];

            comps -= less[0..self.n as usize]
                .iter()
                .filter(|elem| **elem == 1)
                .count() as u8;

            if comps <= max_comparisons {
                return true;
            }

            for i in 0..self.n - 1 {
                if less[i as usize] < 2 {
                    continue;
                }

                'j_loop: for j in i + 1..self.n {
                    if !self.is_less(j, i) {
                        continue;
                    }

                    if greater[j as usize] == 1 {
                        comps -= 1;

                        if comps <= max_comparisons {
                            return true;
                        }
                    } else {
                        for k in 0..self.n {
                            if i != k && j != k && self.is_less(j, k) && self.is_less(k, i) {
                                continue 'j_loop;
                            }
                        }

                        comps -= 1;

                        if comps <= max_comparisons {
                            return true;
                        }
                    }
                }
            }

            comps <= max_comparisons
        } else {
            true
        }
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

    pub fn num_comparisons(&self) -> u8 {
        let (less, _unknown, greater) = self.calculate_relations();

        let mut comps = less[0..self.n as usize]
            .iter()
            .filter(|elem| **elem == 1)
            .count() as u8;

        for i in 0..self.n - 1 {
            if less[i as usize] < 2 {
                continue;
            }

            'j_loop: for j in i + 1..self.n {
                if !self.is_less(j, i) {
                    continue;
                }

                if greater[j as usize] == 1 {
                    comps += 1;
                } else {
                    for k in 0..self.n {
                        if i != k && j != k && self.is_less(j, k) && self.is_less(k, i) {
                            continue 'j_loop;
                        }
                    }

                    comps += 1;
                }
            }
        }

        comps
    }

    pub fn num_compatible_posets(&self) -> usize {
        let canonified = self.canonify_lower_matrix();

        let mut sum = 0;
        for i in 0..canonified.n {
            // assume the ith element is the solution

            let less_than_i = canonified.get_all_less_than(i);
            let greater_than_i = canonified.get_all_greater_than(i);

            let mut less_subsets = Vec::new();
            less_subsets.push(BitSet::empty());

            for j in 0..canonified.n {
                if j == i || greater_than_i.contains(j as usize) {
                    continue;
                }

                let less_than_j = canonified.get_all_less_than(j);

                // try adding j to all previous subsets
                if less_than_i.contains(j as usize) {
                    // all subsets must contain j to be valid

                    let mut next_free = 0;
                    for i in 0..less_subsets.len() {
                        let subset = less_subsets[i];

                        // test if adding j would make a valid subset
                        // we know, that there is no k with p[k] > p[j]
                        if less_than_j.intersect(subset) == less_than_j {
                            let mut new_subset = subset;
                            new_subset.insert(j as usize);
                            less_subsets[next_free] = new_subset;
                            next_free += 1;
                        }
                    }
                    less_subsets.truncate(next_free);
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

    /// Returns all elements, which have an order with i
    fn ordered_with(&self, i: PosetIndex) -> Vec<u8> {
        (0..self.n)
            .filter(|j| i == *j || self.has_order(i, *j))
            .collect()
    }

    fn connected_parts(&self) -> Vec<Vec<PosetIndex>> {
        let mut visited = [false; MAX_N];
        let mut parts = vec![];

        for i in 0..self.n {
            if visited[i as usize] {
                continue;
            }

            let part = self.connected_part(i);

            for &j in &part {
                visited[j as usize] = true;
            }

            parts.push(part);
        }

        parts
    }

    /// Returns all elements connected with i
    fn connected_part(&self, i: PosetIndex) -> Vec<PosetIndex> {
        let mut connected = [false; MAX_N];

        connected[i as usize] = true;

        loop {
            let new_connected = connected
                .iter()
                .enumerate()
                .filter(|(j, is_connected)| {
                    !**is_connected
                        && connected
                            .iter()
                            .enumerate()
                            .any(|(k, c)| *c && self.has_order(*j as PosetIndex, k as PosetIndex))
                })
                .map(|(j, _)| j)
                .collect::<Vec<_>>();

            let has_new = !new_connected.is_empty();

            for j in new_connected {
                connected[j] = true;
            }

            if !has_new {
                break;
            }
        }

        connected
            .into_iter()
            .enumerate()
            .filter(|(_, connected)| *connected)
            .map(|(i, _)| i as u8)
            .collect()
    }

    fn singles(&self) -> usize {
        (0..self.n)
            .filter(|i| !(0..self.n).any(|j| self.has_order(*i, j)))
            .count()
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
    fn test_connected_1() {
        let mut poset = Poset::new(10, 4);

        poset.set_bit(0, 1);
        poset.set_bit(1, 2);
        poset.set_bit(0, 2);
        poset.set_bit(3, 4);
        poset.set_bit(4, 5);
        poset.set_bit(3, 5);

        let mapping = poset.canonify_mapping();

        let connected_1 = poset.connected_part(mapping.iter().position(|i| *i == 1).unwrap() as u8);
        assert_eq!(connected_1.len(), 3);

        assert!(connected_1.contains(&(mapping.iter().position(|i| *i == 0).unwrap() as u8)));
        assert!(connected_1.contains(&(mapping.iter().position(|i| *i == 1).unwrap() as u8)));
        assert!(connected_1.contains(&(mapping.iter().position(|i| *i == 2).unwrap() as u8)));
    }

    #[test]
    fn test_connected_2() {
        let mut poset = Poset::new(10, 4);

        poset.set_bit(0, 1);
        poset.set_bit(1, 2);
        poset.set_bit(0, 2);
        poset.set_bit(3, 4);
        poset.set_bit(4, 5);
        poset.set_bit(3, 5);

        let mapping = poset.canonify_mapping();

        let connected = poset.connected_parts();

        assert_eq!(connected.len(), 6);
        assert!(connected
            .iter()
            .find(
                |part| part.contains(&(mapping.iter().position(|i| *i == 0).unwrap() as u8))
                    && part.contains(&(mapping.iter().position(|i| *i == 1).unwrap() as u8))
                    && part.contains(&(mapping.iter().position(|i| *i == 2).unwrap() as u8))
            )
            .is_some());
        assert!(connected
            .iter()
            .find(
                |part| part.contains(&(mapping.iter().position(|i| *i == 3).unwrap() as u8))
                    && part.contains(&(mapping.iter().position(|i| *i == 4).unwrap() as u8))
                    && part.contains(&(mapping.iter().position(|i| *i == 5).unwrap() as u8))
            )
            .is_some());
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

    #[test]
    fn test() {
        for n in 3..15 {
            for i in 0..(n + 1) / 2 {
                let poset = Poset::new(n, i);
                println!(
                    "n={}, i={}, c={}, {}",
                    n,
                    i,
                    poset.num_compatible_posets().ilog2(),
                    poset.num_compatible_posets()
                );
                assert!(
                    KNOWN_MIN_VALUES[n as usize - 1][i as usize]
                        > poset.num_compatible_posets().ilog2() as u8
                );
            }
        }
    }
}
