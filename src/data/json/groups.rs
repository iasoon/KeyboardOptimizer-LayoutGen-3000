use errors::*;
use model::*;
use utils::{BoundedSet, LookupTable, ElemCount, Countable};
use data::json::kb_conf::KbReader;

use std::collections::HashMap;

pub type LockRepr = HashMap<String, String>;

pub struct Groups {
    pub groups: BoundedSet<Group>,
    pub locks: BoundedSet<Lock>,
    pub frees: BoundedSet<Free>,

    pub token_group: LookupTable<TokenId, GroupId>,
    pub free_group: LookupTable<FreeId, GroupId>,
    pub lock_group: LookupTable<LockId, GroupId>,
}

impl Groups {
    pub fn read<'a>(kb_reader: &'a KbReader<'a>, repr: &Vec<LockRepr>) -> Result<Self> {
        let reader = GroupsReader { kb_reader: kb_reader };
        reader.read_locks(repr)
    }
}

struct GroupsReader<'a> {
    kb_reader: &'a KbReader<'a>,
}

impl<'a> GroupsReader<'a> {
    fn read_locks(&self, repr: &Vec<LockRepr>) -> Result<Groups> {
        let mut lock_vec = Vec::new();

        for lock_data in repr.iter() {
            lock_vec.push(self.read_lock(lock_data)?);
        }

        let token_count = self.kb_reader.tokens.elem_count();
        let locks = Locks::new(token_count, lock_vec);
        Ok(locks.mk_groups())
    }

    pub fn read_lock(&self, repr: &LockRepr) -> Result<Lock> {
        let layer_count = self.kb_reader.layers.elem_count();
        let mut lock = LookupTable::new(layer_count, None);
        for (layer_name, token_name) in repr.iter() {
            let layer_id = self.kb_reader.read_layer(layer_name)?;
            let token_id = self.kb_reader.read_token(token_name)?;
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
        for lock_id in LockId::enumerate(locks.elem_count()) {
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

        let locked_groups = LockId::enumerate(self.elems.elem_count())
            .map(|lock_id| Group::Locked(lock_id));
        let free_groups = FreeId::enumerate(frees.elem_count())
            .map(|free_id| Group::Free(free_id));

        GroupsBuilder {
            token_group: LookupTable::new(self.token_lock.data().clone(), None),
            free_group: LookupTable::new(frees.elem_count(), None),
            lock_group: LookupTable::new(self.elems.elem_count().clone(), None),

            groups: BoundedSet::new(locked_groups.chain(free_groups).collect()),
            locks: self.elems,
            frees: frees,
        }
    }

    fn mk_frees(&self) -> BoundedSet<Free> {
        let frees = self.token_lock
            .iter()
            .filter(|&(_, value)| value.is_none())
            .map(|(token_id, _)| Free { token_id: token_id })
            .collect();
        BoundedSet::new(frees)
    }
}

struct GroupsBuilder {
    groups: BoundedSet<Group>,
    locks: BoundedSet<Lock>,
    frees: BoundedSet<Free>,

    token_group: LookupTable<TokenId, Option<GroupId>>,
    free_group: LookupTable<FreeId, Option<GroupId>>,
    lock_group: LookupTable<LockId, Option<GroupId>>,
}

impl GroupsBuilder {
    fn mk_groups(self) -> Groups {
        Groups {
            groups: self.groups,
            locks: self.locks,
            frees: self.frees,

            token_group: self.token_group.drain_map(|val| val.unwrap()),
            free_group: self.free_group.drain_map(|val| val.unwrap()),
            lock_group: self.lock_group.drain_map(|val| val.unwrap()),
        }
    }

    fn visit_tokens(&mut self) {
        for group_id in GroupId::enumerate(self.groups.elem_count()) {
            match self.groups[group_id] {
                Group::Free(free_id) => self.visit_free(group_id, free_id),
                Group::Locked(lock_id) => self.visit_lock(group_id, lock_id),
            }
        }
    }

    fn visit_free(&mut self, group_id: GroupId, free_id: FreeId) {
        self.free_group[free_id] = Some(group_id);
        let token_id = self.frees[free_id].token_id;
        self.token_group[token_id] = Some(group_id);
    }

    fn visit_lock(&mut self, group_id: GroupId, lock_id: LockId) {
        self.lock_group[lock_id] = Some(group_id);
        for token_id in self.locks[lock_id].members() {
            self.token_group[token_id] = Some(group_id);
        }
    }
}
