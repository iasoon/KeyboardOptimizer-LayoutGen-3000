use errors::*;
use utils::{BoundedSet, HasId, Countable};
use model::{Key, Layer, Token, KeyId, LayerId, TokenId};

use std::collections::HashMap;
use std::vec::Vec;
use std::hash::Hash;


pub struct KbConf {
    pub keys: BoundedSet<Key>,
    pub layers: BoundedSet<Layer>,
    pub tokens: BoundedSet<Token>,
}

impl KbConf {
    pub fn build(keys: Vec<String>, layers: Vec<String>, tokens: Vec<String>) -> Self {
        KbConf {
            keys: into_bounded_set(keys, |key_name| Key { name: key_name }),
            layers: into_bounded_set(layers, |layer_name| Layer { name: layer_name }),
            tokens: into_bounded_set(tokens, |token_name| Token { name: token_name }),
        }
    }

    pub fn reader<'a>(&'a self) -> KbReader<'a> {
        KbReader::new(&self.keys, &self.layers, &self.tokens)
    }
}

pub struct KbReader<'a> {
    pub keys: &'a BoundedSet<Key>,
    pub layers: &'a BoundedSet<Layer>,
    pub tokens: &'a BoundedSet<Token>,

    key_map: HashMap<&'a str, KeyId>,
    layer_map: HashMap<&'a str, LayerId>,
    token_map: HashMap<&'a str, TokenId>,
}

impl<'a> KbReader<'a> {
    pub fn new(keys: &'a BoundedSet<Key>,
               layers: &'a BoundedSet<Layer>,
               tokens: &'a BoundedSet<Token>)
               -> Self
    {
        KbReader {
            key_map: mk_map(keys, |key| key.name.as_ref()),
            layer_map: mk_map(layers, |layer| layer.name.as_ref()),
            token_map: mk_map(tokens, |token| token.name.as_ref()),

            keys: keys,
            layers: layers,
            tokens: tokens,
        }
    }

    pub fn read_token(&self, repr: &String) -> Result<TokenId> {
        self.token_map
            .get(repr.as_str())
            .map(|&id| id)
            .ok_or_else(|| format!("unknown token: {}", repr).into())
    }

    pub fn read_layer(&self, repr: &String) -> Result<LayerId> {
        self.layer_map
            .get(repr.as_str())
            .map(|&id| id)
            .ok_or_else(|| format!("unknown layer: {}", repr).into())
    }

    pub fn read_key(&self, repr: &String) -> Result<KeyId> {
        self.key_map
            .get(repr.as_str())
            .map(|&id| id)
            .ok_or_else(|| format!("unknown key: {}", repr).into())
    }
}

fn mk_map<'a, T, I, F>(set: &'a BoundedSet<T>, fun: F) -> HashMap<I, T::Id>
    where T: HasId,
          I: Eq + Hash,
          T::Id: Copy,
          F: Fn(&'a T) -> I + 'a
{
    T::Id::enumerate(set.elem_count())
        .map(|id| (fun(&set[id]), id))
        .collect()
}

fn into_bounded_set<D, T, F>(data: Vec<D>, fun: F) -> BoundedSet<T>
    where T: HasId,
          F: Fn(D) -> T
{
    BoundedSet::new(data.into_iter().map(fun).collect())
}
