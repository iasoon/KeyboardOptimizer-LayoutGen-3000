use layout::*;
use model::{KbDef, GroupId, TokenId, KeyId, Loc, LayerId};
use utils::Countable;

#[derive(Clone)]
pub struct Layout<'a> {
    pub token_map: TokenMap,
    pub keymap: Keymap,
    pub group_map: GroupMap,

    pub kb_def: &'a KbDef,
}

impl<'a> Layout<'a> {
    pub fn from_token_map(token_map: TokenMap, kb_def: &'a KbDef) -> Self {
        Layout {
            keymap: mk_keymap(&token_map, kb_def),
            group_map: mk_group_map(&token_map, kb_def),
            token_map: token_map,
            kb_def: kb_def,
        }
    }

    pub fn assign(&mut self, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_id, loc } => {
                let group_id = self.kb_def.free_group[free_id];
                self.assign_group(group_id, loc.key(&self.kb_def.loc_data()));
                let token_id = self.kb_def.frees[free_id].token_id;
                self.assign_token(token_id, loc);
            }
            Assignment::Lock { lock_id, key_id } => {
                let group_id = self.kb_def.lock_group[lock_id];
                self.assign_group(group_id, key_id);
                let lock = &self.kb_def.locks[lock_id];
                for (layer_id, token_id) in lock.elems() {
                    let loc = self.kb_def.loc_data().loc(key_id, layer_id);
                    self.assign_token(token_id, loc);
                }
            }
        }
    }

    fn assign_group(&mut self, group_id: GroupId, key_id: KeyId) {
        self.group_map[group_id] = key_id;
    }

    fn assign_token(&mut self, token_id: TokenId, loc: Loc) {
        if self.keymap[loc].is_none() {
            let prev_loc = self.token_map[token_id];
            self.keymap[prev_loc] = None;
        }

        self.keymap[loc] = Some(token_id);
        self.token_map[token_id] = loc;
    }

    pub fn moves<'b>(&'b self) -> Moves<'b> {
        Moves::new(self)
    }

    pub fn print(&self) {
        let loc_data = self.kb_def.loc_data();
        let print_key = |layer_num, key_num| {
            let layer_id = LayerId::from_num(&self.kb_def.layers.elem_count(), layer_num);
            let key_id = KeyId::from_num(&self.kb_def.keys.elem_count(), key_num);
            let loc = loc_data.loc(key_id, layer_id);
            let t = self.keymap[loc].map(|token_id| self.kb_def.tokens[token_id].name.as_str()).unwrap_or(" ");
            print!("{} ", t);
        };

        for layer in 0..2 {
            for key in 0..10 {
                print_key(layer, key);
            }
            println!();
            for key in 10..19 {
                print_key(layer, key);
            }
            println!();
            for key in 19..26 {
                print_key(layer, key);
            }
            println!();
            println!();
        }

    }
}
