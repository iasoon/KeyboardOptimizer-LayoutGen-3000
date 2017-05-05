use std::vec::Vec;
use model::{KbDef, FreeId, LockId, Loc, KeyId, GroupId};

// "move" is a keyword, unfortunately.
#[derive(Debug)]
pub struct Alteration {
    assignments: Vec<Assignment>,
}

impl Alteration {
    pub fn new(assignments: Vec<Assignment>) -> Alteration {
        Alteration {
            assignments
        }
    }

    pub fn assignments<'a>(&'a self) -> impl Iterator<Item = Assignment> + 'a {
        self.assignments.iter().cloned()
    }

    pub fn groups<'a>(&'a self, kb_def: &'a KbDef) -> impl Iterator<Item = GroupId> + 'a {
        self.assignments.iter().map(move |assignment| {
            assignment.group(kb_def)
        })
    }
}

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
}
