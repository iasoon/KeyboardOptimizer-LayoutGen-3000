use model::{FreeId, LockId};

pub enum Group {
    Free(FreeId),
    Locked(LockId),
}

define_id!(Group, GroupId);
