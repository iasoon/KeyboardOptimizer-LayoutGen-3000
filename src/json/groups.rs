use std::collections::HashMap;
use serde::Deserialize;

use cat;
use cat::*;
use cat::ops::*;
use data::*;

use json::errors::*;
use json::config_reader::ConfigReader;

#[derive(Deserialize)]
struct LocData<'a> {
    key: &'a str,
    layer: &'a str,
}

impl<'a> LocData<'a> {
    fn read(self, reader: &ConfigReader) -> Result<Loc> {
        Ok(Loc {
            key_num: reader.read_key(self.key)?,
            layer_num: reader.read_layer(self.layer)?,
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

impl<'a> GroupsData<'a> {
    pub fn read(self, reader: &ConfigReader) -> Result<Groups> {
        let loc_num = LocNum {
            key_count: reader.elements.keys.count(),
            layer_count: reader.elements.layers.count(),
        };

        let mut assignments = Vec::new();
        // process frees
        let mut frees = Vec::with_capacity(self.frees.len());
        for (free_num, free_data) in self.frees.into_iter().enumerate() {
            // free
            frees.push(reader.read_token(free_data.token)?);
            // assignments
            for loc_data in free_data.allowed_locs.into_iter() {
                assignments.push(Assignment::Free {
                    free_num: cat::internal::to_num(free_num),
                    loc_num: loc_num.apply(loc_data.read(reader)?),
                });
            }
        }

        // process locks
        let mut locks = Vec::with_capacity(self.locks.len());
        for (lock_num, lock_data) in self.locks.iter().enumerate() {
            // lock
            let mut lock = reader.elements.layers.map(|_| None);
            for (layer_name, token_name) in lock_data.elems.iter() {
                let layer_num = reader.read_layer(layer_name)?;
                let token_num = reader.read_token(token_name)?;
                *lock.get_mut(layer_num) = Some(token_num);
            }
            locks.push(lock);
            // assignments
            for key_name in lock_data.allowed_keys.iter() {
                assignments.push(Assignment::Lock {
                    lock_num: cat::internal::to_num(lock_num),
                    key_num: reader.read_key(key_name)?,
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
