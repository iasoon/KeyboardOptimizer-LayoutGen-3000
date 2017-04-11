mod lt_parser;
mod key_masks;
mod groups;

use errors::*;

pub use parser::{Parser, KbParser};
pub use self::lt_parser::LtParser;

use model::{LtConf, Groups, GroupId, KeyId};
use data::lt_conf::LtConf as LtConfData;
use utils::LookupTable;


impl<'a> Parser<LtConf> for KbParser<'a> {
    type Repr = LtConfData;

    fn parse(&self, repr: &LtConfData) -> Result<LtConf> {
        LtConfReader::new(self, repr).and_then(|r| r.mk_lt_conf())
    }
}

struct LtConfReader<'a> {
    parser: &'a KbParser<'a>,
    data: &'a LtConfData,
    groups: Groups,
}

impl<'a> LtConfReader<'a> {
    fn new(parser: &'a KbParser<'a>, data: &'a LtConfData) -> Result<Self> {
        Ok(LtConfReader {
            groups: parser.parse(&data.locks)?,
            parser: parser,
            data: data,
        })
    }

    fn read_key_masks(&self) -> Result<LookupTable<(GroupId, KeyId), bool>> {
        LtParser::new(self.parser, &self.groups).parse(&self.data.key_masks)
    }

    fn mk_lt_conf(self) -> Result<LtConf> {
        Ok(LtConf {
            key_masks: self.read_key_masks()?,
            groups: self.groups,
        })
    }
}
