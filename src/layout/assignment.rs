use std::vec::Vec;
use model::*;
use std::ops::{Index, Fn};
use utils::{Countable, ElemCount};

#[derive(Debug, Clone, Copy)]
pub enum Assignment {
    Free { free_id: FreeId, loc: Loc },
    Lock { lock_id: LockId, key_id: KeyId },
}

impl Assignment {
    pub fn group(&self, kb_def: &KbDef) -> GroupId {
        match self {
            &Assignment::Free { free_id, loc: _ } => {
                kb_def.free_group[free_id]
            },
            &Assignment::Lock { lock_id, key_id: _ } => {
                kb_def.lock_group[lock_id]
            }
        }
    }

    pub fn perform<Acceptor>(&self, acceptor: &mut Acceptor, kb_def: &KbDef)
        where Acceptor: AssignmentAcceptor
    {
        match *self {
            Assignment::Free { free_id, loc } => {
                let group_id = kb_def.free_group[free_id];
                let key_id = loc.key(&kb_def.loc_data());
                let token_id = kb_def.frees[free_id].token_id;
                acceptor.assign_group(group_id, key_id);
                acceptor.assign_token(token_id, loc);
            },
            Assignment::Lock { lock_id, key_id } => {
                let group_id = kb_def.lock_group[lock_id];
                let lock = &kb_def.locks[lock_id];
                acceptor.assign_group(group_id, key_id);
                for (layer_id, token_id) in lock.elems() {
                    let loc = kb_def.loc_data().loc(key_id, layer_id);
                    acceptor.assign_token(token_id, loc);
                }
            }
        }
    }
}

// undergoes assignments
pub trait AssignmentAcceptor {
    fn assign_token(&mut self, token_id: TokenId, loc: Loc);
    fn assign_group(&mut self, group_id: GroupId, key_id: KeyId);
}

pub struct AssignmentData {
    free_data: (ElemCount<Free>, LocData),
    lock_data: (ElemCount<Lock>, ElemCount<Key>),
}

impl AssignmentData {
    pub fn new(kb_def: &KbDef) -> Self {
        AssignmentData {
            free_data: (kb_def.frees.elem_count(), kb_def.loc_data()),
            lock_data: (kb_def.locks.elem_count(), kb_def.keys.elem_count()),
        }
    }

    fn free_offset(&self) -> usize {
        <(LockId, KeyId) as Countable>::count(&self.lock_data)
    }
}

impl Countable for Assignment {
    type Data = AssignmentData;

    fn to_num(&self, data: &AssignmentData) -> usize {
        match self {
            &Assignment::Lock{ lock_id, key_id } => {
                (lock_id, key_id).to_num(&data.lock_data)
            },
            &Assignment::Free{ free_id, loc } => {
                data.free_offset() + (free_id, loc).to_num(&data.free_data)
            }
        }
    }

    fn from_num(data: &AssignmentData, num: usize) -> Self {
        if num < data.free_offset() {
            let (lock_id, key_id) = <(LockId, KeyId) as Countable>::from_num(&data.lock_data, num);
            Assignment::Lock {
                lock_id: lock_id,
                key_id: key_id
            }
        } else {
            let (free_id, loc) = <(FreeId, Loc) as Countable>::from_num(&data.free_data, num - data.free_offset());
            Assignment::Free {
                free_id: free_id,
                loc: loc,
            }
        }
    }

    fn count(data: &AssignmentData) -> usize() {
        data.free_offset() + <(FreeId, Loc) as Countable>::count(&data.free_data)
    }
}
