use cat::domain::*;

pub trait Mapping<'m, 't, D: Domain, T: 't>
{
    fn map(&'m self, elem: D::Type) -> T;
}


pub trait Dict<'t, D: FiniteDomain, T: 't> : Mapping<'t, 't, Num<D>, &'t T>
{
    fn get(&'t self, elem: Num<D>) -> &'t T {
        self.map(elem)
    }

    fn get_mut(&'t mut self, elem: Num<D>) -> &'t mut T;
}

/// A mapping that stores its values
/// (This means they can also be changed)
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
