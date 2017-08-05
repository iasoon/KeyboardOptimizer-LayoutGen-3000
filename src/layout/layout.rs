use data::*;
use cat::*;
use cat::ops::*;

use layout::assignable::Assignable;
use layout::move_gen::MoveGen;

#[derive(Clone)]
pub struct Layout<'a> {
    pub keymap: Keymap,
    pub token_map: TokenMap,
    pub kb_def: &'a KbDef,
}

impl<'a> Layout<'a> {
    pub fn from_keymap(kb_def: &'a KbDef, keymap: Keymap) -> Self {
        Layout {
            token_map: mk_token_map(kb_def, &keymap),
            keymap: keymap,
            kb_def: kb_def,
        }
    }

    pub fn mk_group_map(&self) -> GroupMap {
        let mut map = self.kb_def.group_num().map_nums(|_| None);
        for (token_num, &loc_num) in self.token_map.enumerate() {
            let group = self.kb_def.token_group[token_num];
            let group_num = self.kb_def.group_num().apply(group);
            let key_num = self.kb_def.loc_num().apply(loc_num).key_num;
            map[group_num] = Some(key_num);
        }
        return map.map_into(|value| value.unwrap());
    }

    pub fn assign_all(&mut self, assignments: &[Assignment]) {
        for &assignment in assignments.iter() {
            self.assign(self.kb_def, assignment);
        }
    }

    pub fn gen_moves<'b>(&'b self) -> MoveGen<'b> {
        MoveGen::new(self)
    }
}

impl<'a> Assignable for Layout<'a> {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        let prev_loc = self.token_map[token_num];
        self.token_map[token_num] = loc_num;
        // clear prev loc
        if self.keymap[prev_loc] == Some(token_num) {
            self.keymap[prev_loc] = None;
        }
        // assign new loc
        self.keymap[loc_num] = Some(token_num);
    }
}

pub type Keymap = Table<Loc, Option<Num<Token>>>;
pub type TokenMap = Table<Token, Num<Loc>>;
pub type GroupMap = Table<Group, Num<Key>>;

fn mk_token_map(kb_def: &KbDef, keymap: &Keymap) -> TokenMap {
    let mut map = kb_def.tokens.map_nums(|_| None);
    for (loc_num, &value) in keymap.enumerate() {
        if let Some(token_num) = value {
            map[token_num] = Some(loc_num);
        }
    }
    return map.map_into(|value| value.unwrap());
}

impl Assignable for TokenMap {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        self[token_num] = loc_num;
    }
}

impl Assignable for GroupMap {
    fn assign_group(&mut self, group_num: Num<Group>, key_num: Num<Key>) {
        self[group_num] = key_num;
    }
}

// TODO: not entirely correct
impl Assignable for Keymap {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        self[loc_num] = Some(token_num);
    }
}
