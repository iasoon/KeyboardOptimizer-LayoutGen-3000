use data::*;
use cat::*;

pub struct NGram;

impl Domain for NGram {
    type Type = Vec<Num<Group>>;
}

impl FiniteDomain for NGram {}

pub struct NGrams {
    pub elements: SeqTable<NGram, Num<Group>>,
    pub freqs: Table<NGram, f64>,
}

impl NGrams {
    pub fn eval<'e>(&self, cost: NGramCost<'e, Group, Key>) -> f64 {
        self.elements.enumerate().map(|(seq_num, seq)| {
            cost.apply(seq) * self.freqs.get(seq_num)
        }).sum()
    }
}

pub type PathCost<T> = Composed<SeqNum<T>, Table<Seq<T>, f64>>;

pub struct NGramEval {
    pub ngrams: NGrams,
    pub costs: PathCost<Key>,
}

impl NGramEval {
    fn ngram_cost<'e>(&'e self, group_map: &'e Table<Group, Num<Key>>) -> NGramCost<'e, Group, Key> {
        NGramCost {
            mapping: group_map,
            path_cost: &self.costs,
        }
    }

    fn eval(&self, group_map: &Table<Group, Num<Key>>) -> f64 {
        self.ngrams.eval(self.ngram_cost(group_map))
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
