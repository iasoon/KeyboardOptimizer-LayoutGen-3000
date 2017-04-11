use utils::countable::Countable;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct LookupTable<C: Countable, T> {
    table: Vec<T>,
    data: C::Data,
}

impl<C: Countable, T> LookupTable<C, T> {
    pub fn new(data: C::Data, default: T) -> Self
        where T: Clone
    {
        LookupTable {
            table: vec![default; C::count(&data)],
            data: data,
        }
    }

    pub fn data<'a>(&'a self) -> &'a C::Data {
        &self.data
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (C, &'a T)> + 'a {
        self.table.iter().enumerate().map(move |(num, value)| {
            let c = C::from_num(&self.data, num);
            return (c, value);
        })
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.table.iter()
    }

    pub fn from_fn<F>(data: C::Data, fun: F) -> Self
        where F: Fn(C) -> T
    {
        LookupTable {
            table: C::enumerate(&data).map(fun).collect(),
            data: data,
        }
    }

    pub fn drain_map<F, R>(self, fun: F) -> LookupTable<C, R>
        where F: Fn(T) -> R
    {
        LookupTable {
            table: self.table.into_iter().map(fun).collect(),
            data: self.data,
        }
    }
}

impl<C: Countable, T> Index<C> for LookupTable<C, T> {
    type Output = T;

    fn index<'a>(&'a self, idx: C) -> &'a T {
        let num = idx.to_num(&self.data);
        &self.table[num]
    }
}

impl<C: Countable, T> IndexMut<C> for LookupTable<C, T> {
    fn index_mut<'a>(&'a mut self, idx: C) -> &'a mut T {
        let num = idx.to_num(&self.data);
        &mut self.table[num]
    }
}
