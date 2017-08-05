use data::KbDef;
use cat;
use cat::*;
use std::ops::{Index, IndexMut};

/// A key on the keyboard.
pub struct Key;

/// A layer on the keyboard, e.g. 'default', 'shift', 'alt', ...
pub struct Layer;

/// A token that can be assigned to a location, e.g. letters of the alphabet.
pub struct Token;

/// A location on the keyboard, determined by a key and a layer.
pub struct Loc {
    pub key_num: Num<Key>,
    pub layer_num: Num<Layer>,
}

pub struct LocNum {
    pub key_count: Count<Key>,
    pub layer_count: Count<Layer>,
}

impl LocNum {
    fn as_product(&self) -> ProductNum<Layer, Key> {
        ProductNum {
            major_count: self.layer_count,
            minor_count: self.key_count,
        }
    }
}

impl HasCount<Loc> for LocNum {
    fn count(&self) -> Count<Loc> {
        cat::internal::to_count(self.as_product().count().as_usize())
    }
}

impl Mapping<Loc> for LocNum {
    type Result = Num<Loc>;

    fn apply(&self, loc: Loc) -> Num<Loc> {
        let num = self.as_product().apply((loc.layer_num, loc.key_num));
        return cat::internal::to_num(num.as_usize());
    }
}

impl Mapping<Num<Loc>> for LocNum {
    type Result = Loc;
    fn apply(&self, num: Num<Loc>) -> Loc {
        let prod_num = cat::internal::to_num(num.as_usize());
        let (layer_num, key_num) = self.as_product().apply(prod_num);
        return Loc {
            key_num: cat::internal::to_num(key_num.as_usize()),
            layer_num: cat::internal::to_num(layer_num.as_usize()),
        }
    }
}


/// A token that can be moved freely.
/// A free token has two degrees of freedom: which key and which layer it is
/// placed on.
pub struct Free;

/// A group of tokens that is locked together.
/// Each token of the group is uniquely assigned to a layer, so that a 'locked'
/// token only has one degree of freedom: which key the group is assigned to.
/// The tokens in the locked group will always be assigned to the same key.
/// This is useful for requiring lowercase and uppercase letters to be on the
/// same key.
pub struct Lock;

/// Union type for free and locked groups.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Group {
    Free(Num<Free>),
    Lock(Num<Lock>),
}

pub struct GroupNum {
    pub free_count: Count<Free>,
    pub lock_count: Count<Lock>,
}

impl Mapping<Num<Group>> for GroupNum {
    type Result = Group;
    fn apply(&self, num: Num<Group>) -> Group {
        if num.as_usize() < self.free_count.as_usize() {
            Group::Free(cat::internal::to_num(num.as_usize()))
        } else {
            let lock_num = num.as_usize() - self.free_count.as_usize();
            Group::Lock(cat::internal::to_num(lock_num))
        }
    }
}

impl Mapping<Group> for GroupNum {
    type Result = Num<Group>;

    fn apply(&self, group: Group) -> Num<Group> {
        match group {
            Group::Free(free_num) => {
                cat::internal::to_num(free_num.as_usize())
            },
            Group::Lock(lock_num) => {
                let num = self.free_count.as_usize() + lock_num.as_usize();
                cat::internal::to_num(num)
            }
        }
    }
}

impl Mapping<Num<Free>> for GroupNum {
    type Result = Num<Group>;

    fn apply(&self, free_num: Num<Free>) -> Num<Group> {
        self.apply(Group::Free(free_num))
    }
}

impl Mapping<Num<Lock>> for GroupNum {
    type Result = Num<Group>;

    fn apply(&self, lock_num: Num<Lock>) -> Num<Group> {
        self.apply(Group::Lock(lock_num))
    }
}

impl HasCount<Group> for GroupNum {
    fn count(&self) -> Count<Group> {
        let count = self.free_count.as_usize() + self.lock_count.as_usize();
        return cat::internal::to_count(count);
    }
}


/// An assignment either assigns a free token to a location, or a locked group
/// to a key.
#[derive(Debug, Copy, Clone)]
pub enum Assignment {
    Free {
        free_num: Num<Free>,
        loc_num: Num<Loc>,
    },
    Lock {
        lock_num: Num<Lock>,
        key_num: Num<Key>,
    }
}

impl Assignment {
    pub fn group(&self) -> Group {
        match *self {
            Assignment::Free { free_num, loc_num: _ } => {
                Group::Free(free_num)
            },
            Assignment::Lock { lock_num, key_num: _ } => {
                Group::Lock(lock_num)
            }
        }
    }

    pub fn group_num(&self, kb_def: &KbDef) -> Num<Group> {
        kb_def.group_num().apply(self.group())
    }
}

pub struct AssignmentNum {
    pub free_count: Count<Free>,
    pub lock_count: Count<Lock>,
    pub key_count: Count<Key>,
    pub layer_count: Count<Layer>,
}

impl AssignmentNum {
    fn loc_num(&self) -> LocNum {
        LocNum {
            key_count: self.key_count,
            layer_count: self.layer_count,
        }
    }

    fn free_product(&self) -> ProductNum<Free, Loc> {
        ProductNum {
            major_count: self.free_count,
            minor_count: self.loc_num().count(),
        }
    }

    fn lock_product(&self) -> ProductNum<Lock, Key> {
        ProductNum {
            major_count: self.lock_count,
            minor_count: self.key_count,
        }
    }
}

impl HasCount<Assignment> for AssignmentNum {
    fn count(&self) -> Count<Assignment> {
        let free_count = self.free_product().count().as_usize();
        let lock_count = self.lock_product().count().as_usize();
        return cat::internal::to_count(free_count + lock_count);
    }
}

impl Mapping<Assignment> for AssignmentNum {
    type Result = Num<Assignment>;

    fn apply(&self, assignment: Assignment) -> Num<Assignment> {
        match assignment {
            Assignment::Free { free_num, loc_num } => {
                let num = self.free_product().apply((free_num, loc_num));
                cat::internal::to_num(num.as_usize())
            },
            Assignment::Lock { lock_num, key_num } => {
                let num = self.lock_product().apply((lock_num, key_num));
                let offset = self.free_product().count();
                cat::internal::to_num(offset.as_usize() + num.as_usize())
            }
        }
    }
}

/// Marker type for disambiguating between possible and allowed assignments.
pub struct AllowedAssignment;

pub struct AssignmentTable<'a, T> {
    kb_def: &'a KbDef,
    table: Table<AllowedAssignment, T>,
}

impl<'a, T> AssignmentTable<'a, T> {
    pub fn new<F>(kb_def: &'a KbDef, fun: F) -> Self
        where F: FnMut((Num<AllowedAssignment>, &Assignment)) -> T
    {
        let values = kb_def.assignments.enumerate().map(fun).collect();
        AssignmentTable {
            table: Table::from_vec(values),
            kb_def: kb_def,
        }
    }
}

impl<'a, T> Index<Assignment> for AssignmentTable<'a, T> {
    type Output = T;

    fn index<'t>(&'t self, assignment: Assignment) -> &'t T {
        let assignment_num = self.kb_def.assignment_map[assignment].unwrap();
        return &self.table[assignment_num];
    }
}

impl<'a, T> IndexMut<Assignment> for AssignmentTable<'a, T> {
    fn index_mut<'t>(&'t mut self, assignment: Assignment) -> &'t mut T {
        let assignment_num = self.kb_def.assignment_map[assignment].unwrap();
        return &mut self.table[assignment_num];
    }
}
