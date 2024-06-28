use std::collections::vec_deque::Iter;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub(crate) struct ReportFifo {
    dict: HashMap<u8, VecDeque<Vec<u8>>>,
}

impl ReportFifo {
    pub fn new() -> Self {
        ReportFifo {
            dict: HashMap::new(),
        }
    }

    pub fn clear_all(&mut self) {
        for (_, q) in self.dict.iter_mut() {
            q.clear();
        }
        self.dict.clear();
    }

    const ID_MASK: u8 = 0xF0u8;

    /// check if a key exists in dictionary or else insert new queue for the key
    fn check_key(&mut self, k: u8) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.dict.entry(k) {
            e.insert(VecDeque::new());
            false
        } else {
            true
        }
    }

    pub fn clear(&mut self, id: u8) {
        let k = id & Self::ID_MASK;
        if self.check_key(k) {
            self.dict.get_mut(&k).unwrap().clear();
        }
    }

    pub fn push_report(&mut self, data: Vec<u8>) {
        let k = data[0] & Self::ID_MASK;
        self.check_key(k);
        self.dict.get_mut(&k).unwrap().push_back(data);
    }

    pub fn pop_report(&mut self, id: u8) -> Option<Vec<u8>> {
        let k = id & Self::ID_MASK;
        if self.check_key(k) {
            self.dict.get_mut(&k).unwrap().pop_front()
        } else {
            // queue has been just created
            None
        }
    }

    /// number of reports with a ID in queue
    pub fn len(&mut self, id: u8) -> usize {
        let k = id & Self::ID_MASK;
        if self.check_key(k) {
            self.dict[&k].len()
        } else {
            // queue has been just created
            0
        }
    }

    /// iterate and peek reports with a ID in queue
    pub fn iter_peek(&mut self, id: u8) -> Iter<'_, Vec<u8>> {
        let k = id & Self::ID_MASK;
        self.check_key(k);
        self.dict[&k].iter()
    }
}

impl Drop for ReportFifo {
    fn drop(&mut self) {
        self.clear_all();
    }
}
