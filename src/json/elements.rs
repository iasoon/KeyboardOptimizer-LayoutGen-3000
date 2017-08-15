use cat::Table;
use data::{Key, Layer, Token};

use json::errors::*;

pub struct Elements {
    pub keys: Table<Key, String>,
    pub layers: Table<Layer, String>,
    pub tokens: Table<Token, String>,
}


#[derive(Deserialize)]
pub struct ElementsData {
    keys: Vec<String>,
    layers: Vec<String>,
    tokens: Vec<String>,
}

impl ElementsData {
    pub fn read(self) -> Result<Elements> {
        Ok(Elements {
            keys: Table::from_vec(self.keys),
            layers: Table::from_vec(self.layers),
            tokens: Table::from_vec(self.tokens),
        })
    }
}
