use errors::*;
use utils::{Countable, ElemCount, SeqTable};
use model::{KbDef, KeyId};

use data::countable::Assocs;

use std::path::Path;

pub fn read_path_costs(kb_def: &KbDef, path: &Path) -> Result<SeqTable<KeyId, f64>> {
    let assocs = Assocs::read(path, &kb_def.keys.elem_count())?;

    let mut seq_table = SeqTable::new(kb_def.keys.elem_count(), assocs.seq_len, 0.0);
    for (seq, value) in assocs.vec.into_iter() {
        *seq_table.get_mut(seq.into_iter()) += value;
    }

    Ok(seq_table)
}
