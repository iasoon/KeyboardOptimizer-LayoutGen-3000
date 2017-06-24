use std::marker::PhantomData;

use cat::domain::*;
use cat::mapping::*;

pub trait HasCount<D: FiniteDomain> {
    fn count(&self) -> Count<D>;

    fn enumerate<'t, T>(&'t self) -> ElemEnumerator<'t, D, T, Self>
        where Self: Dict<Num<D>, T> + Sized
    {
        ElemEnumerator {
            mapping: self,
            pos: 0,
            phantom_d: PhantomData,
            phantom_t: PhantomData,
        }
    }
}

impl<'h, D, H> HasCount<D> for &'h H
    where H: HasCount<D> + 'h,
          D: FiniteDomain
{
    fn count(&self) -> Count<D> {
        return self.count();
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

pub struct ElemEnumerator<'t, D, T, M>
    where M: Dict<Num<D>, T> + HasCount<D> + 't,
          D: FiniteDomain,
{
    mapping: &'t M,
    pos: usize,
    phantom_d: PhantomData<D>,
    phantom_t: PhantomData<T>,
}

impl<'t, D, T: 't, M> Iterator for ElemEnumerator<'t, D, T, M>
    where M: Dict<Num<D>, T> + HasCount<D> + 't,
          D: FiniteDomain
{
    type Item = (Num<D>, &'t T);

    fn next(&mut self) -> Option<(Num<D>, &'t T)> {
        if self.pos >= self.mapping.count().as_usize() {
            return None;
        } else {
            let num = to_num(self.pos);
            self.pos += 1;
            return Some((num, self.mapping.get(num)));
        }
    }
}
