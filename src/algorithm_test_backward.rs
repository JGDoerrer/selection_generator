#[cfg(test)]
mod test {
    use std::{
        fs::{self, File},
        io::BufReader,
    };

    use hashbrown::HashMap;
    use indicatif::ProgressBar;
    use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

    use crate::{
        constants::MAX_N,
        utils::{advance_permutations3, fac},
        BinaryItem,
    };

    pub fn select(
        algorithm: &HashMap<usize, BinaryItem>,
        permutation: [usize; MAX_N],
        index: usize,
        is_dual: bool,
    ) -> usize {
        let action = algorithm.get(&index).unwrap();
        let new_dual: bool;
        let new_index: usize;
        let mut new_permutation = [0; MAX_N];

        if 0 == action.i && 0 == action.j {
            return permutation[0];
        }

        if (!is_dual && permutation[action.i as usize] < permutation[action.j as usize])
            || (is_dual && permutation[action.j as usize] < permutation[action.i as usize])
        {
            new_index = action.less_index;
            new_dual = if action.less_is_dual {
                !is_dual
            } else {
                is_dual
            };
            for i in 0..algorithm.get(&action.less_index).unwrap().n as usize {
                new_permutation[i] = permutation[action.less_mapping[i] as usize];
            }
        } else {
            new_index = action.greater_index;
            new_dual = if action.greater_is_dual {
                !is_dual
            } else {
                is_dual
            };
            for i in 0..algorithm.get(&action.greater_index).unwrap().n as usize {
                new_permutation[i] = permutation[action.greater_mapping[i] as usize];
            }
        }
        select(algorithm, new_permutation, new_index, new_dual)
    }

    pub fn check(n: usize, i: usize, algorithm: &HashMap<usize, BinaryItem>) {
        const SPLITS: usize = 1;

        let mut numbers = vec![0; n];
        for k in 0..n {
            numbers[k] = k;
        }

        let mut split_cycles = [0; SPLITS];
        for k in 0..SPLITS {
            split_cycles[k] = n - k - 1;
        }

        let mut starters = Vec::with_capacity(fac(n, n - SPLITS));

        starters.push(numbers.clone());

        while !advance_permutations3(&mut numbers, &mut split_cycles) {
            starters.push(numbers.clone());
        }

        assert_eq!(starters.len(), fac(n, n - SPLITS));

        let pb = ProgressBar::new(fac(n, 1) as u64 / 10000);

        starters.par_iter_mut().for_each(|mut permutation| {
            let mut permutation_array = [0; MAX_N];
            permutation_array[..permutation.len()].copy_from_slice(&permutation[..]);
            assert_eq!(
                select(algorithm, permutation_array, 0, false),
                i,
                "n: {n}, i: {i}, permutation: {permutation:?}"
            );

            let mut cycles = vec![0; n - SPLITS];
            for k in 0..n - SPLITS {
                cycles[k] = n - SPLITS - k - 1;
            }
            let mut k: usize = 0;
            let mut temp = vec![];
            for k in SPLITS..permutation.len() {
                temp.push(permutation[k]);
            }
            while !advance_permutations3(&mut temp, &mut cycles) {
                let mut permutation_array = [0; MAX_N];
                permutation_array[..permutation.len()].copy_from_slice(&permutation[..]);
                assert_eq!(
                    select(algorithm, permutation_array, 0, false),
                    i,
                    "n: {n}, i: {i}, permutation: {permutation:?}"
                );
                k += 1;
                if k % 10000 == 0 {
                    pb.inc(1);
                }
            }
        });
    }

    #[test]
    fn test() {
        if let Ok(entries) = fs::read_dir("algorithms") {
            let mut files = vec![];

            for entry in entries.flatten() {
                let filename = entry.file_name().into_string().ok().unwrap();
                let parts: Vec<&str> = filename.split('_').collect();

                if parts.len() == 2 {
                    if let (Ok(n), Ok(i)) = (
                        parts[0].parse::<usize>(),
                        parts[1].split('.').next().unwrap().parse::<usize>(),
                    ) {
                        files.push((n, i, entry));
                    } else {
                        panic!("Error parsing numbers");
                    }
                } else {
                    panic!("Invalid input format");
                }
            }

            files.sort_by(|(n_a, i_a, _), (n_b, i_b, _)| n_a.cmp(n_b).then_with(|| i_a.cmp(i_b)));

            for (n, i, file) in files {
                println!(
                    "check algorithm for n: {n}, i: {i}, file: {}",
                    file.file_name().into_string().ok().unwrap()
                );

                let file = File::open(file.path()).unwrap();

                let mut reader = BufReader::new(file);

                let mut algorithm: HashMap<usize, BinaryItem> = HashMap::new();

                while let Ok(item) = bincode::deserialize_from::<
                    &mut std::io::BufReader<std::fs::File>,
                    BinaryItem,
                >(&mut reader)
                {
                    algorithm.insert(item.index, item);
                }

                check(n, i, &algorithm);
                println!("success");
            }
        } else {
            println!("Failed to read directory");
        }
    }
}
