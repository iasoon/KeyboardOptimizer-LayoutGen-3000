use std::ops::Index;

use utils::LookupTable;

use model::{TokenId, LayerId};

pub struct Lock {
    table: LookupTable<LayerId, Option<TokenId>>,
}

impl Lock {
    pub fn new(table: LookupTable<LayerId, Option<TokenId>>) -> Self {
        Lock {
            table: table,
        }
    }

    pub fn layers<'a>(&'a self) -> impl Iterator<Item = LayerId> + 'a {
        self.table.iter().filter_map(|(layer_id, value)| {
            value.map(|_| layer_id)
        })
    }

    pub fn members<'a>(&'a self) -> impl Iterator<Item = TokenId> + 'a {
        self.table.values().filter_map(|&val| val)
    }

    pub fn elems<'a>(&'a self) -> impl Iterator<Item = (LayerId, TokenId)> + 'a {
        self.table.iter().filter_map(|(layer_id, value)| {
            value.map(|token_id| (layer_id, token_id))
        })
    }

    pub fn overlaps(&self, other: &Lock) -> bool {
        self.table.iter().any(|(layer_id, value)| {
            value.is_some() && other[layer_id].is_some()
        })
    }
}

impl Index<LayerId> for Lock {
    type Output = Option<TokenId>;

    fn index<'a>(&'a self, idx: LayerId) -> &'a Option<TokenId> {
        &self.table[idx]
    }
}

define_id!(Lock, LockId);
