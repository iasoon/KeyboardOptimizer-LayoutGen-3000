use errors::*;

use std::vec::Vec;
use std::ops::{Index, IndexMut};

use parser::lt_conf::{Parser, LtParser};
use model::{Groups, KeyMasks, Group, GroupId, TokenId, KeyId};
use data::lt_conf::{Mask as MaskData, KeyMasks as MasksData};

impl<'a> Parser<KeyMasks> for LtParser<'a> {
    type Repr = MasksData;

    fn parse(&self, repr: &MasksData) -> Result<KeyMasks> {
        let mask_set: MaskSet = self.parse(repr)?;
        Ok(mask_set.to_key_masks())
    }
}

impl<'a> Parser<Mask> for LtParser<'a> {
    type Repr = MaskData;

    fn parse(&self, repr: &MaskData) -> Result<Mask> {
        let mut mask = vec![false; self.num_keys()];
        for key_name in repr.iter() {
            let KeyId(key_num) = self.parse(key_name)?;
            mask[key_num] = true;
        }
        Ok(mask)
    }
}

impl<'a> Parser<MaskSet<'a>> for LtParser<'a> {
    type Repr = MasksData;

    fn parse(&self, repr: &MasksData) -> Result<MaskSet<'a>> {
        let mut mask_set = MaskSet::new(&self.groups, self.num_keys());
        for (token_name, mask_data) in repr.iter() {
            let token_id = self.parse(token_name)?;
            let mask = self.parse(mask_data)?;
            mask_set.set_mask(token_id, mask);
        }
        Ok(mask_set)
    }
}

type Mask = Vec<bool>;

struct MaskSet<'a> {
    masks: Vec<Option<Mask>>,
    groups: &'a Groups,
    num_keys: usize,
}

impl<'a> MaskSet<'a> {
    fn new(groups: &'a Groups, num_keys: usize) -> Self {
        MaskSet {
            masks: vec![None; groups.token_group.len()],
            groups: groups,
            num_keys: num_keys,
        }
    }

    fn set_mask(&mut self, token_id: TokenId, mask: Mask) {
        let TokenId(token_num) = token_id;
        self.masks[token_num] = Some(mask);
    }

    fn group_value(&self, group_id: GroupId, key_id: KeyId) -> bool {
        match self.groups[group_id] {
            Group::Free(free_id) => {
                self.token_value(self.groups[free_id], key_id)
            },
            Group::Locked(lock_id) => {
                self.groups[lock_id].members().all(|token_id| {
                    self.token_value(token_id, key_id)
                })
            }
        }
    }

    fn token_value(&self, token_id: TokenId, key_id: KeyId) -> bool {
        let TokenId(token_num) = token_id;
        let KeyId(key_num) = key_id;
        self.masks[token_num].as_ref().map_or(false, |mask| mask[key_num])
    }

    fn to_key_masks(&self) -> KeyMasks {
        KeyMasks::build(self.groups.groups.len(), self.num_keys, |group_id, key_id| {
            self.group_value(group_id, key_id)
        })
    }
}
