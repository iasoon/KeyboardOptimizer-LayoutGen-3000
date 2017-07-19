use data::*;
use cat::*;

use layout::assignable::Assignable;

pub struct Layout<'a> {
    pub keymap: Keymap,
    pub token_map: TokenMap,
    pub kb_def: &'a KbDef,
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
