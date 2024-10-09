use std::collections::HashSet;
use std::hash::Hash;

pub(crate) struct Dependency<T: Eq + Hash> {
    pub(crate) num_prec: usize,
    pub(crate) succ: HashSet<T>,
}

impl<T: Eq + Hash> Default for Dependency<T> {
    fn default() -> Self {
        Self {
            num_prec: 0,
            succ: HashSet::new(),
        }
    }
}

impl<T: Eq + Hash> Dependency<T> {
    pub(crate) fn new(num_prec: usize) -> Self {
        Self {
            num_prec,
            succ: HashSet::new(),
        }
    }
}
