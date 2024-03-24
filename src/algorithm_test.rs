#[cfg(test)]
mod test {

    use indicatif::ProgressBar;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use crate::utils::{advance_permutations, fac};

    fn select_0([a, b]: [usize; 2]) -> usize {
        return a.min(b);
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

        while !advance_permutations(&mut numbers, &mut split_cycles) {
            starters.push(numbers.clone());
        }

        assert_eq!(starters.len(), fac(N, N - SPLITS));

        let pb = ProgressBar::new(fac(N, 1) as u64 / 10000);

        starters.into_par_iter().for_each(|mut permutation| {
            assert_eq!(select_0(permutation.clone()), I, "{permutation:?}");

            let mut cycles = [0; N - SPLITS];
            for i in 0..N - SPLITS {
                cycles[i] = N - SPLITS - i - 1
            }
            let mut i: usize = 0;
            while !advance_permutations(&mut permutation[SPLITS..], &mut cycles) {
                assert_eq!(select_0(permutation.clone()), I, "{permutation:?}");
                i += 1;
                if i % 10000 == 0 {
                    pb.inc(1);
                }
            }
        });
    }
}
