use utils::{Countable, Seq, SeqSet, LookupTable, SeqNum, SeqIter};

pub struct SeqAssocList<K: Countable, V> {
    seqs: SeqSet<K>,
    values: LookupTable<SeqNum, V>,
}

impl<K: Countable, V> SeqAssocList<K, V> {
    pub fn from_vecs(seqs: Vec<K>, seq_len: usize, values: Vec<V>) -> Self {
        let seq_set = SeqSet::from_vec(seqs, seq_len);
        SeqAssocList {
            values: LookupTable::from_vec(values, seq_set.seq_count()),
            seqs: seq_set,
        }
    }

    pub fn get<'a>(&'a self, idx: SeqNum) -> (SeqIter<'a, K>, &'a V) {
        (self.seqs.get_seq(idx), &self.values[idx])
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (SeqIter<'a, K>, &'a V)> + 'a {
        SeqNum::enumerate(self.seqs.seq_count())
            .map(move |seq_id| self.get(seq_id))
    }

    pub fn seq_len(&self) -> usize {
        return self.seqs.seq_len();
    }
}
