use utils::{Countable, SeqSet, LookupTable, SeqId};
use utils::seq_set::Seq;

pub struct SeqAssocList<K, V> {
    seqs: SeqSet<K>,
    values: LookupTable<SeqId, V>,
}

impl<K, V> SeqAssocList<K, V> {
    pub fn from_vecs(seqs: Vec<K>, seq_len: usize, values: Vec<V>) -> Self {
        let seq_set = SeqSet::from_vec(seqs, seq_len);
        SeqAssocList {
            values: LookupTable::from_vec(values, seq_set.seq_count()),
            seqs: seq_set,
        }
    }

    pub fn get<'a>(&'a self, idx: SeqId) -> (Seq<'a, K>, &'a V) {
        (self.seqs.get_seq(idx), &self.values[idx])
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Seq<'a, K>, &'a V)> + 'a {
        SeqId::enumerate(self.seqs.seq_count())
            .map(move |seq_id| self.get(seq_id))
    }
}
