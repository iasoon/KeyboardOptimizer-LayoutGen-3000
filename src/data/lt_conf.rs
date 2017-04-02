use std::collections::HashMap;
use std::vec::Vec;

pub type Lock = HashMap<String, String>;
pub type Locks = Vec<Lock>;

pub type Mask = Vec<String>;
pub type KeyMasks = HashMap<String, Mask>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LtConf {
    pub locks: Locks,
    pub key_masks: KeyMasks,
}
