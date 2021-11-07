use std::collections::HashMap;
use std::error::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
pub trait  Database {
    fn get(self, key: String) -> Option<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InMemoryDatabase {
    pub map: HashMap<String, String>,
}


impl  InMemoryDatabase  {

    pub fn get(&self, key: &String) -> Option<&String> {
        return self.map.get(key);
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
        let db_file = File::open(path);
        match db_file {
            Ok(file) => serde_cbor::from_reader(file) ,
            Err(e) => Ok(InMemoryDatabase::new()),
        }
    }

    pub fn new() -> InMemoryDatabase {
        let map :HashMap<String, String> = HashMap::new();
        return InMemoryDatabase{ map : map};
    }
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
