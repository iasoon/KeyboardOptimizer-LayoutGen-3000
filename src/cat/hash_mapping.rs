use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;

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

    pub fn get<'t, Q>(&'t self, elem: &Q) -> Option<&'t T>
        where D::Type: Borrow<Q>,
              Q: Hash + Eq + ?Sized
    {
        self.hash_map.get(&elem)
    }

    pub fn set(&mut self, elem: D::Type, value: Option<T>) {
        if let Some(t) = value {
            self.hash_map.insert(elem, t);
        } else {
            self.hash_map.remove(&elem);
        }
    }
}
