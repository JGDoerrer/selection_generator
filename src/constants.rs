pub const MAX_N: usize = 16;

pub const KNOWN_VALUES: [&[usize]; 17] = [
    &[0],                              // n = 0
    &[0],                              // n = 1
    &[1],                              // n = 2
    &[2, 3],                           // n = 3
    &[3, 4],                           // n = 4
    &[4, 6, 6],                        // n = 5
    &[5, 7, 8],                        // n = 6
    &[6, 8, 10, 10],                   // n = 7
    &[7, 9, 11, 12],                   // n = 8
    &[8, 11, 12, 14, 14],              // n = 9
    &[9, 12, 14, 15, 16],              // n = 10
    &[10, 13, 15, 17, 18, 18],         // n = 11
    &[11, 14, 17, 18, 19, 20],         // n = 12
    &[12, 15, 18, 20, 21, 22, 23],     // n = 13
    &[13, 16, 19, 21, 23, 24, 25],     // n = 14
    &[14, 17, 20, 23, 24, 26, 26, 27], // n = 15
    &[15, 18, 21, 24, 26, 27],         // n = 16
];

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

            // TODO: find proof for "V_i(n) <= V_{i + 1}(n) for all i < n/2"
            // if 0 < i && n < KNOWN_VALUES.len() && i - 1 < KNOWN_VALUES[n].len() {
            //     values[n][i] = max(KNOWN_VALUES[n][i - 1], values[n][i]);
            // };

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

const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

const fn max(a: usize, b: usize) -> usize {
    if a < b {
        b
    } else {
        a
    }
}

const fn upper_bound(n: usize, i0: usize) -> usize {
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

const fn lower_bound(n: usize, i0: usize) -> usize {
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
