use errors::*;

use parser::{Parser, KbParser};
use model::Groups;

pub struct LtParser<'a> {
    pub kb_parser: &'a KbParser<'a>,
    pub groups: &'a Groups,
}

impl<'a> LtParser<'a> {
    pub fn new(parser: &'a KbParser<'a>, groups: &'a Groups) -> Self {
        LtParser {
            kb_parser: parser,
            groups: groups,
        }
    }

    pub fn num_keys(&self) -> usize {
        self.kb_parser.kb_conf.keys.len()
    }
}

impl<'a, T> Parser<T> for LtParser<'a>
    where KbParser<'a>: Parser<T>
{
    type Repr = <KbParser<'a> as Parser<T>>::Repr;

    fn parse(&self, repr: &Self::Repr) -> Result<T> {
        self.kb_parser.parse(repr)
    }
}
