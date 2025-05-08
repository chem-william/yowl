use crate::feature::Rnum;
use std::collections::HashMap;

pub struct JoinPool {
    counter: u16,
    mapping: HashMap<(usize, usize), u16>,
}

impl JoinPool {
    pub fn new() -> Self {
        Self {
            counter: 1,
            mapping: HashMap::new(),
        }
    }

    pub fn hit(&mut self, sid: usize, tid: usize) -> Rnum {
        // Sort the pair so (1,4) == (4,1)
        let key = if sid < tid { (sid, tid) } else { (tid, sid) };
        // Get or insert a new ring number
        let num = *self.mapping.entry(key).or_insert_with(|| {
            let n = self.counter;
            self.counter += 1;
            n
        });
        Rnum::new(u8::try_from(num).expect("convert entry from `u16` to `u8`"))
    }
}
