use std::fmt::Debug;

use serde::{Deserialize, Serialize};

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
}

impl Poset {
    const BITS: usize = MAX_N * (MAX_N - 1);
    const BYTES: usize = (Self::BITS + 7) / 8;

    pub fn new(n: u8, i: u8) -> Self {
        debug_assert!(n < MAX_N as u8);

        Poset {
            n,
            i,
            adjacency: [0; Self::BYTES],
        }
    }

    pub fn n(&self) -> u8 {
        self.n
    }

    pub fn i(&self) -> u8 {
        self.i
    }

    /// adapted from [https://www.cs.hut.fi/~cessu/selection/selgen.c.html]
    ///
    /// This is important, as is maps isomorphic posets to the same one (hopefully), which reduces the search space dramatically
    pub fn normalize(&mut self) {
        // how many elements are less than it
        let mut less = [0; MAX_N];
        // how many elements are greater than it
        let mut greater = [0; MAX_N];

        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    less[j as usize] += 1;
                    greater[i as usize] += 1;
                }
            }
        }

        // can the element be ignored, because it is too large/small
        let mut dropped = [false; MAX_N];
        let mut n_less_dropped = 0;

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
                new_indices[new_n as usize] = i;
                new_n += 1;
            } else {
                new_indices[b] = i;
                b -= 1;
            }
        }

        if new_n != self.n.into() {
            // recalculate less/greater for the new indices
            for i in 0..new_n {
                for j in 0..new_n {
                    if self.is_less(new_indices[i], new_indices[j]) {
                        less[j as usize] += 1;
                        greater[i as usize] += 1;
                    }
                }
            }
        }

        // a heuristic to sort the elements
        let mut lessness = [0; MAX_N];

        for i in 0..new_n {
            let new_index = new_indices[i] as usize;
            lessness[new_index] =
                MAX_N * MAX_N - greater[new_index] as usize * MAX_N - less[new_index] as usize;
        }

        let topo_order = self.topological_order();

        new_indices[0..self.n as usize].sort_by(|a, b| {
            let i = topo_order.iter().position(|i| *i == *a).unwrap();
            let j = topo_order.iter().position(|i| *i == *b).unwrap();
            lessness[*a as usize]
                .cmp(&lessness[*b as usize])
                .reverse()
                .then(i.cmp(&j))
        });

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
    }

    fn get_bit_index(&self, i: u8, j: u8) -> u8 {
        i + j * (self.n - 1) - if i > j { 1 } else { 0 }
    }

    fn set_bit(&mut self, i: u8, j: u8) {
        debug_assert!(i != j);
        let bit = self.get_bit_index(i, j);
        let byte = bit / 8;
        let mask = 1 << (bit % 8);

        self.adjacency[byte as usize] |= mask;
    }

    fn clear_bit(&mut self, i: u8, j: u8) {
        debug_assert!(i != j);
        let bit = self.get_bit_index(i, j);
        let byte = bit / 8;
        let mask = 1 << (bit % 8);

        self.adjacency[byte as usize] &= !mask;
    }

    /// is i < j?
    pub fn is_less(&self, i: u8, j: u8) -> bool {
        if i == j {
            return false;
        }

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
                if self.is_less(k, i) {
                    self.add_and_close(k, j)
                }
                if self.is_less(j, k) {
                    self.add_and_close(j, k)
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

    /// is either i < j or j < i?
    pub fn has_order(&self, i: u8, j: u8) -> bool {
        self.is_less(i, j) || self.is_less(j, i)
    }

    ///
    pub fn is_solvable_in(&self, max_comparisons: u8) -> bool {
        let mut less = [0i32; MAX_N];
        let mut greater = [0i32; MAX_N];
        let mut unknown = [0i32; MAX_N];

        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    less[j as usize] += 1;
                    greater[i as usize] += 1;
                } else if !self.is_less(j, i) {
                    unknown[i as usize] += 1;
                    unknown[j as usize] += 1;
                }
            }
        }

        if self.i == 0 || self.i == self.n - 1 {
            max_comparisons >= self.n - 1
        } else if self.i == 1 {
            let mut num_groups = 0;
            let mut s = 0u32;

            for i in 0..self.n as usize {
                if greater[i] == 0 {
                    num_groups += 1;
                    s += 1 << less[i];
                }
            }

            max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 2
        } else if self.i == self.n - 2 {
            let mut num_groups = 0;
            let mut s = 0u32;

            for i in 0..self.n as usize {
                if less[i] == 0 {
                    num_groups += 1;
                    s += 1 << greater[i];
                }
            }

            max_comparisons >= num_groups + (u32::BITS - s.leading_zeros()) as u8 - 2
        } else {
            // todo!();
            true
        }
    }

    pub fn dual(&self) -> Self {
        let mut dual = Poset::new(self.n, self.n - self.i - 1);

        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    dual.add_and_close(j, i); // add without normalizing
                }
            }
        }

        dual.normalize();

        // debug_assert_eq!(dual, *self);
        dual
    }

    pub fn topological_order(&self) -> Vec<u8> {
        let mut less = [0; MAX_N];
        let mut greater = [0; MAX_N];

        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    less[j as usize] += 1;
                    greater[i as usize] += 1;
                }
            }
        }

        let mut roots = vec![];

        let mut copy = self.clone();

        for i in 0..self.n {
            if greater[i as usize] == 0 {
                roots.push(i);
            }
        }

        let mut order = vec![];

        while !roots.is_empty() {
            roots.sort_by(|i, j| less[*i as usize].cmp(&less[*j as usize]));

            let root = roots.pop().unwrap();

            order.push(root);

            for j in 0..self.n {
                if copy.is_less(j, root) {
                    copy.clear_bit(j, root);
                    greater[j as usize] -= 1;

                    if greater[j as usize] == 0 {
                        roots.push(j);
                    }
                }
            }
        }

        // dbg!(self, &order);

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
