use cat::Table;
use data::{Key, Value};

use json::errors::*;

pub struct Elements {
    pub keys: Table<Key, String>,
    pub values: Table<Value, String>,
}


#[derive(Deserialize)]
pub struct ElementsData {
    keys: Vec<String>,
    values: Vec<String>,
}

impl ElementsData {
    pub fn read(self) -> Result<Elements> {
        Ok(Elements {
            keys: Table::from_vec(self.keys),
            values: Table::from_vec(self.values),
        })
    }
}
