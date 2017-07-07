use std::marker::PhantomData;
use cat::composed::Composed;

pub trait Domain {
    type Type;
}

pub trait FiniteDomain: Domain {}

pub trait Mapping<T>
{
    type Result;

    fn apply(&self, elem: T) -> Self::Result;
}

/// A mapping that stores its values
/// A Dict is total; it has a value for each member of its domain.
pub trait Dict<K, T: ?Sized> {
    fn get<'t>(&'t self, elem: K) -> &'t T;
    fn get_mut<'t>(&'t mut self, elem: K) -> &'t mut T;

    fn compose<S, M>(self, mapping: M) -> Composed<M, Self>
        where M: Mapping<S, Result = K>,
              Self: Sized
    {
        Composed::new(mapping, self)
    }
}
