use std::marker::PhantomData;
use cat::composed::Composed;
use cat::table::Table;

use std::ops::Index;

pub trait Mapping<T> {
    type Result;

    fn apply(&self, elem: T) -> Self::Result;
}

impl<'i, T, I> Mapping<T> for &'i I
    where I: Index<T>,
          I::Output: 'i
{
    type Result = &'i I::Output;

    fn apply(&self, elem: T) -> &'i I::Output {
        &self[elem]
    }
}
