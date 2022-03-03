use im::OrdMap;

use crate::util;

type VersionMap = OrdMap<u16, u64>;

#[derive(Debug, Clone, PartialEq)]
pub struct VectorClock {
    versions: VersionMap,
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
            other.versions.get(node).map_or(false, |&other_version| version < other_version)
        })
    }

}

impl PartialOrd for VectorClock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.less_than(other) {
            Some(std::cmp::Ordering::Less)
        } else if other.less_than(self) {
            Some(std::cmp::Ordering::Greater)
        } else if other.versions == self.versions {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Versioned<T> {
    version: VectorClock,
    value: T,
}

impl<T> Versioned<T> {
    pub fn new(value: T) -> Self {
        Versioned {
            version: VectorClock::default(),
            value,
        }
    }

    pub fn with_version(version: VectorClock, value: T) -> Self {
        Versioned { version, value }
    }

    pub fn compare_versions(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}