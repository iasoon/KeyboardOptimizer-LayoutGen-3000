use cat::{Domain, FiniteDomain, Table};
use data::types::*;

pub struct KbDef {
    pub keys: Table<Key, String>,
    pub layers: Table<Layer, String>,
    pub tokens: Table<Token, String>,
}

