use std::marker::PhantomData;
use std::collections::HashMap;
use std::ops::Index;

use cat::mapping::*;
use cat::table::*;

pub trait Domain {
    type Type;
}

pub trait FiniteDomain : Domain {
    // TODO
}

pub struct Num<D: FiniteDomain> {
    num: usize,
    phantom: PhantomData<D>,
}

impl Domain for String {
    type Type = String;
}

impl<D: FiniteDomain> Clone for Num<D> {
    fn clone(&self) -> Self {
        Num {
            num: self.num,
            phantom: PhantomData,
        }
    }
}

impl<D: FiniteDomain> Copy for Num<D> {}

pub fn from_num<D: FiniteDomain>(num: Num<D>) -> usize {
    num.num
}

pub fn to_num<D: FiniteDomain>(num: usize) -> Num<D> {
    Num {
        num: num,
        phantom: PhantomData,
    }
}

pub trait Elements<D: FiniteDomain> : Index<Num<D>, Output = D::Type> {
    fn from_vec(vec: Vec<D::Type>) -> Self;
    fn count(&self) -> usize;

    fn enumerate<'e>(&'e self) -> ElemEnumerator<'e, D, Self>
        where Self: Sized
    {
        ElemEnumerator {
            elems: self,
            pos: 0,
            phantom: PhantomData,
        }
    }
}

pub struct ElemEnumerator<'e, D, E>
    where E: Elements<D> + 'e,
          D: FiniteDomain
{
    elems: &'e E,
    pos: usize,
    phantom: PhantomData<D>,
}

impl<'e, D, E> Iterator for ElemEnumerator<'e, D, E>
    where E: Elements<D> + 'e,
          D: FiniteDomain + 'e
{
    type Item = (Num<D>, &'e D::Type);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.elems.count() {
            return None;
        } else {
            let num = to_num(self.pos);
            self.pos += 1;
            return Some((num, &self.elems[num]));
        }
    }
}

pub struct Subset<'d, D: 'd, M, E>
    where M: PartialDict<'d, D, Num<D>>,
          E: Elements<D>,
          D: FiniteDomain
{
    pub dict: M,
    pub elems: E,
    pub phantom: PhantomData<&'d D>,
}

impl<'d, D: 'd, M, E> Subset<'d, D, M, E>
    where M: PartialDict<'d, D, Num<D>>,
          E: Elements<D>,
          D: FiniteDomain,
{
    pub fn from_elem_vec(vec: Vec<D::Type>) -> Self {
        let elems = E::from_vec(vec);
        let dict = M::construct(&elems, |num, _| Some(num));

        return Subset {
            dict: dict,
            elems: elems,
            phantom: PhantomData,
        };
    }
}
