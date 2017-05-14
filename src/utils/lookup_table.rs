use utils::countable::Countable;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct LookupTable<C: Countable, T> {
    table: Vec<T>,
    data: C::Data,
}

impl<C: Countable, T> LookupTable<C, T> {
    pub fn from_vec(vec: Vec<T>, data: C::Data) -> Self {
        LookupTable {
            table: vec,
            data: data,
        }
    }

    pub fn new(data: C::Data, default: T) -> Self
        where T: Clone
    {
        Self::from_vec(vec![default; C::count(&data)], data)
    }

    pub fn from_fn<F>(data: C::Data, mut fun: F) -> Self
        where F: FnMut(C) -> T
    {
        LookupTable {
            table: (0..C::count(&data)).map(|num| fun(C::from_num(&data, num))).collect(),
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

    pub fn map_mut<F>(&mut self, mut fun: F)
        where F: FnMut(C, &mut T)
    {
        for (num, elem) in self.table.iter_mut().enumerate() {
            let c = C::from_num(&self.data, num);
            fun(c, elem);
        }
    }

    pub fn map<F, R>(&self, fun: F) -> LookupTable<C, R>
        where F: Fn(&T) -> R,
              C::Data: Clone
    {
        LookupTable {
            table: self.table.iter().map(fun).collect(),
            data: self.data.clone(),
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
