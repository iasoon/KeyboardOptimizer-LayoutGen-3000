use parser::{Parser, KbParser};
use model::{Loc, LocData};
use data::score_tree::Loc as LocRepr;

use errors::*;

impl<'a> Parser<Loc> for KbParser<'a> {
    type Repr = LocRepr;

    fn parse(&self, repr: &LocRepr) -> Result<Loc> {
        let data = LocData {
            key_data: self.kb_conf.keys.elem_count(),
            layer_data: self.kb_conf.layers.elem_count(),
        };
        Ok(Loc::new(
            &data,
            self.parse(&repr.key)?,
            self.parse(&repr.layer)?
        ))
    }
}
