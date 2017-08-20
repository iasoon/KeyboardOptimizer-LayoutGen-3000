use std::marker::PhantomData;

use cat::*;

/// Marker type for an n-gram sequence.
pub struct NGram<T> {
    phantom: PhantomData<T>,
}

/// A collection of n-grams along with their frequency.
pub struct NGrams<T> {
    pub elements: SeqTable<NGram<T>, Num<T>>,
    pub freqs: Table<NGram<T>, f64>,
}

impl<T> NGrams<T> {
    /// Given a cost-mapping, evaluates the total cost of these n-grams.
    pub fn eval<'e, P>(&self, cost: NGramCost<'e, T, P>) -> f64 {
        self.elements.enumerate().map(|(seq_num, seq)| {
            cost.apply(seq) * self.freqs[seq_num]
        }).sum()
    }
}

/// A cost-mapping for n-grams.
pub struct NGramCost<'a, D: 'a, T: 'a> {
    /// map n-grams to paths
    pub mapping: &'a Table<D, Num<T>>,
    /// map paths to their cost
    pub path_cost: &'a PathCost<T>,
}

impl<'a, D: 'a, T: 'a> NGramCost<'a, D, T> {
    pub fn apply<'e>(&self, ngram: &'e [Num<D>]) -> f64 {
        let path = ngram.iter().map(|&e| self.mapping[e]);
        return self.path_cost[path];
    }
}

pub type PathCost<T> = Composed<SeqNum<T>, Table<Seq<T>, f64>>;
