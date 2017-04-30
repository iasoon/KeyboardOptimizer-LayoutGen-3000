use utils::{Countable, BoundedSubset};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::vec::Vec;

#[derive(Debug)]
pub struct BoundedSet<T>
    where T: HasId
{
    elems: Vec<T>,
}

impl<T> BoundedSet<T>
    where T: HasId
{
    pub fn new(elems: Vec<T>) -> Self {
        BoundedSet {
            elems: elems,
        }
    }

    pub fn elem_count(&self) -> ElemCount<T> {
        ElemCount::new(self.elems.len())
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.elems.iter()
    }

    pub fn ids(&self) -> impl Iterator<Item = T::Id> {
        T::Id::enumerate(self.elem_count())
    }

    pub fn subset(&self) -> BoundedSubset<T::Id>
        where T::Id: Copy
    {
        BoundedSubset::from_vec(self.elem_count(), self.ids().collect())
    }
}

impl<T: HasId> Index<T::Id> for BoundedSet<T> {
    type Output = T;

    fn index<'a>(&'a self, idx: T::Id) -> &'a T {
        let num = idx.to_num(&self.elem_count());
        &self.elems[num]
    }
}

impl<T: HasId> IndexMut<T::Id> for BoundedSet<T> {

    fn index_mut<'a>(&'a mut self, idx: T::Id) -> &'a mut T {
        let num = idx.to_num(&self.elem_count());
        &mut self.elems[num]
    }
}

pub trait HasId
    where Self: Sized
{
    type Id: Countable<Data = ElemCount<Self>>;
}

#[derive(Debug, Copy)]
pub struct ElemCount<T> {
    phantom: PhantomData<T>,
    num_elems: usize,
}

impl<T> Clone for ElemCount<T> {
    fn clone(&self) -> ElemCount<T> {
        ElemCount {
            phantom: PhantomData,
            num_elems: self.num_elems,
        }
    }
}

impl<T> ElemCount<T> {
    fn new(num_elems: usize) -> Self {
        ElemCount {
            phantom: PhantomData,
            num_elems: num_elems,
        }
    }

    pub fn count(&self) -> usize {
        self.num_elems
    }
}
