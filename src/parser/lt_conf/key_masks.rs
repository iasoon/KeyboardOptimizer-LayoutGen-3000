use errors::*;

use parser::lt_conf::{Parser, LtParser};
use model::{Groups, Group, GroupId, TokenId, Key, KeyId};
use data::lt_conf::{Mask as MaskData, KeyMasks as MasksData};
use utils::{LookupTable, ElemCount};


type KeyMasks = LookupTable<(GroupId, KeyId), bool>;
type Mask = LookupTable<KeyId, bool>;

impl<'a> Parser<LookupTable<(GroupId, KeyId), bool>> for LtParser<'a> {
    type Repr = MasksData;

    fn parse(&self, repr: &MasksData)
             -> Result<LookupTable<(GroupId, KeyId), bool>>
    {
        let mask_set: MaskSet = self.parse(repr)?;
        Ok(mask_set.to_key_masks())
    }
}

impl<'a> Parser<Mask> for LtParser<'a> {
    type Repr = MaskData;

    fn parse(&self, repr: &MaskData) -> Result<Mask> {
        let mut mask = LookupTable::new(self.key_count(), false);
        for key_name in repr.iter() {
            let key_id = self.parse(key_name)?;
            mask[key_id] = true;
        }
        Ok(mask)
    }
}

impl<'a> Parser<MaskSet<'a>> for LtParser<'a> {
    type Repr = MasksData;

    fn parse(&self, repr: &MasksData) -> Result<MaskSet<'a>> {
        let mut mask_set = MaskSet::new(&self.groups, self.key_count());
        for (token_name, mask_data) in repr.iter() {
            let token_id = self.parse(token_name)?;
            let mask = self.parse(mask_data)?;
            mask_set.set_mask(token_id, mask);
        }
        Ok(mask_set)
    }
}

struct MaskSet<'a> {
    masks: LookupTable<TokenId, Option<Mask>>,
    groups: &'a Groups,
    key_count: ElemCount<Key>,
}

impl<'a> MaskSet<'a> {
    fn new(groups: &'a Groups, key_count: ElemCount<Key>) -> Self {
        MaskSet {
            masks: LookupTable::new(groups.token_group.data().clone(), None),
            groups: groups,
            key_count: key_count,
        }
    }

    fn set_mask(&mut self, token_id: TokenId, mask: Mask) {
        self.masks[token_id] = Some(mask);
    }

    fn group_value(&self, group_id: GroupId, key_id: KeyId) -> bool {
        match self.groups[group_id] {
            Group::Free(free_id) => {
                self.token_value(self.groups[free_id].token_id, key_id)
            },
            Group::Locked(lock_id) => {
                self.groups[lock_id].members().all(|token_id| {
                    self.token_value(token_id, key_id)
                })
            }
        }
    }

    fn token_value(&self, token_id: TokenId, key_id: KeyId) -> bool {
        self.masks[token_id].as_ref().map_or(false, |mask| mask[key_id])
    }

    fn to_key_masks(&self) -> KeyMasks {
        let data = (self.groups.groups.elem_count(), self.key_count.clone());
        LookupTable::from_fn(data, |(group_id, key_id)| {
            self.group_value(group_id, key_id)
        })
    }
}
