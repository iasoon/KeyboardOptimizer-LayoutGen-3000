use utils::{Countable, LookupTable, SeqTable, Bag, BagData, BagId, SeqData};

pub struct BagTable<C: Countable, T> {
    seq_table: SeqTable<C, BagId<C>>,
    values: LookupTable<BagId<C>, T>,
}

fn seq_bag_table<C>(bag_data: &BagData<C>) -> SeqTable<C, BagId<C>>
    where C: Countable + Ord + Clone,
          C::Data: Clone
{
    let seq_data = SeqData {
        data: bag_data.data.clone(),
        len: bag_data.len,
    };
    SeqTable::from_fn(seq_data.clone(),
                      |seq_id| seq_id.seq(&seq_data).bag().id(bag_data))
}

impl<C: Countable, T> BagTable<C, T> {
    pub fn new(data: BagData<C>, default: T) -> Self
        where C: Countable + Ord + Clone,
              C::Data: Clone,
              T: Clone
    {
        BagTable {
            seq_table: seq_bag_table(&data),
            values: LookupTable::new(data, default),
        }
    }

    pub fn data<'a>(&'a self) -> &'a BagData<C> {
        self.values.data()
    }

    pub fn table<'a>(&'a self) -> &'a LookupTable<BagId<C>, T> {
        &self.values
    }

    pub fn get<'a, Iter>(&'a self, iter: Iter) -> &'a T
        where Iter: Iterator<Item = &'a C> + 'a
    {
        let bag_id = self.get_bag_id(iter);
        return &self.values[bag_id];
    }

    pub fn get_mut<'a, Iter>(&'a mut self, iter: Iter) -> &'a mut T
        where Iter: Iterator<Item = &'a C> + 'a
    {
        let bag_id = self.get_bag_id(iter);
        return &mut self.values[bag_id];
    }

    pub fn get_bag_id<'a, Iter>(&self, iter: Iter) -> BagId<C>
        where Iter: Iterator<Item = &'a C> + 'a,
              C: 'a
    {
        *self.seq_table.get(iter)
    }

    pub fn drain_map<F, R>(self, fun: F) -> BagTable<C, R>
        where F: Fn(T) -> R
    {
        BagTable {
            seq_table: self.seq_table,
            values: self.values.drain_map(fun),
        }
    }
}
