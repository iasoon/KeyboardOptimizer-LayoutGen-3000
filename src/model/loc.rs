use utils::Countable;
use model::KeyId;

// a location on the keyboard.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Loc {
    pub key_id: KeyId,
    pub layer_id: LayerId,
}


impl KeyId {
    pub fn layer(self, layer_id: LayerId) -> Loc {
        Loc {
            key_id: self,
            layer_id: layer_id,
        }
    }

    pub fn layer_num(self, layer_num: usize) -> Loc {
        self.layer(LayerId(layer_num))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LayerId(pub usize);

impl LayerId {
    pub fn key(self, key_id: KeyId) -> Loc {
        Loc {
            key_id: key_id,
            layer_id: self,
        }
    }
}
