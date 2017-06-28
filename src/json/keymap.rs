use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use data::*;
use cat::*;

#[derive(Serialize, Deserialize)]
pub struct Keymap<'a> {
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

impl<'a> Keymap<'a> {
    fn empty() -> Self {
        Keymap {
            keys: HashMap::new()
        }
    }

    pub fn from_table(kb_def: &'a KbDef, table: &Table<Loc, Option<Num<Token>>>) -> Self {
        let mut keymap = Keymap::empty();
        for (loc_num, value) in table.enumerate() {
            if let &Some(token_num) = value {
                let loc: Loc = kb_def.loc_num().apply(loc_num);

                let key_name = kb_def.keys.get(loc.key_num);
                let layer_name = kb_def.layers.get(loc.layer_num);
                let token_name = kb_def.tokens.get(token_num);

                keymap.add_mapping(key_name, layer_name, token_name);
            }
        }
        keymap
    }

    fn add_mapping (&mut self, key: &'a str, layer: &'a str, token: &'a str) {
        self.keys
            .entry(key)
            .or_insert_with(|| KeyLayers::empty())
            .layers
            .insert(layer, token);
    }
}
