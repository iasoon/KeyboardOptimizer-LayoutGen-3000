use cat::composed::Composed;

use std::ops::{Index, IndexMut};

pub trait Mapping<T> {
    type Result;

    fn apply(&self, elem: T) -> Self::Result;
}

pub trait Dict<K> : Index<K> + IndexMut<K> {
    fn borrow_mut<'a>(&'a mut self) -> BorrowedMut<Self>
        where Self: Sized
    {
        BorrowedMut { a: self }
    }

    fn compose<O, T>(self, other: O) -> Composed<O, Self>
        where Self: Sized,
              O: Mapping<T, Result = K>
    {
        Composed::new(other, self)
    }

    fn compose_fn<F, T>(self, fun: F) -> Composed<FnMapping<F>, Self>
        where Self: Sized,
              F: Fn(T) -> K
    {
        Composed::new(FnMapping::new(fun), self)
    }
}

impl<K, I> Dict<K> for I
    where I: Index<K> + IndexMut<K>
{}

impl<'i, T, I> Mapping<T> for &'i I
    where I: Index<T>,
          I::Output: 'i
{
    type Result = &'i I::Output;

    fn apply(&self, elem: T) -> &'i I::Output {
        &self[elem]
    }
}

pub struct FnMapping<F> {
    function: F,
}

impl<F> FnMapping<F> {
    fn new(function: F) -> Self {
        FnMapping { function }
    }
}

impl<K, T, F> Mapping<K> for FnMapping<F>
    where F: Fn(K) -> T
{
    type Result = F::Output;

    fn apply(&self, elem: K) -> F::Output {
        (self.function)(elem)
    }
}

pub struct BorrowedMut<'a, A: 'a> {
    a: &'a mut A,
}

impl<'a, A: 'a, K> Index<K> for BorrowedMut<'a, A>
    where A: Index<K>
{
    type Output = A::Output;

    fn index<'t>(&'t self, key: K) -> &'t A::Output {
        &self.a[key]
    }
}

impl<'a, A: 'a, K> IndexMut<K> for BorrowedMut<'a, A>
    where A: IndexMut<K>
{
    fn index_mut<'t>(&'t mut self, key: K) -> &'t mut A::Output {
        &mut self.a[key]
    }
}
