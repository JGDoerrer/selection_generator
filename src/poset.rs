use std::fmt::Debug;

pub const MAX_N: usize = 15;

/// A partially ordered set with <
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Poset {
    /// The number of elements
    n: u8,
    i: u8,
    adjacency: [u8; Self::BYTES],
}

impl Poset {
    const BITS: usize = MAX_N * (MAX_N - 1) / 2;
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

    fn normalize(&mut self) {
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

        // recalculate less/greater for the new indices
        if new_n != self.n as usize {
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

        new_indices[0..self.n as usize]
            .sort_by(|a, b| lessness[*a as usize].cmp(&lessness[*b as usize]).reverse());

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
        debug_assert!(new.is_closed());
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

    /// adds i < j to the poset
    pub fn add_less(&mut self, i: u8, j: u8) {
        debug_assert!(!self.is_less(i, j));

        self.close(i, j);
        self.normalize();

        debug_assert!(self.is_closed(), "{self:?}");
    }

    /// adds i < j and makes sure, that i < j && j < k => i < k is true
    fn close(&mut self, i: u8, j: u8) {
        self.set_bit(i, j);
        for k in 0..self.n {
            if i != k && j != k {
                if self.is_less(k, i) {
                    self.close(k, j)
                }
                if self.is_less(j, k) {
                    self.close(j, k)
                }
            }
        }
    }

    pub fn with_less(&self, (i, j): (u8, u8)) -> Self {
        let mut new = self.clone();
        new.add_less(i, j);
        new
    }

    /// is either i < j or j < i?
    pub fn has_order(&self, (i, j): (u8, u8)) -> bool {
        self.is_less(i, j) || self.is_less(j, i)
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
