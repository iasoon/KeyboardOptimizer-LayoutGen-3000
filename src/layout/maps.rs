use model::{KbDef, TokenId, GroupId, KeyId, Loc};
use utils::LookupTable;

pub type Keymap = LookupTable<Loc, Option<TokenId>>;
pub type TokenMap = LookupTable<TokenId, Loc>;
pub type GroupMap = LookupTable<GroupId, KeyId>;

pub fn mk_token_map(keymap: &Keymap, kb_def: &KbDef) -> TokenMap {
    let mut token_map = LookupTable::new(kb_def.tokens.elem_count(), None);
    for (loc, &value) in keymap.iter() {
        if let Some(token_id) = value {
            token_map[token_id] = Some(loc);
        }
    }
    return token_map.drain_map(|value| value.unwrap());
}
