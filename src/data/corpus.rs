use errors::*;
use utils::{Countable, ElemCount, SeqTable, SeqAssocList};
use model::{KbDef, Token, TokenId, GroupId};

use data::countable::Assocs;

use std::path::Path;

pub fn read_corpus(kb_def: &KbDef, path: &Path) -> Result<SeqAssocList<GroupId, f64>> {
    let assocs = Assocs::read(path, &kb_def.tokens.elem_count())?;

    let seqs = assocs.vec.iter()
        .flat_map(|&(ref seq, _)| {
            seq.iter().cloned().map(move |token_id| kb_def.token_group[token_id])
        })
        .collect();
    let values = assocs.vec.into_iter().map(|(_, value)| value).collect();

    Ok(SeqAssocList::from_vecs(seqs, assocs.seq_len, values))
}
