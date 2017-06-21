use std::collections::HashMap;

use cat::{Num, Table, FiniteDomain, HasCount};
use data::{Key, Layer, Token};

use json::elements::Elements;
use json::errors::*;

pub struct ConfigReader<'a> {
    key_map: HashMap<&'a str, Num<Key>>,
    layer_map: HashMap<&'a str, Num<Layer>>,
    token_map: HashMap<&'a str, Num<Token>>,
}

fn mk_name_map<'a, D>(table: &'a Table<D, String>) -> HashMap<&'a str, Num<D>>
    where D: FiniteDomain<Type = String>,
{
    table.enumerate().map(|(num, name)| (name.as_str(), num)).collect()
}

impl<'a> ConfigReader<'a> {
    pub fn build(elements: &'a Elements) -> ConfigReader<'a> {
        ConfigReader {
            key_map: mk_name_map(&elements.keys),
            layer_map: mk_name_map(&elements.layers),
            token_map: mk_name_map(&elements.tokens),
        }
    }

    pub fn read_key(&self, key_name: &str) -> Result<Num<Key>> {
        if let Some(&key_num) = self.key_map.get(key_name) {
            Ok(key_num)
        } else {
            bail!("unknown key: {}", key_name)
        }
    }

    pub fn read_layer(&self, layer_name: &str) -> Result<Num<Layer>> {
        if let Some(&layer_num) = self.layer_map.get(layer_name) {
            Ok(layer_num)
        } else {
            bail!("unknown layer: {}", layer_name)
        }
    }

    pub fn read_token(&self, token_name: &str) -> Result<Num<Token>> {
        if let Some(&token_num) = self.token_map.get(token_name) {
            Ok(token_num)
        } else {
            bail!("unknown token: {}", token_name)
        }
    }
}
