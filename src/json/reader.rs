use std::collections::HashMap;

use cat::*;
use data::{Key, Value};

use json::errors::*;

pub trait Reader<R> {
    type Repr;

    fn read(&self, repr: Self::Repr) -> Result<R>;

    fn read_vec(&self, repr: Vec<Self::Repr>) -> Result<Vec<R>> {
        repr.into_iter().map(|elem_repr| self.read(elem_repr)).collect()
    }
}

pub struct NameReader<'s> {
    keys: Table<Key, &'s str>,
    key_map: HashMap<&'s str, Num<Key>>,
    values: Table<Value, &'s str>,
    value_map: HashMap<&'s str, Num<Value>>,
}

impl<'s> NameReader<'s> {
    pub fn new(keys: Table<Key, &'s str>,
               values: Table<Value, &'s str>)
               -> Self
    {
        NameReader {
            key_map: mk_name_map(&keys),
            value_map: mk_name_map(&values),
            keys: keys,
            values: values,
        }
    }

    pub fn keys<'a>(&'a self) -> &'a Table<Key, &'a str> {
        &self.keys
    }

    pub fn values<'a>(&'a self) -> &'a Table<Value, &'a str> {
        &self.values
    }
}

fn mk_name_map<'a, D>(table: &Table<D, &'a str>) -> HashMap<&'a str, Num<D>> {
    table.enumerate().map(|(num, &name)| (name, num)).collect()
}


impl<'s> Reader<Num<Key>> for NameReader<'s> {
    type Repr = &'s str;

    fn read(&self, key_name: &'s str) -> Result<Num<Key>> {
        if let Some(&key_num) = self.key_map.get(key_name) {
            Ok(key_num)
        } else {
            bail!("unknown key: {}", key_name)
        }
    }
}

impl<'s> Reader<Num<Value>> for NameReader<'s> {
    type Repr = &'s str;

    fn read(&self, value_name: &'s str) -> Result<Num<Value>> {
        if let Some(&value_num) = self.value_map.get(value_name) {
            Ok(value_num)
        } else {
            bail!("unknown value: {}", value_name)
        }
    }
}