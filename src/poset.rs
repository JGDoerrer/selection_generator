use std::{fmt::Debug, os::raw::c_int};

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};
use serde::{Deserialize, Serialize};

use crate::KNOWN_MIN_VALUES;

pub const MAX_N: usize = 15;

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
    fn set_bit(&mut self, i: u8, j: u8) {
        let mask = 1 << j;

        self.adjacency[i as usize] |= mask;
    }

    /// is i < j?
    #[inline]
    pub fn is_less(&self, i: u8, j: u8) -> bool {
        let mask = 1 << j;

        (self.adjacency[i as usize] & mask) != 0
    }

    /// is either i < j or j < i?
    #[inline]
    pub fn has_order(&self, i: u8, j: u8) -> bool {
        self.is_less(i, j) || self.is_less(j, i)
    }

    /// returns how many elements are less, unknown or greater than it
    pub fn calculate_relations(&self) -> ([u8; MAX_N], [u8; MAX_N], [u8; MAX_N]) {
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

        (less, unknown, greater)
    }

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
    fn canonify_mapping(&mut self) -> [u8; MAX_N] {
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
            for i in 0..self.n {
                let i = i as usize;
                hash[i] = Self::hash(sum_hash[i], in_out_degree[i]);
            }
        }

        let mut new_indices: [u8; MAX_N] = (0..MAX_N as u8).collect::<Vec<_>>().try_into().unwrap();

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

        // dbg!(&self, &new);
        debug_assert!(new.is_closed(), "{new:?}");
        *self = new;

        new_indices
    }

    /// Removes elements, that are known to be too large/small
    fn reduce_elements(&mut self) -> [u8; MAX_N] {
        // can the element be ignored, because it is too large/small
        let mut dropped = [false; MAX_N];
        let mut n_less_dropped = 0;

        let (less, _unknown, greater) = self.calculate_relations();

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
    pub fn add_less(&mut self, i: u8, j: u8) {
        debug_assert!(!self.is_less(i, j));
        debug_assert!(!self.is_less(j, i));

        self.add_and_close(i, j);
        self.canonify();

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
        let mut new = *self;
        new.add_less(i, j);
        new
    }

    /// returns a clone of the poset, with i < j added
    pub fn with_less_mapping(&self, i: u8, j: u8) -> (Self, [u8; MAX_N]) {
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
}
