#[derive(Debug, Clone, Copy)]
pub struct TokenId(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct LockId(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct FreeId(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct GroupId(pub usize);

#[derive(Debug, Copy, Clone)]
pub enum Group {
    Free(FreeId),
    Locked(LockId),
}

impl Group {
    pub fn free_id(self) -> Option<FreeId> {
        match self {
            Group::Free(free_id) => Some(free_id),
            _ => None,
        }
    }

    pub fn lock_id(self) -> Option<LockId> {
        match self {
            Group::Locked(lock_id) => Some(lock_id),
            _ => None,
        }
    }
}
