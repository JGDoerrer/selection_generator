pub const DEBUG: bool = true;

pub const ONLY_NAUTY_CANONIFY: bool = true;       // TODO: test because somethimes FALSE
pub const ALWAYS_USE_SUBGRAPH_ISOMORPHISM: bool = false;

pub const MAX_THREAD_COUNT: usize = 20;
pub const MAX_COMPARISONS: usize = 25;
pub const MAX_N: usize = 15;
pub const MAX_N_BITS: usize = ((MAX_N * MAX_N - 1) / 64) + 1;

pub const KNOWN_MIN_VALUES: [&[u8]; 16] = [
  /* n= 0 */ &[],
  /* n= 1 */ &[],
  /* n= 2 */ &[1, 1],
  /* n= 3 */ &[2, 3, 2],
  /* n= 4 */ &[3, 4, 4, 3],
  /* n= 5 */ &[4, 6, 6, 6, 4],
  /* n= 6 */ &[5, 7, 8, 8, 7, 5],
  /* n= 7 */ &[6, 8, 10, 10, 10, 8, 6],
  /* n= 8 */ &[7, 9, 11, 12, 12, 11, 9, 7],
  /* n= 9 */ &[8, 11, 12, 14, 14, 14, 12, 11, 8],
  /* n=10 */ &[9, 12, 14, 15, 16, 16, 15, 14, 12, 9],
  /* n=11 */ &[10, 13, 15, 17, 18, 18, 18, 17, 15, 13, 10],
  /* n=12 */ &[11, 14, 17, 18, 19, 20, 20, 19, 18, 17, 14, 11],
  /* n=13 */ &[12, 15, 18, 20, 21, 22, 23, 22, 21, 20, 18, 15, 12],
  /* n=14 */ &[13, 16, 19, 21, 23, 24, 24, 24, 24, 23, 21, 19, 16, 13],
  /* n=15 */ &[
    14, 17, 20, 23, 25, 25, 23, 24, 24, 23, 25, 25, 25, 23, 20, 17, 14,
  ],
];

pub const min_n_comparisons_len: usize = 15;

pub fn upper_bound(n: i32, i0: i32) -> i32 {
  let i = i0 + 1;
  if 1 == i {
    return n - 1;
  } else if 2 == i {
    return ((n - 2) as f64 + (n as f64).log2().ceil()) as i32;
  } else if 3 == i {
    return ((n + 1) as f64
      + ((n - 1) as f64 / 4.0).log2().ceil()
      + ((n - 1) as f64 / 5.0).log2().ceil()) as i32;
  }

  let res1 = n - i + (i - 1) * ((n + 2 - i) as f64).log2().ceil() as i32;
  let res2 =
    n - (n + 1 - i) + ((n + 1 - i) - 1) * ((n + 2 - (n + 1 - i)) as f64).log2().ceil() as i32;
  res1.min(res2)
}

pub fn lower_bound(n: i32, i0: i32) -> i32 {
  let i = i0 + 1;
  if i == 1 {
    return n - 1;
  } else if i == 2 {
    return n - 2 + (n as f64).log2().ceil() as i32;
  }
  let mut sum = 0;
  for j in 0..=i - 2 {
    sum += ((n - i + 2) as f64 / (i + j) as f64).log2().ceil() as i32;
  }
  n + i - 3 + sum
}
