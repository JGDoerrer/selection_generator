use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{poset::Poset, search::Cost};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    hashmap: HashMap<Poset, Cost>,
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            hashmap: HashMap::default(),
        }
    }
}

impl Cache {
    pub fn get(&self, poset: &Poset) -> Option<&Cost> {
        self.hashmap.get(poset)
    }

    pub fn insert(&mut self, poset: Poset, cost: Cost) {
        self.hashmap.insert(poset, cost);
    }

    pub fn len(&self) -> usize {
        self.hashmap.len()
    }
}
