use std::collections::HashMap;

use data::*;
use cat::*;
use layout::Keymap;

use Result;
use json::reader::{Reader, EvalReader};

#[derive(Serialize, Deserialize)]
pub struct KeymapData<'a>(
    #[serde(borrow)]
    HashMap<&'a str, KeyMapping<'a>>
);

#[derive(Serialize, Deserialize)]
pub struct KeyMapping<'a>(
    #[serde(borrow)]
    HashMap<&'a str, &'a str>
);

impl<'s> Reader<Table<Loc, Option<Num<Token>>>> for EvalReader<'s> {
    type Repr = &'s KeymapData<'s>;

    fn read(&self, keymap_data: &'s KeymapData<'s>) -> Result<Keymap> {
        let &KeymapData(ref key_mappings) = keymap_data;

        let mut keymap = self.kb_def.loc_num().map_nums(|_| None);
        for (&key_name, key_mapping) in key_mappings.iter() {
            let &KeyMapping(ref entries) = key_mapping;
            let key_num = try!(self.read(key_name));
            for (&layer_name, &token_name) in entries.iter() {
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

impl<'a> KeyMapping<'a> {
    fn empty() -> Self {
        KeyMapping(HashMap::new())
    }

    fn insert(&mut self, layer: &'a str, token: &'a str) {
        let &mut KeyMapping(ref mut entries) = self;
        entries.insert(layer, token);
    }
}

impl<'a> KeymapData<'a> {
    fn empty() -> Self {
        KeymapData(HashMap::new())
    }

    fn mapping_mut<'b>(&'b mut self, key: &'a str) -> &'b mut KeyMapping<'a> {
        let &mut KeymapData(ref mut key_mappings) = self;
        return key_mappings.entry(key).or_insert_with(|| KeyMapping::empty());
    }

    pub fn from_table(kb_def: &'a KbDef, keymap: Keymap) -> Self {
        let mut keymap_data = KeymapData::empty();
        for (loc_num, value) in keymap.enumerate() {
            if let &Some(token_num) = value {
                let loc: Loc = kb_def.loc_num().apply(loc_num);

                let key_name = &kb_def.keys[loc.key_num];
                let layer_name = &kb_def.layers[loc.layer_num];
                let token_name = &kb_def.tokens[token_num];

                keymap_data.mapping_mut(key_name).insert(layer_name, token_name);
            }
        }
        keymap_data
    }
}
