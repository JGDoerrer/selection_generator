use std::{collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher};

use serde::{Deserialize, Serialize};

use crate::{poset::Poset, search::Cost};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Entry {
    poset: Poset,
    cost: Cost,
    priority: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    arrays: Box<[[Option<Entry>; 16]]>,
    len: usize,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            poset: Poset::new(0, 0),
            cost: Cost::Minimum(0),
            priority: 0,
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            arrays: vec![[None; 16]; 1 << 24].into_boxed_slice(),
            len: 0,
        }
    }
}

impl Cache {
    pub fn get(&self, poset: &Poset) -> Option<&Cost> {
        let mut hasher = DefaultHasher::new();
        poset.hash(&mut hasher);
        let hash = hasher.finish();

        let row = &self.arrays[hash as usize % self.arrays.len()];

        for i in 0..row.len() {
            if let Some(entry) = &row[i] {
                if entry.poset == *poset {
                    return Some(&entry.cost);
                }
            }
        }

        None
    }

    pub fn get_and_do_stuff(&mut self, poset: &Poset) -> Option<&Cost> {
        let mut hasher = DefaultHasher::new();
        poset.hash(&mut hasher);
        let hash = hasher.finish();

        let row = &mut self.arrays[hash as usize % self.arrays.len()];

        let index = row
            .iter()
            .position(|entry| entry.is_some_and(|entry| entry.poset == *poset));

        if let Some(index) = index {
            for i in 0..row.len() {
                if i != index {
                    if let Some(entry) = &mut row[i] {
                        entry.priority -= 1;
                    }
                }
            }

            let entry = row.get_mut(index).unwrap().as_mut().unwrap();

            entry.priority += 1;

            Some(&entry.cost)
        } else {
            None
        }
    }

    pub fn insert(&mut self, poset: Poset, cost: Cost) {
        let mut hasher = DefaultHasher::new();
        poset.hash(&mut hasher);
        let hash = hasher.finish();

        let row = &mut self.arrays[hash as usize % self.arrays.len()];
        let index = row
            .iter()
            .position(|entry| entry.is_some_and(|entry| entry.poset == poset));

        if let Some(index) = index {
            row[index] = Some(Entry {
                poset,
                cost,
                priority: 0,
            });
        } else {
            let index = row.iter().position(|entry| entry.is_none());

            if let Some(index) = index {
                self.len += 1;
                row[index] = Some(Entry {
                    poset,
                    cost,
                    priority: 0,
                });
            } else {
                let mut lowest_prio = i16::MAX;
                let mut index = 0;

                for i in 0..row.len() {
                    if row[i].unwrap().priority < lowest_prio {
                        index = i;
                        lowest_prio = row[i].unwrap().priority;
                    }
                }

                row[index] = Some(Entry {
                    poset,
                    cost,
                    priority: 0,
                });
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
