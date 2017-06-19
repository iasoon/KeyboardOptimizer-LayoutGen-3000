use cat::{Domain, FiniteDomain, Table, Num};
use data::types::*;

pub struct KbDef {
    pub keys: Table<Key, String>,
    pub layers: Table<Layer, String>,
    pub tokens: Table<Token, String>,

    pub frees: Table<Free, Num<Token>>,
    pub locks: Table<Lock, Table<Layer, Option<Num<Token>>>>,
}
