use std::collections::HashMap;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;

use rocksdb::{DB, Options};

pub trait  StorageBackend {
    fn get(self, key: String) -> Option<String>;
    fn set(self, key: String, value: String) -> Option<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InMemoryDatabase {
    map: HashMap<String, String>,
}


impl  InMemoryDatabase for StorageBackend {

    fn get(&self, key: &String) -> Option<&String> {
        return self.map.get(key);
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        return self.map.insert(key, value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RocksdbBackend {
    map: HashMap<String, String>,
}

#[test]
fn persist_db_on_disk() {


    let mut db = InMemoryDatabase::new();
    let path = "/tmp/ras.db".to_string();
    let key1 = "key1".to_string();
    db.set("key1".to_string(), "value1".to_string());
    db.set("key2".to_string(), "value2".to_string());

    db.persist(path.clone());
    let db2 = InMemoryDatabase::load(path).unwrap();
    let value1 = db2.get(&key1.clone());
    assert_eq!( value1, Some(&"value1".to_string()));
}