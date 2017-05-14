use utils::{Countable, Seq, SeqSet, LookupTable, SeqNum, SeqIter};

pub struct SeqAssocList<K: Countable, V> {
    seqs: SeqSet<K>,
    values: LookupTable<SeqNum, V>,
}

impl<K: Countable, V> SeqAssocList<K, V> {
    pub fn get<'a>(&'a self, idx: SeqNum) -> (SeqIter<'a, K>, &'a V) {
        (self.seqs.get_seq(idx), &self.values[idx])
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (SeqIter<'a, K>, &'a V)> + 'a {
        SeqNum::enumerate(self.seqs.seq_count())
            .map(move |seq_num| self.get(seq_num))
    }

    pub fn seq_len(&self) -> usize {
        return self.seqs.seq_len();
    }
}

#[derive(Clone)]
pub struct SeqAssocListBuilder<K: Countable, V> {
    seqs: Vec<K>,
    values: Vec<V>,
    seq_len: usize,
}

impl<K: Countable, V> SeqAssocListBuilder<K, V> {
    pub fn new(seq_len: usize) -> Self {
        SeqAssocListBuilder {
            seqs: Vec::new(),
            values: Vec::new(),
            seq_len: seq_len,
        }
    }

    pub fn push<Iter>(&mut self, seq: Iter, value: V)
        where Iter: Iterator<Item = K>
    {
        self.seqs.extend(seq);
        self.values.push(value);
    }

    pub fn build(self) -> SeqAssocList<K, V> {
        let seq_set = SeqSet::from_vec(self.seqs, self.seq_len);
        SeqAssocList {
            values: LookupTable::from_vec(self.values, seq_set.seq_count()),
            seqs: seq_set,
        }
    }
}
