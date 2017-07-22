use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use data::*;
use cat::*;
use layout::Keymap;

use json::errors::*;
use json::reader::{Reader, EvalReader};

#[derive(Serialize, Deserialize)]
pub struct KeymapData<'a> {
    #[serde(borrow)]
    keys: HashMap<&'a str, KeyLayers<'a>>,
}

#[derive(Serialize, Deserialize)]
struct KeyLayers<'a> {
    #[serde(borrow)]
    layers: HashMap<&'a str, &'a str>
}

impl<'a> KeyLayers<'a> {
    fn empty() -> Self {
        KeyLayers {
            layers: HashMap::new()
        }
    }
}

impl<'s> Reader<Table<Loc, Option<Num<Token>>>> for EvalReader<'s> {
    type Repr = &'s KeymapData<'s>;

    fn read(&self, keymap_data: &'s KeymapData<'s>) -> Result<Keymap> {
        let mut keymap = self.kb_def.loc_num().map_nums(|_| None);
        for (&key_name, layers) in keymap_data.keys.iter() {
            let key_num = try!(self.read(key_name));
            for (&layer_name, &token_name) in layers.layers.iter() {
                let layer_num = try!(self.read(layer_name));
                let token_num = try!(self.read(token_name));
                let loc_num = self.kb_def.loc_num().apply(Loc {
                    key_num: key_num,
                    layer_num: layer_num,
                });
                keymap[loc_num] = Some(token_num);
            }
        }
        Ok(keymap)
    }
}

impl<'a> KeymapData<'a> {
    fn empty() -> Self {
        KeymapData {
            keys: HashMap::new()
        }
    }

    pub fn from_table(kb_def: &'a KbDef, keymap: Keymap) -> Self {
        let mut keymap_data = KeymapData::empty();
        for (loc_num, value) in keymap.enumerate() {
            if let &Some(token_num) = value {
                let loc: Loc = kb_def.loc_num().apply(loc_num);

                let key_name = &kb_def.keys[loc.key_num];
                let layer_name = &kb_def.layers[loc.layer_num];
                let token_name = &kb_def.tokens[token_num];

                keymap_data.add_mapping(key_name, layer_name, token_name);
            }
        }
        keymap_data
    }

    fn add_mapping (&mut self, key: &'a str, layer: &'a str, token: &'a str) {
        self.keys
            .entry(key)
            .or_insert_with(|| KeyLayers::empty())
            .layers
            .insert(layer, token);
    }
}
