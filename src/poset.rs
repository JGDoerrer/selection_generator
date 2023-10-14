use std::fmt::Debug;

/// A partially ordered set with <
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Poset {
    n: u8,
    adjacency: Vec<u8>,
}

impl Poset {
    pub fn new(n: u8) -> Self {
        let bits = n as usize * n as usize;
        let bytes = (bits + 7) / 8;

        Poset {
            n,
            adjacency: vec![0; bytes],
        }
    }

    pub fn n(&self) -> u8 {
        self.n
    }

    /// i < j
    pub fn add_less(&self, (i, j): (u8, u8)) -> Self {
        let bit = i + j * self.n;
        let byte = bit / 8;
        let mask = 1 << (bit % 8);

        let mut new = self.clone();
        new.adjacency[byte as usize] |= mask;
        new
    }

    /// is i < j?
    pub fn is_less(&self, (i, j): (u8, u8)) -> bool {
        let bit = i + j * self.n;
        let byte = bit / 8;
        let mask = 1 << (bit % 8);

        (self.adjacency[byte as usize] & mask) > 0
    }

    /// is i < ... < j?
    pub fn is_less_rec(&self, (i, j): (u8, u8)) -> bool {
        if self.is_less((i, j)) {
            true
        } else {
            for k in 0..self.n {
                if self.is_less((i, k)) && self.is_less_rec((k, j)) {
                    return true;
                }
            }

            false
        }
    }

    pub fn has_order(&self, (i, j): (u8, u8)) -> bool {
        self.is_less((i, j)) || self.is_less((j, i))
    }

    pub fn is_valid_less(&self, (i, j): (u8, u8)) -> bool {
        !self.is_less_rec((j, i))
    }

    pub fn is_valid_comp(&self, (i, j): (u8, u8)) -> bool {
        self.is_valid_less((j, i)) && self.is_valid_less((j, i))
    }

    pub fn get_ith_element(&self, i: u8) -> Option<u8> {
        for j in 0..self.n {
            let less_count = self.elements_less_than(j);
            let greater_count = self.elements_greater_than(j);

            if less_count == i && less_count + greater_count + 1 == self.n {
                return Some(j);
            }
        }

        None
    }

    pub fn elements_less_than(&self, index: u8) -> u8 {
        let mut count = 0;
        for i in 0..self.n {
            if self.is_less((i, index)) {
                count += 1;
            }
        }
        count
    }

    pub fn elements_greater_than(&self, index: u8) -> u8 {
        let mut count = 0;
        for i in 0..self.n {
            if self.is_less((index, i)) {
                count += 1;
            }
        }
        count
    }

    pub fn next_posets(&self) -> Vec<Poset> {
        let mut posets = vec![];

        for i in 0..self.n {
            for j in 0..self.n {
                if i == j || self.is_less((j, i)) {
                    continue;
                }

                let pair = (i, j);
                if !self.is_less(pair) {
                    posets.push(self.add_less(pair));
                }
            }
        }

        posets
    }

    pub fn do_comparison(&self, (i, j): (u8, u8)) -> (Poset, Poset) {
        let less = self.add_less((i, j));
        let greater = self.add_less((j, i));

        (less, greater)
    }

    pub fn reduce(&self, i: u8) -> Self {
        let mut new = Poset::new(self.n - 1);

        for j in 0..self.n {
            for k in 0..self.n {
                if j == i || k == i {
                    continue;
                }

                if self.is_less((j, k)) {
                    let j = if j > i { j - 1 } else { j };
                    let k = if k > i { k - 1 } else { k };
                    new = new.add_less((j, k));
                }
            }
        }

        new
    }
}

impl Debug for Poset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // nicer debug output
        let adjacency: Vec<String> = (0..self.n)
            .map(|i| {
                (0..self.n)
                    .map(|j| if self.is_less((i, j)) { '1' } else { '0' })
                    .collect()
            })
            .collect();

        f.debug_struct("Poset")
            .field("n", &self.n)
            .field("adjacency", &adjacency)
            .finish()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_less_rec() {
        let poset = Poset::new(5)
            .add_less((0, 1))
            .add_less((1, 2))
            .add_less((2, 3));

        assert!(poset.is_less_rec((0, 3)));
        assert!(poset.is_less_rec((1, 3)));
        assert!(poset.is_less_rec((0, 2)));
    }

    #[test]
    fn test_valid_comparison() {
        let poset = Poset::new(3).add_less((0, 1)).add_less((1, 2));

        assert!(!poset.is_valid_comp((0, 2)));

        let poset = Poset::new(3).add_less((0, 1)).add_less((2, 0));

        assert!(!poset.is_valid_comp((2, 1)));

        let poset = Poset::new(5)
            .add_less((0, 1))
            .add_less((1, 2))
            .add_less((2, 3))
            .add_less((3, 4));

        assert!(!poset.is_valid_comp((0, 4)));
    }
}
