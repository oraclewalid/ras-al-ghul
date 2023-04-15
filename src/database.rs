use std::collections::HashMap;
use std::sync::Arc;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;

use rocksdb::{DB, Options};

use crate::config::StorageConfig;

pub trait  StorageBackend {
    fn get(&self, key: &String) -> Option<String>;
    fn set(&mut self, key: String, value: String) -> Option<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InMemoryDatabase {
    map: HashMap<String, String>,
}

impl  InMemoryDatabase  {

    pub fn get(&self, key: &String) -> Option<String> {
        return self.map.get(key).map(|value| value.clone());
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        return self.map.insert(key, value);
    }

    pub fn persist(&mut self, path: String) -> serde_cbor::Result<()> {
        let persistance_file = File::create(path);
        match persistance_file {
            Ok(file) => serde_cbor::to_writer(file, &self) ,
            Err(e) => Ok(()),
        }
    }
    
    pub fn load(path: String) -> serde_cbor::Result<InMemoryDatabase> {
        let db_file = File::open(path.clone());
        match db_file {
            Ok(file) => {
                serde_cbor::from_reader(file)
            },
            Err(e) => {
                println!("Can't load snapshot from {}, create a new DB", e);
                Ok(InMemoryDatabase::new())
            },
        }
    }

    pub fn new() -> InMemoryDatabase {
        let map :HashMap<String, String> = HashMap::new();
        return InMemoryDatabase{ map : map};
    }
}

impl  StorageBackend for InMemoryDatabase  {

    fn get(&self, key: &String) -> Option<String> {
        return self.map.get(key).map(|v| v.to_string());
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        return self.map.insert(key, value);
    }
}

#[derive(Debug)]
pub struct RocksDB {
    db: Arc<DB>,
}

impl RocksDB {
    pub fn new(path: String) -> RocksDB {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).unwrap();
        return RocksDB{ db: Arc::new(db)};
    }
}

impl  StorageBackend for RocksDB  {

    fn get(&self, key: &String) -> Option<String> {
        match self.db.get(key.as_bytes()) {
            Ok(Some(v)) => {
                return Some(String::from_utf8(v).unwrap());
            },
            Ok(None) => {
                return None;
            },
            Err(e) => {
                println!("Error: {}", e);
                return None;
            },
        }
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        return self.db.put(key.as_bytes(), value.as_bytes()).map_err(|err|  err.into_string()).err();
    }
}

pub fn get_storage_backend(storage_config: StorageConfig) -> Box<dyn StorageBackend + Send> {
    println!("Backend storage {:?}", storage_config.clone());
    match storage_config.backend {
        Inmemory => Box::new(InMemoryDatabase::new()),
        Rocksdb => Box::new(RocksDB::new(storage_config.path.unwrap_or_default())),
    }
}

#[test]
fn persist_inmemorydb_on_disk() {


    let mut db = InMemoryDatabase::new();
    let path = "/tmp/ras_in_memory.db".to_string();
    let key1 = "key1".to_string();
    db.set("key1".to_string(), "value1".to_string());
    db.set("key2".to_string(), "value2".to_string());

    db.persist(path.clone());
    let db2 = InMemoryDatabase::load(path).unwrap();
    let value1 = db2.get(&key1.clone());
    assert_eq!( value1, Some("value1".to_string()));
}


#[test]
fn persist_rockdb_on_disk() {

    let mut  db = RocksDB::new("/tmp/ras_rocksdb.db".to_string());

    let key1 = "key1".to_string();
    db.set("key1".to_string(), "value1".to_string());

    let value1 = db.get(&key1.clone());
    assert_eq!( value1, Some("value1".to_string()));
}