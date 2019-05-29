use std::collections::HashMap;

use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use super::invoice::Invoice;

#[derive(Clone)]
pub struct Database(Arc<RwLock<HashMap<Uuid, Invoice>>>);

impl Database {
    pub fn new() -> Database {
        Database(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn create(&mut self, invoice: &Invoice) -> String {
        let invoice_number = Uuid::new_v4();
        let mut db = self.0.write().unwrap();
        db.insert(invoice_number, invoice.clone());
        invoice_number.to_string()
    }

    pub fn list(&mut self) -> Vec<String> {
        let db = self.0.read().unwrap();
        db.iter().map(|(key, _)| key.to_string()).collect()
    }

    pub fn remove<'a>(&mut self, invoice_number: &'a str) -> Option<&'a str> {
        let key = Uuid::parse_str(invoice_number).unwrap();
        let mut db = self.0.write().unwrap();
        db.remove(&key).map(|_| invoice_number)
    }

    #[allow(dead_code)]
    pub fn read(&self, invoice_number: &str) -> Option<Invoice> {
        let result = Uuid::parse_str(invoice_number);
        match result {
            Ok(key) => {
                let db = self.0.read().unwrap();
                db.get(&key).map(|t| t.clone())
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::invoice;
    use super::Database;

    #[test]
    fn test_create() {
        let mut db = Database::new();
        let inv1 = invoice::Invoice::new();
        let num = db.create(&inv1);
        assert_eq!(db.read(&num), Some(inv1));
    }

    #[test]
    fn test_list() {
        let mut db = Database::new();
        let inv1 = invoice::Invoice::new();
        let num = db.create(&inv1);
        let inv_nums = db.list();
        assert_eq!(inv_nums, vec![num]);
    }

    #[test]
    fn test_remove() {
        let mut db = Database::new();
        let inv1 = invoice::Invoice::new();
        let num = db.create(&inv1);
        db.remove(&num);
        assert_eq!(db.read(&num), None);
    }

    #[test]
    fn test_read_missing() {
        let db = Database::new();
        assert_eq!(db.read("42fa5eee-9f07-487a-91f0-cedd9f08e507"), None);
    }

    #[test]
    fn test_read_bizarre_index() {
        let db = Database::new();
        assert_eq!(db.read("hello, world"), None);
    }
}
