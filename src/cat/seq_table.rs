use std::marker::PhantomData;

use cat::domain::*;
use cat::mapping::*;
use cat::seq::*;

pub struct SeqTable<D: FiniteDomain, T> {
    elems: Vec<T>,
    seq_len: usize,
    phantom: PhantomData<D>,
}

pub struct SeqIter<'e, D: Domain + 'e> {
    slice: &'e [D::Type],
    pos: usize,
}

impl<D, T> Dict<Num<D>, [T]> for SeqTable<D, T>
    where D: FiniteDomain,
{
    fn get<'t>(&'t self, num: Num<D>) -> &'t [T] {
        let offset = num.as_usize() * self.seq_len;
        return &self.elems[offset..offset+self.seq_len];
    }

    fn get_mut<'t>(&'t mut self, num: Num<D>) -> &'t mut [T] {
        let offset = num.as_usize() * self.seq_len;
        return &mut self.elems[offset..offset+self.seq_len];
    }
}

impl<D, T> HasCount<D> for SeqTable<D, T>
    where D: FiniteDomain
{
    fn count(&self) -> Count<D> {
        return to_count(self.elems.len() / self.seq_len);
    }
}