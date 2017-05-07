use layout::{Layout, Alteration, Assignment, AssignmentResolver};
use model::{KbDef, LockId, KeyId};
use utils::{Enumerator, LookupTable};

pub struct Moves<'a> {
    layout: &'a Layout<'a>,

    enumerator: Enumerator<(LockId, KeyId)>,
    assignment_used: LookupTable<(LockId, KeyId), bool>,
}

impl<'a> Moves<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        let data = (layout.kb_def.locks.elem_count(), layout.kb_def.keys.elem_count());
        Moves {
            layout: layout,

            enumerator: Enumerator::new(data.clone()),
            assignment_used: LookupTable::new(data, false),
        }
    }

    fn generate_move(&mut self, lock_id: LockId, key_id: KeyId) -> Vec<Assignment> {
        let mv = self.mk_move(lock_id, key_id);
        self.visit_move(&mv);
        return mv;
    }

    fn mk_move(&self, lock_id: LockId, key_id: KeyId) -> Vec<Assignment> {
        let mut resolver = AssignmentResolver::new(&self.layout.keymap,
                                                   &self.layout.token_map,
                                                   self.layout.kb_def);
        resolver.assign_lock(lock_id, key_id);
        return resolver.resolve();
    }

    fn visit_move(&mut self, assignments: &Vec<Assignment>) {
        for &assignment in assignments.iter() {
            if let Assignment::Lock { lock_id, key_id } = assignment {
                self.assignment_used[(lock_id, key_id)] = true;
            }
        }
    }

    fn assignment_valid(&self, lock_id: LockId, key_id: KeyId) -> bool {
        !self.assignment_used[(lock_id, key_id)] && !self.assignment_fulfilled(lock_id, key_id)
    }

    fn assignment_fulfilled(&self, lock_id: LockId, key_id: KeyId) -> bool {
        let group_id = self.layout.kb_def.lock_group[lock_id];
        return self.layout.group_map[group_id] == key_id;
    }
}


impl<'a> Iterator for Moves<'a> {
    type Item = Vec<Assignment>;

    fn next(&mut self) -> Option<Vec<Assignment>> {
        while let Some((lock_id, key_id)) = self.enumerator.next() {

            if self.assignment_valid(lock_id, key_id) {
                return Some(self.generate_move(lock_id, key_id));
            }
        }
        return None;
    }
}
