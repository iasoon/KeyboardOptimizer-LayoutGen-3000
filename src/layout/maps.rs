use data::{Token, Key, Loc};
use cat::*;

use layout::assignable::Assignable;

pub type Keymap = Table<Loc, Option<Num<Token>>>;

impl Assignable for Keymap {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        *self.get_mut(loc_num) = Some(token_num);
    }
}
