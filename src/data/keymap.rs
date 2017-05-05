use errors::*;
use utils::{LookupTable, json};
use model::{KbDef, KeyId, LayerId, TokenId, Loc, LocData};
use data::json::KbReader;

use std::collections::HashMap;
use std::path::Path;

type Keymap = LookupTable<Loc, Option<TokenId>>;

pub type KeymapData = HashMap<String, KeyMapping>;

pub type KeyMapping = HashMap<String, String>;

pub struct KeymapReader<'a> {
    kb_reader: KbReader<'a>,
    keymap: Keymap,
}

impl<'a> KeymapReader<'a> {
    pub fn read(kb_def: &'a KbDef, path: &Path) -> Result<Keymap> {
        let mut reader = KeymapReader::new(
            KbReader::new(
                &kb_def.keys,
                &kb_def.layers,
                &kb_def.tokens
            )
        );

        let data: KeymapData = json::read(path)?;
        try!(reader.read_keymap(&data));
        Ok(reader.keymap)
    }

    fn new(kb_reader: KbReader<'a>) -> Self {
        let data = LocData {
            key_data: kb_reader.keys.elem_count(),
            layer_data: kb_reader.layers.elem_count(),
        };
        KeymapReader {
            kb_reader: kb_reader,
            keymap: LookupTable::new(data, None),
        }
    }

    fn read_keymap(&mut self, keymap: &KeymapData) -> Result<()> {
        for (key_name, key_mapping) in keymap.iter() {
            let key_id = self.kb_reader.read_key(key_name)?;
            self.read_key_mapping(key_id, key_mapping)?;
        }
        Ok(())
    }

    fn read_key_mapping(&mut self, key_id: KeyId, key_mapping: &KeyMapping) -> Result<()> {
        for (layer_name, token_name) in key_mapping.iter() {
            let loc = self.keymap.data().loc(key_id, self.kb_reader.read_layer(layer_name)?);
            self.keymap[loc] = Some(self.kb_reader.read_token(token_name)?);
        }
        Ok(())
    }
}
