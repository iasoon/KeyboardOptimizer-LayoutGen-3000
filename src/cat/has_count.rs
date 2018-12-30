use std::marker::PhantomData;
use std::ops::Index;
use std::fmt;

use cat::*;
use cat::internal::*;

pub trait HasCount<D> {
    fn count(&self) -> Count<D>;

    fn nums(&self) -> Enumerator<D> {
        Enumerator {
            count: self.count(),
            pos: 0,
        }
    }

    fn map_nums<T, F>(&self, fun: F) -> Table<D, T>
        where F: FnMut(Num<D>) -> T
    {
        Table::from_vec(self.nums().map(fun).collect())
    }


    fn enumerate<'t, T>(&'t self) -> ElemEnumerator<'t, D, T, Self>
        where Self: Index<Num<D>, Output=T> + Sized,
              T: ?Sized
    {
        ElemEnumerator {
            enumerator: self.nums(),
            mapping: self,
            phantom_t: PhantomData,
        }
    }
}

pub struct Count<D> {
    count: usize,
    phantom: PhantomData<D>,
}

impl<D> Count<D> {
    pub fn as_usize(&self) -> usize {
        self.count
    }
}

impl<D> fmt::Debug for Count<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_usize())
    }
}

impl<D> HasCount<D> for Count<D> {
    fn count(&self) -> Count<D> {
        self.clone()
    }
}

impl<D> Clone for Count<D> {
    fn clone(&self) -> Self {
        Count {
            count: self.count,
            phantom: PhantomData,
        }
    }
}

impl<D> Copy for Count<D> {}

pub fn to_count<D>(count: usize) -> Count<D> {
    Count {
        count: count,
        phantom: PhantomData,
    }
}

impl<A, B> HasCount<(A, B)> for (Count<A>, Count<B>)
{
    fn count(&self) -> Count<(A, B)> {
        let &(major_count, minor_count) = self;
        let count = major_count.as_usize() * minor_count.as_usize();
        return to_count(count);
    }
}

#[derive(Debug)]
pub struct Enumerator<D> {
    count: Count<D>,
    pos: usize,
}

impl<D> Iterator for Enumerator<D> {
    type Item = Num<D>;

    fn next(&mut self) -> Option<Num<D>> {
        if self.pos >= self.count.as_usize() {
            return None;
        } else {
            let num = to_num(self.pos);
            self.pos += 1;
            return Some(num);
        }
    }
}

pub struct ElemEnumerator<'t, D, T, M>
    where M: Index<Num<D>, Output = T> + HasCount<D> + 't,
          T: ?Sized
{
    mapping: &'t M,
    enumerator: Enumerator<D>,
    phantom_t: PhantomData<T>,
}

impl<'t, D, T, M> Iterator for ElemEnumerator<'t, D, T, M>
    where M: Index<Num<D>, Output = T> + HasCount<D> + 't,
          T: 't + ?Sized
{
    type Item = (Num<D>, &'t T);

    fn next(&mut self) -> Option<(Num<D>, &'t T)> {
        self.enumerator.next().map(|num| {
            (num, &self.mapping[num])
        })
    }
}
