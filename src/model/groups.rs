use model::*;
use utils::{BoundedSet, LookupTable};
use std::ops::Index;

pub struct Groups {
    pub groups: BoundedSet<Group>,
    pub locks: BoundedSet<Lock>,
    pub frees: BoundedSet<Free>,
    pub token_group: LookupTable<TokenId, GroupId>,
}

impl Index<GroupId> for Groups {
    type Output = Group;

    fn index<'a>(&'a self, idx: GroupId) -> &'a Group {
        &self.groups[idx]
    }
}

impl Index<LockId> for Groups {
    type Output = Lock;

    fn index<'a>(&'a self, idx: LockId) -> &'a Lock {
        &self.locks[idx]
    }
}

impl Index<FreeId> for Groups {
    type Output = Free;

    fn index<'a>(&'a self, idx: FreeId) -> &'a Free {
        &self.frees[idx]
    }
}
