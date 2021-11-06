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

    pub fn new() -> InMemoryDatabase{
        let map :HashMap<String, String> = HashMap::new();
        return InMemoryDatabase{ map : map};
    }
}