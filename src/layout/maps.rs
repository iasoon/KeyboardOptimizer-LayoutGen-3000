use model::{KbDef, TokenId, GroupId, KeyId, Loc};
use utils::LookupTable;

pub type Keymap = LookupTable<Loc, Option<TokenId>>;
pub type TokenMap = LookupTable<TokenId, Loc>;
pub type GroupMap = LookupTable<GroupId, KeyId>;

pub fn mk_keymap(token_map: &TokenMap, kb_def: &KbDef) -> Keymap {
    let mut keymap = LookupTable::new(kb_def.loc_data(), None);
    for (token_id, &loc) in token_map.iter() {
        keymap[loc] = Some(token_id);
    }
    return keymap;
}

pub fn mk_token_map(keymap: &Keymap, kb_def: &KbDef) -> TokenMap {
    let mut token_map = LookupTable::new(kb_def.tokens.elem_count(), None);
    for (loc, &value) in keymap.iter() {
        if let Some(token_id) = value {
            token_map[token_id] = Some(loc);
        }
    }
    return token_map.drain_map(|value| value.unwrap());
}

pub fn mk_group_map(token_map: &TokenMap, kb_def: &KbDef) -> GroupMap {
    let mut map = LookupTable::new(kb_def.groups.elem_count(), None);
    for (token_id, loc) in token_map.iter() {
        let group_id = kb_def.token_group[token_id];
        map[group_id] = Some(loc.key(&kb_def.loc_data()))
    }
    return map.drain_map(|val| val.unwrap());
}
