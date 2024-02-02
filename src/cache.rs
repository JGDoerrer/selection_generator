use std::mem::size_of;

use crate::{
    canonified_poset::CanonifiedPoset,
    constants::{LOWER_BOUNDS, MAX_N},
    poset::Poset,
    search_forward::Cost,
};

#[derive(Debug)]
pub struct Cache {
    arrays: Box<[Row]>,
    len: usize,
}

#[derive(Debug, Clone)]
struct Row {
    entries: [Option<Entry>; Self::ROW_LEN],
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    pub poset: CanonifiedPoset,
    pub cost: Cost,
    pub priority: u16,
}

impl Cache {
    pub fn new(max_bytes: usize) -> Self {
        let len = max_bytes / size_of::<Row>();
        Cache {
            arrays: vec![Row::new(); len].into_boxed_slice(),
            len: 0,
        }
    }

    pub fn max_entries(&self) -> usize {
        self.arrays.len() * Row::ROW_LEN
    }

    fn hash(poset: &CanonifiedPoset) -> u64 {
        let mut hash = poset.n() as u64 + poset.i() as u64 * MAX_N as u64;

        for i in 0..poset.n() {
            hash = hash.wrapping_mul(hash + 1);
            hash = hash.wrapping_add(poset.get_all_greater_than(i).bits() as u64);
        }

        hash
    }

    pub fn get(&self, poset: &CanonifiedPoset) -> Option<Cost> {
        let hash = Self::hash(poset);
        let row = &self.arrays[hash as usize % self.arrays.len()];

        row.get(poset)
    }

    pub fn get_mut(&mut self, poset: &CanonifiedPoset) -> Option<Cost> {
        let hash = Self::hash(poset);

        let row = &mut self.arrays[hash as usize % self.arrays.len()];

        row.get_mut(poset)
    }

    /// returns true if an entry has been replaced
    pub fn insert(&mut self, poset: CanonifiedPoset, cost: Cost) -> bool {
        let hash = Self::hash(&poset);

        let row = &mut self.arrays[hash as usize % self.arrays.len()];

        let (matched, replaced) = row.insert(poset, cost);

        if !matched && !replaced {
            self.len += 1;
        }

        replaced
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return the size of the cache as gigabytes
    pub fn size_as_gigabyte(&self) -> f64 {
        (self.len * size_of::<Entry>()) as f64 / 1024_f64.powf(3.0)
    }
}

impl Row {
    const ROW_LEN: usize = 4;

    pub fn new() -> Self {
        Row {
            entries: [None; Self::ROW_LEN],
        }
    }

    pub fn get(&self, poset: &CanonifiedPoset) -> Option<Cost> {
        for entry in self.entries.iter().map_while(|e| e.as_ref()) {
            if entry.poset == *poset {
                return Some(entry.cost);
            }
        }

        None
    }

    pub fn get_mut(&mut self, poset: &CanonifiedPoset) -> Option<Cost> {
        let index = self
            .entries
            .iter()
            .map_while(|e| *e)
            .position(|entry| entry.poset == *poset);

        if let Some(index) = index {
            let entry = &mut self.entries[index].unwrap();
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
    pub fn insert(&mut self, poset: CanonifiedPoset, cost: Cost) -> (bool, bool) {
        let mut matched = false;
        let mut replaced = false;

        let index = 'index: {
            let mut lowest_prio = u16::MAX;
            let mut lowest_prio_index = 0;

            for i in 0..Self::ROW_LEN {
                match self.entries[i] {
                    Some(entry) => {
                        if entry.poset == poset {
                            matched = true;
                            break 'index i;
                        }
                        if entry.priority < lowest_prio {
                            lowest_prio = entry.priority;
                            lowest_prio_index = i;
                        }
                    }
                    None => break 'index i,
                }
            }

            replaced = true;
            lowest_prio_index
        };

        if let Some(entry) = self.entries[index] {
            let priority = entry.priority;

            for entry in self.entries.iter_mut().map_while(|e| e.as_mut()) {
                entry.priority = entry.priority.saturating_sub(priority);
            }
        }

        let priority = LOWER_BOUNDS[poset.n() as usize - 1]
            [poset.i().min(poset.n() - poset.i() - 1) as usize] as u16;

        self.entries[index] = Some(Entry {
            poset,
            cost,
            priority,
        });

        // let len = if replaced || matched { len } else { len + 1 };

        // self.entries[0..len].sort_by(|a, b| b.unwrap().priority.cmp(&a.unwrap().priority));

        (matched, replaced)
    }
}
