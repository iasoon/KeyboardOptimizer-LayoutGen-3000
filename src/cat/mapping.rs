use std::marker::PhantomData;
use cat::domain::*;

/// A mapping from a domain to a value
pub trait Mapping<'m, 't, D: Domain, T: 't>
{
    fn map(&'m self, elem: D::Type) -> T;
}

/// A mapping that stores its values
/// A PartialDict is not required to be total; it can have values for just
/// some members of its domain.
pub trait PartialDict<'t, D, T: 't> : Mapping<'t, 't, D, Option<&'t T>>
    where D: Domain
{
    fn set(&mut self, elem: D::Type, value: Option<T>);
}

/// A mapping that stores its values
/// A Dict is total; it has a value for each member of its domain.
pub trait Dict<'t, D: Domain, T: 't + ?Sized> : Mapping<'t, 't, D, &'t T>
{
    fn get(&'t self, elem: D::Type) -> &'t T {
        self.map(elem)
    }

    fn get_mut(&'t mut self, elem: D::Type) -> &'t mut T;
}

// pub struct ElemEnumerator<'e, D, E>
//     where E: Dict<'e, Num<D>, D> + 'e,
//           D: FiniteDomain + 'e
// {
//     elems: &'e E,
//     pos: usize,
//     phantom: PhantomData<D>,
// }

// impl<'e, D, E> Iterator for ElemEnumerator<'e, D, E>
//     where E: Dict<'e, Num<D>, D>,
//           D: FiniteDomain + 'e
// {
//     type Item = (Num<D>, &'e D::Type);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.pos >= self.elems.count() {
//             return None;
//         } else {
//             let num = to_num(self.pos);
//             self.pos += 1;
//             return Some((num, &self.elems[num]));
//         }
//     }
// }
