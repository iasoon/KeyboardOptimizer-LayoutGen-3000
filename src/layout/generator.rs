use rand::{thread_rng, Rng};
use std::vec::Vec;
use std::mem;

use data::*;
use cat::*;
use cat::ops::*;
use errors::*;

use layout::assignable::Assignable;

type GroupTable<T> = ComposedDict<Group, Num<Group>, T,
                                  GroupNum, Table<Group, T>>;

pub struct Generator<'a> {
    kb_def: &'a KbDef,

    frees: Table<Free, NumSubset<Loc>>,
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

impl<'a> Assignable for Generator<'a> {
    fn assign_token(&mut self, _: Num<Token>, loc_num: Num<Loc>) {
        let kb_def = &self.kb_def;

        // handle frees
        self.frees.map_mut(|locs| {
            locs.remove(loc_num);
        });

        // handle locks
        let loc: Loc = self.kb_def.loc_num().apply(loc_num);
        self.locks.map_mut_with_key(|lock_num, keys| {
            // check whether assigned loc and lock overlap
            if kb_def.locks.get(lock_num).get(loc.layer_num).is_some() {
                keys.remove(loc.key_num);
            }
        });
    }
}

impl<'a> Generator<'a> {
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
        // TODO: assign
        self.stack.push(Step {
            pos: pos,
            assignment: assignment,
            blocked_idx: self.blocked.len(),
        });
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
    fn new(dict: M) -> Self
        where M: HasCount<D>
    {
        Subset {
            elems: Vec::with_capacity(dict.count().as_usize()),
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
