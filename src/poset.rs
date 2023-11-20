use std::{collections::hash_map::DefaultHasher, fmt::Debug, hash::Hasher};

use serde::{Deserialize, Serialize};

use crate::KNOWN_MIN_VALUES;

pub const MAX_N: usize = 15;

/// A partially ordered set with <
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Poset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    //   i 0 1 2
    // j +------
    // 0 | -
    // 1 |   -
    // 2 |     -
    adjacency: [u8; Self::BYTES],
    /// how many elements are less than it
    less: [u8; MAX_N],
    /// how many elements are greater than it
    greater: [u8; MAX_N],
}

impl Poset {
    const BITS: usize = MAX_N * MAX_N;
    const BYTES: usize = (Self::BITS + 7) / 8;

    pub fn new(n: u8, i: u8) -> Self {
        debug_assert!(n < MAX_N as u8);

        Poset {
            n,
            i,
            adjacency: [0; Self::BYTES],
            less: [0; MAX_N],
            greater: [0; MAX_N],
        }
    }

    pub fn n(&self) -> u8 {
        self.n
    }

    pub fn i(&self) -> u8 {
        self.i
    }

    pub fn less(&self) -> &[u8; MAX_N] {
        &self.less
    }

    pub fn greater(&self) -> &[u8; MAX_N] {
        &self.greater
    }

    fn hash(a: u64, b: u64) -> u64 {
        let mut hash: u64 = 9118271012717746669;

        hash = hash.wrapping_mul(a);
        hash = hash.rotate_left(7);
        hash = hash.wrapping_add(3032928155878307119);
        hash = hash.wrapping_mul(b);
        hash = hash.rotate_right(9);
        hash = hash.wrapping_add(16728691407311227577);

        hash
    }

    /// adapted from [https://www.cs.hut.fi/~cessu/selection/selgen.c.html]
    ///
    /// This is important, as is maps isomorphic posets to the same one (hopefully), which reduces the search space dramatically
    fn normalize(&mut self) {
        self.normalize_mapping();
    }

    fn normalize_mapping(&mut self) -> [u8; MAX_N] {
        debug_assert!(self.check_counts());

        // can the element be ignored, because it is too large/small
        let mut dropped = [false; MAX_N];
        let mut n_less_dropped = 0;

        for i in 0..self.n as usize {
            if self.greater[i] > self.i {
                dropped[i] = true;
            } else if self.less[i] >= self.n - self.i {
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
                new_indices[new_n as usize] = i;
                new_n += 1;
            } else {
                new_indices[b] = i;
                b -= 1;
            }
        }

        if new_n != self.n.into() {
            // recalculate less/greater for the new indices
            self.less = [0; MAX_N];
            self.greater = [0; MAX_N];

            for i in 0..new_n {
                for j in 0..new_n {
                    if self.is_less(new_indices[i], new_indices[j]) {
                        self.less[new_indices[j] as usize] += 1;
                        self.greater[new_indices[i] as usize] += 1;
                    }
                }
            }
        }
        // a heuristic to sort the elements
        let mut lessness = [0; MAX_N];

        for i in 0..new_n {
            let new_index = new_indices[i] as usize;

            lessness[new_index] = MAX_N * MAX_N
                - self.greater[new_index] as usize * MAX_N
                - self.less[new_index] as usize;
        }

        let mut deg_hash = [0; MAX_N];
        let mut hash = [0; MAX_N];

        for i in 0..new_n {
            let i = new_indices[i];
            let i_hash = Self::hash(
                self.less[i as usize].into(),
                self.greater[i as usize].into(),
            );
            deg_hash[i as usize] = i_hash;
            hash[i as usize] = i_hash;
        }

        for _ in 0..3 {
            let mut sum_hash = [0; MAX_N];

            for i in 0..new_n {
                let i = new_indices[i];
                let mut sum = hash[i as usize];

                for j in 0..new_n {
                    let j = new_indices[j];

                    if i == j {
                        continue;
                    }

                    if self.is_less(i, j) {
                        sum = sum.wrapping_add(hash[j as usize]);
                    }
                }

                sum_hash[i as usize] = sum;
            }

            for i in 0..new_n {
                let i = new_indices[i];
                hash[i as usize] = Self::hash(sum_hash[i as usize], deg_hash[i as usize]);
            }
        }

        new_indices[0..new_n].sort_by(|a, b| {
            lessness[*a as usize]
                .cmp(&lessness[*b as usize])
                .then(hash[*a as usize].cmp(&hash[*b as usize]))
                .reverse()
        });
        // new_indices[0..new_n].reverse();

        let mut i = 1;
        while i < new_n {
            if !(lessness[new_indices[i] as usize] == lessness[new_indices[i - 1] as usize]
                && hash[new_indices[i] as usize] == hash[new_indices[i - 1] as usize])
            {
                i += 1;
                continue;
            }

            while i < new_n
                && lessness[new_indices[i] as usize] == lessness[new_indices[i - 1] as usize]
                && hash[new_indices[i] as usize] == hash[new_indices[i - 1] as usize]
            {
                i += 1;
            }

            if i == new_n {
                continue;
            }

            for _ in 0..2 {
                let mut sum_hash = [0; MAX_N];

                for i in i..new_n {
                    let mut sum = hash[new_indices[i] as usize];

                    for j in 0..i {
                        if new_indices[i] == new_indices[j] {
                            continue;
                        }

                        if self.is_less(new_indices[i], new_indices[j]) {
                            sum = sum.wrapping_add(hash[new_indices[j] as usize]);
                        }
                    }

                    sum_hash[new_indices[i] as usize] = sum;
                }

                for i in i..new_n {
                    let i = new_indices[i];
                    hash[i as usize] = Self::hash(sum_hash[i as usize], deg_hash[i as usize]);
                }
            }
            new_indices[i..new_n].sort_by(|a, b| {
                lessness[*a as usize]
                    .cmp(&lessness[*b as usize])
                    .then(hash[*a as usize].cmp(&hash[*b as usize]))
                    .reverse()
            });
        }
        // new_indices[0..new_n].reverse();

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
        debug_assert!(self.check_counts());

        new_indices
    }

    #[inline]
    fn get_bit_index(&self, i: u8, j: u8) -> u8 {
        i + j * self.n
    }

    #[inline]
    fn set_bit(&mut self, i: u8, j: u8) {
        debug_assert!(i != j);
        let bit = self.get_bit_index(i, j);
        let byte = bit / 8;
        let mask = 1 << (bit % 8);

        self.less[j as usize] += 1;
        self.greater[i as usize] += 1;
        self.adjacency[byte as usize] |= mask;
    }

    /// should only be used by the topo sort
    #[inline]
    fn clear_bit(&mut self, i: u8, j: u8) {
        debug_assert!(i != j);
        let bit = self.get_bit_index(i, j);
        let byte = bit / 8;
        let mask = 1 << (bit % 8);

        self.adjacency[byte as usize] &= !mask;
    }

    /// is i < j?
    #[inline]
    pub fn is_less(&self, i: u8, j: u8) -> bool {
        let bit = self.get_bit_index(i, j);
        let byte = bit / 8;
        let mask = 1 << (bit % 8);

        (self.adjacency[byte as usize] & mask) > 0
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

    fn check_counts(&self) -> bool {
        let mut less = [0u8; MAX_N];
        let mut greater = [0u8; MAX_N];
        let mut unknown = [0u8; MAX_N];

        for i in 0..self.n {
            for j in (i + 1)..self.n {
                if self.is_less(i, j) {
                    less[j as usize] += 1;
                    greater[i as usize] += 1;
                } else if self.is_less(j, i) {
                    less[i as usize] += 1;
                    greater[j as usize] += 1;
                } else {
                    unknown[i as usize] += 1;
                    unknown[j as usize] += 1;
                }
            }
        }

        self.less == less && self.greater == greater
    }

    /// adds i < j to the poset and normalize
    pub fn add_less(&mut self, i: u8, j: u8) {
        debug_assert!(!self.is_less(i, j));
        debug_assert!(!self.is_less(j, i));

        self.add_and_close(i, j);
        self.normalize();

        debug_assert!(self.is_closed(), "{self:?}");
    }

    /// adds i < j and makes sure, that i < j && j < k => i < k is true
    pub fn add_and_close(&mut self, i: u8, j: u8) {
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
    pub fn with_less(&self, i: u8, j: u8) -> Self {
        let mut new = self.clone();
        new.add_less(i, j);
        new
    }

    /// returns a clone of the poset, with i < j added
    pub fn with_less_mapping(&self, i: u8, j: u8) -> (Self, [u8; MAX_N]) {
        let mut new = self.clone();

        new.add_and_close(i, j);
        let mapping = new.normalize_mapping();

        (new, mapping)
    }

    /// is either i < j or j < i?
    #[inline]
    pub fn has_order(&self, i: u8, j: u8) -> bool {
        self.is_less(i, j) || self.is_less(j, i)
    }

    /// adapted from [https://www.cs.hut.fi/~cessu/selection/selgen.c.html]
    pub fn is_solvable_in(&self, max_comparisons: u8) -> bool {
        if self.i == 0 || self.i == self.n - 1 {
            max_comparisons >= self.n - 1
        } else if self.i == 1 {
            let mut num_groups = 0;
            let mut s = 0u32;

            for i in 0..self.n as usize {
                if self.greater[i] == 0 {
                    num_groups += 1;
                    s += 1 << self.less[i];
                }
            }

            max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 3
        } else if self.i == self.n - 2 {
            let mut num_groups = 0;
            let mut s = 0u32;

            for i in 0..self.n as usize {
                if self.less[i] == 0 {
                    num_groups += 1;
                    s += 1 << self.greater[i];
                }
            }

            max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 3
        } else if self.n - 1 < KNOWN_MIN_VALUES.len() as u8 {
            let mut comps = KNOWN_MIN_VALUES[self.n as usize - 1]
                [(self.i as usize).min((self.n - self.i - 1) as usize)];

            for i in 0..self.n as usize {
                if self.less[i] == 1 {
                    comps -= 1;
                }

                if comps <= max_comparisons {
                    return true;
                }
            }

            for i in 0..self.n - 1 {
                if self.less[i as usize] < 2 {
                    continue;
                }

                'j_loop: for j in i + 1..self.n {
                    if !self.is_less(j as u8, i as u8) {
                        continue;
                    }

                    if self.greater[j as usize] == 1 {
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

    pub fn dual(&self) -> Self {
        let mut dual = Poset::new(self.n, self.n - self.i - 1);

        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    dual.set_bit(j, i); // add without normalizing
                }
            }
        }

        dual.normalize();

        // debug_assert_eq!(dual, *self);
        dual
    }

    pub fn topological_order(&self) -> Vec<u8> {
        debug_assert!(self.check_counts());

        let mut greater = self.greater.clone();

        let mut roots = vec![];

        let mut copy = self.clone();

        for i in 0..self.n {
            if greater[i as usize] == 0 {
                roots.push(i);
            }
        }

        let mut order = vec![];

        while !roots.is_empty() {
            // roots.sort_by(|i, j| self.less[*i as usize].cmp(&self.less[*j as usize]));

            let root = roots.pop().unwrap();

            order.push(root);

            for j in 0..self.n {
                if copy.is_less(j, root) {
                    copy.clear_bit(j, root);

                    // if greater[j as usize] > 0 {
                    greater[j as usize] -= 1;
                    // }

                    if greater[j as usize] == 0 {
                        roots.push(j);
                    }
                }
            }
        }

        order
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
        let comparisons: Vec<String> = (0..self.n)
            .map(|i| {
                let vecs: Vec<String> = (0..self.n)
                    .map(|j| {
                        if self.is_less(i, j) {
                            vec![format!("{i} < {j}")]
                        } else {
                            vec![]
                        }
                    })
                    .flatten()
                    .collect();
                vecs
            })
            .flatten()
            .collect();

        f.debug_struct("Poset")
            .field("n", &self.n)
            .field("i", &self.i)
            .field("adjacency", &adjacency)
            .field("comparisons", &comparisons)
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
                        poset.normalize();
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
}
