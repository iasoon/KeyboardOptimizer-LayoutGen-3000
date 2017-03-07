use parser::{Parser, KbParser};
use model::Loc;
use data::score_tree::Loc as LocData;

use errors::*;

impl<'a> Parser<Loc> for KbParser<'a> {
    type Repr = LocData;

    fn parse(&self, repr: &LocData) -> Result<Loc> {
        Ok(Loc {
            layer_id: self.parse(&repr.layer)?,
            key_id: self.parse(&repr.key)?,
        })
    }
}
