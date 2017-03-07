use std::vec::Vec;
use std::ops::{Index, IndexMut};

use model::{Loc, LayerId, KeyId};

#[derive(Debug, Clone)]
pub struct LocMap<T> {
    elems: Vec<T>,
    num_keys: usize,
}

impl<T> LocMap<T> {
    pub fn from_fn<F>(num_layers: usize, num_keys: usize, fun: F) -> Self
        where F: Fn(Loc) -> T
    {
        LocMap {
            elems: Self::locs(num_layers, num_keys).map(fun).collect(),
            num_keys: num_keys,
        }
    }

    pub fn empty(num_layers: usize, num_keys: usize) -> Self
        where T: Default + Clone
    {
        LocMap {
            elems: vec![T::default(); num_keys * num_layers],
            num_keys: num_keys,
        }
    }

    pub fn drain_map<F, B>(self, fun: F) -> LocMap<B>
        where F: Fn(T) -> B
    {
        LocMap {
            elems: self.elems.into_iter().map(fun).collect(),
            num_keys: self.num_keys,
        }
    }

    fn locs(num_layers: usize, num_keys: usize) -> impl Iterator<Item = Loc> {
        (0..num_layers).flat_map(move |layer_num| {
            let layer_id = LayerId(layer_num);
            (0..num_keys).map(move |key_num| {
                layer_id.key_num(key_num)
            })
        })
    }

    fn raw_idx(&self, idx: Loc) -> usize {
        let KeyId(key_id) = idx.key_id;
        let LayerId(layer_id) = idx.layer_id;
        return layer_id * self.num_keys + key_id;
    }
}

impl<T> Index<Loc> for LocMap<T> {
    type Output = T;

    fn index<'a>(&'a self, loc: Loc) -> &'a T {
        let idx = self.raw_idx(loc);
        return &self.elems[idx];
    }
}

impl<T> IndexMut<Loc> for LocMap<T> {
    fn index_mut<'a>(&'a mut self, loc: Loc) -> &'a mut T {
        let idx = self.raw_idx(loc);
        return &mut self.elems[idx];
    }
}
