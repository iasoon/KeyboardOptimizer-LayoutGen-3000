use std::vec::Vec;
use std::ops::{Index, IndexMut};

use model::{TokenId, Loc};
use utils::LocMap;

#[derive(Debug)]
pub struct Keymap {
    token_map: Vec<Loc>,
    loc_map: LocMap<Option<TokenId>>,
}

impl Keymap {
    pub fn from_token_map(num_keys: usize, num_layers: usize, token_map: Vec<Loc>) -> Self {
        let mut keymap = Keymap {
            token_map: token_map,
            loc_map: LocMap::empty(num_layers, num_keys),
        };
        keymap.fill_loc_map();
        return keymap;
    }

    fn fill_loc_map(&mut self) {
        for token_num in 0..self.token_map.len() {
            let token_id = TokenId(token_num);
            let token_loc = self[token_id];
            self[token_loc] = Some(token_id);
        }
    }
}

impl Index<TokenId> for Keymap {
    type Output = Loc;

    fn index<'a>(&'a self, token_id: TokenId) -> &'a Loc {
        let TokenId(token_num) = token_id;
        &self.token_map[token_num]
    }
}

impl IndexMut<TokenId> for Keymap {
    fn index_mut<'a>(&'a mut self, token_id: TokenId) -> &'a mut Loc {
        let TokenId(token_num) = token_id;
        &mut self.token_map[token_num]
    }
}

impl Index<Loc> for Keymap {
    type Output = Option<TokenId>;

    fn index<'a>(&'a self, loc: Loc) -> &'a Option<TokenId> {
        &self.loc_map[loc]
    }
}

impl IndexMut<Loc> for Keymap {
    fn index_mut<'a>(&'a mut self, loc: Loc) -> &'a mut Option<TokenId> {
        &mut self.loc_map[loc]
    }
}
