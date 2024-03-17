use std::collections::{HashMap, VecDeque};

pub(crate) struct ReportFifo {
  dict: HashMap<u8, VecDeque<u8>>,
} 

impl ReportFifo {
  pub fn new() -> Self {
    ReportFifo { dict: HashMap::new() }
  }

  pub fn clear_all(&mut self) {
    for (_, q) in self.dict.iter_mut() {
      q.clear();
    }
    self.dict.clear();
  }

  const ID_MASK:u8 = 0xF0u8;

  pub fn push_report(&mut self, data:&[u8]) {
    let k = data[0] & Self::ID_MASK;
    if (!self.dict.contains_key(&k)) {
      self.dict.insert(k, VecDeque::new());
    }
    self.dict[&k].extend(&data[1..]);
  }

  pub fn pop_report(&mut self, id: u8) -> &[u8]{
    let k = id & Self::ID_MASK;
    if (!self.dict.contains_key(&k) ||
      self.dict[&k].len() == 0) {
      &[]
    } else {
      let drained: Vec<u8> = self.dict[&k].drain(..).collect();
      &drained
    }
  }

  pub fn len_byte(&self, id:u8) -> usize{
    let k = id & Self::ID_MASK;
    if (self.dict.contains_key(&k)) {
      self.dict.len()
    } else {
      0
    }
  }
}

impl Drop for ReportFifo {
  fn drop(&mut self) {
    self.clear_all();
  }
}
