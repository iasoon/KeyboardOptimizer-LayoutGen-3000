use rand::{thread_rng, Rng};
use std::vec::Vec;
use std::mem;

use data::*;
use cat::*;
use cat::ops::*;
use errors::*;

use layout::assignable::Assignable;

pub struct Generator<'a> {
    kb_def: &'a KbDef,

    frees: Table<Free, Subset<Loc>>,
    locks: Table<Lock, Subset<Key>>,

    stack: Vec<Step>,
}

struct Step {
    assignment: Assignment,
    pos: usize,
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

    fn step(&mut self, group: Group, pos: usize) {
        self.frees.map_mut(|locs| locs.set_restore_point());
        self.locks.map_mut(|keys| keys.set_restore_point());
        let assignment = self.get_assignment(group, pos);
        self.assign(self.kb_def, assignment);
        self.stack.push(Step {
            pos: pos,
            assignment: assignment,
        });
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


    /// Undo last step and return it.
    fn pop(&mut self) -> Option<Step> {
        if let Some(step) = self.stack.pop() {
            self.frees.map_mut(|locs| locs.restore());
            self.locks.map_mut(|keys| keys.restore());
            return Some(step);
        } else {
            return None;
        }
    }

    /// Go to next node.
    fn next(&mut self) -> bool {
        while let Some(step) = self.pop() {
            let group = assignment_group(self.kb_def, step.assignment);
            if self.group_count(group) > step.pos + 1 {
                self.step(group, step.pos + 1);
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

struct Subset<D: FiniteDomain> {
    // elements currently in this subset
    elems: Vec<Num<D>>,
    // maps an element to its index
    idxs: Table<D, Option<usize>>,

    // (elem, position elem was in)
    // This position is used to restore elems to its original order
    // (provided it was not otherwise modified)
    removed: Vec<(Num<D>, usize)>,
    // holds indices to restore to
    restore_points: Vec<usize>,
}

impl<D: FiniteDomain> Subset<D> {
    fn empty(universe: &Table<D, D::Type>) -> Self
    {
        Subset {
            elems: Vec::with_capacity(universe.count().as_usize()),
            idxs: universe.map(|_| None),
            removed: Vec::with_capacity(universe.count().as_usize()),
            restore_points: Vec::with_capacity(universe.count().as_usize()),
        }
    }

    fn add(&mut self, mut elem: Num<D>, pos: usize) {
        if self.idxs.get(elem).is_none() {
            // swap elem and element in target position
            if pos < self.elems.len() {
                *self.idxs.get_mut(elem) = Some(pos);
                mem::swap(&mut elem, &mut self.elems[pos]);
            }
            // push elem to elems
            *self.idxs.get_mut(elem) = Some(self.elems.len());
            self.elems.push(elem);
        }
    }

    fn get(&self, pos: usize) -> Num<D> {
        return self.elems[pos];
    }

    fn remove(&mut self, elem: Num<D>) {
        if let Some(idx) = self.idxs.get_mut(elem).take() {
            self.elems.swap_remove(idx);
            if idx < self.elems.len() {
                *self.idxs.get_mut(self.elems[idx]) = Some(idx);
            }
            self.removed.push((elem, idx));
        }
    }

    /// save a restore point
    fn set_restore_point(&mut self) {
        let len = self.removed.len();
        self.restore_points.push(len);
    }

    /// revert subset to last restore point
    fn restore(&mut self) {
        if let Some(target) = self.restore_points.pop() {
            while self.removed.len() > target {
                let (elem, idx) = self.removed.pop().unwrap();
                self.add(elem, idx);
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
