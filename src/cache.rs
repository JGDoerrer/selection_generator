use std::{collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher, mem::size_of};

use serde::{Deserialize, Serialize};

use crate::{poset::Poset, search::Cost, KNOWN_MIN_VALUES};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Entry {
    pub poset: Poset,
    pub cost: Cost,
    pub priority: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    arrays: Box<[[Option<Entry>; Self::ROW_LEN]]>,
    len: usize,
}

impl Cache {
    const ROW_LEN: usize = 8;

    pub fn new(max_bytes: usize) -> Self {
        let len = max_bytes / size_of::<[Option<Entry>; Self::ROW_LEN]>();
        Cache {
            arrays: vec![[None; Self::ROW_LEN]; len].into_boxed_slice(),
            len: 0,
        }
    }

    pub fn max_entries(&self) -> usize {
        self.arrays.len() * Self::ROW_LEN
    }

    pub fn get(&self, poset: &Poset) -> Option<Cost> {
        let mut hasher = DefaultHasher::new();
        poset.hash(&mut hasher);
        let hash = hasher.finish();

        let row = &self.arrays[hash as usize % self.arrays.len()];

        for entry in row.iter().flatten() {
            if entry.poset == *poset {
                return Some(entry.cost);
            }
        }

        None
    }

    pub fn get_and_do_stuff(&mut self, poset: &Poset) -> Option<Cost> {
        let mut hasher = DefaultHasher::new();
        poset.hash(&mut hasher);
        let hash = hasher.finish();

        let row = &mut self.arrays[hash as usize % self.arrays.len()];

        let index = row
            .iter()
            .position(|entry| entry.is_some_and(|entry| entry.poset == *poset));

        if let Some(index) = index {
            let entry = row.get_mut(index).unwrap().as_mut().unwrap();
            let cost = entry.cost;
            let priority = KNOWN_MIN_VALUES[poset.n() as usize - 1]
                [poset.i().min(poset.n() - poset.i() - 1) as usize]
                as i16;

            entry.priority = entry.priority.saturating_add(priority);

            Some(cost)
        } else {
            None
        }
    }

    pub fn insert(&mut self, poset: Poset, cost: Cost) {
        let mut hasher = DefaultHasher::new();
        poset.hash(&mut hasher);
        let hash = hasher.finish();

        let row = &mut self.arrays[hash as usize % self.arrays.len()];

        let mut lowest_prio = i16::MAX;
        let mut lowest_prio_index = 0;
        let mut lowest_unsolved_prio = i16::MAX;
        let mut lowest_unsolved_prio_index = None;
        let mut match_index = None;
        let mut free_index = None;

        for (i, entry) in row.iter().enumerate() {
            if let Some(entry) = entry {
                if !entry.cost.is_solved() && entry.priority < lowest_unsolved_prio {
                    lowest_unsolved_prio = entry.priority;
                    lowest_unsolved_prio_index = Some(i);
                }
                if entry.priority < lowest_prio {
                    lowest_prio = entry.priority;
                    lowest_prio_index = i;
                }
                if entry.poset == poset {
                    match_index = Some(i);
                }
            } else {
                free_index = Some(i);
            }
        }

        let index = match match_index {
            Some(i) => i,
            None => match free_index {
                Some(i) => {
                    self.len += 1;
                    i
                }
                None => match lowest_unsolved_prio_index {
                    Some(i) => {
                        let _old_entry = row[i].unwrap();
                        i
                    }
                    None => {
                        let _old_entry = row[lowest_prio_index].unwrap();
                        lowest_prio_index
                    }
                },
            },
        };

        for (i, entry) in row.iter_mut().enumerate() {
            if i != index {
                if let Some(entry) = entry {
                    entry.priority = entry.priority.saturating_sub(1);
                }
            }
        }

        let priority = KNOWN_MIN_VALUES[poset.n() as usize - 1]
            [poset.i().min(poset.n() - poset.i() - 1) as usize] as i16;

        row[index] = Some(Entry {
            poset,
            cost,
            priority,
        });
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> CacheIterator {
        CacheIterator {
            cache: self,
            index: 0,
        }
    }

    pub fn counts(&self) -> Vec<u64> {
        let mut counts = vec![0; 50]; // i hope this is enough

        for row in self.arrays.iter() {
            for entry in row.iter().flatten() {
                counts[entry.cost.value() as usize] += 1;
            }
        }

        while counts.last().copied() == Some(0) {
            counts.pop();
        }

        counts
    }
}

pub struct CacheIterator<'a> {
    cache: &'a Cache,
    index: usize,
}

impl<'a> Iterator for CacheIterator<'a> {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .cache
            .arrays
            .iter()
            .flatten()
            .flatten()
            .skip(self.index)
            .next();
        self.index += 1;

        next.copied()
    }
}
