use data::*;
use cat::*;

use layout::*;

pub struct MoveBuilder<'a> {
    kb_def: &'a KbDef,

    keymap: &'a Keymap,
    token_map: &'a TokenMap,

    assignment_used: &'a mut ElemTable<Assignment, AssignmentNum, bool>,
    assignments: Vec<Assignment>,
}

impl<'a> MoveBuilder<'a> {
    fn build(mut self) -> Vec<Assignment> {
        let mut pos = 0;
        while pos < self.assignments.len() {
            let assignment = self.assignments[pos];
            self.assign(self.kb_def, assignment);
            pos += 1;
        }
        return self.assignments;
    }

    fn queue_assignment(&mut self, assignment: Assignment) {
        *self.assignment_used.get_mut(assignment) = true;
        self.assignments.push(assignment);
    }

    /// Get assignment that will move token_num to loc_num
    fn get_assignment(&self, token_num: Num<Token>, loc_num: Num<Loc>)
                      -> Assignment
    {
        match self.kb_def.token_group.get(token_num) {
            &Group::Free(free_num) => {
                Assignment::Free {
                    free_num: free_num,
                    loc_num: loc_num,
                }
            },
            &Group::Lock(lock_num) => {
                let loc: Loc = self.kb_def.loc_num().apply(loc_num);
                Assignment::Lock {
                    lock_num: lock_num,
                    key_num: loc.key_num,
                }
            }
        }
    }
}

impl<'a> Assignable for MoveBuilder<'a> {
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>) {
        if let &Some(replaced) = self.keymap.get(loc_num) {
            let assignment = self.get_assignment(replaced, loc_num);

            if !self.assignment_used.get(assignment) {
                self.queue_assignment(assignment);
            }
        }
    }
}
