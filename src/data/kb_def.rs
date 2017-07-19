use cat::*;
use data::types::*;

type AssignmentTable<T> = Composed<AssignmentNum, Table<Assignment, T>>;
pub struct KbDef {
    pub keys: Table<Key, String>,
    pub layers: Table<Layer, String>,
    pub tokens: Table<Token, String>,

    pub frees: Table<Free, Num<Token>>,
    pub locks: Table<Lock, Table<Layer, Option<Num<Token>>>>,

    pub assignments: Table<AllowedAssignment, Assignment>,

    pub token_group: Table<Token, Group>,
    pub assignment_map: AssignmentTable<Option<Num<AllowedAssignment>>>,
}

impl KbDef {
    pub fn loc_num(&self) -> LocNum {
        LocNum {
            key_count: self.keys.count(),
            layer_count: self.layers.count(),
        }
    }

    pub fn group_num(&self) -> GroupNum {
        GroupNum {
            free_count: self.frees.count(),
            lock_count: self.locks.count(),
        }
    }

    pub fn assignment_num(&self) -> AssignmentNum {
        AssignmentNum {
            free_count: self.frees.count(),
            lock_count: self.locks.count(),
            key_count: self.keys.count(),
            layer_count: self.layers.count(),
        }
    }
}
