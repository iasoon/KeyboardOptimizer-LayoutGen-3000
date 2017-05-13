use utils::{Countable, Bag};
use std::marker::PhantomData;

pub struct Seq<C: Countable> {
    elems: Vec<C>,
}

impl<C: Countable> Seq<C> {
    pub fn iter<'a>(&'a self) -> SeqIter<'a, C> {
        SeqIter::from_slice(self.elems.as_slice())
    }

    pub fn id(&self, data: &SeqData<C>) -> SeqId<C> {
        SeqId::from_num(data, self.to_num(data))
    }

    pub fn bag(self) -> Bag<C>
        where C: Ord
    {
        Bag::new(self.elems)
    }
}

#[derive(Clone)]
pub struct SeqData<C: Countable> {
    pub data: C::Data,
    pub len: usize,
}

impl<C: Countable> Countable for Seq<C> {
    type Data = SeqData<C>;

    fn to_num(&self, data: &SeqData<C>) -> usize {
        seq_num(data, self.iter())
    }

    fn from_num(data: &SeqData<C>, mut num: usize) -> Self {
        let mut elems = Vec::with_capacity(data.len);
        let c_count = C::count(&data.data);
        for _ in 0..data.len {
            let rem = num % c_count;
            num /= c_count;
            elems.push(C::from_num(&data.data, num));
        }
        elems.reverse();
        return Seq { elems: elems };
    }

    fn count(data: &SeqData<C>) -> usize {
        C::count(&data.data).pow(data.len as u32)
    }
}

pub fn seq_num<'a, C, Iter>(data: &SeqData<C>, iter: Iter) -> usize
    where C: Countable + 'a,
          Iter: Iterator<Item = &'a C> + 'a
{
    iter.fold(0,
              |acc, elem| acc * C::count(&data.data) + elem.to_num(&data.data))
}

pub fn seq_id<'a, C, Iter>(data: &SeqData<C>, iter: Iter) -> SeqId<C>
    where C: Countable + 'a,
          Iter: Iterator<Item = &'a C> + 'a
{
    SeqId {
        num: seq_num(data, iter),
        phantom: PhantomData,
    }
}

pub struct SeqIter<'a, T: 'a> {
    slice: &'a [T],
    pos: usize,
}

impl<'a, C: Countable> SeqIter<'a, C> {
    pub fn from_slice(slice: &'a [C]) -> Self {
        SeqIter {
            slice: slice,
            pos: 0,
        }
    }
}

impl<'a, T: 'a> Iterator for SeqIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.pos >= self.slice.len() {
            None
        } else {
            let item = &self.slice[self.pos];
            self.pos += 1;
            Some(item)
        }
    }
}

#[derive(Clone, Copy)]
pub struct SeqId<C> {
    num: usize,
    phantom: PhantomData<C>,
}

impl<C: Countable> SeqId<C> {
    pub fn seq(&self, data: &SeqData<C>) -> Seq<C> {
        Seq::from_num(data, self.to_num(data))
    }
}

impl<C: Countable> Countable for SeqId<C> {
    type Data = SeqData<C>;

    fn from_num(_: &SeqData<C>, num: usize) -> Self {
        SeqId {
            num: num,
            phantom: PhantomData,
        }
    }

    fn to_num(&self, _: &SeqData<C>) -> usize {
        self.num
    }

    fn count(data: &SeqData<C>) -> usize {
        Seq::count(data)
    }
}
