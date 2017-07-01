use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::fmt;

use cat::*;
use cat::ops::*;
use cat::internal::*;

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

    // TODO: generalize this function
    pub fn map_res_with_idx<'t, F, R, E>(&'t self, mut fun: F) -> Result<Table<D, R>, E>
        where F: FnMut(Num<D>, &'t T) -> Result<R, E>
    {
        let mut elems = Vec::with_capacity(self.elems.len());
        for (num, elem) in self.enumerate() {
            let res = try!(fun(num, elem));
            elems.push(res);
        }
        Ok(Table::from_vec(elems))
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

impl<D: FiniteDomain, T> MapMut<T> for Table<D, T>
{
    fn map_mut<'t, F>(&'t mut self, mut fun: F)
        where F: FnMut(&'t mut T)
    {
        for elem in self.elems.iter_mut() {
            fun(elem);
        }
    }

}

impl<D: FiniteDomain, T> MapMutWithKey<Num<D>, T> for Table<D, T>
{
    fn map_mut_with_key<'t, F>(&'t mut self, mut fun: F)
        where F: FnMut(Num<D>, &'t mut T)
    {
        for (num, elem) in self.elems.iter_mut().enumerate() {
            fun(to_num(num), elem);
        }
    }

}
