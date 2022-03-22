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
        let old_versions: Vec<VectorClock> = old_value
            .iter()
            .map(|v| v.version.clone())
            .collect();

        let version_succ = old_versions
            .iter()
            .cloned()
            .find(|version| {
                version >= &value.version
            });
        match version_succ {
            Some(ver) => anyhow::bail!("Version {:?} > {:?}", ver, value.version),
            None => {
                new_value.append(&mut old_value);
                self
                    .storage_map
                    .insert(key, new_value);
                Ok(old_value.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::store::InMemoryStore;
    use crate::util;
    use crate::version::Versioned;

    use super::*;

    #[test]
    fn test_stale_put() {
        let mut store = InMemoryStore::new();
        let put_res = store.put("foo".to_string(), Versioned::new("bar".to_string()));
        assert!(put_res.is_ok(), "first put should succeed");
        let put_res = store.put("foo".to_string(), Versioned::new("quux".to_string()));
        assert!(put_res.is_err(), "should log an error if same version used");
    }

    #[test]
    fn test_put_with_greater_version() {
        let mut store = InMemoryStore::new();
        let key = String::from("bar");
        let val_and_ver = Versioned::new(String::from("foo"));
        let put_res = store.put(key.clone(), val_and_ver.clone());
        assert!(put_res.is_ok(), "first put should succeed");
        let new_ver = val_and_ver.version.incremented(0, 1u64, util::current_time_millis());
        let val_and_ver = Versioned::with_version(new_ver, String::from("quux"));
        let put_res = store.put(key, val_and_ver);
        assert!(put_res.is_ok(), "can overwrite with a larger value");
        let results = store.get("bar".to_string()).unwrap();
        assert_eq!(results[0].value, String::from("quux"));
    }
}