use std::collections::HashMap;
use serde::Deserialize;

use cat::{Num, Table, Dict};
use cat::ops::*;
use data::{Key, Layer, Token, Loc, Free, Lock};

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
        // process frees
        let mut frees = Vec::with_capacity(self.frees.len());
        for free_data in self.frees.iter() {
            frees.push(reader.read_token(free_data.token)?);
        }

        // process locks
        let mut locks = Vec::with_capacity(self.locks.len());
        for lock_data in self.locks.iter() {
            let mut lock = reader.elements.layers.map(|_| None);
            for (layer_name, token_name) in lock_data.elems.iter() {
                let layer_num = reader.read_layer(layer_name)?;
                let token_num = reader.read_token(token_name)?;
                *lock.get_mut(layer_num) = Some(token_num);
            }
            locks.push(lock);
        }

        Ok(Groups {
            frees: Table::from_vec(frees),
            locks: Table::from_vec(locks),
        })
    }
}

pub struct Groups {
    pub frees: Table<Free, Num<Token>>,
    pub locks: Table<Lock, Table<Layer, Option<Num<Token>>>>,
}
