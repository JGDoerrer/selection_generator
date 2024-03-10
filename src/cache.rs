use std::{mem::size_of, sync::{atomic::{AtomicUsize, Ordering}, Mutex}};

use crate::{
    canonified_poset::CanonifiedPoset,
    constants::{LOWER_BOUNDS, MAX_N},
    poset::Poset,
    search_forward::Cost,
};

pub struct Cache {
    arrays: Box<[Bucket]>,
    len: usize,
    entries: AtomicUsize,
}

struct Bucket {
    inner: Mutex<[Row; Self::BUCKET_SIZE]>,
}

#[derive(Debug, Clone, Copy)]
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
        let len = max_bytes / size_of::<Bucket>();
        let mut arrays = Vec::with_capacity(len);
        for _ in 0..len {
            arrays.push(Bucket::new());
        }
        Cache {
            arrays: arrays.into_boxed_slice(),
            len,
            entries: AtomicUsize::new(0),
        }
    }

    pub fn max_entries(&self) -> usize {
        self.arrays.len() * Bucket::BUCKET_SIZE * Row::ROW_LEN
    }

    pub fn hash(poset: &CanonifiedPoset) -> u64 {
        let mut hash = poset.n() as u64 + poset.i() as u64 * MAX_N as u64;

        for i in 0..poset.n() {
            hash = hash.wrapping_mul(hash + 1);
            hash = hash.wrapping_add(poset.get_all_greater_than(i).bits() as u64);
        }

        hash
    }

    pub fn get(&self, poset: &CanonifiedPoset) -> Option<Cost> {
        let hash = Self::hash(poset) as usize;
        let bucket = &self.arrays[(hash / Bucket::BUCKET_SIZE)  % self.len];
        let row = bucket.inner.lock().unwrap()[hash % Bucket::BUCKET_SIZE];

        row.get(poset)
    }

    pub fn get_mut(&self, poset: &CanonifiedPoset) -> Option<Cost> {
        let hash = Self::hash(poset) as usize;
        let bucket = &self.arrays[(hash / Bucket::BUCKET_SIZE)  % self.len];
        let row = &mut bucket.inner.lock().unwrap()[hash % Bucket::BUCKET_SIZE];

        row.get_mut(poset)
    }

    /// returns true if an entry has been replaced
    pub fn insert(&self, poset: CanonifiedPoset, cost: Cost) -> bool {
        let hash = Self::hash(&poset) as usize;
        let bucket = &self.arrays[(hash / Bucket::BUCKET_SIZE)  % self.len];
        let row = &mut bucket.inner.lock().unwrap()[hash % Bucket::BUCKET_SIZE];

        let (matched, replaced) = row.insert(poset, cost);

        if !matched && !replaced {
            self.entries.fetch_add(1, Ordering::Relaxed);
        }

        replaced
    }

    pub fn len(&self) -> usize {
        self.entries.load(Ordering::Relaxed)
    }
	
	/// Return the size of the cache as gigabytes
    pub fn size_as_gigabyte(&self) -> f64 {
        (self.len * size_of::<Entry>()) as f64 / 1024_f64.powf(3.0)
    }
}


impl Bucket {
    const BUCKET_SIZE: usize = 128;
    fn new() -> Bucket {
        Bucket {
            inner: [Row::new(); Self::BUCKET_SIZE].into(),
        }
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
