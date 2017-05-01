use utils::LookupTable;
use model::*;
use layout::{Keymap, TokenMap};

#[derive(Debug, Clone, Copy)]
pub enum Assignment {
    Free { free_id: FreeId, loc: Loc },
    Lock { lock_id: LockId, key_id: KeyId },
}

pub struct AssignmentResolver<'a> {
    kb_def: &'a KbDef,
    keymap: &'a Keymap,
    token_map: &'a TokenMap,
    free_used: LookupTable<FreeId, bool>,
    lock_used: LookupTable<LockId, bool>,
    assignments: Vec<Assignment>,
}

impl<'a> AssignmentResolver<'a> {
    pub fn new(keymap: &'a Keymap, token_map: &'a TokenMap, kb_def: &'a KbDef) -> Self {
        AssignmentResolver {
            free_used: LookupTable::new(kb_def.frees.elem_count(), false),
            lock_used: LookupTable::new(kb_def.locks.elem_count(), false),
            kb_def: kb_def,
            keymap: keymap,
            token_map: token_map,
            assignments: Vec::new(),
        }
    }

    pub fn resolve(mut self) -> Vec<Assignment> {
        let mut pos = 0;
        while pos < self.assignments.len() {
            let assignment = self.assignments[pos];
            self.resolve_assignment(assignment);
            pos += 1;
        }
        return self.assignments;
    }

    fn resolve_assignment(&mut self, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_id, loc } => {
                let token_id = self.kb_def.frees[free_id].token_id;
                self.assign_token(token_id, loc);
            }
            Assignment::Lock { lock_id, key_id } => {
                let lock = &self.kb_def.locks[lock_id];
                for (layer_id, token_id) in lock.elems() {
                    self.assign_token(token_id,
                                      Loc::new(&self.kb_def.loc_data(), key_id, layer_id));
                }
            }
        }
    }

    fn assign_token(&mut self, token_id: TokenId, loc: Loc) {
        if let Some(t) = self.keymap[loc] {
            let current_loc = self.token_map[token_id];
            match self.kb_def.groups[self.kb_def.token_group[t]] {
                Group::Free(free_id) => {
                    if !self.free_used[free_id] {
                        self.assign_free(free_id, current_loc)
                    }
                }
                Group::Locked(lock_id) => {
                    if !self.lock_used[lock_id] {
                        let key_id = current_loc.key(&self.kb_def.loc_data());
                        self.assign_lock(lock_id, key_id);
                    }
                }
            }
        }
    }

    pub fn assign_free(&mut self, free_id: FreeId, loc: Loc) {
        self.free_used[free_id] = true;
        self.assignments.push(Assignment::Free {
            free_id: free_id,
            loc: loc,
        })
    }

    pub fn assign_lock(&mut self, lock_id: LockId, key_id: KeyId) {
        self.lock_used[lock_id] = true;
        self.assignments.push(Assignment::Lock {
            lock_id: lock_id,
            key_id: key_id,
        })
    }
}
