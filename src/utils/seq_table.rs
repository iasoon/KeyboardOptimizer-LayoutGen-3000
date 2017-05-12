use utils::{Countable, SeqAssocList, LookupTable, SeqId, SeqData, seq_id};

use std::vec::Vec;

pub struct SeqTable<C: Countable, T> {
    table: LookupTable<SeqId<C>, T>,
}

impl<C: Countable, T> SeqTable<C, T> {
    pub fn new(data: SeqData<C>, default: T) -> Self
        where T: Copy,
    {
        SeqTable {
            table: LookupTable::new(data, default),
        }
    }

    pub fn get<'a, Iter>(&'a self, iter: Iter) -> &'a T
        where Iter: Iterator<Item = &'a C> + 'a
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
