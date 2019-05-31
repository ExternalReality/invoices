use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use uuid::Uuid;

use crate::invoice::Invoice;

pub struct InvoiceStore(
    // General store
    HashMap<Uuid, Arc<Invoice>>,
    // Indexed by company name
    HashMap<String, HashMap<Uuid, Arc<Invoice>>>,
);

type SharedDb = Arc<RwLock<InvoiceStore>>;

#[derive(Clone)]
pub struct Database(SharedDb);

impl Database {
    pub fn new() -> Database {
        Database(Arc::new(RwLock::new(InvoiceStore(
            HashMap::new(),
            HashMap::new(),
        ))))
    }

    pub fn create(&mut self, invoice: &Invoice) -> String {
        let invoice_number = Uuid::new_v4();
        let mut db = self.0.write().unwrap();
        let item = Arc::new(invoice.clone());
        db.0.insert(invoice_number.clone(), Arc::clone(&item));
        db.1.entry(invoice.company_name.clone())
            .or_insert(HashMap::new())
            .insert(invoice_number.clone(), Arc::clone(&item));
        invoice_number.to_string()
    }

    pub fn list(&mut self) -> Vec<String> {
        let db = self.0.read().unwrap();
        db.0.iter().map(|(key, _)| key.to_string()).collect()
    }

    pub fn remove<'a>(&mut self, invoice_number: &'a str) -> Option<&'a str> {
        let key = Uuid::parse_str(invoice_number).unwrap();
        let mut db = self.0.write().unwrap();
        let inv = db.0.remove(&key)?;
        db.1.get_mut(&inv.company_name)
            .and_then(|v| v.remove(&key))?;
        Some(invoice_number)
    }

    #[allow(dead_code)]
    pub fn read_by_company(&mut self, company_name: &str) -> Vec<Invoice> {
        let db = self.0.read().unwrap();
        match db.1.get(company_name) {
            Some(v) => v
                .iter()
                .map(|(_, v)| Arc::make_mut(&mut v.clone()).clone())
                .collect(),
            None => Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn read(&self, invoice_number: &str) -> Option<Invoice> {
        let result = Uuid::parse_str(invoice_number);
        match result {
            Ok(key) => {
                let db = self.0.read().unwrap();
                match db.0.get(&key) {
                    Some(m) => Some(Arc::make_mut(&mut Arc::clone(m)).clone()),
                    None => None,
                }
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
    fn create() {
        let mut db = Database::new();
        let inv1 = invoice::Invoice::new();
        let num = db.create(&inv1);
        assert_eq!(db.read(&num), Some(inv1));
    }

    #[test]
    fn list() {
        let mut db = Database::new();
        let inv1 = invoice::Invoice::new();
        let num = db.create(&inv1);
        let inv_nums = db.list();
        assert_eq!(inv_nums, vec![num]);
    }

    #[test]
    fn remove() {
        let mut db = Database::new();
        let inv1 = invoice::Invoice::new();
        let num = db.create(&inv1);
        db.remove(&num);
        assert_eq!(db.read(&num), None);
    }

    #[test]
    fn read_missing() {
        let db = Database::new();
        assert_eq!(db.read("42fa5eee-9f07-487a-91f0-cedd9f08e507"), None);
    }

    #[test]
    fn read_bizarre_index() {
        let db = Database::new();
        assert_eq!(db.read("hello, world"), None);
    }

    #[test]
    fn read_company() {
        let mut db = Database::new();
        let mut inv1 = invoice::Invoice::new();
        let key = "Company";
        inv1.set_company_name(String::from(key));
        db.create(&inv1);
        assert_eq!(db.read_by_company(key), vec![inv1]);
    }

    #[test]
    fn read_missing_company() {
        let mut db = Database::new();
        assert_eq!(db.read_by_company("Not There"), vec![]);
    }

    #[test]
    fn remove_from_company() {
        let mut db = Database::new();
        let mut inv1 = invoice::Invoice::new();
        let key = "Company";
        inv1.set_company_name(String::from(key));
        let invoice_number = db.create(&inv1);
        db.remove(&invoice_number);
        assert_eq!(db.read_by_company(key), vec![]);
    }
}
