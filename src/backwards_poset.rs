use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};

use crate::constants::MAX_N;

const fn init_table() -> [([(u8, u8); MAX_N * MAX_N], usize); MAX_N] {
    let mut table1 = [([(0u8, 0u8); MAX_N * MAX_N], 0); MAX_N];
    table1[0] = ([(0, 0); MAX_N * MAX_N], 0);
    table1[1] = ([(0, 0); MAX_N * MAX_N], 1);
    let mut n = 2;
    while n < MAX_N {
        table1[n].1 = (n * n - n) / 2;
        let mut pos = 0;
        while pos < table1[n].1 {
            let mut a = 0;
            let mut k = 0;
            while k < MAX_N {
                if pos < (k * k + k) / 2 {
                    break;
                }
                a = k;
                k += 1;
            }
            let b: usize = pos - ((a * a + a) / 2);
            table1[n].0[pos] = ((a + 1) as u8, b as u8);
            pos += 1;
        }
        n += 1;
    }
    table1
}
const TABLE_ORDER: [([(u8, u8); MAX_N * MAX_N], usize); MAX_N] = init_table();

/// A partially ordered set with <
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BackwardsPoset {
    /// The number of elements
    n: u8,
    i: u8,
    /// The comparisons as an adjacency matrix
    adjacency: [u16; MAX_N],
}

impl fmt::Debug for BackwardsPoset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        // debug_assert!(i < n);
        debug_assert!((n as usize) < MAX_N);

        Self {
            n,
            i,
            adjacency: [0; MAX_N],
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
        0 != self.adjacency[i as usize] & (1 << (j as usize))
    }

    pub fn set_less(&mut self, i: u8, j: u8, value: bool) {
        if value {
            self.adjacency[i as usize] |= 1 << (j as usize);
        } else {
            self.adjacency[i as usize] &= !(1 << (j as usize));
        }
    }

    pub fn adjacency_size(&self) -> usize {
        TABLE_ORDER[self.n as usize].1
    }

    pub fn is_index(&self, pos: usize) -> bool {
        let item = TABLE_ORDER[self.n as usize].0[pos];
        self.is_less(item.0, item.1)
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
        debug_assert!((self.n as usize) < MAX_N);
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
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(!self.is_less(j, i));
        debug_assert!(self.is_less(i, j));
        // debug_assert!(self.is_closed());
    }

    pub fn with_less(&self, i: u8, j: u8) -> Self {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed());

        let mut poset = self.clone();
        poset.add_less(i, j);

        // postcondition
        debug_assert!(poset.i < poset.n);
        debug_assert!((poset.n as usize) < MAX_N);
        debug_assert!(!poset.is_less(j, i));
        debug_assert!(poset.is_less(i, j));
        // debug_assert!(poset.is_closed());
        debug_assert!(!poset.is_redundant(i, j));

        poset
    }

    pub fn with_less_normalized(&self, i: u8, j: u8) -> Self {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        debug_assert_ne!(i, j);
        debug_assert!(!self.is_less(j, i));
        // debug_assert!(self.is_closed());

        let mut poset = self.clone();
        poset.add_less(i, j);
        poset.normalize();

        // postcondition
        debug_assert!(poset.i < poset.n);
        debug_assert!((poset.n as usize) < MAX_N);
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

    fn can_be_swapped(&self, i: u8, j: u8) -> bool {
        for k in 0..self.n {
            if self.is_less(i, k) != self.is_less(j, k) || self.is_less(k, i) != self.is_less(k, j)
            {
                return false;
            }
        }
        true
    }

    fn dual(&mut self, indicies: (u8, u8)) -> (u8, u8) {
        self.i = (self.n - 1) - self.i;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                self.swap_positions(i, j, j, i);
            }
        }
        (indicies.1, indicies.0)
    }

    // canonify
    fn canonify_nauty_indicies(&self) -> Vec<u8> {
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

        let mut result = vec![0u8; self.n as usize];
        for i in 0..self.n as usize {
            result[i] = labels[i] as u8;
        }
        result
    }

    pub fn canonify(&mut self) {
        self.canonify_transform((0, 0));
    }

    pub fn canonify_transform(&mut self, mut indicies: (u8, u8)) -> (u8, u8) {
        if self.n <= 2 * self.i {
            indicies = self.dual(indicies);
        }

        indicies = self.canonify_without_dual(indicies);

        if self.n - 1 == 2 * self.i {
            let mut dualed = self.clone();
            let mut transformed_dual = dualed.dual(indicies);
            transformed_dual = dualed.canonify_without_dual(transformed_dual);
            let mut is_dual = false;
            'break_all: for i in 0..self.n {
                for j in 0..self.n {
                    if self.is_less(i, j) != dualed.is_less(i, j) {
                        is_dual = dualed.is_less(i, j);
                        break 'break_all;
                    }
                }
            }
            if is_dual {
                indicies = transformed_dual;
                *self = dualed;
            }
        }
        indicies
    }

    pub fn canonify_without_dual(&mut self, indicies: (u8, u8)) -> (u8, u8) {
        // precondition
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        // debug_assert!(self.is_closed());

        let (less, greater) = self.calculate_relations();

        let mut in_out_degree = [0u64; MAX_N];
        for i in 0..self.n as usize {
            in_out_degree[i] = (MAX_N as u64) * (greater[i] as u64) + (less[i] as u64);
        }

        let mut hash = in_out_degree;
        for _ in 0..2 {
            let mut sum_hash = [0; MAX_N];

            for i in 0..self.n {
                let mut sum = hash[i as usize];

                for j in 0..self.n {
                    if i != j && (self.is_less(i, j) || self.is_less(j, i)) {
                        sum += hash[j as usize];
                    }
                }

                sum_hash[i as usize] = sum;
            }

            for i in 0..self.n as usize {
                hash[i] = sum_hash[i] * (MAX_N as u64 * MAX_N as u64) + in_out_degree[i];
            }
        }

        let cmpr = |&a: &u8, &b: &u8| {
            in_out_degree[a as usize]
                .cmp(&in_out_degree[b as usize])
                .then_with(|| hash[a as usize].cmp(&hash[b as usize]))
        };

        let mut new_indices: Vec<u8> = (0..self.n).collect();
        new_indices.sort_by(cmpr);

        let mut is_unique = true;
        for i in 1..self.n {
            if std::cmp::Ordering::is_eq(cmpr(
                &new_indices[i as usize - 1],
                &new_indices[i as usize],
            )) && !self.can_be_swapped(new_indices[i as usize - 1], new_indices[i as usize])
            {
                is_unique = false;
                break;
            }
        }

        if !is_unique {
            new_indices = self.canonify_nauty_indicies();
        }

        let old_poset = self.clone();
        for i in 0..self.n {
            for j in 0..self.n {
                self.set_less(
                    i,
                    j,
                    old_poset.is_less(new_indices[i as usize], new_indices[j as usize]),
                );
            }
        }

        // TODO: gib immer kleinsten zurück
        let mut transformed: (u8, u8) = (0, 0);
        for i in 0..self.n as usize {
            if new_indices[i] == indicies.0 {
                transformed.0 = i as u8;
            }
            if new_indices[i] == indicies.1 {
                transformed.1 = i as u8;
            }
        }

        // postcondition
        // debug_assert!(2 * self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        // debug_assert!(self.is_closed());
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                debug_assert!(!self.is_less(i, j));
            }
        }
        transformed
    }

    pub fn reduce_elements(&mut self) {
        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(self.is_closed());

        let (less, greater) = self.calculate_relations(); //TODO: in normal: less and greater swapped

        let mut new_indices = [0u8; MAX_N];
        let mut n_less_dropped = 0;
        let mut new_n = 0u8;
        let mut b = (self.n - 1) as usize;

        for i in 0..self.n {
            if self.i < less[i as usize] {
                new_indices[b] = i;
                b -= 1;
            } else if (self.n - 1) - self.i < greater[i as usize] {
                n_less_dropped += 1;
                new_indices[b] = i;
                b -= 1;
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
                        self.is_less(new_indices[i as usize], new_indices[j as usize]),
                    );
                }
            }
            *self = new_poset;
        }

        debug_assert!(self.i < self.n);
        debug_assert!((self.n as usize) < MAX_N);
        debug_assert!(self.is_closed());
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

    #[inline(always)]
    fn fun_name(
        temp_set_level: &mut (HashSet<(Self, (u8, u8))>, HashSet<(Self, (u8, u8))>),
        itq: Self,
        old_indices: (u8, u8),
        poset_cache: &HashSet<Self>,
        removed: &mut HashSet<(Self, (u8, u8))>,
    ) {
        if temp_set_level.0.contains(&(itq.clone(), old_indices))
            || temp_set_level.1.contains(&(itq.clone(), old_indices))
        {
            return;
        }

        if poset_cache.contains(&itq.with_less_normalized(old_indices.1, old_indices.0)) {
            removed.insert((itq.clone(), old_indices));
            temp_set_level.0.insert((itq, old_indices));
        } else {
            temp_set_level.1.insert((itq, old_indices));
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn enlarge_and_remove_less(
        &self,
        interrupt: &Arc<AtomicBool>,
        poset_cache: &HashSet<Self>,
        table: &[[bool; 15]; 15],
        n: u8,
        i: u8,
    ) -> HashSet<Self> {
        debug_assert!(2 * self.i < self.n);
        debug_assert!(table[self.n as usize][self.i as usize]);

        if self.n == n && self.i == i {
            let mut result = HashSet::new();
            for (item, (i, j)) in self.remove_less(None) {
                if !poset_cache.contains(&item)
                    && poset_cache.contains(&item.with_less_normalized(j, i))
                {
                    result.insert(item);
                }
            }
            return result;
        }

        let mut enlarged = HashSet::new();
        if table[self.n as usize + 1][self.i as usize] {
            self.enlarge_n(interrupt, &mut enlarged);
            if interrupt.load(Ordering::Relaxed) {
                return HashSet::new();
            }
        }

        let condition = 2 * (self.i + 1) < self.n + 1;
        if (condition && table[self.n as usize + 1][self.i as usize + 1])
            || (!condition && table[self.n as usize + 1][self.n as usize - self.i as usize - 1])
        {
            self.enlarge_nk(interrupt, &mut enlarged);
            if interrupt.load(Ordering::Relaxed) {
                return HashSet::new();
            }
        }

        let mut temp_set_level: [[(HashSet<(Self, (u8, u8))>, HashSet<(Self, (u8, u8))>); MAX_N];
            MAX_N] = Default::default();
        for n0 in 0..MAX_N {
            for i0 in 0..MAX_N {
                temp_set_level[n0][i0] = (HashSet::new(), HashSet::new());
            }
        }

        let mut enlarged_canonifed: HashMap<Self, u8> = HashMap::new();
        for mut item in enlarged {
            let new_ind = item.canonify_transform((item.n() - 1, item.n() - 1)).0;
            enlarged_canonifed.insert(item, new_ind);
        }

        let mut removed = HashSet::new();
        for (itq, old_indices) in self.remove_less(None) {
            Self::fun_name(
                &mut temp_set_level[itq.n as usize][itq.i as usize],
                itq,
                old_indices,
                poset_cache,
                &mut removed,
            );
        }
        for (it, num) in enlarged_canonifed {
            for (itq, old_indices) in it.remove_less(Some(num)) {
                Self::fun_name(
                    &mut temp_set_level[itq.n as usize][itq.i as usize],
                    itq,
                    old_indices,
                    poset_cache,
                    &mut removed,
                );
            }

            if interrupt.load(Ordering::Relaxed) {
                return HashSet::new();
            }
        }

        for n0 in 1..n {
            for i0 in 0..=i {
                for (item, indices) in &temp_set_level[n0 as usize][i0 as usize].0.clone() {
                    let mut super_enlarged: HashSet<(Self, (u8, u8))> = HashSet::new();
                    if table[n0 as usize + 1][i0 as usize] {
                        item.super_enlarge_n(*indices, &mut super_enlarged);
                    }

                    let condition = 2 * (i0 + 1) < n0 + 1;
                    if (condition && table[n0 as usize + 1][i0 as usize + 1])
                        || (!condition
                            && i0 < n0
                            && table[n0 as usize + 1][n0 as usize - i0 as usize - 1])
                    {
                        item.super_enlarge_nk(*indices, &mut super_enlarged);
                    }

                    for (itq, new_indices) in super_enlarged {
                        debug_assert_eq!(
                            itq.with_less_normalized(new_indices.0, new_indices.1),
                            *self
                        );

                        Self::fun_name(
                            &mut temp_set_level[itq.n as usize][itq.i as usize],
                            itq,
                            new_indices,
                            poset_cache,
                            &mut removed,
                        );
                    }
                }
            }

            if interrupt.load(Ordering::Relaxed) {
                return HashSet::new();
            }
        }

        let mut result = HashSet::new();
        for (item, _) in removed {
            if !poset_cache.contains(&item) {
                result.insert(item);
            }
        }

        result
    }

    pub fn remove_less(&self, only_last: Option<u8>) -> HashSet<(Self, (u8, u8))> {
        // // precondition
        // debug_assert!(self.i < self.n);
        // debug_assert!((self.n as usize) < MAX_N);
        // debug_assert!(self.is_closed());
        // debug_assert!(self.is_canonified());

        let mut result = HashSet::new();
        for i in 0..self.n {
            for j in 0..self.n {
                if let Some(value) = only_last {
                    if i != value && j != value {
                        continue;
                    }
                }

                if !self.is_less(i, j) || self.is_redundant(i, j) {
                    continue;
                }

                let mut poset_initial = self.clone();
                poset_initial.set_less(i, j, false);

                result.insert((poset_initial.clone(), i, j));

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

                            let mut poset_next = poset.clone();
                            poset_next.set_less(i1, j1, false);

                            if result.contains(&(poset_next.clone(), i, j))
                                || *self != poset_next.with_less(i, j)
                            {
                                continue;
                            }

                            result.insert((poset_next.clone(), i, j));
                            queue.push(poset_next);
                        }
                    }
                }
            }
        }

        // // postcondition:
        // for item in &result {
        //   debug_assert!(item.i < item.n);
        //   debug_assert!((item.n as usize) < MAX_N);
        //   debug_assert!(item.is_closed());
        //   debug_assert!(item.is_canonified());
        //   debug_assert!({
        //     let mut is_possible = false;
        //     'all_for: for i in 0..item.n() {
        //       for j in 0..item.n() {
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

        let mut cleaned_result = HashSet::new();
        for (mut item, i, j) in result {
            let size = item.n();
            item.reduce_elements();
            if size != item.n() {
                continue;
            }
            let indices = item.canonify_transform((i, j));
            cleaned_result.insert((item, indices));
        }
        cleaned_result
    }

    fn can_reduce_element_greater(&self, element: u8) -> bool {
        let mut greater = 0u8;
        for k in 0..self.n {
            if self.is_less(k, element) {
                greater += 1;
            }
        }
        self.i < greater
    }

    pub fn super_enlarge_n(&self, (k, j): (u8, u8), result: &mut HashSet<(Self, (u8, u8))>) {
        debug_assert!(!self.is_less(k, j) && !self.is_less(j, k));

        let mut init_poset = Self::new(self.n + 1, self.i);
        for i in 0..self.n {
            for j in 0..self.n {
                init_poset.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut unfiltered = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((init_poset, -1));
        while let Some((poset, number)) = swap_init.pop_back() {
            for index in ((number + 1) as u8)..(poset.n - 1) {
                if poset.is_less(index, poset.n - 1) || poset.is_less(poset.n - 1, index) {
                    continue;
                }

                let new_poset = poset.with_less(index, poset.n - 1);
                if new_poset.is_less(j, k) || new_poset.is_less(k, j) {
                    continue;
                }

                swap_init.push_back((new_poset.clone(), index as i32));

                if new_poset
                    .with_less(k, j)
                    .can_reduce_element_greater(poset.n - 1)
                {
                    unfiltered.insert(new_poset);
                }
            }
        }

        for mut item in unfiltered {
            let size = item.n();
            item.reduce_elements();
            if size != item.n() {
                continue;
            }

            let indices = item.canonify_transform((k, j));
            result.insert((item, indices));
        }
    }

    fn enlarge_n(&self, interrupt: &Arc<AtomicBool>, result: &mut HashSet<Self>) {
        let mut temp = Self::new(self.n + 1, self.i);
        for i in 0..self.n {
            for j in 0..self.n {
                temp.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut swap_init = VecDeque::new();
        swap_init.push_back((temp, -1));
        while let Some((poset, number)) = swap_init.pop_back() {
            for k in ((number + 1) as u8)..(poset.n - 1) {
                if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
                    let new_poset = poset.with_less(k, poset.n - 1);
                    swap_init.push_back((new_poset.clone(), k as i32));
                    if new_poset.can_reduce_element_greater(new_poset.n - 1) {
                        result.insert(new_poset);
                    }
                }
            }
            if interrupt.load(Ordering::Relaxed) {
                return;
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

    pub fn super_enlarge_nk(&self, (k, j): (u8, u8), result: &mut HashSet<(Self, (u8, u8))>) {
        debug_assert!(!self.is_less(k, j) && !self.is_less(j, k));

        let mut init_poset = Self::new(self.n + 1, self.i + 1);
        for i in 0..self.n {
            for j in 0..self.n {
                init_poset.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut unfiltered = HashSet::new();
        let mut swap_init = VecDeque::new();
        swap_init.push_back((init_poset, -1));
        while let Some((poset, number)) = swap_init.pop_back() {
            for index in ((number + 1) as u8)..(poset.n - 1) {
                if poset.is_less(index, poset.n - 1) || poset.is_less(poset.n - 1, index) {
                    continue;
                }

                let new_poset = poset.with_less(poset.n - 1, index);
                if new_poset.is_less(j, k) || new_poset.is_less(k, j) {
                    continue;
                }

                swap_init.push_back((new_poset.clone(), index as i32));

                if new_poset
                    .with_less(k, j)
                    .can_reduce_element_less(poset.n - 1)
                {
                    unfiltered.insert(new_poset);
                }
            }
        }

        for mut item in unfiltered {
            let size = item.n();
            item.reduce_elements();
            if size != item.n() {
                continue;
            }

            let indices = item.canonify_transform((k, j));
            result.insert((item, indices));
        }
    }

    fn enlarge_nk(&self, interrupt: &Arc<AtomicBool>, result: &mut HashSet<Self>) {
        let mut temp = Self::new(self.n + 1, self.i + 1);
        for i in 0..self.n {
            for j in 0..self.n {
                temp.set_less(i, j, self.is_less(i, j));
            }
        }

        let mut swap_init = VecDeque::new();
        swap_init.push_back((temp, -1));
        while let Some((poset, number)) = swap_init.pop_back() {
            for k in ((number + 1) as u8)..(poset.n - 1) {
                if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
                    let new_poset = poset.with_less(poset.n - 1, k);
                    swap_init.push_back((new_poset.clone(), k as i32));
                    if new_poset.can_reduce_element_less(new_poset.n - 1) {
                        result.insert(new_poset);
                    }
                }
            }
            if interrupt.load(Ordering::Relaxed) {
                return;
            }
        }
    }

    pub fn rec_temp(table: &mut [[bool; MAX_N]; MAX_N], n: usize, i: usize) {
        table[n][i] = true;
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
}
