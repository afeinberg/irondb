use std::collections::HashMap;
pub trait Store: fmt::Debug {
    type Key;
    type Value;
    type Error;

    fn get(&self, key: Self::Key) -> Result<Vec<Versioned<Self::Value>>, Self::Error>;
    fn put(&mut self, key: Self::Key, value: Versioned<Self::Value>) -> Result<(), Self::Error>; 
}

pub struct InMemoryStore<K, V> {
    map: HashMap<K, Vec<Versioned<V>>>,
}

impl Store for InMemoryStore<String, String> {
    type Key = Box<[u8]>;
    type Value = Box<[u8]>;
    type Error = Box<dyn std::error::Error>;

    fn get(&self, key: Self::Key) -> Result<Vec<Versioned<Self::Value>>, Self::Error> {
        let value = self.map.get(&key)?;
        Ok(value.clone())
    }

    fn put(&mut self, key: Self::Key, value: Versioned<Self::Value>) -> Result<(), Self::Error> {
        todo!()
    }
}