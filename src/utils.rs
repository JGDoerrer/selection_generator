use chrono::prelude::*;
use std::{process::Command, time::Instant};

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
pub fn advance_combinations(indices: &mut [usize], n: usize) -> bool {
    let k = indices.len();
    for i in (0..k).rev() {
        if indices[i] < n - k + i + 1 {
            indices[i] += 1;
            for j in i + 1..k {
                indices[j] = indices[j - 1] + 1;
            }
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
