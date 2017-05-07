use layout::{GroupMap, Alteration, Assignment};
use model::{KbDef, GroupId, KeyId};


pub struct GroupMapWalker<'a> {
    start: &'a GroupMap,
    pos: GroupMap,
    kb_def: &'a KbDef,
}

impl<'a> GroupMapWalker<'a> {
    pub fn new(start: &'a GroupMap, kb_def: &'a KbDef) -> Self {
        GroupMapWalker {
            pos: start.clone(),
            start: start,
            kb_def: kb_def,
        }
    }

    pub fn pos<'b>(&'b self) -> &'b GroupMap {
        &self.pos
    }

    pub fn do_move(&mut self, alteration: &Alteration) {
        for assignment in alteration.assignments() {
            self.assign(assignment);
        }
    }

    pub fn reset_move(&mut self, alteration: &Alteration) {
        for assignment in alteration.assignments() {
            self.reset_assign(assignment);
        }
    }

    pub fn assign(&mut self, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_id, loc } => {
                let group_id = self.kb_def.free_group[free_id];
                self.assign_group(group_id, loc.key(&self.kb_def.loc_data()));
            },
            Assignment::Lock { lock_id, key_id } => {
                let group_id = self.kb_def.lock_group[lock_id];
                self.assign_group(group_id, key_id);
            }
        }
    }

    pub fn reset_assign(&mut self, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_id , loc: _ } => {
                let group_id = self.kb_def.free_group[free_id];
                self.reset_group(group_id);
            },
            Assignment::Lock { lock_id, key_id: _ } => {
                let group_id = self.kb_def.lock_group[lock_id];
                self.reset_group(group_id);
            }
        }
    }

    fn assign_group(&mut self, group_id: GroupId, key_id: KeyId) {
        self.pos[group_id] = key_id;
    }

    fn reset_group(&mut self, group_id: GroupId) {
        self.pos[group_id] = self.start[group_id];
    }
}
