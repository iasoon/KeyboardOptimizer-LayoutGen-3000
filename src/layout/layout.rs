use data::{Token, Key, Loc, KbDef};
use cat::*;

use layout::assignable::Assignable;

pub struct Layout<'a> {
    pub keymap: Keymap,
    pub token_map: TokenMap,
    pub kb_def: &'a KbDef,
}

pub type Keymap = Table<Loc, Option<Num<Token>>>;
pub type TokenMap = Table<Token, Num<Loc>>;

impl Assignable for Keymap {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        *self.get_mut(loc_num) = Some(token_num);
    }
}
