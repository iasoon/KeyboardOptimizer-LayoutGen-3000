use errors::*;
use parser::{Parser, KbParser};
use data::lt_conf::{Lock as LockData, Locks as LocksData};
use model::*;
use utils::{BoundedSet, LookupTable, ElemCount, Countable};

impl<'a> Parser<Groups> for KbParser<'a> {
    type Repr = LocksData;

    fn parse(&self, repr: &LocksData) -> Result<Groups> {
        let mut lock_vec = Vec::new();
        for lock_data in repr.iter() {
            lock_vec.push(self.parse(lock_data)?);
        }

        let locks = Locks::new(self.kb_conf.tokens.elem_count(), lock_vec);
        Ok(locks.mk_groups())
    }
}

impl<'a> Parser<Lock> for KbParser<'a> {
    type Repr = LockData;

    fn parse(&self, repr: &LockData) -> Result<Lock> {
        let mut lock = LookupTable::new(self.kb_conf.layers.elem_count(), None);
        for (layer_name, token_name) in repr.iter() {
            let layer_id = self.parse(layer_name)?;
            let token_id = self.parse(token_name)?;
            lock[layer_id] = Some(token_id);
        }
        Ok(Lock::new(lock))
    }
}

struct Locks {
    elems: BoundedSet<Lock>,
    token_lock: LookupTable<TokenId, Option<LockId>>,
}

impl Locks {
    fn new(token_count: ElemCount<Token>, lock_vec: Vec<Lock>) -> Self {
        let locks = BoundedSet::new(lock_vec);

        let mut token_lock = LookupTable::new(token_count, None);
        for lock_id in LockId::enumerate(&locks.elem_count()) {
            for token_id in locks[lock_id].members() {
                token_lock[token_id] = Some(lock_id);
            }
        }

        Locks {
            elems: locks,
            token_lock: token_lock,
        }
    }

    fn mk_groups(self) -> Groups {
        let mut builder = self.mk_groups_builder();
        builder.visit_tokens();
        builder.mk_groups()
    }

    fn mk_groups_builder(self) -> GroupsBuilder {
        let frees = self.mk_frees();
        let free_count = frees.elem_count();
        let lock_count = self.elems.elem_count();

        let locked_groups = LockId::enumerate(&lock_count)
            .map(|lock_id| Group::Locked(lock_id));
        let free_groups = FreeId::enumerate(&free_count)
            .map(|free_id| Group::Free(free_id));

        GroupsBuilder {
            groups: BoundedSet::new(locked_groups.chain(free_groups).collect()),
            locks: self.elems,
            frees: frees,
            token_group: LookupTable::new(self.token_lock.data().clone(), None),
        }
    }

    fn mk_frees(&self) -> BoundedSet<Free> {
        let frees = self.token_lock.iter()
            .filter(|&(_, value)| value.is_none())
            .map(|(token_id, _)| Free {token_id: token_id})
            .collect();
        BoundedSet::new(frees)
    }

}

struct GroupsBuilder {
    groups: BoundedSet<Group>,
    locks: BoundedSet<Lock>,
    frees: BoundedSet<Free>,
    token_group: LookupTable<TokenId, Option<GroupId>>,
}

impl GroupsBuilder {
    fn mk_groups(self) -> Groups {
        Groups {
            groups: self.groups,
            locks: self.locks,
            frees: self.frees,
            token_group: self.token_group.drain_map(|val| val.unwrap()),
        }
    }

    fn visit_tokens(&mut self) {
        for group_id in GroupId::enumerate(&self.groups.elem_count()) {
            match self.groups[group_id] {
                Group::Free(free_id) => self.visit_free(group_id, free_id),
                Group::Locked(lock_id) => self.visit_lock(group_id, lock_id),
            }
        }
    }

    fn visit_free(&mut self, group_id: GroupId, free_id: FreeId) {
        let token_id = self.frees[free_id].token_id;
        self.token_group[token_id] = Some(group_id);
    }

    fn visit_lock(&mut self, group_id: GroupId, lock_id: LockId) {
        for token_id in self.locks[lock_id].members() {
            self.token_group[token_id] = Some(group_id);
        }
    }
}
