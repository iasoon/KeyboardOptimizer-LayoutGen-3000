use std::collections::HashMap;
use std::hash::Hash;

use cat::domain::*;
use cat::mapping::*;

use std::fmt::Display;

pub struct HashMapping<D, T>
    where D::Type: Hash + Eq,
          D: Domain
{
    hash_map: HashMap<D::Type, T>,
}

impl<D, T> HashMapping<D, T>
    where D::Type: Hash + Eq,
          D: Domain
{
    pub fn empty() -> Self {
        HashMapping {
            hash_map: HashMap::new(),
        }
    }
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
}
