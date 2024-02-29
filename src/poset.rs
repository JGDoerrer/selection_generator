use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::mem::swap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::fmt;

use crate::util::OPTIMISE_BACKWARD_WRONG;
use crate::util::{MAX_N, ONLY_NAUTY_CANONIFY};

use const_for::const_for;
use std::os::raw::c_int;

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};

const fn init_table() -> [([(u8, u8); MAX_N * MAX_N], usize); MAX_N] {
  let mut table1 = [([(0u8, 0u8); MAX_N * MAX_N], 0); MAX_N];
  table1[0] = ([(0, 0); MAX_N * MAX_N], 0);
  table1[1] = ([(0, 0); MAX_N * MAX_N], 1);
  const_for!(n in 2..MAX_N => {
    table1[n].1 = (n * n - n) / 2;
    const_for!(pos in 0..table1[n].1 => {
      let mut a = 0;
      const_for!(k in 0..MAX_N => {
        if pos < (k * k + k) / 2 {
          break;
        }
        a = k;
      });
      let b: usize = pos - ((a * a + a) / 2);
      table1[n].0[pos] = ((a + 1) as u8, b as u8);
    });
  });
  table1
}
const TABLE_ORDER: [([(u8, u8); MAX_N * MAX_N], usize); MAX_N] = init_table();

#[derive(Clone, PartialEq, Eq)]
pub struct Poset {
  n: u8,
  i: u8,
  adjacency: [u16; MAX_N],
}

impl Hash for Poset {
  fn hash<H: Hasher>(&self, ra_expand_state: &mut H) {
    // self.n.hash(ra_expand_state);
    // self.i.hash(ra_expand_state);
    self.adjacency.hash(ra_expand_state);
  }
}

impl fmt::Debug for Poset {
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

impl Poset {
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

  fn subset_of_brute_force_rec(
    &self,
    poset: &Poset,
    new_indices: &mut Vec<u8>,
    is_possible: [[bool; MAX_N]; MAX_N],
    k: u8,
  ) -> bool {
    for i in k..self.n {
      if is_possible[k as usize][new_indices[i as usize] as usize] {
        new_indices.swap(i as usize, k as usize);

        let mut maybe = true;
        for q in 0..self.n {
          if self.is_less(k, q) && !poset.is_less(new_indices[k as usize], new_indices[q as usize])
          {
            maybe = false;
            break;
          }
        }

        if maybe
          && (k + 1 == self.n
            || self.subset_of_brute_force_rec(poset, new_indices, is_possible, k + 1))
        {
          return true;
        }

        new_indices.swap(i as usize, k as usize);
      }
    }
    false
  }

  // new: 96.148s': n = 8, i = 3, (cache_l: 5535, cache_u: 2387, noSol: 0, bruteForce: 416), cache = 683
  // old: 0.089s': n = 8, i = 3, (cache_l: 11590, cache_u: 3508, noSol: 0, bruteForce: 778), cache = 1081
  pub fn subset_of_brute_force(&self, poset: &Poset) -> bool {
    // TODO: eigentlich nicht nötig
    if self.n != poset.n || self.i != poset.i {
      return false;
    }

    let mut rows_poset = [0; MAX_N];
    let mut cols_poset = [0; MAX_N];
    let mut rows_self = [0; MAX_N];
    let mut cols_self = [0; MAX_N];
    for i in 0..self.n {
      for j in 0..self.n {
        if poset.is_less(i, j) {
          rows_poset[i as usize] += 1;
          cols_poset[j as usize] += 1;
        }
        if self.is_less(i, j) {
          rows_self[i as usize] += 1;
          cols_self[j as usize] += 1;
        }
      }
    }

    let mut is_possible = [[true; MAX_N]; MAX_N];
    for i in 0..self.n {
      for j in 0..self.n {
        if rows_self[i as usize] > rows_poset[j as usize]
          || cols_self[i as usize] > cols_poset[j as usize]
        {
          is_possible[i as usize][j as usize] = false;
        }
      }
    }

    let mut changed = true;
    while changed {
      changed = false;
      for i in 0..self.n {
        let mut count_row = 0;
        let mut count_col = 0;
        let mut num_row = 0;
        let mut num_col = 0;
        for j in 0..self.n {
          if is_possible[i as usize][j as usize] {
            count_row += 1;
            num_row = j;
          }
          if is_possible[j as usize][i as usize] {
            count_col += 1;
            num_col = j;
          }
        }
        if 1 == count_row {
          for j in 0..self.n {
            if i != j && is_possible[j as usize][num_row as usize] {
              // changed = true;
              is_possible[j as usize][num_row as usize] = false;
            }
          }
        }
        if 1 == count_col {
          for j in 0..self.n {
            if i != j && is_possible[num_col as usize][j as usize] {
              // changed = true;
              is_possible[num_col as usize][j as usize] = false;
            }
          }
        }
      }
    }

    let mut new_indices: Vec<u8> = (0..poset.n).collect::<Vec<_>>();
    self.subset_of_brute_force_rec(poset, &mut new_indices, is_possible, 0)
  }

  pub fn subset_of(&self, other: &Poset) -> bool {
    if !(self.n == other.n && self.i == other.i) {
      return false;
    }
    // TODO: instead of `0..self.n`: faster due to loop unrolling?
    for k in 0..self.n as usize {
      if 0 != self.adjacency[k] & !other.adjacency[k] {
        return false;
      }
    }
    true
  }

  // TODO: implement Iterator instead of `adjacency_size` and `is_index`
  pub fn adjacency_size(&self) -> usize {
    TABLE_ORDER[self.n as usize].1
  }

  pub fn is_index(&self, pos: usize) -> bool {
    let item = TABLE_ORDER[self.n as usize].0[pos];
    self.is_less(item.0, item.1)
  }

  pub fn set_index(&mut self, pos: usize, value: bool) {
    let item = TABLE_ORDER[self.n as usize].0[pos];
    self.set_less(item.0, item.1, value);
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

  pub fn with_less(&self, i: u8, j: u8) -> Poset {
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

  pub fn with_less_normalized(&self, i: u8, j: u8) -> Poset {
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

  fn swap(&mut self, i: u8, j: u8) {
    for k in 0..self.n {
      if i != k && j != k {
        self.swap_positions(i, k, j, k);
        self.swap_positions(k, i, k, j);
      }
    }
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
  fn canonify_nauty_indicies(&self) -> Vec<u8> {
    let mut options = optionblk {
      getcanon: TRUE,
      defaultptn: FALSE,
      digraph: TRUE,
      ..Default::default()
    };
    let mut stats = statsblk::default();

    let mut labels: [c_int; MAX_N] = (0..MAX_N as c_int).collect::<Vec<_>>().try_into().unwrap();

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
    if self.n <= 2 * self.i {
      self.dual();
    }

    self.canonify_without_dual();

    if self.n - 1 == 2 * self.i {
      let mut dualed = self.clone();
      dualed.dual();
      dualed.canonify_without_dual();
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
        *self = dualed;
      }
    }
  }

  pub fn canonify_without_dual(&mut self) {
    // precondition
    debug_assert!(self.i < self.n);
    debug_assert!((self.n as usize) < MAX_N);
    // debug_assert!(self.is_closed());

    let old_poset = self.clone();
    let mut new_indices: Vec<u8>;

    if ONLY_NAUTY_CANONIFY {
      new_indices = self.canonify_nauty_indicies();
    } else {
      // TODO: sometimes FALSE
      let (less, greater) = self.calculate_relations();

      let mut in_out_degree = [0u64; MAX_N];
      for i in 0..self.n as usize {
        in_out_degree[i] = (MAX_N as u64) * (greater[i] as u64) + (less[i] as u64);
      }

      let mut hash = in_out_degree.clone();
      for _ in 0..2 {
        let mut sum_hash = [0; MAX_N];

        for i in 0..self.n {
          let mut sum = hash[i as usize];

          for j in 0..self.n {
            if i != j && (self.is_less(i, j) || self.is_less(j, i)) {
              sum ^= hash[j as usize]; // TODO: `sum += hash[j as usize]` is faster ...
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

      new_indices = (0..self.n).collect();
      new_indices.sort_by(cmpr);

      let mut is_unique = true;
      for i in 1..self.n {
        if in_out_degree[new_indices[(i - 1) as usize] as usize]
          == in_out_degree[new_indices[i as usize] as usize]
          && hash[new_indices[(i - 1) as usize] as usize] == hash[new_indices[i as usize] as usize]
        {
          self.swap(new_indices[(i - 1) as usize], new_indices[i as usize]);
          if *self != old_poset {
            is_unique = false;
            break;
          }
        }
      }

      if !is_unique {
        new_indices = self.canonify_nauty_indicies();
        new_indices.sort_by(cmpr);
      }
    }

    for i in 0..self.n {
      for j in 0..self.n {
        self.set_less(
          i,
          j,
          old_poset.is_less(new_indices[i as usize], new_indices[j as usize]),
        );
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
      let mut new_poset = Poset::new(new_n, self.i - n_less_dropped);
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

  pub fn remove_less_bruteForce<F>(&self, hash_set: &HashSet<Poset>, test1: F) -> HashSet<Poset>
  where
    F: Fn(&Poset) -> bool,
  {
    let mut result = HashSet::new();
    for poset_next in hash_set {
      'all_for: for i in 0..poset_next.n() {
        for j in 0..poset_next.n() {
          if i != j && !poset_next.is_less(i, j) && !poset_next.is_less(j, i) {
            let mut poset_next_normal = poset_next.with_less(i, j);
            poset_next_normal.canonify();
            if self != &poset_next_normal {
              continue;
            }

            let poset_check = poset_next.with_less_normalized(j, i);
            if !test1(&poset_check) {
              continue;
            }

            let mut poset_norm = poset_next.clone();
            poset_norm.normalize();
            result.insert(poset_norm);
            break 'all_for;
          }
        }
      }
    }
    result
  }

  #[allow(clippy::too_many_lines)]
  pub fn enlarge_and_remove_less<F>(
    &self,
    interrupt: &Arc<AtomicBool>,
    table: &[[bool; 15]; 15],
    n: u8,
    i: u8,
    test: F,
  ) -> HashSet<Poset>
  where
    F: Fn(&Poset) -> bool,
  {
    debug_assert!(2 * self.i < self.n);
    debug_assert!(table[self.n as usize][self.i as usize]);

    let mut destination = self.remove_less(&test);

    if self.n == n && self.i == i {
      return destination;
    }

    let mut temp_set_level: [(HashSet<Poset>, HashSet<Poset>); MAX_N] = Default::default();
    let mut temp_set_next: [(HashSet<Poset>, HashSet<Poset>); MAX_N] = Default::default();
    for i0 in 0..MAX_N {
      temp_set_level[i0] = (HashSet::new(), HashSet::new());
      temp_set_next[i0] = (HashSet::new(), HashSet::new());
    }

    temp_set_level[self.i as usize].0.insert(self.clone());

    for n0 in self.n..n {
      for i0 in 0..=i {
        let mut temp_dest = HashSet::new();
        for item in &temp_set_level[i0 as usize].0 {
          let mut result1 = HashSet::new();
          if table[n0 as usize + 1][i0 as usize] {
            item.enlarge_n(interrupt, &mut result1, false);
            if interrupt.load(Ordering::Relaxed) {
              return HashSet::new();
            }
          }

          let condition = 2 * (i0 + 1) < n0 + 1;
          if (condition && table[n0 as usize + 1][i0 as usize + 1])
            || (!condition && table[n0 as usize + 1][n0 as usize - i0 as usize - 1])
          {
            item.enlarge_nk(interrupt, &mut result1, false);
            if interrupt.load(Ordering::Relaxed) {
              return HashSet::new();
            }
          }

          for it in result1 {
            debug_assert_eq!(
              {
                let mut norm = it.clone();
                norm.normalize();
                norm
              },
              *self
            );

            if temp_set_next[it.i as usize].0.contains(&it)
              || temp_set_next[it.i as usize].1.contains(&it)
            {
              continue;
            }

            let mut found = false;
            for item in it.remove_less(&test) {
              found |= !destination.contains(&item);
              temp_dest.insert(item);
            }
            if found {
              temp_set_next[it.i as usize].0.insert(it);
            } else {
              temp_set_next[it.i as usize].1.insert(it);
            }

            if interrupt.load(Ordering::Relaxed) {
              return HashSet::new();
            }
          }
        }
        for item in temp_dest {
          destination.insert(item);
        }
      }

      for i0 in 0..MAX_N {
        swap(&mut temp_set_level[i0], &mut temp_set_next[i0]);
        temp_set_next[i0].0.clear();
        temp_set_next[i0].1.clear();
      }
    }

    destination
  }

  #[allow(clippy::too_many_lines)]
  pub fn remove_less<F>(&self, test: F) -> HashSet<Poset>
  where
    F: Fn(&Poset) -> bool,
  {
    // // precondition
    // debug_assert!(self.i < self.n);
    // debug_assert!((self.n as usize) < MAX_N);
    // debug_assert!(self.is_closed());
    // debug_assert!(self.is_canonified());

    let mut real_result = HashSet::new();
    real_result.insert(self.clone());

    let mut result = HashSet::new();
    for self_r in real_result {
      for i in 0..self_r.n {
        for j in 0..self_r.n {
          if !self_r.is_less(i, j) || self_r.is_redundant(i, j) {
            continue;
          }

          let mut poset_initial = self_r.clone();
          poset_initial.set_less(i, j, false);

          if !test(&poset_initial.with_less_normalized(j, i)) {
            continue;
          }

          result.insert(poset_initial.clone());

          let mut queue = Vec::new();
          queue.push(poset_initial);

          while let Some(poset) = queue.pop() {
            for i1 in 0..self_r.n {
              for j1 in 0..self_r.n {
                if i1 == j1
                  // || (j as i32 - i as i32).abs() >= (j1 as i32 - i1 as i32).abs()
                  || !poset.is_less(i1, j1)
                  || poset.is_redundant(i1, j1)
                {
                  continue;
                }

                let mut poset_next = poset.clone();
                poset_next.set_less(i1, j1, false);

                if result.contains(&poset_next)
                  || self_r != poset_next.with_less(i, j)
                  || !test(&poset_next.with_less_normalized(j, i))
                {
                  continue;
                }

                result.insert(poset_next.clone());
                queue.push(poset_next);
              }
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
    for mut item in result {
      let size = item.n();
      item.reduce_elements();
      if size == item.n() {
        item.canonify();
        cleaned_result.insert(item);
      }
    }
    cleaned_result
  }

  // enlarge
  pub fn filter(interrupt: &Arc<AtomicBool>, unfiltered: &HashSet<Poset>) -> HashSet<Poset> {
    // TODO: in theory faster, practical not, Rroblem: cache only normalized items
    // let mut tree: CacheTreeItem<true> = CacheTreeItem::new(n, i);
    // for item in unfiltered {
    //   tree.insert(item);
    // }
    // tree.entries()

    let mut filtered: Vec<Poset> = vec![];

    for item in unfiltered {
      if !filtered.iter().any(|poset| poset.subset_of(item)) {
        filtered.retain(|poset| !item.subset_of(poset));
        filtered.push(item.clone());
      }
      if interrupt.load(Ordering::Relaxed) {
        break;
      }
    }

    let mut set1 = HashSet::new();
    for item in filtered {
      set1.insert(item);
    }
    set1
  }

  fn filter_subgraph(interrupt: &Arc<AtomicBool>, unfiltered: &HashSet<Poset>) -> HashSet<Poset> {
    let mut filtered: Vec<Poset> = vec![];

    for item in unfiltered {
      if !filtered
        .iter()
        .any(|poset| poset.subset_of_brute_force(item))
      {
        filtered.retain(|poset| !item.subset_of_brute_force(poset));
        filtered.push(item.clone());
      }
      if interrupt.load(Ordering::Relaxed) {
        break;
      }
    }

    let mut result: HashSet<Poset> = HashSet::new();
    for item in filtered {
      result.insert(item);
    }
    result
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

  fn enlarge_n(
    &self,
    interrupt: &Arc<AtomicBool>,
    result: &mut HashSet<Poset>,
    should_not_canonify: bool,
  ) {
    let mut temp = Poset::new(self.n + 1, self.i);
    for i in 0..self.n {
      for j in 0..self.n {
        temp.set_less(i, j, self.is_less(i, j));
      }
    }

    let mut unfiltered = HashSet::new();
    let mut swap_init = VecDeque::new();
    swap_init.push_back((temp, -1));
    while let Some((poset, number)) = swap_init.pop_back() {
      for k in ((number + 1) as u8)..(poset.n - 1) {
        if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
          let new_poset = poset.with_less(k, poset.n - 1);
          swap_init.push_back((new_poset.clone(), k as i32));
          if new_poset.can_reduce_element_greater(new_poset.n - 1) {
            unfiltered.insert(new_poset);
          }
        }
      }
      if interrupt.load(Ordering::Relaxed) {
        return;
      }
    }

    for mut item in if OPTIMISE_BACKWARD_WRONG {
      Poset::filter(interrupt, &unfiltered)
    } else {
      unfiltered
    } {
      if !should_not_canonify {
        item.canonify();
      }
      result.insert(item);
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

  fn enlarge_nk(
    &self,
    interrupt: &Arc<AtomicBool>,
    result: &mut HashSet<Poset>,
    should_not_canonify: bool,
  ) {
    let mut temp = Poset::new(self.n + 1, self.i + 1);
    for i in 0..self.n {
      for j in 0..self.n {
        temp.set_less(i, j, self.is_less(i, j));
      }
    }

    let mut unfiltered = HashSet::new();
    let mut swap_init = VecDeque::new();
    swap_init.push_back((temp, -1));
    while let Some((poset, number)) = swap_init.pop_back() {
      for k in ((number + 1) as u8)..(poset.n - 1) {
        if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
          let new_poset = poset.with_less(poset.n - 1, k);
          swap_init.push_back((new_poset.clone(), k as i32));
          if new_poset.can_reduce_element_less(new_poset.n - 1) {
            unfiltered.insert(new_poset);
          }
        }
      }
      if interrupt.load(Ordering::Relaxed) {
        return;
      }
    }

    for mut item in if OPTIMISE_BACKWARD_WRONG {
      Poset::filter(interrupt, &unfiltered)
    } else {
      unfiltered
    } {
      if !should_not_canonify {
        item.canonify();
      }
      result.insert(item);
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

  pub fn enlarge_brute_force(
    really_all: &HashSet<Poset>,
    interrupt: &Arc<AtomicBool>,
    set_of_posets: &HashSet<Poset>,
    n: u8,
    i: u8,
  ) -> HashSet<Poset> {
    // sei set_of_posets = {p}
    // suche alle Posets, die durch Aufruf von `normalize` zu p werden
    let mut result = HashSet::new();
    for poset in really_all {
      let mut norm = poset.clone();
      norm.normalize();
      if set_of_posets.contains(&norm) {
        debug_assert!(poset.is_canonified());
        result.insert(poset.clone());
      }
    }

    result
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

  fn is_canonified(&self) -> bool {
    let mut canon = self.clone();
    canon.canonify();
    if *self != canon {
      dbg!(self);
      dbg!(&canon);
    }
    *self == canon
  }

  fn is_normalized(&self) -> bool {
    let mut canon = self.clone();
    canon.normalize();
    if *self != canon {
      dbg!(self);
      dbg!(&canon);
    }
    *self == canon
  }
}

pub fn iterate_all_closed_canonified_rec(
  result: &mut HashSet<Poset>,
  poset: &mut Poset,
  index: usize,
) {
  if 0 == index {
    let mut success = true;
    'break_all: for i in 0..poset.n() {
      for j in 0..poset.n() {
        if i == j {
          if poset.is_less(i, j) {
            success = false;
            break 'break_all;
          }
        } else {
          if poset.is_less(i, j) && poset.is_less(j, i) {
            success = false;
            break 'break_all;
          }
          for k in 0..poset.n() {
            if poset.is_less(i, j) && poset.is_less(j, k) && !poset.is_less(i, k) {
              success = false;
              break 'break_all;
            }
          }
        }
      }
    }

    if success {
      let mut new_poset = poset.clone();
      new_poset.canonify();
      result.insert(new_poset);
    }
  } else {
    poset.set_index(index - 1, false);
    iterate_all_closed_canonified_rec(result, poset, index - 1);
    poset.set_index(index - 1, true);
    iterate_all_closed_canonified_rec(result, poset, index - 1);
  }
}

pub fn iterate_all_closed_canonified(n: u8, i: u8) -> HashSet<Poset> {
  let mut poset = Poset::new(n, i);
  let mut result = HashSet::new();
  iterate_all_closed_canonified_rec(&mut result, &mut poset, TABLE_ORDER[n as usize].1);
  result
}

pub fn iterate_all_closed_normalized(n: u8, i: u8) -> HashSet<Poset> {
  let mut result = HashSet::new();
  for mut item in iterate_all_closed_canonified(n, i) {
    item.normalize();
    result.insert(item);
  }
  result
}

// #[test]
// pub fn test_remove_less() {
//   env::set_var("RUST_BACKTRACE", "1");

//   let max_n = 7;

//   for n in 2..max_n as u8 {
//     for i in 0..((n + 1) / 2) {
//       let all_posets = iterate_all_closed_canonified(n, i);
//       dbg!(n, i, all_posets.len());

//       for poset in &all_posets {
//         let result1 = poset.remove_less(|_| true);
//         let result2 = poset.remove_less_bruteForce(&all_posets, |_| true); // TODO: anstatt `all_posets` auch Posets mit Größe n + 1?
//         if result1 != result2 {
//           dbg!(&result1, &result2);
//           dbg!(result1.len(), result2.len());
//           dbg!(poset);
//           panic!();
//         }
//       }
//     }
//   }
// }

// #[test]
// pub fn test_enlarge() {
//   let interrupt = Arc::new(AtomicBool::new(false));
//   env::set_var("RUST_BACKTRACE", "1");

//   let mut really_all: HashSet<Poset> = HashSet::new();
//   really_all.insert(Poset::new(1, 0));
//   for n in 2..MAX_N as u8 {
//     for i in 0..((n + 1) / 2) {
//       let all_posets = iterate_all_closed_normalized(n, i);
//       let temp = iterate_all_closed_canonified(n, i);
//       for item in &all_posets {
//         really_all.insert(item.clone());
//       }

//       let len = all_posets.len();
//       println!("n: {n}, i: {i}, len: {len}");

//       for new_poset in &really_all {
//         let mut set_of_posets = HashSet::new();
//         set_of_posets.insert(new_poset.clone());
//         let result1 = Poset::enlarge(&interrupt, &set_of_posets, n, i);
//         let result2 = Poset::enlarge_brute_force(&temp, &interrupt, &set_of_posets, n, i);
//         if result1 != result2 {
//           dbg!(&result1, &result2);
//           dbg!(result1.len(), result2.len());
//           dbg!(new_poset);
//           dbg!(n, i);
//           panic!();
//         }
//       }
//     }
//   }
//   panic!("SUCESS ==============================");
// }

#[test]
pub fn test1() {
  let mut poset = Poset::new(4, 1);
  poset.add_less(2, 3);
  dbg!(&poset);
}
