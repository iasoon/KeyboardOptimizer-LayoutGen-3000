use errors::*;
use parser::{Parser, KbParser};

use data::keymap::{KeyMapping, Keymap as KeymapData};
use model::{KeyId, Loc, LocData};
use layout::Keymap;
use utils::LookupTable;

impl<'a> Parser<Keymap> for KbParser<'a> {
    type Repr = KeymapData;

    fn parse(&self, repr: &KeymapData) -> Result<Keymap> {
        let mut keymap_reader = KeymapReader::new(self);
        keymap_reader.read_keymap(repr)?;
        Ok(keymap_reader.keymap)
    }
}

pub struct KeymapReader<'a> {
    parser: &'a KbParser<'a>,
    keymap: Keymap,
}

impl<'a> KeymapReader<'a> {
    fn new(parser: &'a KbParser<'a>) -> Self {
        let data = LocData {
            key_data: parser.kb_conf.keys.elem_count(),
            layer_data: parser.kb_conf.layers.elem_count(),
        };
        KeymapReader {
            parser: parser,
            keymap: LookupTable::new(data, None),
        }
    }

    fn read_keymap(&mut self, keymap: &KeymapData) -> Result<()> {
        for (key_name, key_mapping) in keymap.iter() {
            let key_id = self.parser.parse(key_name)?;
            self.read_key_mapping(key_id, key_mapping)?;
        }
        Ok(())
    }

    fn read_key_mapping(&mut self, key_id: KeyId, key_mapping: &KeyMapping) -> Result<()> {
        for (layer_name, token_name) in key_mapping.iter() {
            let loc = Loc::new(
                self.keymap.data(),
                key_id,
                self.parser.parse(layer_name)?
            );
            self.keymap[loc] = Some(self.parser.parse(token_name)?);
        }
        Ok(())
    }
}
