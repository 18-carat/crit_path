use crate::dependency::Dependency;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

/// Graph of dependencies used for topological sorting

pub struct DependencyGraph<T: Copy + Eq + Hash> {
    top: HashMap<T, Dependency<T>>,
}

impl<T: Copy + Eq + Hash> Default for DependencyGraph<T> {
    fn default() -> Self {
        Self {
            top: HashMap::new(),
        }
    }
}

impl<T: Copy + Eq + Hash> DependencyGraph<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a dependency to the graph such that `succ` depends on `prec`

    pub fn depend_on(&mut self, prec: T, succ: T) {
        if prec == succ {
            return;
        }

        match self.top.entry(prec) {
            Entry::Vacant(e) => {
                let mut dep = Dependency::default();

                _ = dep.succ.insert(succ);
                _ = e.insert(dep);
            }

            Entry::Occupied(e) => {
                if !e.into_mut().succ.insert(succ) {
                    return;
                }
            }
        }

        match self.top.entry(succ) {
            Entry::Vacant(e) => _ = e.insert(Dependency::new(1)),
            Entry::Occupied(e) => e.into_mut().num_prec += 1,
        }
    }

    /// Sorts the dependencies into topological layers.
    /// Returns None if there are cyclic dependencies.

    pub fn layers(&mut self) -> Option<Vec<Vec<T>>> {
        let mut top_layers = Vec::new();

        while !self.top.is_empty() {
            let keys = self
                .top
                .iter()
                .filter(|&(_, v)| v.num_prec == 0)
                .map(|(k, _)| *k)
                .collect::<Vec<_>>();

            if keys.is_empty() {
                return None;
            }

            keys.iter().for_each(|k| self.remove(k));
            top_layers.push(keys);
        }

        Some(top_layers)
    }

    /// Sorts the dependencies into a 1D vector.
    /// Returns None if there are cyclic dependencies.

    pub fn sorted(&mut self) -> Option<Vec<T>> {
        if let Some(top_layers) = self.layers() {
            return Some(top_layers.iter().flatten().copied().collect());
        }

        None
    }

    fn remove(&mut self, prec: &T) {
        if let Some(ref p) = &self.top.remove(prec) {
            p.succ.iter().for_each(|s| {
                if let Some(dep) = self.top.get_mut(s) {
                    dep.num_prec -= 1
                }
            })
        }
    }
}
