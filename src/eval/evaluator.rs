use utils::{SeqTable, SeqAssocList, LookupTable};
use model::{GroupId, KeyId};

pub struct Evaluator {
    seqs: SeqAssocList<GroupId, f64>,
    path_costs: SeqTable<KeyId, f64>,
}

impl Evaluator {
    pub fn new(seqs: SeqAssocList<GroupId, f64>, path_costs: SeqTable<KeyId, f64>) -> Self {
        Evaluator {
            seqs: seqs,
            path_costs: path_costs,
        }
    }

    pub fn evaluate(&self, table: &LookupTable<GroupId, KeyId>) -> f64 {
        self.seqs.iter().map(|(group_seq, value)| {
            let key_seq = group_seq.cloned().map(|group_id| table[group_id]);
            return self.path_costs.get(key_seq) * value;
        }).sum()
    }
}
