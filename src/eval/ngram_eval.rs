use std::marker::PhantomData;

use data::*;
use cat::*;

pub struct NGram<T> {
    phantom: PhantomData<T>,
}

impl<T> Domain for NGram<T>
    where T: FiniteDomain
{
    type Type = Vec<Num<T>>;
}

impl<T> FiniteDomain for NGram<T>
    where T: FiniteDomain
{}

pub struct NGrams<T>
    where T: FiniteDomain
{
    pub elements: SeqTable<NGram<T>, Num<T>>,
    pub freqs: Table<NGram<T>, f64>,
}

impl<T> NGrams<T>
    where T: FiniteDomain
{
    pub fn eval<'e, P>(&self, cost: NGramCost<'e, T, P>) -> f64
        where P: FiniteDomain
    {
        self.elements.enumerate().map(|(seq_num, seq)| {
            cost.apply(seq) * self.freqs.get(seq_num)
        }).sum()
    }
}

pub type PathCost<T> = Composed<SeqNum<T>, Table<Seq<T>, f64>>;

pub struct NGramEval<T, P>
    where T: FiniteDomain,
          P: FiniteDomain
{
    pub ngrams: NGrams<T>,
    pub costs: PathCost<P>,
}

impl<T, P> NGramEval<T, P>
    where T: FiniteDomain,
          P: FiniteDomain
{
    fn ngram_cost<'e>(&'e self, mapping: &'e Table<T, Num<P>>) -> NGramCost<'e, T, P> {
        NGramCost {
            mapping: mapping,
            path_cost: &self.costs,
        }
    }

    pub fn eval(&self, mapping: &Table<T, Num<P>>) -> f64 {
        self.ngrams.eval(self.ngram_cost(mapping))
    }
}

struct NGramCost<'a, D, T>
    where D: FiniteDomain + 'a,
          T: FiniteDomain + 'a
{
    mapping: &'a Table<D, Num<T>>,
    path_cost: &'a PathCost<T>,
}

impl<'a, D, T> NGramCost<'a, D, T>
    where D: FiniteDomain + 'a,
          T: FiniteDomain + 'a
{
    fn apply<'e>(&self, ngram: &'e [Num<D>]) -> f64 {
        let path = ngram.iter().map(|&e| *self.mapping.get(e));
        return *self.path_cost.get(path);
    }
}
