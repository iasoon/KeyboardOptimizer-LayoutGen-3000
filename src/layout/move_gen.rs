use data::*;
use cat::*;

use layout::*;

/// In order to generate correct moves, all lock-assignments should be visited
/// first, then all free-assignments. The easiest way to accomplish this is to
/// make sure that AllowedAssignments are ordered in this way.
/// See further below for more information.
pub struct MoveGen<'a> {
    kb_def: &'a KbDef,
    keymap: &'a Keymap,
    token_map: &'a TokenMap,
    assignment_used: Table<AllowedAssignment, Option<usize>>,

    enumerator: Enumerator<AllowedAssignment>,
    iteration: usize,
}

impl<'a> MoveGen<'a> {
    fn next_assignment(&mut self) -> Option<Assignment> {
        self.enumerator.next().map(|num| {
            *self.kb_def.assignments.get(num)
        })
    }

    fn next_move(&mut self) -> Option<Vec<Assignment>> {
        while let Some(assignment) = self.next_assignment() {
            self.iteration += 1;
            if let Ok(assignments) = self.build_move(assignment) {
                return Some(assignments);
            }
        }
        return None;
    }

    fn build_move(&mut self, assignment: Assignment) -> Result<Vec<Assignment>> {
        let mut builder = self.move_builder();
        try!(builder.queue_assignment(assignment));
        try!(builder.resolve());
        Ok(builder.assignments)
    }

    fn move_builder<'b>(&'b mut self) -> MoveBuilder<'b> {
        MoveBuilder {
            kb_def: self.kb_def,
            keymap: self.keymap,
            token_map: self.token_map,
            assignment_used: &mut self.assignment_used,
            iteration: self.iteration,
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


type Result<T> = ::std::result::Result<T, ()>;

pub struct MoveBuilder<'a> {
    kb_def: &'a KbDef,

    keymap: &'a Keymap,
    token_map: &'a TokenMap,

    assignment_used: &'a mut Table<AllowedAssignment, Option<usize>>,
    assignments: Vec<Assignment>,
    iteration: usize,
}

enum AssignmentState {
    /// Free to use
    Allowed(Num<AllowedAssignment>),
    /// Used by this builder
    Used,
    /// Used by a previous builder or not allowed
    Forbidden,
}

use self::AssignmentState::*;

impl<'a> MoveBuilder<'a> {
    fn resolve(&mut self) -> Result<()> {
        let mut pos = 0;
        while pos < self.assignments.len() {
            let assignment = self.assignments[pos];
            try!(self.resolve_assignment(assignment));
            pos += 1;
        }
        Ok(())
    }

    fn assignment_state(&self, assignment: Assignment) -> AssignmentState {
        match *self.kb_def.assignment_map.get(assignment) {
            Some(num) => {
                match *self.assignment_used.get(num) {
                    Some(iteration) if iteration == self.iteration => Used,
                    Some(_) => Forbidden,
                    None => Allowed(num),
                }
            },
            None => Forbidden
        }
    }

    fn resolve_assignment(&mut self, assignment: Assignment) -> Result<()> {
        match assignment {
            Assignment::Free { free_num, loc_num } => {
                let &token_num = self.kb_def.frees.get(free_num);
                try!(self.resolve_token(token_num, loc_num));
                Ok(())
            },
            Assignment::Lock { lock_num, key_num } => {
                let lock = self.kb_def.locks.get(lock_num);
                for (layer_num, &value) in lock.enumerate() {
                    if let Some(token_num) = value {
                        let loc_num = self.kb_def.loc_num().apply(Loc {
                            key_num: key_num,
                            layer_num: layer_num,
                        });
                        try!(self.resolve_token(token_num, loc_num));
                    }
                }
                Ok(())
            }
        }
    }

    /// Resolve assignment of token_num to loc_num
    fn resolve_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>)
                     -> Result<()>
    {

        if let &Some(replaced) = self.keymap.get(loc_num) {
            let &prev_loc = self.token_map.get(token_num);
            try!(self.assign_token(replaced, prev_loc));
        }
        Ok(())
    }

    /// queue assignment that will move token_num to loc_num
    fn assign_token(&mut self, token_num: Num<Token>, loc_num: Num<Loc>)
                    -> Result<()>
    {
        self.get_assignment(token_num, loc_num).and_then(|assignment| {
            self.queue_assignment(assignment)
        })
    }


    fn queue_assignment(&mut self, assignment: Assignment)
                        -> Result<()>
    {
        match self.assignment_state(assignment) {
            Forbidden => Err(()),
            Used => Ok(()),
            Allowed(num) => {
                *self.assignment_used.get_mut(num) = Some(self.iteration);
                self.assignments.push(assignment);
                return Ok(());
            }
        }
    }

    /// Construct an assignment that will move token_num to loc_num.
    fn get_assignment(&self, token_num: Num<Token>, loc_num: Num<Loc>)
                      -> Result<Assignment>
    {
        match self.kb_def.token_group.get(token_num) {
            &Group::Free(free_num) => Ok(
                Assignment::Free {
                    free_num: free_num,
                    loc_num: loc_num,
                }
            ),
            &Group::Lock(lock_num) => {
                let lock = self.kb_def.locks.get(lock_num);
                let loc = self.kb_def.loc_num().apply(loc_num);
                // Swapping a locked token is only possible when it does not
                // move layers.
                if lock.get(loc.layer_num) == &Some(token_num) {
                    Ok(Assignment::Lock {
                        lock_num: lock_num,
                        key_num: loc.key_num,
                    })
                } else {
                    Err(())
                }
            }
        }
    }
}
