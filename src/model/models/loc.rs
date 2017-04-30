use model::{Key, KeyId, Layer, LayerId};
use utils::{Countable, ElemCount};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loc(usize);

pub struct LocData {
    pub key_data: ElemCount<Key>,
    pub layer_data: ElemCount<Layer>,
}

impl Loc {
    pub fn new(data: &LocData, key_id: KeyId, layer_id: LayerId) -> Loc {
        let key_num = key_id.to_num(&data.key_data);
        let layer_num = layer_id.to_num(&data.layer_data);
        let num_keys = KeyId::count(&data.key_data);
        Loc(layer_num * num_keys + key_num)
    }

    pub fn key(&self, data: &LocData) -> KeyId {
        let num = self.to_num(data);
        KeyId::from_num(&data.key_data, num % data.key_data.count())
    }
}

impl Countable for Loc {
    type Data = LocData;

    fn from_num(_: &LocData, num: usize) -> Loc {
        Loc(num)
    }

    fn to_num(&self, _: &LocData) -> usize {
        let &Loc(num) = self;
        num
    }

    fn count(data: &LocData) -> usize {
        KeyId::count(&data.key_data) * LayerId::count(&data.layer_data)
    }
}
