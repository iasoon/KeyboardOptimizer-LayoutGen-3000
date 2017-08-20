use cat::*;

use eval::ngram_eval::types::*;

/// A cursor over fixed-length subsequences of a given sequence.
pub struct SubSeqs<'t, T: 't> {
    seq: &'t [T],
    idxs: Vec<usize>,
}

impl<'t, T: 't> SubSeqs<'t, T> {
    pub fn new(seq: &'t [T], len: usize) -> Self {
        SubSeqs {
            seq: seq,
            idxs: vec![0; len],
        }
    }

    /// Returns the current sequence.
    pub fn seq<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.idxs.iter().map(move |&idx| &self.seq[idx])
    }

    /// Move to next sequence
    /// Returns whether the subsequences are depleted.
    pub fn next(&mut self) -> bool {
        self.increment();
        return self.pos_valid(0);
    }

    fn increment(&mut self) {
        let mut pos = self.idxs.len();
        loop {
            pos -= 1;
            self.idxs[pos] += 1;
            if pos == 0 || self.pos_valid(pos) {
                for i in 1..(self.idxs.len() - pos) {
                    self.idxs[pos + i] = self.idxs[pos] + i;
                }
                return;
            }
        }
    }

    fn min_value(&self, pos: usize) -> usize {
        pos
    }

    fn max_value(&self, pos: usize) -> usize {
        self.seq.len() - self.idxs.len() + pos
    }

    fn pos_valid(&self, pos: usize) -> bool {
        let val = self.idxs[pos];
        return val <= self.max_value(pos) && val >= self.min_value(pos);
    }

}



/// Builds a subset of n-gram / freq pairs
pub struct NGramsSubsetBuilder<T> {
    nums: Vec<Num<NGram<T>>>,
    next_allowed: usize,
}

impl<T> NGramsSubsetBuilder<T> {
    pub fn new() -> Self {
        NGramsSubsetBuilder {
            nums: Vec::new(),
            next_allowed: 0,
        }
    }

    pub fn push(&mut self, num: Num<NGram<T>>) {
        // avoid pushing the same sequence twice
        if num.as_usize() >= self.next_allowed {
            self.nums.push(num);
            self.next_allowed = num.as_usize() + 1;
        }
    }

    pub fn build(&self, ngrams: &NGrams<T>) -> NGrams<T> {
        let mut elems = Vec::with_capacity(
            self.nums.len() * ngrams.elements.seq_len());
        let mut freqs = Vec::with_capacity(self.nums.len());
        for &num in self.nums.iter() {
            elems.extend(ngrams.elements[num].iter().cloned());
            freqs.push(ngrams.freqs[num]);
        }
        return NGrams {
            elements: SeqTable::from_elem_vec(elems, ngrams.elements.seq_len()),
            freqs: Table::from_vec(freqs),
        }
    }
}
