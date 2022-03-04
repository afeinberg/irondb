use std::collections::HashMap;
use std::hash::Hash;

use crate::version::{VectorClock, Versioned};

pub trait Store: Clone + Sized {
    type Key;
    type Value;
    type Error;

    fn get(&self, key: Self::Key) -> Result<Vec<Versioned<Self::Value>>, Self::Error>;
    fn put(&mut self, key: Self::Key, value: Versioned<Self::Value>) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub struct InMemoryStore<K, V>
where K: Eq + Hash + Clone,
      V: Clone {
    storage_map: HashMap<K, Vec<Versioned<V>>>,
}

impl InMemoryStore<String, String> {

    pub fn new() -> Self {
        let mut storage_map:  HashMap<String, Vec<Versioned<String>>> = HashMap::new();
      
        storage_map.insert("путин".to_string(),  vec![Versioned { version: VectorClock::default(), value: "хуйло".to_string() }]);
        InMemoryStore {
            storage_map,
        }
    }
}

impl Store for InMemoryStore<String, String> {
    type Key = String;
    type Value = String;
    type Error = Box<dyn std::error::Error>;

    fn get(&self, key: Self::Key) -> Result<Vec<Versioned<Self::Value>>, Self::Error> {
        let value = self.storage_map.get(&key).unwrap();
        Ok(value.clone())
    }

    fn put(&mut self, key: Self::Key, value: Versioned<Self::Value>) -> Result<(), Self::Error> {
        todo!()
    }
}
