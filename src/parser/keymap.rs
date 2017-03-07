use errors::*;
use parser::{Parser, KbParser};

use data::keymap::{KeyMapping, Keymap as KeymapData};
use model::{TokenId, KeyId, Loc};
use layout::Keymap;

impl<'a> Parser<Keymap> for KbParser<'a> {
    type Repr = KeymapData;

    fn parse(&self, repr: &KeymapData) -> Result<Keymap> {
        let mut keymap_reader = KeymapReader::new(self);
        keymap_reader.read_keymap(repr)?;
        Ok(keymap_reader.extract_keymap()?)
    }
}

pub struct KeymapReader<'a> {
    parser: &'a KbParser<'a>,
    map: Vec<Option<Loc>>,
}

impl<'a> KeymapReader<'a> {
    fn new(parser: &'a KbParser<'a>) -> Self {
        KeymapReader {
            parser: parser,
            map: vec![None; parser.kb_conf.tokens.len()],
        }
    }

    fn extract_keymap(&self) -> Result<Keymap> {
        Ok(Keymap::from_token_map(self.parser.kb_conf.keys.len(),
                                  self.parser.kb_conf.layers.len(),
                                  self.mk_token_map()?))
    }

    fn mk_token_map(&self) -> Result<Vec<Loc>> {
        let mut token_map: Vec<Loc> = Vec::with_capacity(self.map.len());
        for token_num in 0..self.map.len() {
            token_map.push(self.get_token_loc(token_num)?);
        }
        Ok(token_map)
    }

    fn get_token_loc(&self, token_num: usize) -> Result<Loc> {
        self.map[token_num].ok_or_else(|| {
            format!("token {} not assigned",
                    self.parser.kb_conf.tokens[token_num])
                .into()
        })
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
            let TokenId(token_num) = self.parser.parse(token_name)?;
            self.map[token_num] = Some(key_id.layer(self.parser.parse(layer_name)?));
        }
        Ok(())
    }
}
