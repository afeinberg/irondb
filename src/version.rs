use std::assert_matches::*;

use im::OrdMap;

use crate::util;

type VersionMap = OrdMap<u16, u64>;

#[derive(Debug, Clone, PartialEq)]
pub struct VectorClock {
    pub versions: VersionMap,
    ts: u128,
}

impl Default for VectorClock {
    fn default() -> Self {
        VectorClock {
            versions: VersionMap::new(),
            ts: util::current_time_millis(),
        }
    }
}

impl VectorClock {
    pub fn incremented(&self, node: u16, by: u64, ts: u128) -> Self {
        let new_value = self.versions.get(&node).map_or(by, |&old| old + by);
        VectorClock {
            versions: self.versions.update(node, new_value),
            ts,
        }
    }

    pub fn less_than(&self, other: &Self) -> bool {
        self.versions.iter().all(|(node, &version)| {
            other
                .versions
                .get(node)
                .map_or(false, |&other_version| version < other_version)
        })
    }
}

impl PartialOrd for VectorClock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other.versions == self.versions {
            Some(std::cmp::Ordering::Equal)
        } else if self.less_than(other) {
            Some(std::cmp::Ordering::Less)
        } else if other.less_than(self) {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Versioned<T> {
    pub version: VectorClock,
    pub value: T,
}

impl<T> Versioned<T> {
    pub fn new(value: T) -> Self {
        Versioned {
            version: VectorClock::default(),
            value,
        }
    }

    pub fn from(other: Versioned<T>) -> Self {
        Versioned {
            version: other.version,
            value: other.value,
        }
    }
    pub fn with_version(version: VectorClock, value: T) -> Self {
        Versioned { version, value }
    }

    pub fn compare_versions(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_vector_clock() {
        let a = VectorClock::default();
        let b = VectorClock::default();
        assert_matches!(a.partial_cmp(&b), Some(Ordering::Equal));
        let b = b.incremented(0, 1, util::current_time_millis());
        assert_matches!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_matches!(b.partial_cmp(&a), Some(Ordering::Greater));
        let a = a.incremented(1, 1, util::current_time_millis());
        assert_matches!(a.partial_cmp(&b), None);
        let b = b.incremented(1, 1, util::current_time_millis());
        let b = b.incremented(0, 1, util::current_time_millis());
        let b = b.incremented(1, 1, util::current_time_millis());
        assert_matches!(b.partial_cmp(&a), Some(Ordering::Greater));
    }
}