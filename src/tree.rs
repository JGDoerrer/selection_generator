use std::{
    hash::Hash,
    sync::{atomic::AtomicU64, Arc, RwLock},
};

use crate::{canonified_poset::CanonifiedPoset, poset::Poset, search_forward::Cost};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum OtherState {
    Solved(u8),
    Open(CanonifiedPoset),
}

#[derive(Clone, Debug)]
pub enum Parent {
    Parent(Arc<SearchState>),
    Root(u8),
    Estimate(Box<Task>),
}

impl Hash for Parent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Parent::Parent(parent) => {
                parent.poset.hash(state);
            }
            Parent::Root(max_comparisons) => max_comparisons.hash(state),
            Parent::Estimate(task) => task.hash(state),
        }
    }
}

#[derive(Debug, Hash, Clone)]
pub struct Task {
    pub poset: CanonifiedPoset,
    pub parent: Parent,
    pub other: OtherState,
    pub depth: u8,
    pub comparison: (u8, u8),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Priority {
    pub max_comparisons: u8,
    pub compatible_posets: usize,
    pub hardness: u32,
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
    pub comparison: (u8, u8),
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
            Parent::Estimate(task) => task.max_comparisons(),
        }
    }

    pub fn make_second_task(&self, cost: Cost) -> Option<Task> {
        match cost {
            Cost::Solved(solved) => match &self.other {
                OtherState::Solved(_) => None,
                OtherState::Open(poset) => {
                    if solved <= self.max_comparisons() {
                        Some(Task {
                            poset: *poset,
                            parent: self.parent.clone(),
                            other: OtherState::Solved(solved),
                            depth: self.depth,
                            comparison: self.comparison.clone(),
                        })
                    } else {
                        None
                    }
                }
            },
            Cost::Minimum(_) => None,
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
            Parent::Estimate(task) => task.max_comparisons(),
        }
    }

    pub fn expand(self) -> (Arc<SearchState>, impl Iterator<Item = (Task, Priority)>) {
        let pairs = self.poset.get_comparison_pairs();
        let n_pairs = pairs.len() as u64;

        let state = Arc::new(SearchState {
            current_best: Cost::Minimum(self.max_comparisons() + 1).into(),
            poset: self.poset,
            parent: self.parent,
            other: self.other,
            total_children: n_pairs,
            open_children: n_pairs.into(),
            depth: self.depth,
            comparison: self.comparison,
        });
        (
            state.clone(),
            pairs
                .into_iter()
                .map(move |(first, second, i, j, hardness)| {
                    let task = Task {
                        poset: first,
                        parent: Parent::Parent(state.clone()),
                        other: OtherState::Open(second),
                        depth: state.depth + 1,
                        comparison: (i, j),
                    };
                    let max_comparisons = task.max_comparisons();
                    let compatible_posets = task.poset.num_compatible_posets();
                    (
                        task,
                        Priority {
                            max_comparisons,
                            compatible_posets,
                            hardness,
                        },
                    )
                }),
        )
    }

    pub fn to_estimate_solvable_task(self, (i, j): (u8, u8)) -> Task {
        return Task {
            poset: self.poset.with_less(i, j),
            other: OtherState::Solved(0),
            depth: self.depth + 1,
            comparison: (i, j),
            parent: Parent::Estimate(Box::new(self)),
        };
    }

    pub fn make_second_task(&self, cost: Cost) -> Option<Task> {
        match cost {
            Cost::Solved(solved) => match &self.other {
                OtherState::Solved(_) => None,
                OtherState::Open(poset) => {
                    if solved <= self.max_comparisons() {
                        Some(Task {
                            poset: *poset,
                            parent: self.parent.clone(),
                            other: OtherState::Solved(solved),
                            depth: self.depth,
                            comparison: self.comparison.clone(),
                        })
                    } else {
                        None
                    }
                }
            },
            Cost::Minimum(_) => None,
        }
    }

    pub fn priority(&self) -> Priority {
        Priority {
            max_comparisons: self.max_comparisons(),
            compatible_posets: self.poset.num_compatible_posets(),
            hardness: self.poset.estimate_hardness(),
        }
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
            && self.depth == other.depth
            && self.parent == other.parent
            && self.other == other.other
            && false
    }
}

impl Eq for Task {}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .max_comparisons
            .cmp(&self.max_comparisons)
            .then(other.compatible_posets.cmp(&self.compatible_posets))
            .then(self.hardness.cmp(&other.hardness))
    }
}

mod test {
    use std::cmp::Reverse;

    use priority_queue::PriorityQueue;

    use super::Priority;

    #[test]
    fn priority() {
        let mut queue = PriorityQueue::new();
        let p0 = Priority {
            max_comparisons: 2,
            compatible_posets: 3,
            hardness: 10,
        };
        let p1 = Priority {
            max_comparisons: 3,
            compatible_posets: 2,
            hardness: 1,
        };
        let p2 = Priority {
            max_comparisons: 2,
            compatible_posets: 2,
            hardness: 11,
        };
        queue.push(0, p0);
        queue.push(1, p1);
        queue.push(2, p2);

        assert_eq!(queue.pop().unwrap().1, p2);
        assert_eq!(queue.pop().unwrap().1, p0);
        assert_eq!(queue.pop().unwrap().1, p1);

    }

    #[test]
    fn sort() {
        let p0 = Priority {
            max_comparisons: 2,
            compatible_posets: 3,
            hardness: 10,
        };
        let p1 = Priority {
            max_comparisons: 3,
            compatible_posets: 2,
            hardness: 1,
        };
        let p2 = Priority {
            max_comparisons: 2,
            compatible_posets: 2,
            hardness: 11,
        };

        let mut queue = vec![p0, p1, p2];
        queue.sort_unstable_by_key(|e| e.clone());

        assert_eq!(queue, vec![p1, p0, p2]);
    }
}
