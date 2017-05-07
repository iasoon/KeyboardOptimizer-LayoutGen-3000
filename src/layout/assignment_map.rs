use model::{KbDef, LockId, KeyId, FreeId, Loc};
use model::{Lock, Key, Free, LocData};
use utils::{LookupTable, ElemCount};
use layout::Assignment;

use std::ops::{Index, IndexMut};

fn lock_map_data(kb_def: &KbDef) -> (ElemCount<Lock>, ElemCount<Key>) {
    (kb_def.locks.elem_count(), kb_def.keys.elem_count())
}

fn free_map_data(kb_def: &KbDef) -> (ElemCount<Free>, LocData) {
    (kb_def.frees.elem_count(), kb_def.loc_data())
}

pub struct AssignmentMap<T> {
    lock_map: LookupTable<(LockId, KeyId), T>,
    free_map: LookupTable<((FreeId, Loc)), T>,
}

impl<T> AssignmentMap<T> {
    pub fn from_fn<F>(kb_def: &KbDef, mut fun: F) -> Self
        where F: FnMut(Assignment) -> T
    {
        AssignmentMap {
            lock_map: LookupTable::from_fn(
                lock_map_data(kb_def),
                |(lock_id, key_id)| fun(Assignment::Lock { lock_id, key_id })),
            free_map: LookupTable::from_fn(
                free_map_data(kb_def),
                |(free_id, loc)| fun(Assignment::Free { free_id, loc })),
        }
    }

    pub fn map_mut<F>(&mut self, mut fun: F)
        where F: FnMut(Assignment, &mut T)
    {
        self.lock_map.map_mut(|(lock_id, key_id), elem| {
            let assignment = Assignment::Lock { lock_id, key_id };
            fun(assignment, elem);
        });
        self.free_map.map_mut(|(free_id, loc), elem| {
            let assignment = Assignment::Free { free_id, loc };
            fun(assignment, elem);
        });
    }
}


impl<T> Index<Assignment> for AssignmentMap<T> {
    type Output = T;

    fn index<'a>(&'a self, idx: Assignment) -> &'a T {
        match idx {
            Assignment::Lock { lock_id, key_id } => {
                &self.lock_map[(lock_id, key_id)]
            },
            Assignment::Free { free_id, loc } => {
                &self.free_map[(free_id, loc )]
            }
        }
    }
}

impl<T> IndexMut<Assignment> for AssignmentMap<T> {

    fn index_mut<'a>(&'a mut self, idx: Assignment) -> &'a mut T {
        match idx {
            Assignment::Lock { lock_id, key_id } => {
                &mut self.lock_map[(lock_id, key_id)]
            },
            Assignment::Free { free_id, loc } => {
                &mut self.free_map[(free_id, loc )]
            }
        }
    }
}
