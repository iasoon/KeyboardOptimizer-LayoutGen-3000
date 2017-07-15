use std::collections::HashMap;

use cat::*;
use data::{KbDef, Key, Layer, Token};

use json::elements::Elements;
use json::errors::*;

pub trait Reader<R> {
    type Repr;

    fn read(&self, repr: Self::Repr) -> Result<R>;
}

pub struct GroupsReader<'s> {
    elem_reader: ElemReader<'s>,
    pub elements: &'s Elements,
}

impl<'s> GroupsReader<'s> {
    pub fn new(elements: &'s Elements) -> Self {
        GroupsReader {
            elem_reader: ElemReader::new(
                &elements.keys,
                &elements.layers,
                &elements.tokens),
            elements: elements,
        }
    }
}

impl<'s, T> Reader<T> for GroupsReader<'s>
    where ElemReader<'s>: Reader<T>
{
    type Repr = <ElemReader<'s> as Reader<T>>::Repr;

    fn read(&self, repr: Self::Repr) -> Result<T> {
        self.elem_reader.read(repr)
    }
}

pub struct EvalReader<'s> {
    elem_reader: ElemReader<'s>,
    pub kb_def: &'s KbDef,
}

impl<'s> EvalReader<'s> {
    pub fn new(kb_def: &'s KbDef) -> Self {
        EvalReader {
            elem_reader: ElemReader::new(
                &kb_def.keys,
                &kb_def.layers,
                &kb_def.tokens),
            kb_def: kb_def,
        }
    }
}

impl<'s, T> Reader<T> for EvalReader<'s>
    where ElemReader<'s>: Reader<T>
{
    type Repr = <ElemReader<'s> as Reader<T>>::Repr;

    fn read(&self, repr: Self::Repr) -> Result<T> {
        self.elem_reader.read(repr)
    }
}

pub struct ElemReader<'s> {
    key_map: HashMap<&'s str, Num<Key>>,
    layer_map: HashMap<&'s str, Num<Layer>>,
    token_map: HashMap<&'s str, Num<Token>>,
}

impl<'s> ElemReader<'s> {
    fn new(keys: &'s Table<Key, String>,
           layers: &'s Table<Layer, String>,
           tokens: &'s Table<Token, String>)
           -> Self
    {
        ElemReader {
            key_map: mk_name_map(keys),
            layer_map: mk_name_map(layers),
            token_map: mk_name_map(tokens),
        }
    }
}

fn mk_name_map<'a, D>(table: &'a Table<D, String>) -> HashMap<&'a str, Num<D>>
    where D: FiniteDomain<Type = String>,
{
    table.enumerate().map(|(num, name)| (name.as_str(), num)).collect()
}


impl<'s> Reader<Num<Key>> for ElemReader<'s> {
    type Repr = &'s str;

    fn read(&self, key_name: &'s str) -> Result<Num<Key>> {
        if let Some(&key_num) = self.key_map.get(key_name) {
            Ok(key_num)
        } else {
            bail!("unknown key: {}", key_name)
        }
    }
}

impl<'s> Reader<Num<Layer>> for ElemReader<'s> {
    type Repr = &'s str;

    fn read(&self, layer_name: &'s str) -> Result<Num<Layer>> {
        if let Some(&layer_num) = self.layer_map.get(layer_name) {
            Ok(layer_num)
        } else {
            bail!("unknown layer: {}", layer_name)
        }
    }
}

impl<'s> Reader<Num<Token>> for ElemReader<'s> {
    type Repr = &'s str;

    fn read(&self, token_name: &'s str) -> Result<Num<Token>> {
        if let Some(&token_num) = self.token_map.get(token_name) {
            Ok(token_num)
        } else {
            bail!("unknown token: {}", token_name)
        }
    }
}
