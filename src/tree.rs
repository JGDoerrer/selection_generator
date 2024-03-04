use std::{hash::Hash, sync::{atomic::{AtomicU64, Ordering}, Arc, RwLock}};

use crate::{cache::Cache, canonified_poset::CanonifiedPoset, search::Cost};


#[derive(Clone, Debug, Hash, PartialEq)]
pub enum OtherState {
    Solved(u8),
    Open(CanonifiedPoset),
}

#[derive(Clone, Debug)]
pub enum Parent {
    Parent(Arc<SearchState>),
    Root(u8),
}

impl Hash for Parent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Parent::Parent(parent) => {
                parent.poset.hash(state);
            },
            Parent::Root(max_comparisons) => max_comparisons.hash(state),
        }
    }
}

#[derive(Debug, Hash)]
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

    fn sibling_completion(&self) -> u64 {
        match &self.parent {
            Parent::Parent(parent) => ((parent.total_children - parent.open_children.load(Ordering::Relaxed)) * 100) / parent.total_children,
            Parent::Root(_) => 1,
        }
    }

    pub fn priority(&self) -> u64 {
        if self.depth < 5 {
            return self.depth as u64;
        }
        self.sibling_completion().saturating_sub(self.depth as u64 * 10) + 6
    }
}

impl PartialEq for Parent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Parent(l0), Self::Parent(r0)) => Arc::ptr_eq(l0, r0),
            (Self::Root(l0), Self::Root(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.poset == other.poset
            && self.depth == other.depth && self.parent == other.parent && self.other == other.other
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