use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use cat::*;
use cat::internal::to_count;

pub struct SeqTable<D, T> {
    elems: Vec<T>,
    seq_len: usize,
    phantom: PhantomData<D>,
}

impl<D, T> SeqTable<D, T> {
    pub fn from_elem_vec(elems: Vec<T>, len: usize) -> Self {
        SeqTable {
            elems: elems,
            seq_len: len,
            phantom: PhantomData,
        }
    }

    pub fn seq_len(&self) -> usize {
        return self.seq_len;
    }
}

impl<D, T> Index<Num<D>> for SeqTable<D, T> {
    type Output = [T];

    fn index<'t>(&'t self, num: Num<D>) -> &'t [T] {
        let offset = num.as_usize() * self.seq_len;
        return &self.elems[offset..offset+self.seq_len];
    }
}

impl<D, T> IndexMut<Num<D>> for SeqTable<D, T> {
    fn index_mut<'t>(&'t mut self, num: Num<D>) -> &'t mut [T] {
        let offset = num.as_usize() * self.seq_len;
        return &mut self.elems[offset..offset+self.seq_len];
    }
}

impl<D, T> HasCount<D> for SeqTable<D, T> {
    fn count(&self) -> Count<D> {
        return to_count(self.elems.len() / self.seq_len);
    }
}
