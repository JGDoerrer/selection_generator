#[cfg(test)]
mod test {

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    /// n = 1
    fn select_1([a]: [usize; 1]) -> usize {
        a
    }
    /// n = 2, i = 0
    fn select_0([a, b]: [usize; 2]) -> usize {
        if b > a {
            select_1([a])
        } else {
            select_1([b])
        }
    }

    fn advance(indices: &mut [usize], cycles: &mut [usize]) -> bool {
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
                return true;
            }
        }
        false
    }

    const fn fac(n: usize, k: usize) -> usize {
        let mut r = 1;
        let mut i = k + 1;
        while i <= n {
            r *= i;
            i += 1;
        }
        r
    }

    #[test]
    fn test() {
        const N: usize = 2;
        const I: usize = 0;
        const SPLITS: usize = 2;

        let mut numbers = [0; N];
        for i in 0..N {
            numbers[i] = i;
        }

        let mut split_cycles = [0; SPLITS];
        for i in 0..SPLITS {
            split_cycles[i] = N - i - 1
        }

        let mut starters = Vec::with_capacity(fac(N, N - SPLITS));

        starters.push(numbers.clone());

        while advance(&mut numbers, &mut split_cycles) {
            starters.push(numbers.clone());
        }

        assert_eq!(starters.len(), fac(N, N - SPLITS));

        starters.into_par_iter().for_each(|mut permutation| {
            assert_eq!(search_0(permutation.clone()), I, "{permutation:?}");

            let mut cycles = [0; N - SPLITS];
            for i in 0..N - SPLITS {
                cycles[i] = N - SPLITS - i - 1
            }
            while advance(&mut permutation[SPLITS..], &mut cycles) {
                assert_eq!(search_0(permutation.clone()), I, "{permutation:?}");
            }
        });
    }
}
