use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::fmt;

use cat::domain::*;
use cat::mapping::*;
use cat::has_count::*;
use cat::ops::*;

pub struct Table<D: FiniteDomain, T> {
    elems: Vec<T>,
    phantom: PhantomData<D>,
}

impl<D: FiniteDomain, T> Table<D, T> {
    pub fn from_vec(vec: Vec<T>) -> Self {
        Table {
            elems: vec,
            phantom: PhantomData,
        }
    }

    pub fn iter<'t>(&'t self) -> impl Iterator<Item = (Num<D>, &'t T)> {
        self.elems.iter().enumerate().map(|(num, item)| (to_num(num), item))
    }
}

impl<D: FiniteDomain, T> Dict<Num<D>, T> for Table<D, T> {
    fn get<'t>(&'t self, num: Num<D>) -> &'t T {
        return &self.elems[num.as_usize()];
    }

    fn get_mut<'t>(&'t mut self, num: Num<D>) -> &'t mut T {
        return &mut self.elems[num.as_usize()];
    }
}

impl<D: FiniteDomain, T> HasCount<D> for Table<D, T> {
    fn count(&self) -> Count<D> {
        return to_count(self.elems.len());
    }
}

impl<D: FiniteDomain, T> fmt::Debug for Table<D, T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elems)
    }
}

impl<D: FiniteDomain, T, V> Map<T, V, Table<D, V>> for Table<D, T>
{
    fn map<'t, F>(&'t self, mut fun: F) -> Table<D, V>
        where F: FnMut(&'t T) -> V
    {
        Table::from_vec(self.elems.iter().map(fun).collect())
    }

}
