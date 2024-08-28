use super::Value;
use std::collections::HashMap;

pub struct Table {
    map: HashMap<String, Value>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: String, val: Value) {
        self.map.insert(id, val);
    }

    pub fn get(&self, id: &String) -> Option<Value> {
        if self.map.contains_key(id) {
            return Some(self.map.get(id).unwrap().clone());
        }
        return None;
    }

    pub fn overwrite(&mut self, id: String, val: Value) -> Option<Value> {
        if self.map.contains_key(&id) {
            return Some(self.map.insert(id, val).unwrap().clone());
        }
        return None;
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str += "{ ";
        for (key, value) in self.map.iter() {
            str += &format!(" \"{}\": {}, ", key, value);
        }
        str += " }";
        write!(f, "{}", str)
    }
}
