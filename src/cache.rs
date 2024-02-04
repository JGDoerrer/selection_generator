use std::{
    collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher, mem::size_of, os::unix::process,
};

use serde::{Deserialize, Serialize};

use crate::{constants::LOWER_BOUNDS, poset::Poset, search::Cost};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Entry {
    pub poset: Poset,
    pub cost: Cost,
    pub priority: u16,
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
            let priority = LOWER_BOUNDS[poset.n() as usize - 1]
                [poset.i().min(poset.n() - poset.i() - 1) as usize]
                as u16;

            entry.priority = entry.priority.saturating_add(priority);

            Some(cost)
        } else {
            None
        }
    }

    /// returns true if an entry has been replaced
    pub fn insert(&mut self, poset: Poset, cost: Cost) -> bool {
        let mut hasher = DefaultHasher::new();
        poset.hash(&mut hasher);
        let hash = hasher.finish();

        let row = &mut self.arrays[hash as usize % self.arrays.len()];

        let mut lowest_prio = u16::MAX;
        let mut lowest_prio_index = 0;
        let mut lowest_unsolved_prio = u16::MAX;
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

        let mut replace = false;
        let index = match match_index {
            Some(i) => i,
            None => match free_index {
                Some(i) => {
                    self.len += 1;
                    i
                }
                None => lowest_prio_index,
            },
        };

        if let Some(entry) = row[index] {
            replace = true;
            let priority = entry.priority;

            for entry in row.iter_mut().flatten() {
                entry.priority = entry.priority.saturating_sub(priority);
            }
        }

        let priority = LOWER_BOUNDS[poset.n() as usize - 1]
            [poset.i().min(poset.n() - poset.i() - 1) as usize] as u16;

        row[index] = Some(Entry {
            poset,
            cost,
            priority,
        });

        replace
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
}

pub struct CacheIterator<'a> {
    cache: &'a Cache,
    index: usize,
}

impl<'a> Iterator for CacheIterator<'a> {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.cache.arrays.iter().flatten().flatten().nth(self.index);
        self.index += 1;

        next.copied()
    }
}
