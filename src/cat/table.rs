use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use cat::domain::*;
use cat::mapping::*;

pub struct Table<D: FiniteDomain, T> {
    elems: Vec<T>,
    phantom: PhantomData<D>,
}

impl<D: FiniteDomain, T> Table<D, T> {
    fn from_vec(vec: Vec<T>) -> Self {
        Table {
            elems: vec,
            phantom: PhantomData,
        }
    }

    pub fn iter<'t>(&'t self) -> impl Iterator<Item = (Num<D>, &'t T)> {
        self.elems.iter().enumerate().map(|(num, item)| (to_num(num), item))
    }
}

impl<'t, D: FiniteDomain, T: 't> Mapping<'t, 't, Num<D>, &'t T> for Table<D, T> {
    fn map(&'t self, num: Num<D>) -> &'t T {
        let idx = from_num(num);
        return &self.elems[idx];
    }
}

impl<'t, D: FiniteDomain, T: 't> Dict<'t, Num<D>, T> for Table<D, T> {
    fn get_mut(&'t mut self, num: Num<D>) -> &'t mut T {
        let idx = from_num(num);
        return &mut self.elems[idx];
    }
}

impl<D: FiniteDomain, T> HasCount<D> for Table<D, T> {
    fn count(&self) -> Count<D> {
        return to_count(self.elems.len());
    }
}

impl<D: FiniteDomain, T> Index<Num<D>> for Table<D, T> {
    type Output = T;

    fn index<'t>(&'t self, num: Num<D>) -> &'t T {
        let idx = from_num(num);
        return &self.elems[idx];
    }
}
