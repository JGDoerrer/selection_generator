use std::collections::HashMap;

use poset::Poset;

mod poset;

fn main() {
    let mut cache = HashMap::new();
    for n in 0.. {
        for i in 0..(n + 1) / 2 {
            if let Some((comps, tree)) = search_1(Poset::new(n), i, &mut cache) {
                println!("n = {n}, i = {i}, comps = {comps}");
                // println!("tree = {tree:?}");
            } else {
            }
        }
    }
}

#[derive(Debug, Clone)]
enum DecisionTree {
    Compare(u8, u8, Box<DecisionTree>, Box<DecisionTree>),
    End(u8),
}

fn search_1(
    poset: Poset,
    index: u8,
    cache: &mut HashMap<(Poset, u8), Option<(u8, DecisionTree)>>,
) -> Option<(u8, DecisionTree)> {
    if let Some(result) = cache.get(&(poset.clone(), index)) {
        return result.clone();
    }

    // dbg!(&poset.n(), index, comparisons);
    if let Some(i) = poset.get_ith_element(index) {
        cache.insert((poset, index), Some((0, DecisionTree::End(i))));
        return Some((0, DecisionTree::End(i)));
    }

    for j in 0..index {
        if let Some(i) = poset.get_ith_element(j) {
            // dbg!(i);

            // for i in 0..poset.n() {
            //     for j in 0..poset.n() {
            //         if poset.is_less((i, j)) {
            //             println!("{i} < {j}")
            //         }
            //     }
            // }
            // dbg!(&poset);
            // println!("---");

            let poset = poset.reduce(i);

            // for i in 0..poset.n() {
            //     for j in 0..poset.n() {
            //         if poset.is_less((i, j)) {
            //             println!("{i} < {j}")
            //         }
            //     }
            // }
            // dbg!(&poset);
            // println!("+++");

            return search_1(poset, index - 1, cache);
        }
    }

    for j in (index + 1)..poset.n() {
        if let Some(i) = poset.get_ith_element(j) {
            // dbg!(i);

            // for i in 0..poset.n() {
            //     for j in 0..poset.n() {
            //         if poset.is_less((i, j)) {
            //             println!("{i} < {j}")
            //         }
            //     }
            // }
            // dbg!(&poset);
            // println!("---");

            let poset = poset.reduce(i);

            // for i in 0..poset.n() {
            //     for j in 0..poset.n() {
            //         if poset.is_less((i, j)) {
            //             println!("{i} < {j}")
            //         }
            //     }
            // }
            // dbg!(&poset);
            // println!("+++");

            return search_1(poset, index, cache);
        }
    }

    let mut result = None;

    for i in 0..poset.n() {
        for j in 0..poset.n() {
            if i == j || poset.has_order((i, j)) {
                continue;
            }

            let (less, greater) = poset.do_comparison((i, j));

            if poset.is_valid_less((i, j)) && !poset.is_valid_less((j, i)) {
                let less_result = search_1(less, index, cache);

                if let Some((comps_less, dt_less)) = less_result {
                    if let Some((min, min_dt)) = &mut result {
                        if *min > comps_less {
                            *min = comps_less;
                            *min_dt = dt_less
                        }
                    } else {
                        result = Some((comps_less, dt_less))
                    }
                }
            } else if !poset.is_valid_less((i, j)) && poset.is_valid_less((j, i)) {
                let greater_result = search_1(greater, index, cache);

                if let Some((comps_greater, dt_greater)) = greater_result {
                    if let Some((min, min_dt)) = &mut result {
                        if *min > comps_greater {
                            *min = comps_greater;
                            *min_dt = dt_greater
                        }
                    } else {
                        result = Some((comps_greater, dt_greater))
                    }
                }
            } else {
                let less_result = search_1(less, index, cache);
                let greater_result = search_1(greater, index, cache);

                if let Some((comps_less, dt_less)) = less_result {
                    if let Some((comps_greater, dt_greater)) = greater_result {
                        let comps_max = comps_less.max(comps_greater) + 1;

                        if let Some((min, min_dt)) = &mut result {
                            if *min > comps_max {
                                *min = comps_max;
                                *min_dt = DecisionTree::Compare(
                                    i,
                                    j,
                                    Box::new(dt_less),
                                    Box::new(dt_greater),
                                )
                            }
                        } else {
                            result = Some((
                                comps_max,
                                DecisionTree::Compare(
                                    i,
                                    j,
                                    Box::new(dt_less),
                                    Box::new(dt_greater),
                                ),
                            ))
                        }
                    }
                }
            }
        }
    }

    cache.insert((poset, index), result.clone());

    result
}
