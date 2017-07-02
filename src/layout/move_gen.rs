use data::*;
use cat::*;

use layout::*;

pub struct MoveGen<'a> {
    kb_def: &'a KbDef,
    keymap: &'a Keymap,
    assignment_used: ElemTable<Assignment, AssignmentNum, bool>,
}

impl<'a> MoveGen<'a> {
    fn next_assignment(&mut self) -> Option<Assignment> {
        unimplemented!()
    }

    fn next_move(&mut self) -> Option<Vec<Assignment>> {
        while let Some(assignment) = self.next_assignment() {
            let mut builder = self.move_builder();
            builder.assignments.push(assignment);
            builder.resolve();
            if builder.move_allowed() {
                return Some(builder.assignments);
            }
        }
        return None;
    }

    fn move_builder<'b>(&'b mut self) -> MoveBuilder<'b> {
        MoveBuilder {
            kb_def: self.kb_def,
            keymap: self.keymap,
            assignment_used: &mut self.assignment_used,
            assignments: Vec::with_capacity(
                // an assignment swaps tokens between two keys, and each key has
                // at most #layers tokens. Thus, 2*#layers is a (liberal) upper
                // bound for the number of assignments in a move.
                2 * self.kb_def.layers.count().as_usize()
            ),
        }
    }
}

impl<'a> Iterator for MoveGen<'a> {
    type Item = Vec<Assignment>;

    fn next(&mut self) -> Option<Vec<Assignment>> {
        self.next_move()
    }
}

/// Careful: this MoveBuilder does not always yield correct moves.
/// The moves it produces will however always be correct when
/// - it is seeded with a lock
/// - all lock-assignments are already marked as used
/// In both these cases, the caveat in get_assignment does not occur.
pub struct MoveBuilder<'a> {
    kb_def: &'a KbDef,

    keymap: &'a Keymap,

    assignment_used: &'a mut ElemTable<Assignment, AssignmentNum, bool>,
    assignments: Vec<Assignment>,
}

impl<'a> MoveBuilder<'a> {
    fn move_allowed(&self) -> bool {
        self.assignments.iter().all(|&assignment| {
            self.kb_def.assignment_map.get(assignment).is_some()
        })
    }

    fn resolve(&mut self) {
        let mut pos = 0;
        while pos < self.assignments.len() {
            let assignment = self.assignments[pos];
            self.assign(self.kb_def, assignment);
            pos += 1;
        }
    }

    fn queue_assignment(&mut self, assignment: Assignment) {
        *self.assignment_used.get_mut(assignment) = true;
        self.assignments.push(assignment);
    }

    /// Get assignment that will move token_num to loc_num.
    /// When swapping a free with a lock, free tokens should never change layer,
    /// as this opens doors to all kinds of difficult situations.
    /// This method assumes that this never happens.
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
