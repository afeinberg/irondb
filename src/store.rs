use std::collections::HashMap;
use std::hash::Hash;

use crate::version::{VectorClock, Versioned};

type VersionedVec<T> = Vec<Versioned<T>>;

pub trait Store: Clone {
    type Key;
    type Value;
    type Error;

    fn get(&self, key: Self::Key) -> Result<Vec<Versioned<Self::Value>>, Self::Error>;
    fn put(
        &mut self,
        key: Self::Key,
        value: Versioned<Self::Value>,
    ) -> Result<VersionedVec<Self::Value>, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct InMemoryStore<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
{
    storage_map: HashMap<K, Vec<Versioned<V>>>,
}

impl InMemoryStore<String, String> {
    pub fn new() -> Self {
        let storage_map: HashMap<String, Vec<Versioned<String>>> = HashMap::new();
        InMemoryStore { storage_map }
    }
}

impl Store for InMemoryStore<String, String> {
    type Key = String;
    type Value = String;
    type Error = anyhow::Error;

    fn get(&self, key: Self::Key) -> Result<Vec<Versioned<Self::Value>>, Self::Error> {
        let value = self
            .storage_map
            .get(&key)
            .map_or_else(|| vec![], |v| v.clone());
        Ok(value.clone())
    }

    fn put(
        &mut self,
        key: Self::Key,
        value: Versioned<Self::Value>,
    ) -> Result<VersionedVec<Self::Value>, Self::Error> {
        let mut old_value = self
            .storage_map
            .get(&key)
            .map_or_else(|| Vec::new(), |v| v.clone());
        let mut new_value = vec![value.clone()];
        let old_versions: Vec<VectorClock> = old_value.iter().map(|v| v.version.clone()).collect();
        let version_succ = old_versions.iter().cloned().find(|version| {
            version > &value.version
        });
        match version_succ {
            Some(ver) => anyhow::bail!("Version {:?} > {:?}", ver, value.version),
            None => {
                new_value.append(&mut old_value);
                self.storage_map.insert(key, new_value);
                Ok(old_value.clone())
            }
        }
    }
}
