use std::vec::Vec;
use std::ops::Index;

use model::{TokenId, LayerId};

#[derive(Debug)]
pub struct Lock {
    vec: Vec<Option<TokenId>>,
}

impl Lock {
    pub fn new(layer_map: Vec<Option<TokenId>>) -> Self {
        Lock {
            vec: layer_map,
        }
    }

    pub fn members<'a>(&'a self) -> impl Iterator<Item = TokenId> + 'a {
        self.vec.iter().filter_map(|&val| val)
    }

    pub fn elems<'a>(&'a self) -> impl Iterator<Item = (LayerId, TokenId)> + 'a {
        self.vec.iter().enumerate().filter_map(|(layer_num, val)| val.map(|token_id| {
            let layer_id = LayerId(layer_num);
            return (layer_id, token_id);
        }))
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Option<TokenId>> {
        self.vec.iter()
    }
}

impl Index<LayerId> for Lock {
    type Output = Option<TokenId>;

    fn index<'a>(&'a self, idx: LayerId) -> &'a Option<TokenId> {
        let LayerId(layer_id) = idx;
        &self.vec[layer_id]
    }
}
