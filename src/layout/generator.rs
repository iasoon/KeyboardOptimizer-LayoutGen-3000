use rand::{thread_rng, Rng};
use std::vec::Vec;
use std::mem;

use data::*;
use cat::*;
use cat::ops::*;
use errors::*;

use layout::assignable::Assignable;

// TODO: try to generalize this pattern
// abstract away Num<D> <-> D isomorphism, and its composition with a
// D -> T table.
type GroupTable<T> = ComposedDict<Group, Num<Group>, T,
                                  GroupNum, Table<Group, T>>;


pub struct Generator<'a> {
    kb_def: &'a KbDef,

    frees: Table<Free, Subset<Num<Loc>, Table<Loc, Option<usize>>>>,
    locks: Table<Lock, NumSubset<Key>>,
    unassigned: Subset<Group, GroupTable<Option<usize>>>,

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
    fn new(kb_def: &'a KbDef) -> Self {
        let mut generator = Generator {
            frees: kb_def.frees.map(|_| Subset {
                elems: Vec::with_capacity(kb_def.loc_num().count().as_usize()),
                idxs: kb_def.loc_num().map_nums(|_| None)
            }),

            locks: kb_def.locks.map(|_| Subset {
                elems: Vec::with_capacity(kb_def.keys.count().as_usize()),
                idxs: kb_def.keys.map_nums(|_| None)
            }),

            unassigned: Subset {
                elems: Vec::with_capacity(kb_def.group_num().count().as_usize()),
                idxs: kb_def.group_num()
                    .map_nums(|_| None)
                    .compose(kb_def.group_num())
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

    fn generate(&mut self) -> Result<()> {
        if let Some(initial_group) = self.next_group() {
            // initial step, 'root node'
            self.step(initial_group, 0);
            while self.next() {
                if let Some(group) = self.next_group() {
                    // descend
                    self.step(group, 0);
                } else {
                    // no groups remaining; generation complete
                    return Ok(());
                }
            }
            bail!("Layout generation failed. Check constraints for conflicts.")
        }
        // no groups exist
        Ok(())
    }

    fn next_group(&self) -> Option<Group> {
        unimplemented!()
    }

    /// perform a step: descend one level in the backtracking tree.
    fn step(&mut self, group: Group, pos: usize) {
        let assignment = self.get_assignment(group, pos);

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
            while self.blocked.len() > step.blocked_idx {
                let (assignment, pos) = self.blocked.pop().unwrap();
                self.add_assignment(assignment, pos);
            }
            unimplemented!()
        })
    }

    /// Register an assignment as available.
    fn add_assignment(&mut self, assignment: Assignment, pos: usize) {
        match assignment {
            Assignment::Free { free_num, loc_num } => {
                self.frees.get_mut(free_num).add(loc_num, pos);
            },
            Assignment::Lock { lock_num, key_num } => {
                self.locks.get_mut(lock_num).add(key_num, pos);
            }
        }
    }

    /// Make this assignment unavailable.
    fn remove_assignment(&mut self, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_num, loc_num } => {
                self.frees.get_mut(free_num).remove(loc_num);
            },
            Assignment::Lock { lock_num, key_num } => {
                self.locks.get_mut(lock_num).remove(key_num);
            }
        }
    }

    fn get_assignment(&self, group: Group, pos: usize) -> Assignment {
        match group {
            Group::Free(free_num) => {
                Assignment::Free {
                    free_num: free_num,
                    loc_num: self.frees.get(free_num).get(pos),
                }
            },
            Group::Lock(lock_num) => {
                Assignment::Lock {
                    lock_num: lock_num,
                    key_num: self.locks.get(lock_num).get(pos),
                }
            }
        }
    }


    /// Go to next node.
    fn next(&mut self) -> bool {
        while let Some((group, pos)) = self.pop() {
            if self.group_count(group) > pos + 1 {
                self.step(group, pos + 1);
                return true;
            }
        }
        return false;
    }

    /// How many assignments remain available for given group
    fn group_count(&self, group: Group) -> usize {
        match group {
            Group::Free(free_id) => {
                self.frees.get(free_id).size()
            },
            Group::Lock(lock_id) => {
                self.locks.get(lock_id).size()
            }
        }
    }
}

fn overlaps(kb_def: &KbDef, assignment: Assignment) -> Vec<Assignment> {
    match assignment {
        Assignment::Free { free_num: _, loc_num } => {
            return free_overlaps(kb_def, loc_num);
        },
        Assignment::Lock { lock_num, key_num } => {
            return lock_overlaps(kb_def, lock_num, key_num);
        }
    }
}

fn free_overlaps(kb_def: &KbDef, loc_num: Num<Loc>) -> Vec<Assignment> {
    let loc: Loc = kb_def.loc_num().apply(loc_num);
    kb_def.assignments.enumerate()
        .map(|(_, &assignment)| assignment)
        .filter(|&assignment| {
        match assignment {
            Assignment::Free { free_num: _, loc_num: other_loc_num } => {
                return loc_num == other_loc_num;
            },
            Assignment::Lock { lock_num, key_num } => {
                let entry = kb_def.locks.get(lock_num).get(loc.layer_num);
                return key_num == loc.key_num && entry.is_some();
            }
        }
    }).collect()
}

fn lock_overlaps(kb_def: &KbDef, lock_num: Num<Lock>, key_num: Num<Key>)
                 -> Vec<Assignment>
{
    let lock = kb_def.locks.get(lock_num);
    kb_def.assignments.enumerate()
        .map(|(_, &assignment)| assignment)
        .filter(|&assignment| {
            match assignment {
                Assignment::Free { free_num: _, loc_num } => {
                    let loc: Loc = kb_def.loc_num().apply(loc_num);
                    key_num == loc.key_num && lock.get(loc.layer_num).is_some()
                },
                Assignment::Lock { lock_num: loc2_num, key_num: key2_num } => {
                    let locks_overlap = kb_def.locks.get(loc2_num).enumerate()
                        .any(|(layer_num, value)| {
                            value.is_some() && lock.get(layer_num).is_some()
                        });
                    return key_num == key2_num && locks_overlap;
                }
            }
        }).collect()
}

fn assignment_group(kb_def: &KbDef, assignment: Assignment) -> Group {
    match assignment {
        Assignment::Free { free_num, loc_num: _} => {
            Group::Free(free_num)
        },
        Assignment::Lock { lock_num, key_num: _} => {
            Group::Lock(lock_num)
        }
    }
}

type NumSubset<D> = Subset<Num<D>, Table<D, Option<usize>>>;

struct Subset<D, M>
    where M: Dict<D, Option<usize>>,
          D: FiniteDomain
{
    // elements currently in this subset
    elems: Vec<D::Type>,
    // maps an element to its index
    idxs: M,
}

impl<D, M> Subset<D, M>
    where M: Dict<D, Option<usize>>,
          D::Type: Copy,
          D: FiniteDomain
{
    fn empty(count: Count<D>, dict: M) -> Self {
        Subset {
            elems: Vec::with_capacity(count.as_usize()),
            idxs: dict,
        }
    }

    fn add(&mut self, mut elem: D::Type, pos: usize) {
        if self.idxs.get(elem).is_none() {
            // swap elem and element in target position
            if pos < self.elems.len() {
                *self.idxs.get_mut(elem.clone()) = Some(pos);
                mem::swap(&mut elem, &mut self.elems[pos]);
            }
            // push elem to elems
            *self.idxs.get_mut(elem) = Some(self.elems.len());
            self.elems.push(elem);
        }
    }

    fn get(&self, pos: usize) -> D::Type {
        return self.elems[pos];
    }

    fn remove(&mut self, elem: D::Type) {
        if let Some(idx) = self.idxs.get_mut(elem).take() {
            self.elems.swap_remove(idx);
            if idx < self.elems.len() {
                *self.idxs.get_mut(self.elems[idx]) = Some(idx);
            }
        }
    }

    fn size(&self) -> usize {
        self.elems.len()
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(self.elems.as_mut_slice());
        // fix index
        for (idx, &elem) in self.elems.iter().enumerate() {
            *self.idxs.get_mut(elem) = Some(idx);
        }
    }
}
