use std::sync::{atomic::AtomicU64, Arc, RwLock};

use crate::{cache::Cache, canonified_poset::CanonifiedPoset, search::Cost};


#[derive(Clone, Debug)]
pub enum OtherState {
    Solved(u8),
    Open(CanonifiedPoset),
}

#[derive(Clone, Debug)]
pub enum Parent {
    Parent(Arc<SearchState>),
    Root(u8),
}

#[derive(Debug)]
pub struct Task {
    pub poset: CanonifiedPoset,
    pub parent: Parent,
    pub other: OtherState,
    pub depth: u8,
}

#[derive(Debug)]
pub struct SearchState {
    pub poset: CanonifiedPoset,
    pub parent: Parent,
    pub other: OtherState,
    pub total_children: u64,
    pub open_children: AtomicU64,
    pub depth: u8,
    pub current_best: RwLock<Cost>,
}


impl SearchState {
    pub fn max_comparisons(&self) -> u8 {
        match &self.parent {
            Parent::Parent(parent) => parent
                .current_best()
                .value()
                .saturating_sub(2)
                .min(parent.max_comparisons() - 1),
            Parent::Root(max_comparisons) => *max_comparisons,
        }
    }

    pub fn current_best(&self) -> Cost {
        self.current_best.read().unwrap().clone()
    }
}

impl Task {
    pub fn max_comparisons(&self) -> u8 {
        match &self.parent {
            Parent::Parent(parent) => parent
                .current_best()
                .value()
                .saturating_sub(2)
                .min(parent.max_comparisons() - 1),
            Parent::Root(max_comparisons) => *max_comparisons,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.poset == other.poset
            && self.depth == other.depth
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.depth.cmp(&other.depth) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        Cache::hash(&self.poset).cmp(&Cache::hash(&self.poset))
    }
}