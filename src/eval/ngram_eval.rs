use std::marker::PhantomData;

use data::*;
use cat::*;
use cat::ops::*;
use layout::Assignable;
use layout::Layout;
use eval::walker::*;
use eval::evaluator::Evaluator;

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
    ngrams: NGrams<T>,
    costs: PathCost<P>,
    intersections: BagTable<T, NGrams<T>>,
}

impl<T, P> NGramEval<T, P>
    where T: FiniteDomain,
          P: FiniteDomain
{
    pub fn new(t_count: Count<T>, ngrams: NGrams<T>, costs: PathCost<P>) -> Self {
        NGramEval {
            intersections: mk_intersections(t_count, &ngrams),
            ngrams: ngrams,
            costs: costs,
        }
    }

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

impl<'e> Evaluator<'e> for NGramEval<Group, Key> {
    type Walker = NGramWalker<'e, Group, Key>;

    fn eval(&self, layout: &Layout) -> f64 {
        unimplemented!()
    }

    fn walker(&'e self, kb_def: &KbDef, lt_walker: &mut LtWalker) -> Self::Walker {
        NGramWalker {
            eval: self,
            // TODO
            assignment_delta: kb_def.assignment_num().map_nums(|_| 0.0),
        }
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

struct SubSeqs<'t, T: 't> {
    seq: &'t [T],
    idxs: Vec<usize>,
}

impl<'t, T: 't> SubSeqs<'t, T> {

    fn new(seq: &'t [T], len: usize) -> Self {
        SubSeqs {
            seq: seq,
            idxs: vec![0; len],
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

    fn increment(&mut self) {
        let mut pos = self.idxs.len() - 1;
        loop {
            self.idxs[pos] += 1;
            if pos == 0 || !self.pos_valid(pos) {
                for i in 1..(self.idxs.len() - pos) {
                    self.idxs[pos + i] = self.idxs[pos] + i;
                }
                return;
            }
        }
    }

    fn next(&mut self) -> bool {
        self.increment();
        return self.pos_valid(0);
    }

    fn seq<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.idxs.iter().map(move |&idx| &self.seq[idx])
    }
}

fn mk_intersections<T>(count: Count<T>, ngrams: &NGrams<T>) -> BagTable<T, NGrams<T>>
    where T: FiniteDomain
{
    let seq_bag = SeqBag::new(count, ngrams.elements.seq_len());
    let mut builder = seq_bag
        .map_nums(|_| NGramsSubsetBuilder::new())
        .compose(seq_bag);
    for (ngram_num, ngram) in ngrams.elements.enumerate() {
        let mut subseqs = SubSeqs::new(ngram, 2);
        while subseqs.next() {
            builder.get_mut(subseqs.seq().cloned()).push(ngram_num);
        }
    }
    return builder.map_into(|b| b.build(ngrams));
}

struct NGramsSubsetBuilder<T>
    where T: FiniteDomain
{
    nums: Vec<Num<NGram<T>>>,
    next_allowed: usize,
}

impl<T> NGramsSubsetBuilder<T>
    where T: FiniteDomain
{
    fn new() -> Self {
        NGramsSubsetBuilder {
            nums: Vec::new(),
            next_allowed: 0,
        }
    }

    fn push(&mut self, num: Num<NGram<T>>) {
        // avoid pushing the same sequence twice
        if num.as_usize() >= self.next_allowed {
            self.nums.push(num);
            self.next_allowed = num.as_usize() + 1;
        }
    }

    fn build(&self, ngrams: &NGrams<T>) -> NGrams<T> {
        let mut elems = Vec::with_capacity(
            self.nums.len() * ngrams.elements.seq_len());
        let mut freqs = Vec::with_capacity(self.nums.len());
        for &num in self.nums.iter() {
            elems.extend(ngrams.elements.get(num).iter().cloned());
            freqs.push(*ngrams.freqs.get(num));
        }
        return NGrams {
            elements: SeqTable::from_elem_vec(elems, ngrams.elements.seq_len()),
            freqs: Table::from_vec(freqs),
        }
    }
}

trait HasMapping<T, P>
    where T: FiniteDomain,
          P: FiniteDomain
{
    fn mapping<'m>(&'m self) -> &'m Table<T, Num<P>>;
}

pub struct NGramWalker<'e, T, P>
    where T: FiniteDomain + 'e,
          P: FiniteDomain + 'e
{
    eval: &'e NGramEval<T, P>,
    assignment_delta: Table<Assignment, f64>,
}

impl<'e, T, P> Assignable for NGramWalker<'e, T, P>
    where T: FiniteDomain + 'e,
          P: FiniteDomain + 'e
{
    fn assign(&mut self, _: &KbDef, _: Assignment) {
        // Do nothing
    }
}

impl<'e, T, P> Walker<'e, NGramWalker<'e, T, P>>
    where Self: HasMapping<T, P>,
          T: FiniteDomain + 'e,
          P: FiniteDomain + 'e
{
    fn cost<'a>(&'a self) -> NGramCost<'a, T, P> {
        self.evaluator().ngram_cost(self.mapping())
    }

    fn evaluator<'a>(&'a self) -> &'a NGramEval<T, P> {
        self.eval_walker.eval
    }

    fn eval(&self) -> f64 {
        self.evaluator().ngrams.eval(self.cost())
    }

    fn eval_intersection(&self, ts: [Num<T>; 2]) -> f64
    {
        let ngrams = self.evaluator().intersections.get(ts.iter().cloned());
        return ngrams.eval(self.cost());
    }
}

impl<'e, T, P> EvalWalker for Walker<'e, NGramWalker<'e, T, P>>
    where Self: HasMapping<T, P>,
          T: FiniteDomain + 'e,
          P: FiniteDomain + 'e
{
    fn eval_delta(&mut self, delta: &[Assignment]) -> f64 {
        self.measure_effect(
            |walker| walker.assign_all(delta),
            |walker| walker.eval()
        )
    }

    fn update(&mut self, delta: &[Assignment]) {
        // do nothing
    }
}

impl<'e> HasMapping<Group, Key> for Walker<'e, NGramWalker<'e, Group, Key>> {
    fn mapping<'m>(&'m self) -> &'m Table<Group, Num<Key>> {
        self.lt_walker.group_map()
    }
}
