use data::*;
use cat::*;
use cat::ops::*;

use layout::assignable::Assignable;

pub struct Layout<'a> {
    pub keymap: Keymap,
    pub token_map: TokenMap,
    pub kb_def: &'a KbDef,
}

impl<'a> Layout<'a> {
    pub fn mk_group_map(&self) -> GroupMap {
        let mut map = self.kb_def.group_num().map_nums(|_| None);
        for (token_num, &loc_num) in self.token_map.enumerate() {
            let group = *self.kb_def.token_group.get(token_num);
            let group_num = self.kb_def.group_num().apply(group);
            let key_num = self.kb_def.loc_num().apply(loc_num).key_num;
            *map.get_mut(group_num) = Some(key_num);
        }
        return map.map_into(|value| value.unwrap());
    }
}

pub type Keymap = Table<Loc, Option<Num<Token>>>;
pub type TokenMap = Table<Token, Num<Loc>>;
pub type GroupMap = Table<Group, Num<Key>>;

impl Assignable for TokenMap {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        *self.get_mut(token_num) = loc_num;
    }
}

impl Assignable for GroupMap {
    fn assign_group(&mut self, group_num: Num<Group>, key_num: Num<Key>) {
        *self.get_mut(group_num) = key_num;
    }
}

// TODO: not entirely correct
impl Assignable for Keymap {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        *self.get_mut(loc_num) = Some(token_num);
    }
}
