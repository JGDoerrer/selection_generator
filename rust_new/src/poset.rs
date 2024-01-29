use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};

use super::cache_tree::CacheTreeItem;
use super::util::{MAX_N, ONLY_NAUTY_CANONIFY};

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
    if !self.is_less(i, j) {
      self.add_and_close_recursive(i, j);
    }
  }

  pub fn with_less(&self, i: u8, j: u8) -> Poset {
    let mut poset = self.clone();
    poset.add_less(i, j);
    poset
  }

  pub fn with_less_normalized(&self, i: u8, j: u8) -> Poset {
    let mut poset = self.clone();
    poset.add_less(i, j);
    poset.normalize();
    poset
  }

  // reduce
  pub fn calculate_relations(&self) -> ([u8; MAX_N], [u8; MAX_N]) {
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

  fn reduce_elements(&mut self) {
    let (less, greater) = self.calculate_relations();

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

      if self.n <= 2 * self.i {
        self.dual();
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
    let old_poset = self.clone();
    let mut new_indices: Vec<u8>;

    if ONLY_NAUTY_CANONIFY {
      new_indices = self.canonify_nauty_indicies();
    } else {
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

    for i in 0..self.n {
      for j in (i + 1)..self.n {
        debug_assert!(!self.is_less(i, j));
      }
    }
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

    let poset_check = poset_initial.with_less_normalized(j, i);
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

          let poset_next_normal = poset_next.with_less(i, j);
          if self != &poset_next_normal {
            continue;
          }

          let poset_check = poset_next.with_less_normalized(j, i);
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
  fn filter(unfiltered: &HashSet<Poset>, n: u8, i: u8) -> HashSet<Poset> {
    // TODO: in theory faster, practical not
    // let mut tree: CacheTreeItem<true> = CacheTreeItem::new(n, i);
    // for item in unfiltered {
    //   tree.insert(item);
    // }
    // tree.entries()

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
    let mut greater = 0u8;
    for k in 0..self.n {
      if self.is_less(k, element) {
        greater += 1;
      }
    }
    self.i < greater
  }

  fn enlarge_n(&self, result: &mut HashSet<Poset>) {
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
          if new_poset.can_reduce_element_greater(poset.n - 1) {
            unfiltered.insert(new_poset);
          }
        }
      }
    }

    for item in Poset::filter(&unfiltered, self.n + 1, self.i) {
      let mut it = item.clone();
      it.canonify();
      result.insert(it);
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

  fn enlarge_nk(&self, result: &mut HashSet<Poset>) {
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
          if new_poset.can_reduce_element_less(poset.n - 1) {
            unfiltered.insert(new_poset);
          }
        }
      }
    }

    for item in Poset::filter(&unfiltered, self.n + 1, self.i + 1) {
      let mut it = item.clone();
      it.canonify();
      result.insert(it);
    }
  }

  pub fn enlarge(set_of_posets: &HashSet<Poset>, n: u8, i: u8) -> HashSet<Poset> {
    debug_assert!(2 * i < n);

    let mut temp_set: [[CacheTreeItem<true>; MAX_N]; MAX_N] = Default::default();
    for n0 in 0..=n {
      for i0 in 0..=i {
        temp_set[n0 as usize][i0 as usize] = CacheTreeItem::new(n0, i0);
      }
    }

    for item in set_of_posets {
      if item.n <= n && item.i <= i {
        debug_assert!(2 * item.i < item.n);
        temp_set[item.n as usize][item.i as usize].insert(item);
      }
    }

    for n0 in 1..n {
      let mut result: HashSet<Poset> = HashSet::new();

      for i0 in 0..i {
        for item in &temp_set[n0 as usize][i0 as usize].entries() {
          item.enlarge_nk(&mut result);
        }
        temp_set[n0 as usize][i0 as usize].reset();
      }

      for item in &temp_set[n0 as usize][i as usize].entries() {
        item.enlarge_n(&mut result);
      }
      temp_set[n0 as usize][i as usize].reset();

      for item in result {
        temp_set[item.n as usize][item.i as usize].insert(&item);
      }
    }

    temp_set[n as usize][i as usize].entries()
  }
}

#[test]
pub fn test1() {
  let mut poset = Poset::new(4, 1);
  poset.add_less(2, 3);
  dbg!(&poset);
}
