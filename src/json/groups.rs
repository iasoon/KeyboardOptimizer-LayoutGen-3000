use std::collections::HashMap;

use cat;
use cat::*;
use cat::ops::*;
use data::*;

use json::errors::*;
use json::reader::{Reader, ElemReader, GroupsReader};

#[derive(Deserialize)]
pub struct LocData<'a> {
    key: &'a str,
    layer: &'a str,
}

impl<'s> Reader<Loc> for ElemReader<'s> {
    type Repr = &'s LocData<'s>;

    fn read(&self, repr: Self::Repr) -> Result<Loc> {
        Ok(Loc {
            key_num: self.read(repr.key)?,
            layer_num: self.read(repr.layer)?,
        })
    }
}

#[derive(Deserialize)]
struct FreeData<'a> {
    token: &'a str,
    allowed_locs: Vec<LocData<'a>>,
}

#[derive(Deserialize)]
struct LockData<'a> {
    #[serde(borrow)]
    elems: HashMap<&'a str, &'a str>,
    allowed_keys: Vec<&'a str>,
}

#[derive(Deserialize)]
pub struct GroupsData<'a> {
    #[serde(borrow)]
    frees: Vec<FreeData<'a>>,
    #[serde(borrow)]
    locks: Vec<LockData<'a>>,
}

impl<'s> Reader<Groups> for GroupsReader<'s> {
    type Repr = GroupsData<'s>;

    fn read(&self, repr: Self::Repr) -> Result<Groups> {
        let loc_num = LocNum {
            key_count: self.elements.keys.count(),
            layer_count: self.elements.layers.count(),
        };

        let mut assignments = Vec::new();
        // process frees
        let mut frees = Vec::with_capacity(repr.frees.len());
        for (free_num, free_data) in repr.frees.into_iter().enumerate() {
            // free
            frees.push(self.read(free_data.token)?);
            // assignments
            for loc_data in free_data.allowed_locs.into_iter() {
                let loc: Loc = self.read(&loc_data)?;
                assignments.push(Assignment::Free {
                    free_num: cat::internal::to_num(free_num),
                    loc_num: loc_num.apply(loc),
                });
            }
        }

        // process locks
        let mut locks = Vec::with_capacity(repr.locks.len());
        for (lock_num, lock_data) in repr.locks.iter().enumerate() {
            // lock
            let mut lock = self.elements.layers.map(|_| None);
            for (&layer_name, &token_name) in lock_data.elems.iter() {
                let layer_num = self.read(layer_name)?;
                let token_num = self.read(token_name)?;
                lock[layer_num] = Some(token_num);
            }
            locks.push(lock);
            // assignments
            for &key_name in lock_data.allowed_keys.iter() {
                assignments.push(Assignment::Lock {
                    lock_num: cat::internal::to_num(lock_num),
                    key_num: self.read(key_name)?,
                })
            }
        }

        Ok(Groups {
            frees: Table::from_vec(frees),
            locks: Table::from_vec(locks),
            assignments: Table::from_vec(assignments),
        })
    }
}

pub struct Groups {
    pub frees: Table<Free, Num<Token>>,
    pub locks: Table<Lock, Table<Layer, Option<Num<Token>>>>,
    pub assignments: Table<AllowedAssignment, Assignment>,
}
