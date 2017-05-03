mod kb_conf;
mod groups;

use errors::*;
use utils::json;
use model::KbDef;

use self::kb_conf::KbConf;
pub use self::kb_conf::KbReader;
use self::groups::{Groups, LockRepr};

use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct KbDefData {
    pub keys: Vec<String>,
    pub layers: Vec<String>,
    pub tokens: Vec<String>,

    pub locks: Vec<LockRepr>,
}

impl KbDefData {
    pub fn read(self) -> Result<KbDef> {
        let kb_conf = KbConf::build(self.keys, self.layers, self.tokens);
        let groups = Groups::read(&kb_conf.reader(), &self.locks)?;

        Ok(KbDef {
            keys: kb_conf.keys,
            layers: kb_conf.layers,
            tokens: kb_conf.tokens,

            groups: groups.groups,
            locks: groups.locks,
            frees: groups.frees,

            token_group: groups.token_group,
            free_group: groups.free_group,
            lock_group: groups.lock_group,
        })
    }

    pub fn read_from_path(path: &Path) -> Result<KbDef> {
        let data: KbDefData = json::read(path)?;
        return data.read();
    }
}
