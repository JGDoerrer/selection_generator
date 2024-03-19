use std::fmt::Debug;

use crate::{bitset::BitSet, constants::MAX_N};

pub trait Poset: Sized + Debug {
    fn new(n: u8, i: u8) -> Self;
    fn n(&self) -> u8;
    fn i(&self) -> u8;

    /// is i < j?
    fn is_less(&self, i: u8, j: u8) -> bool;

    /// returns a bitset of all elements greater than i
    fn get_all_greater_than(&self, i: u8) -> BitSet;

    /// returns a bitset of all elements less than i
    fn get_all_less_than(&self, i: u8) -> BitSet;

    /// is either i < j or j < i?
    #[inline]
    fn has_order(&self, i: u8, j: u8) -> bool {
        self.is_less(i, j) || self.is_less(j, i)
    }

    /// returns how many elements are less or greater than it
    #[inline]
    fn calculate_relations(&self) -> ([u8; MAX_N], [u8; MAX_N]) {
        let mut less = [0u8; MAX_N];
        let mut greater = [0u8; MAX_N];

        for (i, greater) in greater.iter_mut().enumerate().take(self.n() as usize) {
            *greater = self.get_all_greater_than(i as u8).len() as u8;
        }

        for (i, less) in less.iter_mut().enumerate().take(self.n() as usize) {
            let i_bitset = BitSet::single(i);
            for j in 0..self.n() {
                *less += (!self.get_all_greater_than(j).is_disjoint(i_bitset)) as u8;
            }
        }

        (less, greater)
    }

    /// for debugging
    fn is_lower_triangle_matrix(&self) -> bool {
        for i in 0..self.n() {
            for j in 0..self.n() {
                if self.is_less(i, j) && i > j {
                    return false;
                }
            }
        }
        true
    }

    /// for debugging
    fn is_closed(&self) -> bool {
        for i in 0..self.n() {
            for j in 0..self.n() {
                for k in 0..self.n() {
                    if self.is_less(i, j) && self.is_less(j, k) && !self.is_less(i, k) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn estimate_hardness(&self) -> u32 {
        let (less, greater) = self.calculate_relations();

        let mut sum = 0;

        for i in 0..self.n() as usize {
            sum += (MAX_N as u32 - (self.i() - less[i]) as u32).pow(2);
            sum += (MAX_N as u32 - (self.n() - self.i() - 1 - greater[i]) as u32).pow(2);
        }

        sum
    }

    fn num_compatible_posets(&self) -> usize {
        debug_assert!(self.is_lower_triangle_matrix());

        let all_less_than = {
            let mut bitsets = [BitSet::empty(); MAX_N];
            bitsets
                .iter_mut()
                .take(self.n() as usize)
                .enumerate()
                .for_each(|(i, bs)| *bs = self.get_all_less_than(i as u8));
            bitsets
        };

        let mut less_subsets = Vec::with_capacity(1000);

        let mut sum = 0;
        for i in 0..self.n() as usize {
            // assume the ith element is the solution

            let less_than_i = all_less_than[i];

            if less_than_i.len() == self.i() as usize {
                sum += 1;
                continue;
            }
            if less_than_i.len() > self.i() as usize {
                continue;
            }

            let greater_than_i = self.get_all_greater_than(i as u8);
            let ordered_with_i = less_than_i.union(greater_than_i);

            less_subsets.clear();
            less_subsets.push(less_than_i);

            for j in 0..self.n() as usize {
                if j == i || ordered_with_i.contains(j) {
                    continue;
                }

                let less_than_j = all_less_than[j];

                // try adding j to all previous subsets
                for i in 0..less_subsets.len() {
                    let subset = less_subsets[i];

                    // test if adding j would make a valid subset
                    // we know, that there is no k with p[k] > p[j]
                    if less_than_j.intersect(subset) == less_than_j {
                        let mut new_subset = subset;
                        new_subset.insert(j);
                        less_subsets.push(new_subset);
                    }
                }
            }

            sum += less_subsets
                .iter()
                .filter(|s| s.len() == self.i() as usize)
                .count();
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{canonified_poset::CanonifiedPoset, normal_poset::NormalPoset};

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
                hashset.insert(NormalPoset::new(n, i).with_less(j, k));
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

                        let mut poset = NormalPoset::new(n, i);
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
        let mut poset = NormalPoset::new(4, 0);
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
        assert_eq!(NormalPoset::new(5, 2).num_compatible_posets(), 30); // 5 * (4 choose 2)
        assert_eq!(NormalPoset::new(10, 4).num_compatible_posets(), 1260); // 10 * (9 choose 4)
        let mut poset = NormalPoset::new(10, 4);
        poset.add_and_close(0, 1);
        poset.canonify();
        // dbg!(poset, poset.num_compatible_posets());
        assert_eq!(poset.num_compatible_posets(), 854); // i don't know if this is correct

        let mut poset = NormalPoset::new(10, 4);
        poset.add_and_close(0, 1);
        poset.add_and_close(1, 2);
        poset.canonify();
        // dbg!(poset, poset.num_compatible_posets());
        assert_eq!(poset.num_compatible_posets(), 483); // i don't know if this is correct

        let mut poset = NormalPoset::new(6, 1);
        poset.add_and_close(2, 0);
        poset.add_and_close(3, 0);
        poset.add_and_close(4, 1);
        poset.add_and_close(5, 1);
        poset.canonify();
        // dbg!(poset, poset.num_compatible_posets());
        assert_eq!(poset.num_compatible_posets(), 12); // i don't know if this is correct
    }

    #[test]
    fn a() {
        for i in 0..MAX_N {
            for j in (i + 1)..MAX_N {
                let mut poset = CanonifiedPoset::new(MAX_N as u8, 0);
                poset.set_is_less(i as u8, j as u8);
                assert!(
                    dbg!(poset.get_all_greater_than(i as u8)).contains(j),
                    "{i}, {j}"
                );
            }
        }
    }
}
