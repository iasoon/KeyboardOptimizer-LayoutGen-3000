use std::collections::HashMap;

use data::KbConf;
use model::{TokenId, KeyId, LayerId};
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

    fn mk_token_map(tokens: &'a Vec<String>) -> HashMap<&'a str, TokenId> {
        tokens.iter().enumerate().map(|(num, name)| (name.as_str(), TokenId(num))).collect()
    }

    fn mk_key_map(keys: &'a Vec<String>) -> HashMap<&'a str, KeyId> {
        keys.iter().enumerate().map(|(num, name)| (name.as_str(), KeyId(num))).collect()
    }

    fn mk_layer_map(layers: &'a Vec<String>) -> HashMap<&'a str, LayerId> {
        layers.iter().enumerate().map(|(num, name)| (name.as_str(), LayerId(num))).collect()
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
