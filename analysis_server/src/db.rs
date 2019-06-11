use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::invoice::{DetectDuplicateReply_Result, Invoice};

#[derive(Clone)]
struct TimedInvoice(Invoice, Instant);

#[derive(Clone)]
pub struct DetectDuplicateStore {
    db: Arc<Mutex<HashMap<String, Vec<TimedInvoice>>>>,
    ttl: Duration,
}

impl DetectDuplicateStore {
    pub fn new(ttl: Duration) -> DetectDuplicateStore {
        DetectDuplicateStore {
            db: Arc::new(Mutex::new(HashMap::new())),
            ttl: ttl,
        }
    }

    pub fn detect_duplicate(&mut self, invoice: &Invoice) -> DetectDuplicateReply_Result {
        self.prune_expired();
        let val = TimedInvoice(invoice.clone(), Instant::now());
        let mut store = self.db.lock().unwrap();
        let res = store.get_mut(&invoice.company_name);
        match res {
            Some(vec) => {
                for ti in vec.iter() {
                    if &ti.0 != invoice {
                        continue;
                    } else {
                        return DetectDuplicateReply_Result::DUPLICATE;
                    }
                }
                vec.push(val);
                DetectDuplicateReply_Result::OK
            }
            None => {
                let key = invoice.company_name.clone();
                store.insert(key, vec![val]);
                DetectDuplicateReply_Result::OK
            }
        }
    }

    fn prune_expired(&mut self) {
        let ttl = self.ttl;
        let mut store = self.db.lock().unwrap();
        for (_, v) in store.iter_mut() {
            v.retain(|ti| ti.1.elapsed() < ttl);
        }
        store.retain(|_, v| !v.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::super::invoice;
    use super::{DetectDuplicateReply_Result, DetectDuplicateStore, Duration};

    extern crate quickcheck;

    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};

    #[test]
    fn detect_no_duplicate() {
        let mut db = DetectDuplicateStore::new(Duration::new(0, 0));
        let inv1 = invoice::Invoice::new();
        let res = db.detect_duplicate(&inv1);
        assert_eq!(res, DetectDuplicateReply_Result::OK);
    }

    #[test]
    fn detect_duplicate() {
        let mut db = DetectDuplicateStore::new(Duration::new(5 * 60, 0));
        let inv1 = invoice::Invoice::new();
        db.detect_duplicate(&inv1);
        let res = db.detect_duplicate(&inv1);
        assert_eq!(res, DetectDuplicateReply_Result::DUPLICATE);
    }

    // The following test relies on real time and thus is more likely to fail
    // non-deterministicly.
    #[test]
    fn detect_no_duplicate_after_expire() {
        let mut db = DetectDuplicateStore::new(Duration::new(2, 0));
        let inv1 = invoice::Invoice::new();
        db.detect_duplicate(&inv1);
        let res = db.detect_duplicate(&inv1);
        assert_eq!(res, DetectDuplicateReply_Result::DUPLICATE);
        sleep(Duration::new(2, 5000));
        let res = db.detect_duplicate(&inv1);
        assert_eq!(res, DetectDuplicateReply_Result::OK);
    }
}
