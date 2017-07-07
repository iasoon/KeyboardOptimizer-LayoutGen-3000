use std::vec::Vec;

use data::*;
use cat::*;
use cat::ops::*;
use errors::*;

use layout::*;
use layout::utils::{Subset, IndexedList};

pub struct Generator<'a> {
    kb_def: &'a KbDef,

    frees: Table<Free, Subset<Loc>>,
    locks: Table<Lock, Subset<Key>>,
    // TODO: abstract away this pattern
    // (ElemTable<D, D->Num<D>, T>)
    unassigned: IndexedList<Group, ElemTable<Group, GroupNum, Option<usize>>>,

    /// The stack describes the path taken to get to the current position.
    stack: Vec<Step>,

    /// FIFO stack of (blocked assignment, position) pairs.
    /// Position information is needed to restore assignment sets to their
    /// original order.
    blocked: Vec<(Assignment, usize)>,
}

struct Step {
    assignment: Assignment,
    pos: usize,
    blocked_idx: usize,
}

impl<'a> Generator<'a> {
    // TODO: eww.
    // find a proper way to initialize subsets, and make fields private.
    pub fn new(kb_def: &'a KbDef) -> Self {
        let mut generator = Generator {
            frees: kb_def.frees.map(|_| Subset::empty(kb_def.loc_num().count())),
            locks: kb_def.locks.map(|_| Subset::empty(kb_def.keys.count())),
            unassigned: IndexedList {
                elems: Vec::with_capacity(kb_def.group_num().count().as_usize()),
                idxs: Composed::new(
                    kb_def.group_num(),
                    kb_def.group_num().map_nums(|_| None)),
            },

            stack: Vec::with_capacity(kb_def.group_num().count().as_usize()),
            blocked: Vec::with_capacity(kb_def.assignments.count().as_usize()),

            kb_def: kb_def,
        };

        // add all groups as unassigned
        for group_num in kb_def.group_num().nums() {
            let group = kb_def.group_num().apply(group_num);
            generator.unassigned.add(group, 0);
        }

        for (_, &assignment) in kb_def.assignments.enumerate() {
            generator.add_assignment(assignment, 0);
        }

        return generator;
    }

    pub fn generate(&mut self) -> Result<Keymap> {
        self.reset();
        self.shuffle();
        try!(self.construct());
        return Ok(self.extract_keymap());
    }

    fn extract_keymap(&self) -> Keymap {
        let mut keymap = self.kb_def.loc_num().map_nums(|_| None);
        for step in self.stack.iter() {
            keymap.assign(self.kb_def, step.assignment);
        }
        return keymap;
    }


    fn shuffle(&mut self) {
        self.frees.map_mut(|locs| locs.shuffle());
        self.locks.map_mut(|keys| keys.shuffle());
    }

    fn reset(&mut self) {
        while self.stack.len() > 0 {
            self.pop();
        }
    }

    fn construct(&mut self) -> Result<()> {
        while let Some(group) = self.next_group() {
            if self.group_count(group) > 0 {
                // descend
                self.step(group, 0);
            } else {
                // backtrack
                try!(self.next_node());
            }
        }
        Ok(())
    }

    // which group to assign next
    fn next_group(&self) -> Option<Group> {
        self.unassigned
            .iter()
            .min_by_key(|&group| self.group_count(group))
    }

    /// Go to next node in the backtracking tree.
    fn next_node(&mut self) -> Result<()> {
        while let Some((group, pos)) = self.pop() {
            if self.group_count(group) > pos + 1 {
                self.step(group, pos + 1);
                return Ok(())
            }
        }
        bail!("layout space exhaustively searched")
    }

    /// perform a step: descend one level in the backtracking tree.
    fn step(&mut self, group: Group, pos: usize) {
        let assignment = self.get_assignment(group, pos);

        // assign group
        self.unassigned.remove(group);

        // update stack
        self.stack.push(Step {
            pos: pos,
            assignment: assignment,
            blocked_idx: self.blocked.len(),
        });

        // remove conflicts
        for a in overlaps(self.kb_def, assignment).into_iter() {
            self.remove_assignment(a);
        }
    }

    /// Undo last step, and return its group and position.
    fn pop(&mut self) -> Option<(Group, usize)> {
        self.stack.pop().map(|step| {
            let group = assignment_group(self.kb_def, step.assignment);
            // unassign group
            self.unassigned.add(group, 0);
            // unblock assignments
            while self.blocked.len() > step.blocked_idx {
                let (assignment, pos) = self.blocked.pop().unwrap();
                self.add_assignment(assignment, pos);
            }
            // return position
            (group, step.pos)
        })
    }

    /// Register an assignment as available.
    fn add_assignment(&mut self, assignment: Assignment, pos: usize) {
        match assignment {
            Assignment::Free { free_num, loc_num } => {
                self.frees.get_mut(free_num).add(loc_num, pos);
            }
            Assignment::Lock { lock_num, key_num } => {
                self.locks.get_mut(lock_num).add(key_num, pos);
            }
        }
    }

    /// Make this assignment unavailable.
    fn remove_assignment(&mut self, assignment: Assignment) {
        let idx = match assignment {
            Assignment::Free { free_num, loc_num } => {
                self.frees.get_mut(free_num).remove(loc_num)
            }
            Assignment::Lock { lock_num, key_num } => {
                self.locks.get_mut(lock_num).remove(key_num)
            }
        };
        if let Some(num) = idx {
            self.blocked.push((assignment, num));
        }
    }

    fn get_assignment(&self, group: Group, pos: usize) -> Assignment {
        match group {
            Group::Free(free_num) => {
                Assignment::Free {
                    free_num: free_num,
                    loc_num: self.frees.get(free_num).get(pos),
                }
            }
            Group::Lock(lock_num) => {
                Assignment::Lock {
                    lock_num: lock_num,
                    key_num: self.locks.get(lock_num).get(pos),
                }
            }
        }
    }

    /// How many assignments remain available for given group
    fn group_count(&self, group: Group) -> usize {
        match group {
            Group::Free(free_id) => self.frees.get(free_id).size(),
            Group::Lock(lock_id) => self.locks.get(lock_id).size(),
        }
    }
}

fn overlaps(kb_def: &KbDef, assignment: Assignment) -> Vec<Assignment> {
    match assignment {
        Assignment::Free { free_num: _, loc_num } => {
            return free_overlaps(kb_def, loc_num);
        }
        Assignment::Lock { lock_num, key_num } => {
            return lock_overlaps(kb_def, lock_num, key_num);
        }
    }
}

fn free_overlaps(kb_def: &KbDef, loc_num: Num<Loc>) -> Vec<Assignment> {
    let loc: Loc = kb_def.loc_num().apply(loc_num);
    kb_def.assignments
        .enumerate()
        .map(|(_, &assignment)| assignment)
        .filter(|&assignment| {
            match assignment {
                Assignment::Free { free_num: _, loc_num: other_loc_num } => {
                    return loc_num == other_loc_num;
                }
                Assignment::Lock { lock_num, key_num } => {
                    let entry = kb_def.locks.get(lock_num).get(loc.layer_num);
                    return key_num == loc.key_num && entry.is_some();
                }
            }
        })
        .collect()
}

fn lock_overlaps(kb_def: &KbDef, lock_num: Num<Lock>, key_num: Num<Key>) -> Vec<Assignment> {
    let lock = kb_def.locks.get(lock_num);
    kb_def.assignments
        .enumerate()
        .map(|(_, &assignment)| assignment)
        .filter(|&assignment| {
            match assignment {
                Assignment::Free { free_num: _, loc_num } => {
                    let loc: Loc = kb_def.loc_num().apply(loc_num);
                    key_num == loc.key_num && lock.get(loc.layer_num).is_some()
                }
                Assignment::Lock { lock_num: loc2_num, key_num: key2_num } => {
                    let locks_overlap = kb_def.locks
                        .get(loc2_num)
                        .enumerate()
                        .any(|(layer_num, value)| value.is_some() && lock.get(layer_num).is_some());
                    return key_num == key2_num && locks_overlap;
                }
            }
        })
        .collect()
}

// TODO: move to Assignment
fn assignment_group(kb_def: &KbDef, assignment: Assignment) -> Group {
    match assignment {
        Assignment::Free { free_num, loc_num: _ } => Group::Free(free_num),
        Assignment::Lock { lock_num, key_num: _ } => Group::Lock(lock_num),
    }
}
