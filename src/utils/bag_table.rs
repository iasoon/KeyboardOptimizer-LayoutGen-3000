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
              T: Copy
    {
        BagTable {
            seq_table: seq_bag_table(&data),
            values: LookupTable::new(data, default),
        }
    }

    pub fn get<'a, Iter>(&'a self, iter: Iter) -> &'a T
        where Iter: Iterator<Item = &'a C> + 'a
    {
        let bag_id = self.get_bag_id(iter);
        return &self.values[bag_id];
    }

    pub fn get_bag_id<'a, Iter>(&'a self, iter: Iter) -> BagId<C>
        where Iter: Iterator<Item = &'a C> + 'a
    {
        *self.seq_table.get(iter)
    }
}
