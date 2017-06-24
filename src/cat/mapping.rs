use std::marker::PhantomData;
use cat::domain::*;

/// A mapping from a domain to a value
pub trait Mapping<D: Domain, T>
{
    fn apply(&self, elem: D::Type) -> T;
}

/// A mapping that stores its values
/// A Dict is total; it has a value for each member of its domain.
pub trait Dict<D: Domain, T: ?Sized> {
    fn get<'t>(&'t self, elem: D::Type) -> &'t T;
    fn get_mut<'t>(&'t mut self, elem: D::Type) -> &'t mut T;
}
