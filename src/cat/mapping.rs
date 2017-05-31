use std::marker::PhantomData;
use std::collections::HashMap;
use std::hash::Hash;

use cat::universe::*;

pub trait Mapping<'m, 't, D: Domain, T: 't>
{
    fn map(&'m self, elem: D::Type) -> T;
}

/// A mapping that stores its values
pub trait PartialDict<'t, D, T: 't> : Mapping<'t, 't, D, Option<&'t T>>
    where D: Domain
{
    fn set(&mut self, elem: D::Type, value: Option<T>);

    fn construct<E, F>(elems: &E, fun: F) -> Self
        where E: Elements<D>,
              D: FiniteDomain,
              F: FnMut(Num<D>, &D::Type) -> Option<T>,
              Self: Sized;
}
