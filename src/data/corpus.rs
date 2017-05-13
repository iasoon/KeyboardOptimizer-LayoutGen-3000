use errors::*;
use utils::{Countable, ElemCount, SeqTable, SeqAssocList, SeqAssocListBuilder};
use model::{KbDef, Token, TokenId, GroupId};

use data::countable::Assocs;

use std::path::Path;

pub fn read_corpus(kb_def: &KbDef, path: &Path) -> Result<SeqAssocList<GroupId, f64>> {
    let assocs = Assocs::read(path, &kb_def.tokens.elem_count())?;

    let mut builder = SeqAssocListBuilder::new(assocs.seq_len);
    for (raw_seq, value) in assocs.vec.into_iter() {
        let seq = raw_seq.into_iter().map(move |token_id| kb_def.token_group[token_id]);
        builder.push(seq, value);
    }
    Ok(builder.build())
}
