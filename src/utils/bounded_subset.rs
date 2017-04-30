use utils::{Countable, LookupTable};

use std::vec::Vec;

pub struct BoundedSubset<C: Countable> {
    vec: Vec<C>,
    idx: LookupTable<C, Option<usize>>,
}

impl<C> BoundedSubset<C>
    where C: Countable + Copy
{
    pub fn new(data: C::Data) -> Self {
        BoundedSubset {
            vec: Vec::with_capacity(C::count(&data)),
            idx: LookupTable::new(data, None),
        }
    }

    pub fn from_vec(data: C::Data, vec: Vec<C>) -> Self
    {
        let mut idx_table = LookupTable::new(data, None);
        for (idx, elem) in vec.iter().cloned().enumerate() {
            idx_table[elem] = Some(idx);
        }
        BoundedSubset {
            vec: vec,
            idx: idx_table,
        }
    }

    pub fn first(&self) -> Option<C> {
        self.vec.get(0).map(|&e| e)
    }

    pub fn add(&mut self, elem: C)
    {
        if !self.contains(elem) {
            self.idx[elem] = Some(self.vec.len());
            self.vec.push(elem);
        }
    }

    pub fn contains(&self, elem: C) -> bool {
        self.idx[elem].is_some()
    }

    pub fn remove(&mut self, elem: C) {
        if let Some(idx) = self.idx[elem].take() {
            self.vec.swap_remove(idx);
            if idx < self.vec.len() {
                self.idx[self.vec[idx]] = Some(idx);
            }
        }
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn cursor(self) -> SubsetCursor<C>
    {
        SubsetCursor::new(self)
    }

}

pub struct SubsetCursor<C: Countable> {
    pub subset: BoundedSubset<C>,
    pos: Option<C>,
    idx: usize,
}

impl<C> SubsetCursor<C>
    where C: Countable + Copy
{
    fn new(subset: BoundedSubset<C>) -> Self {
        SubsetCursor {
            idx: 0,
            pos: None,
            subset: subset,
        }
    }

    pub fn pos(&self) -> Option<C> {
        self.pos
    }

    pub fn next(&mut self) -> bool {
        self.pos = self.subset.vec.get(self.idx).map(|&e| e);
        self.idx += 1;
        self.pos().is_some()
    }
}
