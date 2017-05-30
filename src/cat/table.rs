use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use cat::universe::{ElemType, Num, NumberedSet, to_num, from_num};

pub struct Table<K: ElemType, V> {
    elems: Vec<V>,
    phantom: PhantomData<K>,
}

impl<K: ElemType, V> Table<K, V> {
    pub fn from_vec(vec: Vec<V>) -> Self {
        Table {
            elems: vec,
            phantom: PhantomData,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Num<K>, &'a V)> {
        self.elems.iter().enumerate().map(|(num, item)| (to_num(num), item))
    }
}

impl<T: ElemType> NumberedSet<T> for Table<T, T::Type> {

    fn get<'a>(&'a self, num: Num<T>) -> &'a T::Type {
        &self[num]
    }
}

impl<K: ElemType, V> Index<Num<K>> for Table<K, V> {
    type Output = V;

    fn index<'a>(&'a self, idx: Num<K>) -> &'a V {
        let num = from_num(idx);
        return &self.elems[num];
    }
}

impl<K: ElemType, V> IndexMut<Num<K>> for Table<K, V> {
    fn index_mut<'a>(&'a mut self, idx: Num<K>) -> &'a mut V {
        let num = from_num(idx);
        return &mut self.elems[num];
    }
}
