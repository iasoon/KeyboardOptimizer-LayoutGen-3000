use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use cat::universe::*;

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

impl<D: FiniteDomain> Elements<D> for Table<D, D::Type> {
    fn from_vec(vec: Vec<D::Type>) -> Self {
        Self::from_vec(vec)
    }

    fn count(&self) -> usize {
        self.elems.len()
    }
    // fn get<'t>(&'t self, num: Num<D>) -> &'t D::Type {
    //     &self[num]
    // }
}

impl<D: FiniteDomain, T> Index<Num<D>> for Table<D, T> {
    type Output = T;

    fn index<'t>(&'t self, num: Num<D>) -> &'t T {
        let idx = from_num(num);
        return &self.elems[idx];
    }
}

// impl<K: ElemType, V> IndexMut<Num<K>> for Table<K, V> {
//     fn index_mut<'a>(&'a mut self, idx: Num<K>) -> &'a mut V {
//         let num = from_num(idx);
//         return &mut self.elems[num];
//     }
// }
