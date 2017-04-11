use std::vec::Vec;
use model::{GroupId, KeyId};

pub struct KeyMasks {
    vec: Vec<bool>,
    num_keys: usize,
}

impl KeyMasks {
    pub fn empty(num_groups: usize, num_keys: usize) -> Self {
        KeyMasks {
            vec: vec![false; num_groups * num_keys],
            num_keys: num_keys,
        }
    }

    pub fn build<F>(num_groups: usize, num_keys: usize, fun: F) -> Self
        where F: Fn(GroupId, KeyId) -> bool
    {
        let mut key_masks = KeyMasks::empty(num_groups, num_keys);
        for group_id in (0..num_groups).map(|n| GroupId(n)) {
            for key_id in (0..num_keys).map(|n| KeyId(n)) {
                key_masks.set(group_id, key_id, fun(group_id, key_id));
            }
        }
        return key_masks;
    }

    fn calc_idx(&self, group_id: GroupId, key_id: KeyId) -> usize {
        let KeyId(key_num) = key_id;
        let GroupId(group_num) = group_id;
        return group_num * self.num_keys + key_num;
    }

    pub fn get(&self, group_id: GroupId, key_id: KeyId) -> bool {
        self.vec[self.calc_idx(group_id, key_id)]
    }

    pub fn set<'a>(&mut self, group_id: GroupId, key_id: KeyId, value: bool) {
        let idx = self.calc_idx(group_id, key_id);
        self.vec[idx] = value;
    }
}
