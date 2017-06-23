use serde::Deserialize;

use data::KbDef;

use json::errors::*;
use json::config_reader::ConfigReader;
use json::elements::ElementsData;
use json::groups::{GroupsData};

#[derive(Deserialize)]
pub struct ConfigData<'a> {
    elements: ElementsData,
    #[serde(borrow)]
    groups: GroupsData<'a>,
}

impl<'a> ConfigData<'a> {
    pub fn read(self) -> Result<KbDef> {
        let elements = self.elements.read()?;
        let groups;
        {
            // use a new syntactic block here to contain this borrow
            let reader = ConfigReader::new(&elements);
            groups = self.groups.read(&reader)?
        }

        Ok(KbDef {
            keys: elements.keys,
            layers: elements.layers,
            tokens: elements.tokens,

            frees: groups.frees,
            locks: groups.locks,
            assignments: groups.assignments,
        })
    }
}
