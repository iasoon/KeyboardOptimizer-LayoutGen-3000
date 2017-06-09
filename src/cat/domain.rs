use std::marker::PhantomData;
use std::ops::Index;

use cat::mapping::Dict;

pub trait Domain {
    type Type;
}

pub trait FiniteDomain: Domain {}

pub struct Num<D: FiniteDomain> {
    num: usize,
    phantom: PhantomData<D>,
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

impl<D: FiniteDomain> Domain for Num<D> {
    type Type = Num<D>;
}

impl<D: FiniteDomain> FiniteDomain for Num<D> {}

impl<D: FiniteDomain> Num<D> {
    pub fn as_usize(&self) -> usize {
        self.num
    }
}

pub fn to_num<D: FiniteDomain>(num: usize) -> Num<D> {
    Num {
        num: num,
        phantom: PhantomData,
    }
}

pub trait HasCount<D: FiniteDomain> {
    fn count(&self) -> Count<D>;

    fn enumerate<'t>(&'t self) -> ElemEnumerator<'t, D, Self>
        where Self: Dict<Num<D>, D::Type> + Sized
    {
        ElemEnumerator {
            elems: self,
            pos: 0,
            phantom: PhantomData,
        }
    }
}

pub struct Count<D: FiniteDomain> {
    count: usize,
    phantom: PhantomData<D>,
}

impl<D: FiniteDomain> Count<D> {
    pub fn as_usize(&self) -> usize {
        self.count
    }
}

impl<D: FiniteDomain> Clone for Count<D> {
    fn clone(&self) -> Self {
        Count {
            count: self.count,
            phantom: PhantomData,
        }
    }
}

impl<D: FiniteDomain> Copy for Count<D> {}

pub fn to_count<D: FiniteDomain>(count: usize) -> Count<D> {
    Count {
        count: count,
        phantom: PhantomData,
    }
}

pub struct ElemEnumerator<'e, D, E>
    where E: Dict<Num<D>, D::Type> + 'e,
          D: FiniteDomain + 'e
{
    elems: &'e E,
    pos: usize,
    phantom: PhantomData<D>,
}

impl<'e, D, E> Iterator for ElemEnumerator<'e, D, E>
    where E: Dict<Num<D>, D::Type> + HasCount<D>,
          D: FiniteDomain + 'e
{
    type Item = (Num<D>, &'e D::Type);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.elems.count().as_usize() {
            return None;
        } else {
            let num = to_num(self.pos);
            self.pos += 1;
            return Some((num, self.elems.get(num)));
        }
    }
}
