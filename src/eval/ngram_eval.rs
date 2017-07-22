use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

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

pub struct NGrams<T> {
    pub elements: SeqTable<NGram<T>, Num<T>>,
    pub freqs: Table<NGram<T>, f64>,
}

impl<T> NGrams<T> {
    pub fn eval<'e, P>(&self, cost: NGramCost<'e, T, P>) -> f64 {
        self.elements.enumerate().map(|(seq_num, seq)| {
            cost.apply(seq) * self.freqs[seq_num]
        }).sum()
    }
}

pub type PathCost<T> = Composed<SeqNum<T>, Table<Seq<T>, f64>>;

pub struct NGramEval<T, P> {
    ngrams: NGrams<T>,
    costs: PathCost<P>,
    intersections: BagTable<Group, NGrams<T>>,
}

impl NGramEval<Group, Key> {
    pub fn new(t_count: Count<Group>, ngrams: NGrams<Group>, costs: PathCost<Key>) -> Self {
        let items = [1,2,3,4,5];
        let mut s = SubSeqs::new(&items, 2);
        NGramEval {
            intersections: mk_intersections(t_count, &ngrams),
            ngrams: ngrams,
            costs: costs,
        }
    }
}

impl<T, P> NGramEval<T, P> {
    fn ngram_cost<'e>(&'e self, mapping: &'e Table<T, Num<P>>) -> NGramCost<'e, T, P> {
        NGramCost {
            mapping: mapping,
            path_cost: &self.costs,
        }
    }
}

struct NGramCost<'a, D: 'a, T: 'a> {
    mapping: &'a Table<D, Num<T>>,
    path_cost: &'a PathCost<T>,
}

impl<'a, D: 'a, T: 'a> NGramCost<'a, D, T> {
    fn apply<'e>(&self, ngram: &'e [Num<D>]) -> f64 {
        let path = ngram.iter().map(|&e| self.mapping[e]);
        return self.path_cost[path];
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

    fn next(&mut self) -> bool {
        self.increment();
        return self.pos_valid(0);
    }

    fn seq<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.idxs.iter().map(move |&idx| &self.seq[idx])
    }
}

fn mk_intersections<T>(count: Count<T>, ngrams: &NGrams<T>) -> BagTable<T, NGrams<T>> {
    let seq_bag = SeqBag::new(count, ngrams.elements.seq_len());
    let mut builder = seq_bag
        .map_nums(|_| NGramsSubsetBuilder::new())
        .compose(seq_bag);
    for (ngram_num, ngram) in ngrams.elements.enumerate() {
        let mut subseqs = SubSeqs::new(ngram, 2);
        while subseqs.next() {
            builder[subseqs.seq().cloned()].push(ngram_num);
        }
    }
    return builder.map_into(|b| b.build(ngrams));
}

struct NGramsSubsetBuilder<T> {
    nums: Vec<Num<NGram<T>>>,
    next_allowed: usize,
}

impl<T> NGramsSubsetBuilder<T> {
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
            elems.extend(ngrams.elements[num].iter().cloned());
            freqs.push(ngrams.freqs[num]);
        }
        return NGrams {
            elements: SeqTable::from_elem_vec(elems, ngrams.elements.seq_len()),
            freqs: Table::from_vec(freqs),
        }
    }
}

trait HasMapping<T, P> {
    fn mapping<'m>(&'m self) -> &'m Table<T, Num<P>>;
}

struct AssignmentTable<'a, T> {
    kb_def: &'a KbDef,
    table: Table<AllowedAssignment, T>,
}

impl<'a, T> AssignmentTable<'a, T> {
    fn new<F>(kb_def: &'a KbDef, fun: F) -> Self
        where F: FnMut((Num<AllowedAssignment>, &Assignment)) -> T
    {
        let values = kb_def.assignments.enumerate().map(fun).collect();
        AssignmentTable {
            table: Table::from_vec(values),
            kb_def: kb_def,
        }
    }
}

impl<'a, T> Index<Assignment> for AssignmentTable<'a, T> {
    type Output = T;

    fn index<'t>(&'t self, assignment: Assignment) -> &'t T {
        let assignment_num = self.kb_def.assignment_map[assignment].unwrap();
        return &self.table[assignment_num];
    }
}

impl<'a, T> IndexMut<Assignment> for AssignmentTable<'a, T> {
    fn index_mut<'t>(&'t mut self, assignment: Assignment) -> &'t mut T {
        let assignment_num = self.kb_def.assignment_map[assignment].unwrap();
        return &mut self.table[assignment_num];
    }
}

pub struct NGramWalker<'e, T: 'e, P: 'e> {
    eval: &'e NGramEval<T, P>,
    assignment_delta: AssignmentTable<'e, f64>,
}

impl<'e, T: 'e, P: 'e> Assignable for NGramWalker<'e, T, P> {
    fn assign(&mut self, _: &KbDef, _: Assignment) {
        // Do nothing
    }
}

impl<'a, 'e, T: 'e, P: 'e> Walker<'a, 'e, NGramWalker<'e, T, P>>
    where Self: HasMapping<T, P>,
{
    fn cost<'b>(&'b self) -> NGramCost<'b, T, P> {
        self.evaluator().ngram_cost(self.mapping())
    }

    fn evaluator<'b>(&'b self) -> &'b NGramEval<T, P> {
        self.eval.eval
    }

    fn eval(&self) -> f64 {
        self.evaluator().ngrams.eval(self.cost())
    }

    fn eval_intersection(&self, ts: [Num<Group>; 2]) -> f64
    {
        let ngrams = &self.evaluator().intersections[ts.iter().cloned()];
        return ngrams.eval(self.cost());
    }

    fn recalc_delta(&mut self, assignment: Assignment) {
        let delta = self.measure_effect(
            |walker| walker.assign(assignment),
            |walker| walker.eval()
        );
        self.eval.assignment_delta[assignment] = delta;
    }

    fn eval_delta_delta(&mut self, assignment: Assignment, assignments: &[Assignment]) -> f64 {
        let kb_def = self.driver.kb_def;
        self.excursion(|walker| {
            assignments.iter().map(|&a| {
                walker.measure_effect_mut(
                    |walker| walker.assign(a),
                    |walker| walker.measure_effect(
                        |walker| walker.assign(assignment),
                        |walker| walker.eval_intersection([
                            assignment.group_num(kb_def),
                            a.group_num(kb_def)
                        ])
                    )
                )
            }).sum()
        })
    }
}

impl<'e, T: 'e, P: 'e> WalkableEval<'e> for NGramWalker<'e, T, P>
    where for<'w> Walker<'w, 'e, Self> : HasMapping<T, P>
{
    fn eval_delta<'w>(&'w mut self, driver: &'w mut WalkerDriver<'e>, assignments: &[Assignment]) -> f64 {
        let d: f64 = assignments.iter().enumerate().map(|(idx, &assignment)| {
            let delta = self.assignment_delta[assignment];
            let delta_delta = driver.drive(self)
                .eval_delta_delta(assignments[idx], &assignments[0..idx]);
            return delta + delta_delta;
        }).sum();
        let expected = driver.drive(self).measure_effect(
            |walker| walker.assign_all(assignments),
            |walker| walker.eval()
        );
        let tol = (10.0 as f64).powi(-12);
        if (d - expected).abs() > tol {
            println!("PANIC! WRONG RESULT! was {} but expected {}", d, expected);
        }
        return d;
    }

    fn update<'w>(&'w mut self, driver: &'w mut WalkerDriver<'e>, delta: &[Assignment]) {
        // do nothing
    }
}

impl<'w, 'e> HasMapping<Group, Key> for Walker<'w, 'e, NGramWalker<'e, Group, Key>> {
    fn mapping<'m>(&'m self) -> &'m Table<Group, Num<Key>> {
        self.driver.group_map()
    }
}

impl Evaluator for NGramEval<Group, Key> {
    fn eval(&self, layout: &Layout) -> f64 {
        let group_map = layout.mk_group_map();
        return self.ngrams.eval(self.ngram_cost(&group_map));
    }

    fn walker<'e>(&'e self, driver: &mut WalkerDriver<'e>) -> Box<WalkableEval<'e> + 'e> {
        let mut walker = NGramWalker {
            eval: self,
            assignment_delta: AssignmentTable::new(driver.kb_def, |_| 0.0),
        };
        // init cache
        for (_, &assignment) in driver.kb_def.assignments.enumerate() {
            driver.drive(&mut walker).recalc_delta(assignment);
        }
        return Box::new(walker);
    }
}
