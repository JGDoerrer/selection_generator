use chrono::prelude::*;
use std::{cmp::Ordering, process::Command, time::Instant};

/// Print the git sha of the last commit
/// the sha is equal to: git rev-parse --short --head
pub fn print_git_info() {
    println!();
    println!("Branch: {}", env!("GIT_BRANCH"));
    println!("SHA: {}", env!("GIT_HASH"));
    println!();
}

/// Print a subset of the information of lscpu.
/// Interesting values are:
/// Model name, Core, Thread, min `MHz`, max MHZ
pub fn print_lscpu() {
    let lscpu = Command::new("sh")
        .arg("-c")
        .arg("lscpu | grep -E 'Model Name|Core|Thread|min MHz|max MHz' | tr -s ' '")
        .output()
        .unwrap();
    let lscpu = String::from_utf8(lscpu.stdout).unwrap();

    println!("{lscpu}");
}

pub fn print_current_time() {
    // Get the current time in the local time zone
    let local_time = Local::now();

    // Format the time as a human-readable string
    let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Print the formatted time
    println!("Current time: {formatted_time}");
    println!("===============");
}

#[allow(unused)]
pub fn advance_permutations(indices: &mut [usize], cycles: &mut [usize]) -> bool {
    advance_permutations2(indices, indices.len(), cycles, cycles.len())
}

pub fn advance_permutations2(
    indices: &mut [usize],
    n: usize,
    cycles: &mut [usize],
    k: usize,
) -> bool {
    for i in (0..k).rev() {
        if cycles[i] == 0 {
            cycles[i] = n - i - 1;
            indices[i..].rotate_left(1);
        } else {
            let si = n - cycles[i];
            indices.swap(i, si);
            cycles[i] -= 1;
            return false;
        }
    }
    true
}

#[allow(unused)]
pub fn advance_permutations3(indices: &mut Vec<usize>, cycles: &mut [usize]) -> bool {
    let n = indices.len();
    let k = cycles.len();
    for i in (0..k).rev() {
        if cycles[i] == 0 {
            cycles[i] = n - i - 1;
            indices[i..].rotate_left(1);
        } else {
            let si = n - cycles[i];
            indices.swap(i, si);
            cycles[i] -= 1;
            return false;
        }
    }
    true
}

#[allow(unused)]
pub const fn fac(n: usize, k: usize) -> usize {
    let mut r = 1;
    let mut i = k + 1;
    while i <= n {
        r *= i;
        i += 1;
    }
    r
}

/// Print out a human readable duration in the format:
/// days, hours, minutes, seconds
pub fn format_duration(start: Instant) -> String {
    // Calculate the values for a human readable duration

    let duration = start.elapsed();
    let seconds = duration.as_secs_f32() % 60.0;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / (60 * 60)) % 24;
    let days = duration.as_secs() / (60 * 60 * 24);

    format!("Duration: {days}d {hours}h {minutes}m {seconds}s")
}

pub fn format_memory(memory: u64) -> String {
    format!("{:.3}", memory as f64 / ((10usize).pow(9) as f64))
}

/**
 * Takes a sorted `Vec<T>`, `a`, and a sorted slice, `b`, as input and merge it into the sorted `Vec<T>`
 */
#[inline]
pub fn extend_sorted<T, F>(a: &mut Vec<T>, b: &[T], mut compare: F)
where
    T: Copy + Default,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut a_pos = a.len();
    let mut b_pos = b.len();

    a.resize(a_pos + b_pos, T::default());

    while 0 < a_pos && 0 < b_pos {
        a[a_pos + b_pos] = if compare(&a[a_pos - 1], &b[b_pos - 1]).is_ge() {
            a_pos -= 1;
            a[a_pos]
        } else {
            b_pos -= 1;
            b[b_pos]
        };
    }

    if 0 != b_pos {
        a[..b_pos].copy_from_slice(&b[..b_pos]);
    }
}

// {
//   let mut rng = rand::thread_rng();
//   for _ in 0..100 {
//       for a_len in 0..5 {
//           for b_len in 0..5 {
//               let mut a: Vec<usize> = vec![];
//               for _ in 0..a_len {
//                   a.push(rng.gen::<usize>() % 20);
//               }
//               a.sort_unstable();
//               let mut b: Vec<usize> = vec![];
//               for _ in 0..b_len {
//                   b.push(rng.gen::<usize>() % 20);
//               }
//               b.sort_unstable();

//               let mut c = a.clone();
//               c.extend(&b);
//               c.sort_unstable();

//               let mut new_a = a.clone();
//               extend_sorted(&mut new_a, &b, std::cmp::Ord::cmp);

//               dbg!(a, b);
//               assert_eq!(new_a, c);
//           }
//       }
//   }
// }
