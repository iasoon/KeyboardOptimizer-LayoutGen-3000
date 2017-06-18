use cat::{Domain, FiniteDomain, Table};

pub struct KbDef {
    pub keys: Table<Key, String>,
    pub layers: Table<Layer, String>,
    pub tokens: Table<Token, String>,
}

pub struct Key;

impl Domain for Key {
    type Type = String;
}

impl FiniteDomain for Key {}

pub struct Layer;

impl Domain for Layer {
    type Type = String;
}

impl FiniteDomain for Layer {}


pub struct Token;

impl Domain for Token {
    type Type = String;
}

impl FiniteDomain for Token {}
