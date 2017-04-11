use std::collections::HashMap;

use model::{KbConf, Key, KeyId, Token, TokenId, Layer, LayerId};
use utils::{Countable, BoundedSet};
use errors::*;

use parser::Parser;


pub struct KbParser<'a> {
    pub kb_conf: &'a KbConf,
    token_map: HashMap<&'a str, TokenId>,
    key_map: HashMap<&'a str, KeyId>,
    layer_map: HashMap<&'a str, LayerId>,
}

impl<'a> KbParser<'a> {
    pub fn new(kbconf: &'a KbConf) -> Self {
        KbParser {
            kb_conf: kbconf,
            token_map: Self::mk_token_map(&kbconf.tokens),
            key_map: Self::mk_key_map(&kbconf.keys),
            layer_map: Self::mk_layer_map(&kbconf.layers),
        }
    }

    fn mk_token_map(tokens: &'a BoundedSet<Token>) -> HashMap<&'a str, TokenId> {
        TokenId::enumerate(&tokens.elem_count()).map(|token_id| {
            (tokens[token_id].name.as_str(), token_id)
        }).collect()
    }

    fn mk_key_map(keys: &'a BoundedSet<Key>) -> HashMap<&'a str, KeyId> {
        KeyId::enumerate(&keys.elem_count()).map(|key_id| {
            (keys[key_id].name.as_str(), key_id)
        }).collect()
    }

    fn mk_layer_map(layers: &'a BoundedSet<Layer>) -> HashMap<&'a str, LayerId> {
        LayerId::enumerate(&layers.elem_count()).map(|layer_id| {
            (layers[layer_id].name.as_str(), layer_id)
        }).collect()
    }
}

impl<'a> Parser<TokenId> for KbParser<'a> {
    type Repr = String;

    fn parse(&self, repr: &String) -> Result<TokenId> {
        self.token_map.get(repr.as_str()).map(|&id| id).ok_or_else(|| {
            format!("unknown token: {}", repr).into()
        })
    }
}

impl<'a> Parser<LayerId> for KbParser<'a> {
    type Repr = String;

    fn parse(&self, repr: &String) -> Result<LayerId> {
        self.layer_map.get(repr.as_str()).map(|&id| id).ok_or_else(|| {
            format!("unknown layer: {}", repr).into()
        })
    }
}

impl<'a> Parser<KeyId> for KbParser<'a> {
    type Repr = String;

    fn parse(&self, repr: &String) -> Result<KeyId> {
        self.key_map.get(repr.as_str()).map(|&id| id).ok_or_else(|| {
            format!("unknown key: {}", repr).into()
        })
    }
}
