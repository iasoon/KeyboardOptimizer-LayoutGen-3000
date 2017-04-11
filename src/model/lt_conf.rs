use model::{Groups, GroupId, KeyId};
use utils::LookupTable;

pub struct LtConf {
    pub groups: Groups,
    pub key_masks: LookupTable<(GroupId, KeyId), bool>,
}
