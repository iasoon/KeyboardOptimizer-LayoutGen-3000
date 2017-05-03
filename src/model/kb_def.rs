use utils::{BoundedSet, LookupTable};
use model::*;

pub struct KbDef {
    pub keys: BoundedSet<Key>,
    pub layers: BoundedSet<Layer>,
    pub tokens: BoundedSet<Token>,

    pub groups: BoundedSet<Group>,
    pub locks: BoundedSet<Lock>,
    pub frees: BoundedSet<Free>,

    pub token_group: LookupTable<TokenId, GroupId>,
    pub free_group: LookupTable<FreeId, GroupId>,
    pub lock_group: LookupTable<LockId, GroupId>,

}

impl KbDef {
    pub fn loc_data(&self) -> LocData {
        LocData {
            key_data: self.keys.elem_count(),
            layer_data: self.layers.elem_count(),
        }
    }
}
