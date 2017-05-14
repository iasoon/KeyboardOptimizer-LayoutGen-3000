use utils::{Countable, SeqAssocList, LookupTable, SeqId, SeqData, seq_id};

use std::vec::Vec;

pub struct SeqTable<C: Countable, T> {
    table: LookupTable<SeqId<C>, T>,
}

impl<C: Countable, T> SeqTable<C, T> {
    pub fn new(data: SeqData<C>, default: T) -> Self
        where T: Copy
    {
        SeqTable { table: LookupTable::new(data, default) }
    }

    pub fn from_fn<F>(data: SeqData<C>, fun: F) -> Self
        where F: Fn(SeqId<C>) -> T
    {
        SeqTable { table: LookupTable::from_fn(data, fun) }
    }

    pub fn get<'a, 'b, Iter>(&'a self, iter: Iter) -> &'a T
        where Iter: Iterator<Item = &'b C> + 'b,
              C: 'b
    {
        &self.table[seq_id(self.table.data(), iter)]
    }

    pub fn get_mut<'a, Iter>(&'a mut self, iter: Iter) -> &'a mut T
        where Iter: Iterator<Item = &'a C> + 'a
    {
        let seq_id = seq_id(self.table.data(), iter);
        &mut self.table[seq_id]
    }

    fn seq_len(&self) -> usize {
        self.table.data().len
    }
}
