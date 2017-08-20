use cat::*;
use cat::ops::*;
use data::*;
use layout::*;

use eval::*;
use eval::walker::*;
use eval::ngram_eval::types::*;
use eval::ngram_eval::walker::*;
use eval::ngram_eval::utils::*;


pub type GroupTable<T> = Composed<GroupNum, Table<Group, T>>;

/// N-gram evaluator
pub struct NGramEval<T, P> {
    ngrams: NGrams<T>,
    costs: PathCost<P>,
    group_ngrams: GroupTable<NGrams<T>>,
    intersections: BagTable<Group, NGrams<T>>,
}

impl<T, P> NGramEval<T, P> {
    pub fn ngram_cost<'e>(&'e self, mapping: &'e Table<T, Num<P>>) -> NGramCost<'e, T, P> {
        NGramCost {
            mapping: mapping,
            path_cost: &self.costs,
        }
    }

    pub fn intersection<'e>(&'e self, ts: [Num<Group>; 2]) -> &'e NGrams<T> {
        &self.intersections[ts.iter().cloned()]
    }

    pub fn group_ngrams<'e>(&'e self, group: Group) -> &'e NGrams<T> {
        &self.group_ngrams[group]
    }
}

fn mk_group_ngrams(kb_def: &KbDef, ngrams: &NGrams<Group>) -> GroupTable<NGrams<Group>> {
    let mut table = kb_def.group_num().map_nums(|_| NGramsSubsetBuilder::new());
    for (ngram_num, ngram) in ngrams.elements.enumerate() {
        for &group_num in ngram.iter() {
            table[group_num].push(ngram_num);
        }
    }
    return table.map_into(|b| b.build(ngrams)).compose(kb_def.group_num());
}

fn mk_intersections(kb_def: &KbDef, ngrams: &NGrams<Group>) -> BagTable<Group, NGrams<Group>> {
    let seq_bag = SeqBag::new(kb_def.group_num().count(), ngrams.elements.seq_len());
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

impl NGramEval<Group, Key> {
    pub fn new(kb_def: &KbDef, ngrams: NGrams<Group>, costs: PathCost<Key>) -> Self {
        NGramEval {
            group_ngrams: mk_group_ngrams(kb_def, &ngrams),
            intersections: mk_intersections(kb_def, &ngrams),
            ngrams: ngrams,
            costs: costs,
        }
    }
}

impl Evaluator for NGramEval<Group, Key> {
    fn eval(&self, layout: &Layout) -> f64 {
        let group_map = layout.mk_group_map();
        return self.ngrams.eval(self.ngram_cost(&group_map));
    }

    fn walker<'e>(&'e self, driver: &mut WalkerDriver<'e>) -> Box<WalkableEval<'e> + 'e> {
        Box::new(NGramWalker::new(self, driver))
    }
}

impl<'w, 'e> HasMapping<Group, Key> for Walker<'w, 'e, NGramWalker<'e, Group, Key>> {
    fn mapping<'m>(&'m self) -> &'m Table<Group, Num<Key>> {
        self.driver.group_map()
    }
}
