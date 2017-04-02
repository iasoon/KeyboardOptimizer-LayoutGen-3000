use std::vec::Vec;
use std::ops::Index;

use model::*;

#[derive(Debug)]
pub struct Groups {
    pub token_group: Vec<GroupId>,
    pub groups: Vec<Group>,
    pub locks: Vec<Lock>,
    pub frees: Vec<TokenId>,
}

impl Index<TokenId> for Groups {
    type Output = GroupId;

    fn index<'a>(&'a self, idx: TokenId) -> &'a GroupId {
        let TokenId(token_num) = idx;
        return &self.token_group[token_num];
    }
}

impl Index<GroupId> for Groups {
    type Output = Group;

    fn index<'a>(&'a self, idx: GroupId) -> &'a Group {
        let GroupId(group_num) = idx;
        return &self.groups[group_num];
    }
}

impl Index<LockId> for Groups {
    type Output = Lock;

    fn index<'a>(&'a self, idx: LockId) -> &'a Lock {
        let LockId(lock_num) = idx;
        return &self.locks[lock_num];
    }
}

impl Index<FreeId> for Groups {
    type Output = TokenId;

    fn index<'a>(&'a self, idx: FreeId) -> &'a TokenId {
        let FreeId(free_num) = idx;
        return &self.frees[free_num];
    }
}
