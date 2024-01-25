use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::Hash;

use super::cache_tree::*;
use super::util::{MAX_N, MAX_N_BITS};

use std::os::raw::c_int;

use nauty_Traces_sys::{densenauty, optionblk, statsblk, FALSE, TRUE};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Poset {
  n: u8,
  nth_smallest: u8,
  comparison_table: [u8; MAX_N * MAX_N],
}

impl fmt::Debug for Poset {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "n = {}, nth_smallest = {}", self.n, self.nth_smallest)?;

    for i in 0..self.n {
      write!(f, "\n")?;
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
  pub fn new(n: u8, nth_smallest: u8) -> Poset {
    Poset {
      n,
      nth_smallest,
      comparison_table: [0; MAX_N * MAX_N],
    }
  }

  // getter
  pub fn n(&self) -> u8 {
    self.n
  }

  pub fn nth_smallest(&self) -> u8 {
    self.nth_smallest
  }

  pub fn get_index(&self, pos: u8) -> bool {
    return self.comparison_table[pos as usize] == 1;
    // (self.comparison_table[(pos as usize) >> 6] & (1 << ((pos as usize) & 63))) != 0
  }

  pub fn set_index(&mut self, pos: u8, value: bool) {
    if value {
      self.comparison_table[pos as usize] = 1;
      //   self.comparison_table[pos as usize >> 6] |= 1 << (pos & 63);
    } else {
      self.comparison_table[pos as usize] = 0;
      // self.comparison_table[pos as usize >> 6] &= !(1 << (pos & 63));
    }
  }

  pub fn get_comparison_table_size(&self) -> usize {
    (self.n as usize) * (self.n as usize)
  }

  fn to_internal_pos(&self, i: u8, j: u8) -> usize {
    (i as usize) * (self.n as usize) + (j as usize)
  }

  pub fn is_less(&self, i: u8, j: u8) -> bool {
    self.get_index(self.to_internal_pos(i, j) as u8)
  }

  pub fn set_less(&mut self, i: u8, j: u8, value: bool) {
    self.set_index(self.to_internal_pos(i, j) as u8, value);
  }

  pub fn subset_of(&self, other: &Poset) -> bool {
    if !(self.n == other.n && self.nth_smallest == other.nth_smallest) {
      return false;
    }
    for i in 0..(self.n * self.n) {
      if self.get_index(i as u8) && !other.get_index(i as u8) {
        return false;
      }
    }
    true
  }

  // add
  fn add_and_close_recursive(&mut self, i: u8, j: u8) {
    if !self.is_less(i, j) {
      self.set_less(i, j, true);

      for k in 0..self.n as u8 {
        if i != k && j != k {
          if self.is_less(j, k) && !self.is_less(i, k) {
            self.add_and_close_recursive(i, k);
          } else if self.is_less(k, i) && !self.is_less(k, j) {
            self.add_and_close_recursive(k, j);
          }
        }
      }
    }
  }

  pub fn add_less(&mut self, i: u8, j: u8) {
    self.add_and_close_recursive(i, j);
  }

  pub fn with_less(&self, i: u8, j: u8) -> Poset {
    let mut poset = self.clone();
    poset.add_less(i, j);
    poset
  }

  // reduce
  pub fn calculate_relations(&self, less: &mut [u8; MAX_N], greater: &mut [u8; MAX_N]) {
    for i in 0..self.n {
      for j in 0..self.n {
        if self.is_less(i, j) {
          less[j as usize] += 1;
          greater[i as usize] += 1;
        }
      }
    }
  }

  fn swap(&mut self, i: u8, j: u8) {
    for k in 0..self.n {
      let temp = self.is_less(i, k);
      self.set_less(i, k, self.is_less(j, k));
      self.set_less(j, k, temp);
    }
    for k in 0..self.n {
      let temp = self.is_less(k, i);
      self.set_less(k, i, self.is_less(k, j));
      self.set_less(k, j, temp);
    }
  }

  fn dual(&mut self) {
    self.nth_smallest = (self.n - 1) - self.nth_smallest;
    for i in 0..self.n {
      for j in (i + 1)..self.n {
        let temp = self.is_less(i, j);
        self.set_less(i, j, self.is_less(j, i));
        self.set_less(j, i, temp);
      }
    }
  }

  fn reduce_n(&mut self) {
    let mut less = [0; MAX_N];
    let mut greater = [0; MAX_N];
    self.calculate_relations(&mut greater, &mut less);

    let mut new_indices = [0 as u8; MAX_N];
    let mut n_less_dropped = 0;
    let mut new_n = 0 as u8;
    let mut b = (self.n - 1) as usize;

    for i in 0..self.n as u8 {
      if self.nth_smallest < greater[i as usize] {
        new_indices[b] = i;
        b -= 1;
      } else if (self.n - 1) - self.nth_smallest < less[i as usize] {
        n_less_dropped += 1;
        new_indices[b] = i;
        b -= 1;
      } else {
        new_indices[new_n as usize] = i;
        new_n += 1;
      }
    }

    if new_n != self.n {
      let old_poset = self.clone();
      self.n = new_n as u8;
      self.nth_smallest -= n_less_dropped;
      self.comparison_table = [0; MAX_N * MAX_N];
      for i in 0..new_n {
        for j in 0..new_n {
          self.set_less(
            i,
            j,
            old_poset.is_less(new_indices[i as usize], new_indices[j as usize]),
          );
        }
      }

      if self.n <= 2 * self.nth_smallest {
        self.dual();
      }
    }
  }

  // canonify
  fn canonify_nauty_indicies(&self) -> Vec<u8> {
    let n = self.n as usize;

    let mut options = optionblk {
      getcanon: TRUE,
      defaultptn: FALSE,
      digraph: TRUE,
      ..Default::default()
    };
    let mut stats = statsblk::default();

    let mut labels: [c_int; MAX_N] = (0..MAX_N as c_int).collect::<Vec<_>>().try_into().unwrap();

    let mut ptn = [c_int::from(0); MAX_N];
    ptn[n - 1] = 0;
    let mut zeroes2 = [c_int::from(0); MAX_N];

    // use nauty_Traces_sys::bit as bitmask for the adjacency matrix.
    // E.g. (g[i] & bit[j]) != 0 checks whether there is an edge i -> j.
    let mut dg = [0; MAX_N];
    for (i, mask) in dg.iter_mut().enumerate().take(n) {
      for j in 0..n {
        if self.is_less(i as u8, j as u8) {
          *mask |= nauty_Traces_sys::bit[j];
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
        n as c_int,
        canonical.as_mut_ptr(),
      );
    }

    let mut result: Vec<u8> = Vec::new();
    for i in 0..self.n {
      result.push(labels[i as usize] as u8);
    }
    result
  }

  pub fn canonify(&mut self) {
    const ONLY_NAUTY: bool = false;

    let old_poset = self.clone();
    let mut new_indices: Vec<u8>;

    if ONLY_NAUTY {
      new_indices = self.canonify_nauty_indicies();
    } else {
      let mut less = [0; MAX_N];
      let mut greater = [0; MAX_N];
      self.calculate_relations(&mut less, &mut greater);

      let mut in_out_degree = [0 as u64; MAX_N];
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
              sum ^= hash[j as usize];
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

    for i in 0..self.n {
      for j in (i + 1)..self.n {
        debug_assert!(!self.is_less(i, j));
      }
    }
  }

  // normalize
  pub fn normalize(&mut self) {
    self.reduce_n();
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

  pub fn remove_less<F>(&self, i: u8, j: u8, test: F) -> HashSet<Poset>
  where
    F: Fn(&Poset) -> bool,
  {
    let mut result = HashSet::new();

    if !self.is_less(i, j) || self.is_redundant(i, j) {
      return result;
    }

    let mut poset_initial = self.clone();
    poset_initial.set_less(i, j, false);

    let mut poset_check = poset_initial.clone();
    poset_check.add_less(j, i);
    poset_check.normalize();
    if !test(&poset_check) {
      return result;
    }

    let mut queue = Vec::new();
    queue.push(poset_initial.clone());

    poset_initial.canonify();
    result.insert(poset_initial);

    while let Some(poset) = queue.pop() {
      for i1 in 0..self.n {
        for j1 in 0..self.n {
          if i1 == j1
            || (j as i32 - i as i32).abs() >= (j1 as i32 - i1 as i32).abs()
            || !poset.is_less(i1, j1)
            || poset.is_redundant(i1, j1)
          {
            continue;
          }

          let mut poset_next = poset.clone();
          poset_next.set_less(i1, j1, false);

          let mut poset_next_normal = poset_next.clone();
          poset_next_normal.add_less(i, j);
          if self != &poset_next_normal {
            continue;
          }

          let mut poset_check = poset_next.clone();
          poset_check.add_less(j, i);
          poset_check.normalize();
          if !test(&poset_check) {
            continue;
          }

          let mut poset_norm = poset_next.clone();
          poset_norm.canonify();
          if result.contains(&poset_norm) {
            continue;
          }
          result.insert(poset_norm);

          queue.push(poset_next);
        }
      }
    }

    result
  }

  // enlarge
  fn filter(unfiltered: &HashSet<Poset>) -> HashSet<Poset> {
    let mut filtered = HashSet::new();

    for item in unfiltered {
      let mut found = false;

      for temp in unfiltered {
        if temp != item && temp.subset_of(item) {
          found = true;
          break;
        }
      }

      if !found {
        filtered.insert(item.clone());
      }
    }

    filtered
  }

  fn can_reduce_element_greater(&self, element: u8) -> bool {
    let mut greater = 0 as u8;
    for k in 0..self.n {
      if self.is_less(k, element) {
        greater += 1;
      }
    }
    self.nth_smallest < greater
  }

  fn enlarge_n(&self, result: &mut HashSet<Poset>) {
    let mut temp = Poset::new(self.n + 1, self.nth_smallest);
    for i in 0..self.n {
      for j in 0..self.n {
        temp.set_less(i, j, self.is_less(i, j));
      }
    }

    let mut unfiltered = HashSet::new();
    let mut swap_init = VecDeque::new();
    swap_init.push_back((temp, -1 as i32));
    while let Some((poset, number)) = swap_init.pop_back() {
      for k in ((number + 1) as u8)..(poset.n - 1) {
        if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
          let new_poset = poset.with_less(k, poset.n - 1);
          swap_init.push_back((new_poset.clone(), k as i32));
          if new_poset.can_reduce_element_greater(poset.n - 1) {
            unfiltered.insert(new_poset);
          }
        }
      }
    }

    for item in Poset::filter(&unfiltered) {
      let mut it = item.clone();
      it.canonify();
      result.insert(item);
    }
  }

  fn can_reduce_element_less(&self, element: u8) -> bool {
    let mut less = 0 as u8;
    for k in 0..self.n as u8 {
      if self.is_less(element, k) {
        less += 1;
      }
    }
    (self.n - 1) - self.nth_smallest < less
  }

  fn enlarge_nk(&self, result: &mut HashSet<Poset>) {
    let mut temp = Poset::new(self.n + 1, self.nth_smallest + 1);
    for i in 0..self.n {
      for j in 0..self.n {
        temp.set_less(i, j, self.is_less(i, j));
      }
    }

    let mut unfiltered = HashSet::new();
    let mut swap_init = VecDeque::new();
    swap_init.push_back((temp, -1 as i32));
    while let Some((poset, number)) = swap_init.pop_back() {
      for k in ((number + 1) as u8)..(poset.n - 1) {
        if !poset.is_less(k, poset.n - 1) && !poset.is_less(poset.n - 1, k) {
          let new_poset = poset.with_less(poset.n - 1, k);
          swap_init.push_back((new_poset.clone(), k as i32));
          if new_poset.can_reduce_element_less(poset.n - 1) {
            unfiltered.insert(new_poset);
          }
        }
      }
    }

    for item in Poset::filter(&unfiltered) {
      let mut it = item.clone();
      it.canonify();
      result.insert(item);
    }
  }

  pub fn enlarge(set_of_posets: &HashSet<Poset>, n: u8, k: u8) -> HashSet<Poset> {
    let mut temp_set: Vec<Vec<HashSet<Poset>>> = Vec::with_capacity((n + 1) as usize);
    for _ in 0..(n + 1) {
      let mut inner_vec: Vec<HashSet<Poset>> = Vec::with_capacity((k + 1) as usize);
      for _ in 0..(k + 1) {
        inner_vec.push(HashSet::new());
      }
      temp_set.push(inner_vec);
    }

    for item in set_of_posets.iter() {
      if item.n <= n && item.nth_smallest <= k {
        temp_set[item.n as usize][item.nth_smallest as usize].insert(item.clone());
      }
    }

    for n0 in 0..n {
      let mut result: HashSet<Poset> = HashSet::new();

      for k0 in 0..k {
        for item in &temp_set[n0 as usize][k0 as usize] {
          item.enlarge_nk(&mut result);
        }
      }

      for item in &temp_set[n0 as usize][k as usize] {
        item.enlarge_n(&mut result);
      }

      for item in result {
        temp_set[item.n as usize][item.nth_smallest as usize].insert(item.clone());
      }
    }

    temp_set[n as usize][k as usize].clone()
  }

  // pub fn enlarge(set_of_posets: &HashSet<Poset>, n: u8, k: u8) -> HashSet<Poset> {
  //   let mut temp_set: Vec<Vec<CacheTreeFixed<true>>> = Vec::with_capacity((n + 1) as usize);
  //   for _ in 0..(n + 1) {
  //     let mut inner_vec: Vec<CacheTreeFixed<true>> = Vec::with_capacity((k + 1) as usize);
  //     for _ in 0..(k + 1) {
  //       inner_vec.push(CacheTreeFixed::new(n, k));
  //     }
  //     temp_set.push(inner_vec);
  //   }

  //   for item in set_of_posets.iter() {
  //     if item.n <= n && item.nth_smallest <= k {
  //       temp_set[item.n as usize][item.nth_smallest as usize].insert(&item);
  //     }
  //   }

  //   for n0 in 0..n {
  //     let mut result: HashSet<Poset> = HashSet::new();

  //     for k0 in 0..k {
  //       for item in &temp_set[n0 as usize][k0 as usize].entries() {
  //         item.enlarge_nk(&mut result);
  //       }
  //     }

  //     for item in &temp_set[n0 as usize][k as usize].entries() {
  //       item.enlarge_n(&mut result);
  //     }

  //     for item in result {
  //       temp_set[item.n as usize][item.nth_smallest as usize].insert(&item);
  //     }
  //   }

  //   temp_set[n as usize][k as usize].entries()
  // }

  pub fn test() {
    let mut poset = Poset::new(4, 1);
    poset.add_less(2, 3);
    dbg!(&poset);
  }
}
