use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

pub struct Database {
    bucket: HashMap<usize, String>,
}

impl Database {
    pub fn new() -> Database {
        Database { bucket: HashMap::new() }
    }

    pub fn get_instance() -> MutexGuard<'static, Database> {
        DATABASE.lock().unwrap()
    }

    pub fn get(&self, id: usize) -> Option<&String> {
        self.bucket.get(&id)
    }

    pub fn add(&mut self, id: usize, str: String) -> () {
        self.bucket.insert(id, str);
    }

    pub fn clear(&mut self) -> () {
        self.bucket.clear();
    }
}

lazy_static! {
    static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Database;

    #[test]
    fn test() {
        let mut database = Database::get_instance();
        database.add(100, String::from("test"));
        let result = database.get(100);
        assert_eq!(result.unwrap(), "test");
    }
}
