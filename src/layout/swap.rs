use utils::{BoundedSubset};
use model::{KbDef, KeyId, LockId, LayerId, Loc, GroupId, Group};
use layout::Keymap;

struct SwapMaskBuilder<'a> {
    mask: SwapMask,
    keymap: &'a Keymap,
    kb_def: &'a KbDef,
}

impl<'a> SwapMaskBuilder<'a> {
    fn new(keys: [KeyId; 2], keymap: &'a Keymap, kb_def: &'a KbDef) -> Self {
        SwapMaskBuilder {
            mask: SwapMask::new(keys, kb_def),
            keymap: keymap,
            kb_def: kb_def,
        }
    }

    fn include_layer(&mut self, layer_id: LayerId) {
        self.mask.layers.add(layer_id);
        for n in 0..2 {
            let loc = Loc::new(&self.kb_def.loc_data(), self.mask.keys[n], layer_id);
            self.include_loc(loc);
        }
    }

    fn include_loc(&mut self, loc: Loc) {
        if let Some(token_id) = self.keymap[loc] {
            let group_id = self.kb_def.token_group[token_id];
            if !self.mask.groups.contains(group_id) {
                self.include_group(group_id);
            }
        }
    }

    fn include_group(&mut self, group_id: GroupId) {
        self.mask.groups.add(group_id);
        match self.kb_def.groups[group_id] {
            Group::Free(_) => {},
            Group::Locked(lock_id) => self.include_lock(lock_id),
        }
    }

    fn include_lock(&mut self, lock_id: LockId) {
        for layer_id in self.kb_def.locks[lock_id].layers() {
            if !self.mask.layers.contains(layer_id) {
                self.include_layer(layer_id);
            }
        }
    }
}

pub struct SwapMask {
    groups: BoundedSubset<GroupId>,
    layers: BoundedSubset<LayerId>,
    keys: [KeyId; 2],
}

impl SwapMask {
    fn new(keys: [KeyId; 2], kb_def: &KbDef) -> Self {
        SwapMask {
            groups: BoundedSubset::new(kb_def.groups.elem_count()),
            layers: BoundedSubset::new(kb_def.layers.elem_count()),
            keys: keys,
        }
    }
}
