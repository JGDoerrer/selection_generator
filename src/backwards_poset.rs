use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};

use crate::backward_cache::BackwardCache;
use crate::bitset::BitSet;
use crate::constants::MAX_N;
use crate::free_poset::FreePoset;
use crate::poset::Poset;
use crate::search_backward::{COUTNER_USE_NAUTY, COUTNER_USE_NOT_NAUTY};

/// A partially ordered set with <
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BackwardsPoset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    adjacency: [BitSet; MAX_N],
}

// impl Hash for BackwardsPoset {
//     fn hash<H: Hasher>(&self, ra_expand_state: &mut H) {
//         // self.n.hash(ra_expand_state);
//         // self.i.hash(ra_expand_state);
//         self.adjacency.hash(ra_expand_state);
//     }
// }

impl Debug for BackwardsPoset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "n = {}, i = {}", self.n, self.i)?;

        for i in 0..self.n {
            writeln!(f)?;
            for j in 0..self.n {
                if self.is_less(i, j) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
            }
        }

        Ok(())
    }
}

impl BackwardsPoset {
    // constructor
    pub fn new(n: u8, i: u8) -> Self {
        debug_assert!(n as usize <= MAX_N);

        Self {
            n,
            i,
            adjacency: [BitSet::empty(); MAX_N],
        }
    }

    // getter
    pub fn n(&self) -> u8 {
        self.n
    }

    pub fn i(&self) -> u8 {
        self.i
    }

    pub fn is_less(&self, i: u8, j: u8) -> bool {
        self.adjacency[i as usize].contains(j as usize)
    }

    pub fn set_less(&mut self, i: u8, j: u8, value: bool) {
        if value {
            self.adjacency[i as usize].insert(j as usize);
        } else {
            self.adjacency[i as usize].remove(j as usize);
        }
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

    pub fn add_less(&mut self, i: u8, j: u8) {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed());
        // TODO: could also add assert !is_less(i, j)

        if !self.is_less(i, j) {
            self.add_and_close_recursive(i, j);
        }

        // postcondition
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(!self.is_less(j, i));
        debug_assert!(self.is_less(i, j));
        // debug_assert!(self.is_closed());
    }

    pub fn with_less(&self, i: u8, j: u8) -> Self {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed());

        let mut poset = *self;
        poset.add_less(i, j);

        // postcondition
        debug_assert!(poset.i < poset.n);
        debug_assert!(poset.n as usize <= MAX_N);
        debug_assert!(!poset.is_less(j, i));
        debug_assert!(poset.is_less(i, j));
        // debug_assert!(poset.is_closed());
        debug_assert!(!poset.is_redundant(i, j));

        poset
    }

    pub fn with_less_normalized(&self, i: u8, j: u8) -> Self {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed());

        let mut poset = *self;
        poset.add_less(i, j);
        poset.normalize();

        // postcondition
        debug_assert!(poset.i < poset.n);
        debug_assert!(poset.n as usize <= MAX_N);
        // debug_assert!(poset.is_closed());
        // debug_assert!(poset.is_normalized());

        poset
    }

    // reduce
    pub fn calculate_relations(&self) -> ([u8; MAX_N], [u8; MAX_N]) {
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
            // if i != k && j != k {
            self.swap_positions(i, k, j, k);
            self.swap_positions(k, i, k, j);
            // }
        }
    }
    fn can_be_swapped(&self, i: u8, j: u8) -> bool {
        for k in 0..self.n {
            if self.is_less(i, k) != self.is_less(j, k) || self.is_less(k, i) != self.is_less(k, j)
            {
                return false;
            }
        }
        true
    }

    fn dual(&mut self) {
        self.i = (self.n - 1) - self.i;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                self.swap_positions(i, j, j, i);
            }
        }
    }

    // canonify
    fn canonify_nauty_indices(&self) -> [u8; MAX_N] {
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

        let mut result = [0u8; MAX_N];
        for i in 0..self.n as usize {
            result[i] = labels[i] as u8;
        }
        result
    }

    pub fn canonify(&mut self) {
        *self = self.canonify_transform_indices().0;
    }

    pub fn canonify_transform_indices(&self) -> (BackwardsPoset, ([u8; MAX_N], bool)) {
        if self.n - 1 == 2 * self.i {
            let mut canonified = *self;
            let indices = canonified.canonify_without_dual();

            let mut canonified_dual = *self;
            canonified_dual.dual();
            let indices_dual = canonified_dual.canonify_without_dual();

            let mut is_dual = false;
            'break_all: for i in 0..self.n {
                for j in 0..self.n {
                    if canonified.is_less(i, j) != canonified_dual.is_less(i, j) {
                        is_dual = canonified_dual.is_less(i, j);
                        break 'break_all;
                    }
                }
            }
            if is_dual {
                (canonified_dual, (indices_dual, true))
            } else {
                (canonified, (indices, false))
            }
        } else if self.n <= 2 * self.i {
            let mut canonified_dual = *self;
            canonified_dual.dual();
            let indices_dual = canonified_dual.canonify_without_dual();
            (canonified_dual, (indices_dual, true))
        } else {
            let mut canonified = *self;
            let indices = canonified.canonify_without_dual();
            (canonified, (indices, false))
        }
    }

    pub fn apply_indices(&self, indices: [u8; MAX_N]) -> Self {
        let mut new_poset = *self;
        for i in 0..self.n {
            for j in 0..self.n {
                new_poset.set_less(i, j, self.is_less(indices[i as usize], indices[j as usize]));
            }
        }
        new_poset
    }

    pub fn canonify_indices(
        n: u8,
        (indices, is_dual): ([u8; MAX_N], bool),
        (i, j): (u8, u8),
    ) -> (u8, u8) {
        let mut transformed = [0u8; MAX_N];
        for k in 0..n {
            transformed[indices[k as usize] as usize] = k as u8;
        }

        if is_dual {
            (transformed[j as usize], transformed[i as usize])
        } else {
            (transformed[i as usize], transformed[j as usize])
        }
    }

    pub fn decide_deterministic_internal(
        &self,
        indices_0: &[u8; MAX_N],
        indices_1: &[u8; MAX_N],
    ) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(indices_0[i as usize], indices_0[j as usize])
                    != self.is_less(indices_1[i as usize], indices_1[j as usize])
                {
                    return self.is_less(indices_0[i as usize], indices_0[j as usize]);
                }
            }
        }
        true
    }

    pub fn canonify_pairwise_elements(
        &self,
        equal_items: &Vec<(usize, usize)>,
        index: usize,
        mut new_indices: [u8; MAX_N],
    ) -> [u8; MAX_N] {
        if index == equal_items.len() {
            return new_indices;
        }

        let indices_0 = self.canonify_pairwise_elements(equal_items, index + 1, new_indices);

        new_indices.swap(equal_items[index].0, equal_items[index].1);
        let indices_1 = self.canonify_pairwise_elements(equal_items, index + 1, new_indices);

        if self.decide_deterministic_internal(&indices_0, &indices_1) {
            indices_0
        } else {
            indices_1
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn canonify_without_dual(&mut self) -> [u8; MAX_N] {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        // debug_assert!(self.is_closed());

        let (less, greater) = self.calculate_relations();

        let mut in_out_degree = [0; MAX_N];
        for i in 0..self.n as usize {
            in_out_degree[i] = greater[i] as u64 * MAX_N as u64 + less[i] as u64;
        }

        let mut hash = in_out_degree;
        for _ in 0..3 {
            let mut sum_hash = hash;

            for i in 0..self.n {
                for j in 0..self.n {
                    if i != j && (self.is_less(i, j) || self.is_less(j, i)) {
                        sum_hash[i as usize] = sum_hash[i as usize].wrapping_add(hash[j as usize]);
                    }
                }
            }

            // calc new hash based on neighbours hashes
            for i in 0..self.n as usize {
                hash[i] = sum_hash[i]
                    .wrapping_mul(MAX_N.pow(2) as u64)
                    .wrapping_add(in_out_degree[i]);
            }
        }

        let hash = hash;

        let comparator = |&a: &u8, &b: &u8| {
            in_out_degree[a as usize]
                .cmp(&in_out_degree[b as usize])
                .then_with(|| hash[a as usize].cmp(&hash[b as usize]))
        };

        let mut new_indices = [0u8; MAX_N];
        new_indices
            .iter_mut()
            .enumerate()
            .take(self.n as usize)
            .for_each(|(i, index)| *index = i as u8);
        new_indices[0..self.n as usize].sort_unstable_by(comparator);

        let mut is_unique = true;
        let mut can_canonify_pairwise = true;
        let mut equal_items: Vec<(usize, usize)> = vec![];
        let mut index = 1;
        while index < self.n as usize {
            let begin = index - 1;
            while index < self.n as usize
                && comparator(&new_indices[index - 1], &new_indices[index]).is_eq()
            {
                index += 1;
            }
            let (from, to) = (begin, index - 1);
            if from != to {
                debug_assert!(from < to);
                let mut last = from;
                let mut delete = true;

                for new_one in (from + 1)..=to {
                    if !self.can_be_swapped(new_indices[last], new_indices[new_one]) {
                        delete = false;
                        break;
                    }
                    last = new_one;
                }

                if !delete {
                    is_unique = false;
                    if from + 1 != to {
                        can_canonify_pairwise = false;
                        break;
                    }
                    equal_items.push((from, to));
                }
            }
            index += 1;
        }

        if !is_unique && can_canonify_pairwise {
            is_unique = true;
            new_indices = self.canonify_pairwise_elements(&equal_items, 0, new_indices);
        }

        if is_unique {
            COUTNER_USE_NOT_NAUTY.inc();
        } else {
            COUTNER_USE_NAUTY.inc();
            new_indices = self.canonify_nauty_indices();
        }

        *self = self.apply_indices(new_indices);

        // postcondition
        // debug_assert!(2 * self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        // debug_assert!(self.is_closed());
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                debug_assert!(!self.is_less(i, j));
            }
        }

        new_indices
    }

    pub fn reduce_elements(&mut self) -> [u8; MAX_N] {
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(self.is_closed());

        let (less, greater) = self.calculate_relations();

        let mut new_indices = [0u8; MAX_N];
        let mut n_less_dropped = 0;
        let mut new_n = 0u8;

        for i in 0..self.n {
            if self.i < less[i as usize] {
            } else if (self.n - 1) - self.i < greater[i as usize] {
                n_less_dropped += 1;
            } else {
                new_indices[new_n as usize] = i;
                new_n += 1;
            }
        }

        if new_n != self.n {
            let mut new_poset = Self::new(new_n, self.i - n_less_dropped);
            for i in 0..new_poset.n {
                for j in 0..new_poset.n {
                    new_poset.set_less(
                        i,
                        j,
                        self.is_less(new_indices[i as usize] as u8, new_indices[j as usize] as u8),
                    );
                }
            }
            *self = new_poset;
        }

        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(self.is_closed());

        new_indices
    }

    pub fn can_reduce_any_element(&self) -> bool {
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(self.is_closed());

        let (less, greater) = self.calculate_relations();

        for i in 0..self.n {
            if self.i < less[i as usize] || (self.n - 1) - self.i < greater[i as usize] {
                return true;
            }
        }

        false
    }

    // normalize
    pub fn normalize(&mut self) {
        self.reduce_elements();
        self.canonify();
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

    pub fn count_min_comparisons(&self) -> usize {
        let mut counter = 0;
        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) && !self.is_redundant(i, j) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn handle_poset(
        current_bucket: &mut [(
            HashSet<(Self, (u8, u8))>,
            HashSet<(Self, (u8, u8))>,
        ); MAX_N],
        poset: Self,
        indices: (u8, u8),
        poset_cache: &BackwardCache,
        removed: &mut HashSet<(Self, (u8, u8))>,
    ) {
        let current = &mut current_bucket[poset.i as usize];
        if !current.0.contains(&(poset, indices)) && !current.1.contains(&(poset, indices)) {
            if poset_cache.contains(&poset.with_less_normalized(indices.1, indices.0)) {
                removed.insert((poset, indices));
                current.0.insert((poset, indices));
            } else {
                current.1.insert((poset, indices));
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn enlarge_and_remove_less(
        &self,
        interrupt: &Arc<AtomicBool>,
        poset_cache: &BackwardCache,
        table: &[[bool; MAX_N]; MAX_N],
        n: u8,
        i: u8,
        max_remaining_comparisons: usize,
    ) -> HashMap<Self, (u8, u8)> {
        assert!(2 * self.i < self.n);
        assert!(1 <= self.n);
        assert!(table[self.n as usize - 1][self.i as usize]);

        let mut current_level: [(HashSet<(Self, (u8, u8))>, HashSet<(Self, (u8, u8))>); MAX_N] =
            Default::default();
        let mut removed = HashSet::new();
        self.remove_less(
            &mut current_level,
            max_remaining_comparisons,
            poset_cache,
            &mut removed,
        );

        if self.n != n || self.i != i {
            if interrupt.load(Ordering::Relaxed) {
                return HashMap::new();
            }

            let mut next_level: [(HashSet<(Self, (u8, u8))>, HashSet<(Self, (u8, u8))>); MAX_N] =
                Default::default();
            if table[self.n as usize][self.i as usize] {
                self.enlarge_n(
                    &mut next_level,
                    max_remaining_comparisons,
                    poset_cache,
                    &mut removed,
                );
            }

            let condition = 2 * (self.i + 1) < self.n + 1;
            if (condition && table[self.n as usize][self.i as usize + 1])
                || (!condition && table[self.n as usize][self.n as usize - self.i as usize - 1])
            {
                self.enlarge_nk(
                    &mut next_level,
                    max_remaining_comparisons,
                    poset_cache,
                    &mut removed,
                );
            }

            if interrupt.load(Ordering::Relaxed) {
                return HashMap::new();
            }

            for n0 in self.n..n {
                for i0 in self.i..=i {
                    for (item, indices) in &current_level[i0 as usize].0 {
                        if table[n0 as usize][i0 as usize] {
                            item.super_enlarge_n(
                                &mut next_level,
                                max_remaining_comparisons,
                                poset_cache,
                                &mut removed,
                                *indices,
                            );
                        }

                        let condition = 2 * (i0 + 1) < n0 + 1;
                        if (condition && table[n0 as usize][i0 as usize + 1])
                            || (!condition
                                && i0 < n0
                                && table[n0 as usize][n0 as usize - i0 as usize - 1])
                        {
                            item.super_enlarge_nk(
                                &mut next_level,
                                max_remaining_comparisons,
                                poset_cache,
                                &mut removed,
                                *indices,
                            );
                        }
                    }

                    if interrupt.load(Ordering::Relaxed) {
                        return HashMap::new();
                    }
                }
                current_level = next_level;
                next_level = Default::default();
            }
        }

        let mut result = HashMap::new();
        for (item, indices) in removed {
            if !poset_cache.contains(&item) {
                result.insert(item, indices);
            }
        }

        result
    }

    pub fn remove_less(
        &self,
        poset_buckets: &mut [(
            HashSet<(BackwardsPoset, (u8, u8))>,
            HashSet<(BackwardsPoset, (u8, u8))>,
        ); MAX_N],
        max_remaining_comparisons: usize,
        poset_cache: &BackwardCache,
        removed: &mut HashSet<(BackwardsPoset, (u8, u8))>,
    ) {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!(self.n as usize <= MAX_N);
        debug_assert!(self.is_closed());
        // debug_assert!(self.is_canonified());

        let mut result = HashSet::new();
        for i in 0..self.n {
            for j in 0..self.n {
                if !self.is_less(i, j) || self.is_redundant(i, j) {
                    continue;
                }

                let mut poset_initial = *self;
                poset_initial.set_less(i, j, false);

                result.insert((poset_initial, i, j));

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

                            let mut poset_next = poset;
                            poset_next.set_less(i1, j1, false);

                            if result.contains(&(poset_next, i, j))
                                || *self != poset_next.with_less(i, j)
                            {
                                continue;
                            }

                            result.insert((poset_next, i, j));
                            queue.push(poset_next);
                        }
                    }
                }
            }
        }

        // // postcondition:
        // for item in &result {
        //   debug_assert!(item.i < item.n);
        //   debug_assert!((item.n as usize) <= MAX_N);
        //   debug_assert!(item.is_closed());
        //   debug_assert!(item.is_canonified());
        //   debug_assert!({
        //     let mut is_possible = false;
        //     'all_for: for i in 0..item.n {
        //       for j in 0..item.n {
        //         if i != j && !item.is_less(i, j) && !item.is_less(j, i) {
        //           let mut temp = item.with_less(i, j);
        //           temp.canonify();
        //           if &temp == self {
        //             is_possible = true;
        //             break 'all_for;
        //           }
        //         }
        //       }
        //     }
        //     is_possible
        //   });
        // }

        for (item, i, j) in result {
            debug_assert!(!item.can_reduce_any_element());
            if max_remaining_comparisons < item.count_min_comparisons() {
                continue;
            }
            let (canonified, indices) = item.canonify_transform_indices();
            Self::handle_poset(
                poset_buckets,
                canonified,
                Self::canonify_indices(canonified.n, indices, (i, j)),
                poset_cache,
                removed,
            );
        }
    }

    // fn assert_is_canonified(&self) -> bool {
    //     let mut canon = self.clone();
    //     canon.canonify();
    //     assert_eq!(self, &canon);
    //     true
    // }

    fn can_reduce_element_greater(&self, element: u8) -> bool {
        let mut greater = 0u8;
        for k in 0..self.n {
            if self.is_less(k, element) {
                greater += 1;
            }
        }
        self.i < greater
    }

    pub fn super_enlarge_n(
        &self,
        poset_buckets: &mut [(
            HashSet<(BackwardsPoset, (u8, u8))>,
            HashSet<(BackwardsPoset, (u8, u8))>,
        ); MAX_N],
        max_remaining_comparisons: usize,
        poset_cache: &BackwardCache,
        removed: &mut HashSet<(BackwardsPoset, (u8, u8))>,
        (k, j): (u8, u8),
    ) {
        debug_assert!(!self.is_less(k, j) && !self.is_less(j, k));

        let mut init_poset = Self::new(self.n + 1, self.i);
        for i in 0..self.n {
            for j in 0..self.n {
                init_poset.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut unfiltered = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((init_poset, 0, self.count_min_comparisons()));
        while let Some((poset, number, min_comparisons_done)) = swap_init.pop_back() {
            for index in number..(poset.n - 1) {
                if poset.is_less(index, poset.n - 1)
                    || poset.is_less(poset.n - 1, index)
                    || max_remaining_comparisons < min_comparisons_done + 1
                {
                    continue;
                }

                let new_poset = poset.with_less(index, poset.n - 1);
                if new_poset.is_less(j, k)
                    || new_poset.is_less(k, j)
                    || new_poset.can_reduce_any_element()
                {
                    continue;
                }

                swap_init.push_back((new_poset, index + 1, min_comparisons_done + 1));

                if new_poset
                    .with_less(k, j)
                    .can_reduce_element_greater(poset.n - 1)
                {
                    unfiltered.insert(new_poset);
                }
            }
        }

        for item in unfiltered {
            let (canonified, indices) = item.canonify_transform_indices();
            let new_indices = Self::canonify_indices(canonified.n, indices, (k, j));
            Self::handle_poset(poset_buckets, canonified, new_indices, poset_cache, removed);
        }
    }

    fn enlarge_n(
        &self,
        poset_buckets: &mut [(
            HashSet<(BackwardsPoset, (u8, u8))>,
            HashSet<(BackwardsPoset, (u8, u8))>,
        ); MAX_N],
        max_remaining_comparisons: usize,
        poset_cache: &BackwardCache,
        removed: &mut HashSet<(BackwardsPoset, (u8, u8))>,
    ) {
        let mut init_poset = Self::new(self.n + 1, self.i);
        for i in 0..self.n {
            for j in 0..self.n {
                init_poset.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut enlarged = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((init_poset, 0));
        while let Some((poset, number)) = swap_init.pop_back() {
            for k in number..(poset.n - 1) {
                if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
                    let new_poset = poset.with_less(k, poset.n - 1);
                    if !new_poset.can_reduce_element_greater(new_poset.n - 1) {
                        swap_init.push_back((new_poset, k + 1));
                    }
                }
            }
            enlarged.insert(poset);
        }

        for item in &enlarged {
            if max_remaining_comparisons < item.count_min_comparisons() {
                continue;
            }
            let mut canonified = None;
            for k in 0..item.n - 1 {
                if !item.is_less(k, item.n - 1)
                    && !item.is_less(item.n - 1, k)
                    && item
                        .with_less(k, item.n - 1)
                        .can_reduce_element_greater(item.n - 1)
                {
                    if canonified.is_none() {
                        canonified = Some(item.canonify_transform_indices());
                    }
                    if let Some((canonified, indices)) = canonified {
                        Self::handle_poset(
                            poset_buckets,
                            canonified,
                            Self::canonify_indices(canonified.n, indices, (k, item.n - 1)),
                            poset_cache,
                            removed,
                        );
                    }
                }
            }
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

    pub fn super_enlarge_nk(
        &self,
        poset_buckets: &mut [(
            HashSet<(BackwardsPoset, (u8, u8))>,
            HashSet<(BackwardsPoset, (u8, u8))>,
        ); MAX_N],
        max_remaining_comparisons: usize,
        poset_cache: &BackwardCache,
        removed: &mut HashSet<(BackwardsPoset, (u8, u8))>,
        (k, j): (u8, u8),
    ) {
        debug_assert!(!self.is_less(k, j) && !self.is_less(j, k));

        let mut init_poset = Self::new(self.n + 1, self.i + 1);
        for i in 0..self.n {
            for j in 0..self.n {
                init_poset.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut unfiltered = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((init_poset, 0, self.count_min_comparisons()));
        while let Some((poset, number, min_comparisons_done)) = swap_init.pop_back() {
            for index in number..(poset.n - 1) {
                if poset.is_less(index, poset.n - 1)
                    || poset.is_less(poset.n - 1, index)
                    || max_remaining_comparisons < min_comparisons_done + 1
                {
                    continue;
                }

                let new_poset = poset.with_less(poset.n - 1, index);
                if new_poset.is_less(j, k)
                    || new_poset.is_less(k, j)
                    || new_poset.can_reduce_any_element()
                {
                    continue;
                }

                swap_init.push_back((new_poset, index + 1, min_comparisons_done + 1));

                if new_poset
                    .with_less(k, j)
                    .can_reduce_element_less(poset.n - 1)
                {
                    unfiltered.insert(new_poset);
                }
            }
        }

        for item in unfiltered {
            let (canonified, indices) = item.canonify_transform_indices();
            let new_indices = Self::canonify_indices(canonified.n, indices, (k, j));
            Self::handle_poset(poset_buckets, canonified, new_indices, poset_cache, removed);
        }
    }

    fn enlarge_nk(
        &self,
        poset_buckets: &mut [(
            HashSet<(BackwardsPoset, (u8, u8))>,
            HashSet<(BackwardsPoset, (u8, u8))>,
        ); MAX_N],
        max_remaining_comparisons: usize,
        poset_cache: &BackwardCache,
        removed: &mut HashSet<(BackwardsPoset, (u8, u8))>,
    ) {
        let mut init_poset = Self::new(self.n + 1, self.i + 1);
        for i in 0..self.n {
            for j in 0..self.n {
                init_poset.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut enlarged = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((init_poset, 0));
        while let Some((poset, number)) = swap_init.pop_back() {
            for k in number..(poset.n - 1) {
                if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
                    let new_poset = poset.with_less(poset.n - 1, k);
                    if !new_poset.can_reduce_element_less(new_poset.n - 1) {
                        swap_init.push_back((new_poset, k + 1));
                    }
                }
            }
            enlarged.insert(poset);
        }

        for item in &enlarged {
            if max_remaining_comparisons < item.count_min_comparisons() {
                continue;
            }
            let mut canonified = None;
            for k in 0..item.n - 1 {
                if !item.is_less(item.n - 1, k)
                    && !item.is_less(k, item.n - 1)
                    && item
                        .with_less(item.n - 1, k)
                        .can_reduce_element_less(item.n - 1)
                {
                    if canonified.is_none() {
                        canonified = Some(item.canonify_transform_indices());
                    }
                    if let Some((canonified, indices)) = canonified {
                        Self::handle_poset(
                            poset_buckets,
                            canonified,
                            Self::canonify_indices(canonified.n, indices, (item.n - 1, k)),
                            poset_cache,
                            removed,
                        );
                    }
                }
            }
        }
    }

    pub fn rec_temp(table: &mut [[bool; MAX_N]; MAX_N], n: usize, i: usize) {
        table[n - 1][i] = true;
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

    fn is_closed(&self) -> bool {
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

    pub fn to_normal(&self) -> FreePoset {
        let mut result = FreePoset::new(self.n, self.i);
        for i in 0..self.n {
            for j in 0..self.n {
                if self.is_less(i, j) {
                    result.set_less(i, j, true);
                }
            }
        }
        result
    }

    pub fn with_less_mapping(&self, i: u8, j: u8) -> (Self, ([u8; MAX_N], bool)) {
        let mut new = *self;

        new.add_less(i, j);
        let mapping_reduce = new.reduce_elements();
        let (poset, (mapping_canonify, is_dual)) = new.canonify_transform_indices();
        new = poset;

        let mut mapping = [0; MAX_N];
        for i in 0..MAX_N {
            mapping[i] = mapping_reduce[mapping_canonify[i] as usize];
        }

        (new, (mapping, is_dual))
    }
}
