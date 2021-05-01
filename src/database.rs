use std::collections::HashMap;

pub trait  Database {
    fn get(self, key: String) -> Option<String>;
}
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
    pub fn new() -> InMemoryDatabase{
        let map :HashMap<String, String> = HashMap::new();
        return InMemoryDatabase{ map : map};
    }
}