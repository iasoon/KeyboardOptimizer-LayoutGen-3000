use std::path::Path;
use serde::Deserialize;
use utils::json;
use errors::*;

pub trait Parser<T> {
    type Repr;

    fn parse(&self, repr: &Self::Repr) -> Result<T>;

    fn read_json(&self, path: &Path) -> Result<T>
        where Self::Repr: Deserialize
    {
        self.parse(&json::read(path)?)
    }
}
