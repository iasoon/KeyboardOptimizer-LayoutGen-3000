use std::marker::PhantomData;
use std::ops::Index;

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

pub fn from_num<D: FiniteDomain>(num: Num<D>) -> usize {
    num.num
}

pub fn to_num<D: FiniteDomain>(num: usize) -> Num<D> {
    Num {
        num: num,
        phantom: PhantomData,
    }
}

pub struct Count<D: FiniteDomain> {
    count: usize,
    phantom: PhantomData<D>,
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

pub fn from_count<D: FiniteDomain>(count: Count<D>) -> usize {
    count.count
}

pub fn to_count<D: FiniteDomain>(count: usize) -> Count<D> {
    Count {
        count: count,
        phantom: PhantomData,
    }
}

pub trait Elements<D: FiniteDomain>: Index<Num<D>, Output = D::Type> {
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
