use crate::error;
use std::collections::HashMap;
use crate::command::{GET, UNSET};

pub struct KVStore {
    store: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> Self {
        return KVStore {
            store: HashMap::<String, String>::new(),
        };
    }

    pub fn get(&self, key: &str) -> Result<&String, error::KeyNotFoundError> {
        match self.store.get(key) {
            Some(value) => Ok(value),
            None => Err(error::KeyNotFoundError),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_string(), value.to_string());
    }

    pub fn unset(&mut self, key: &str) -> Result<String, error::KeyNotFoundError> {
        match self.store.get(key) {
            Some(_) => Ok(self.store.remove(key).expect("key unset")),
            None => Err(error::KeyNotFoundError),
        }
    }

    pub fn execute(&mut self, command: &str, key: &str) {
        match command {
            GET => match self.get(key) {
                Ok(value) => println!("{}", value),
                Err(_) => println!("key unset"),
            },
            UNSET => match self.unset(key) {
                Ok(value) => println!("{}", value),
                Err(_) => println!("key unset"),
            },
            _ => println!("Invalid command for execute")
        }
    }
}
