use std::collections::HashMap;
use std::hash::Hash;

use cat::universe::*;
use cat::mapping::*;

use std::fmt::Display;

pub struct HashMapping<D, T>
    where D::Type: Hash + Eq,
          D: Domain
{
    hash_map: HashMap<D::Type, T>,
}

impl<'t, D, T: 't> Mapping<'t, 't, D, Option<&'t T>> for HashMapping<D, T>
    where D::Type: Hash + Eq,
          D: Domain
{
    fn map(&'t self, elem: D::Type) -> Option<&'t T> {
        self.hash_map.get(&elem)
    }
}

impl<'t, D, T: 't> PartialDict<'t, D, T> for HashMapping<D, T>
    where D::Type: Hash + Eq + Clone,
          D: Domain, D::Type: Display,
{
    fn set(&mut self, elem: D::Type, value: Option<T>) {
        if let Some(t) = value {
            self.hash_map.insert(elem, t);
        } else {
            self.hash_map.remove(&elem);
        }
    }

    fn construct<E, F>(elems: &E, mut fun: F) -> Self
        where E: Elements<D>,
              D: FiniteDomain,
              F: FnMut(Num<D>, &D::Type) -> Option<T>,
              Self: Sized
    {
        let mut map = HashMap::with_capacity(elems.count());
        for (num, elem) in elems.enumerate() {
            if let Some(value) = fun(num, elem) {
                map.insert(elem.clone(), value);
            }
        }
        return HashMapping { hash_map: map };
    }
}
