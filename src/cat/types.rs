use std::marker::PhantomData;
use cat::composed::Composed;

pub trait Domain {
    type Type;
}

pub trait FiniteDomain: Domain {}

/// A mapping from a domain to a value
pub trait Mapping<D: Domain>
{
    type Target: Domain;

    fn apply(&self, elem: D::Type) -> <Self::Target as Domain>::Type;
}

/// A mapping that stores its values
/// A Dict is total; it has a value for each member of its domain.
pub trait Dict<D: Domain, T: ?Sized> {
    fn get<'t>(&'t self, elem: D::Type) -> &'t T;
    fn get_mut<'t>(&'t mut self, elem: D::Type) -> &'t mut T;

    fn compose<S, M>(self, mapping: M) -> Composed<M, Self>
        where M: Mapping<S, Target = D>,
              Self: Sized,
              S: Domain
    {
        Composed::new(mapping, self)
    }
}
