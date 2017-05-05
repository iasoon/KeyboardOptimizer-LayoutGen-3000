use utils::{Countable, LookupTable, BoundedSubset, SubsetCursor};
use model::{KbDef, TokenId, LockId, FreeId, KeyId, Loc};

use rand::{thread_rng, Rng};
use std::vec::Vec;

type Keymap = LookupTable<Loc, Option<TokenId>>;

pub struct Generator<'a> {
    kb_def: &'a KbDef,
    stack: Vec<Step>,
    unassigned: Vec<Unassigned>,
    pub keymap: Keymap,
}

fn mk_unassigned(kb_def: &KbDef) -> Vec<Unassigned> {
    let mut rng = thread_rng();
    LockId::enumerate(kb_def.locks.elem_count())
        .map(|lock_id| {
            let mut keys: Vec<KeyId> = KeyId::enumerate(kb_def.keys.elem_count()).collect();
            rng.shuffle(keys.as_mut());
            Unassigned::Lock(UnassignedLock {
                lock_id: lock_id,
                keys: BoundedSubset::from_vec(kb_def.keys.elem_count(), keys),
            })
        })
        .collect()
}

impl<'a> Generator<'a> {
    pub fn new(kb_def: &'a KbDef) -> Self {
        Generator {
            stack: Vec::with_capacity(kb_def.groups.elem_count().count()),
            unassigned: mk_unassigned(kb_def),
            keymap: LookupTable::new(kb_def.loc_data(), None),
            kb_def: kb_def,
        }
    }

    pub fn generate(kb_def: &KbDef) -> Keymap {
        let mut gen = Generator::new(kb_def);
        gen.backtrack();
        return gen.keymap;
    }

    fn backtrack(&mut self) {
        self.descend();

        while self.next() && self.unassigned.len() > 0 {
            self.assign();
            self.descend();
        }

        if self.unassigned.len() > 0 {
            panic!("Something went wrong");
        }

        self.assign();
    }

    fn descend(&mut self) {
        // TODO: pick smarter
        if let Some(unassigned) = self.unassigned.pop() {
            self.stack.push(unassigned.to_step());
        }
    }

    fn next(&mut self) -> bool {
        while self.stack.len() > 0 {
            let mut step = self.stack.pop().unwrap();
            step.unassign(self);

            while step.next() {
                if step.valid(self) {
                    self.stack.push(step);
                    return true;
                }
            }

            // ascend
            self.unassigned.push(step.to_unassigned());
        }
        return false;
    }

    fn assign(&mut self) {
        let step = self.stack.pop().unwrap();
        step.assign(self);
        self.stack.push(step);
    }

    // perform a move in the backtracking tree
    fn do_move<A: Move>(&mut self, action: A) {
        action.keymap(&self.kb_def, &mut self.keymap);

        for unassigned in self.unassigned.iter_mut() {
            match unassigned {
                &mut Unassigned::Lock(ref mut unassigned_lock) => {
                    action.unassigned_lock(&self.kb_def, unassigned_lock);
                }
                &mut Unassigned::Free(_) => unimplemented!(),
            }
        }
    }
}

enum Step {
    Lock {
        id: LockId,
        cursor: SubsetCursor<KeyId>,
    },
    Free {
        id: FreeId,
        cursor: SubsetCursor<Loc>,
    },
}

impl Step {
    fn unassign(&self, gen: &mut Generator) {
        match self {
            &Step::Lock { id, ref cursor } => {
                if let Some(pos) = cursor.pos() {
                    gen.do_move(UnassignLock {
                        lock_id: id,
                        key_id: pos,
                    });
                }
            }
            &Step::Free { id, ref cursor } => {
                unimplemented!();
            }
        }
    }

    fn valid(&self, gen: &Generator) -> bool {
        match self {
            &Step::Lock { id, ref cursor } => {
                cursor.pos()
                    .map_or(false, |pos| {
                        gen.kb_def.locks[id].elems().all(|(layer_id, _)| {
                            let loc = gen.kb_def.loc_data().loc(cursor.pos().unwrap(), layer_id);
                            return gen.keymap[loc].is_none();
                        })
                    })
            }
            &Step::Free { id, ref cursor } => unimplemented!(),
        }
    }

    fn assign(&self, gen: &mut Generator) {
        match self {
            &Step::Lock { id, ref cursor } => {
                gen.do_move(AssignLock {
                    lock_id: id,
                    key_id: cursor.pos().unwrap(),
                });
            }
            &Step::Free { id, ref cursor } => unimplemented!(),
        }
    }

    fn next(&mut self) -> bool {
        match self {
            &mut Step::Lock { id, ref mut cursor } => cursor.next(),
            &mut Step::Free { id, ref mut cursor } => cursor.next(),
        }
    }

    fn to_unassigned(self) -> Unassigned {
        match self {
            Step::Lock { id, cursor } => {
                Unassigned::Lock(UnassignedLock {
                    lock_id: id,
                    keys: cursor.subset,
                })
            }
            Step::Free { id, cursor } => {
                Unassigned::Free(UnassignedFree {
                    free_id: id,
                    locs: cursor.subset,
                })
            }
        }
    }
}


trait Move {
    fn keymap(&self, kb_def: &KbDef, keymap: &mut Keymap) {}
    fn unassigned_lock(&self, kb_def: &KbDef, unassigned: &mut UnassignedLock) {}
}

struct AssignLock {
    lock_id: LockId,
    key_id: KeyId,
}

impl Move for AssignLock {
    fn keymap(&self, kb_def: &KbDef, keymap: &mut Keymap) {
        for (layer_id, token_id) in kb_def.locks[self.lock_id].elems() {
            let loc = kb_def.loc_data().loc(self.key_id, layer_id);
            keymap[loc] = Some(token_id);
        }
    }

    fn unassigned_lock(&self, kb_def: &KbDef, unassigned: &mut UnassignedLock) {
        if kb_def.locks[self.lock_id].overlaps(&kb_def.locks[unassigned.lock_id]) {
            unassigned.keys.remove(self.key_id);
        }
    }
}

struct UnassignLock {
    lock_id: LockId,
    key_id: KeyId,
}

impl Move for UnassignLock {
    fn keymap(&self, kb_def: &KbDef, keymap: &mut Keymap) {
        for (layer_id, token_id) in kb_def.locks[self.lock_id].elems() {
            let loc = kb_def.loc_data().loc(self.key_id, layer_id);
            keymap[loc] = None;
        }
    }

    fn unassigned_lock(&self, kb_def: &KbDef, unassigned: &mut UnassignedLock) {
        if kb_def.locks[self.lock_id].overlaps(&kb_def.locks[unassigned.lock_id]) {
            unassigned.keys.add(self.key_id);
        }
    }
}

enum Unassigned {
    Lock(UnassignedLock),
    Free(UnassignedFree),
}

impl Unassigned {
    fn to_step(self) -> Step {
        match self {
            Unassigned::Lock(unassigned_lock) => {
                Step::Lock {
                    id: unassigned_lock.lock_id,
                    cursor: unassigned_lock.keys.cursor(),
                }
            }
            Unassigned::Free(unassigned_free) => {
                Step::Free {
                    id: unassigned_free.free_id,
                    cursor: unassigned_free.locs.cursor(),
                }
            }
        }
    }
}


struct UnassignedLock {
    lock_id: LockId,
    keys: BoundedSubset<KeyId>,
}

struct UnassignedFree {
    free_id: FreeId,
    locs: BoundedSubset<Loc>,
}
