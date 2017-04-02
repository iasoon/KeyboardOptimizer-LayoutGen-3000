use std::collections::HashMap;
use std::vec::Vec;

pub type Lock = HashMap<String, String>;
pub type Locks = Vec<Lock>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LtConf {
    pub locks: Locks,
}
