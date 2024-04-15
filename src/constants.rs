pub const MAX_N: usize = 15;

pub const KNOWN_VALUES: [&[usize]; 16] = [
    &[0],                          // n = 0
    &[0],                          // n = 1
    &[1],                          // n = 2
    &[2, 3],                       // n = 3
    &[3, 4],                       // n = 4
    &[4, 6, 6],                    // n = 5
    &[5, 7, 8],                    // n = 6
    &[6, 8, 10, 10],               // n = 7
    &[7, 9, 11, 12],               // n = 8
    &[8, 11, 12, 14, 14],          // n = 9
    &[9, 12, 14, 15, 16],          // n = 10
    &[10, 13, 15, 17, 18, 18],     // n = 11
    &[11, 14, 17, 18, 19, 20],     // n = 12
    &[12, 15, 18, 20, 21, 22, 23], // n = 13
    &[13, 16, 19, 21, 23, 24, 25], // n = 14
    &[14, 17, 20, 23],             // n = 15
];

pub const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

pub const fn max(a: usize, b: usize) -> usize {
    if a < b {
        b
    } else {
        a
    }
}

pub const LOWER_BOUNDS: [[usize; MAX_N]; MAX_N + 1] = {
    let mut values = [[0; MAX_N]; MAX_N + 1];

    let mut n = 0;
    while n < MAX_N + 1 {
        let mut i = 0;
        while i < n {
            values[n][i] = lower_bound(n, i);
            if i < n - 1 {
                let index = min(i, (n - 1) - i - 1);
                if (n - 1) < KNOWN_VALUES.len() && index < KNOWN_VALUES[n - 1].len() {
                    values[n][i] = max(values[n][i], KNOWN_VALUES[n - 1][index] + 1);
                    // (9): https://dl.acm.org/doi/pdf/10.1145/360336.360339
                }
            }

            i += 1;
        }
        n += 1;
    }

    values
};

pub const UPPER_BOUNDS: [[usize; MAX_N]; MAX_N + 1] = {
    let mut values = [[0; MAX_N]; MAX_N + 1];

    let mut n = 0;
    while n < MAX_N + 1 {
        let mut i = 0;
        while i < n {
            values[n][i] = upper_bound(n, i);

            i += 1;
        }
        n += 1;
    }

    values
};

pub const fn upper_bound(n: usize, i0: usize) -> usize {
    let i = i0 + 1;
    match i {
        1 => n - 1,
        2 => n - 2 + n.next_power_of_two().ilog2() as usize,
        3 => {
            n + 1
                + ((n + 2) / 4).next_power_of_two().ilog2() as usize
                + ((n + 3) / 5).next_power_of_two().ilog2() as usize
        }
        _ => {
            let res1 = n - i + (i - 1) * (n + 2 - i).next_power_of_two().ilog2() as usize;
            let res2 = i - 1 + (n - i) * (1 + i).next_power_of_two().ilog2() as usize;

            if res1 < res2 {
                res1
            } else {
                res2
            }
        }
    }
}

pub const fn lower_bound(n: usize, i0: usize) -> usize {
    let i = i0 + 1;

    match i {
        1 => n - 1,
        2 => n - 2 + n.next_power_of_two().ilog2() as usize,
        _ => {
            let mut sum = 0;
            let mut j = 0;
            while j <= i - 2 {
                sum += ((n + j + 1) / (i + j)).next_power_of_two().ilog2() as usize;
                j += 1;
            }
            n + i - 3 + sum
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn upper_bound1(n: i32, i0: i32) -> i32 {
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
        let res2 = n - (n + 1 - i)
            + ((n + 1 - i) - 1) * ((n + 2 - (n + 1 - i)) as f64).log2().ceil() as i32;
        res1.min(res2)
    }

    fn lower_bound1(n: i32, i0: i32) -> i32 {
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

    #[test]
    fn test() {
        for n in 0..=MAX_N {
            for i in 0..(n + 1) / 2 {
                assert_eq!(lower_bound(n, i), lower_bound1(n as i32, i as i32) as usize);
                assert_eq!(upper_bound(n, i), upper_bound1(n as i32, i as i32) as usize);
            }
        }
    }
}
