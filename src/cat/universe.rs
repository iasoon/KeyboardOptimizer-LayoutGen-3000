use std::marker::PhantomData;
use std::hash::Hash;
use std::collections::HashMap;

pub trait ElemType {
    type Type;
}

pub struct Num<T: ElemType> {
    num: usize,
    phantom: PhantomData<T>,
}

impl<T: ElemType> Clone for Num<T> {
    fn clone(&self) -> Self {
        Num {
            num: self.num,
            phantom: PhantomData,
        }
    }
}

impl<T: ElemType> Copy for Num<T> {}

pub fn from_num<T: ElemType>(num: Num<T>) -> usize {
    num.num
}

pub fn to_num<T: ElemType>(num: usize) -> Num<T> {
    Num {
        num: num,
        phantom: PhantomData,
    }
}

pub trait PartialMapping<'a, S, T: 'a> {
    fn partial_map(&'a self, s: S) -> Option<T>;
}

impl<'a, S, T> PartialMapping<'a, S, T> for HashMap<S, T>
    where S: Hash + Eq,
          T: Copy + 'a
{
    fn partial_map(&'a self, s: S) -> Option<T> {
        self.get(&s).map(|&e| e)
    }
}

pub trait NumberedSet<T: ElemType> {
    fn get<'a>(&'a self, num: Num<T>) -> &'a T::Type;
}

pub struct Universe<'a, T, M, S>
    where T: ElemType + 'a,
          M: PartialMapping<'a, T::Type, Num<T>>,
          S: NumberedSet<T>
{
    pub mapping: M,
    pub set: S,
    pub phantom: PhantomData<&'a T>,
}
