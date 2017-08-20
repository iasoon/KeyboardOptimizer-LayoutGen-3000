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
    /// Fetch a n-gram -> cost mapping for a given T -> P mapping
    pub fn ngram_cost<'e>(&'e self, mapping: &'e Table<T, Num<P>>) -> NGramCost<'e, T, P> {
        NGramCost {
            mapping: mapping,
            path_cost: &self.costs,
        }
    }

    /// Fetch the set of n-grams which contain both given groups
    pub fn intersection<'e>(&'e self, ts: [Num<Group>; 2]) -> &'e NGrams<T> {
        &self.intersections[ts.iter().cloned()]
    }

    /// Fetch all n-grams in which given group occurs
    pub fn group_ngrams<'e>(&'e self, group: Group) -> &'e NGrams<T> {
        &self.group_ngrams[group]
    }
}

impl<T, P> Evaluator for NGramEval<T, P> where
    // It should be possible to derive a T->P mapping from a layout
    for<'a> Table<T, Num<P>>: From<&'a Layout<'a>>,
    // It should be possible to get a T->P mapping from aa walker
    for<'a, 'e> Walker<'a, 'e, NGramWalker<'e, T, P>>: HasMapping<T, P>,
{
    /// Evaluate a layout
    fn eval(&self, layout: &Layout) -> f64 {
        return self.ngrams.eval(self.ngram_cost(&layout.into()));
    }

    /// Construct a walkable eval for this evaluator
    fn walker<'e>(&'e self, driver: &mut WalkerDriver<'e>) -> Box<WalkableEval<'e> + 'e> {
        Box::new(NGramWalker::new(self, driver))
    }
}


/// Evaluator construction

/// For types that can be converted into a group
pub trait HasGroup {
    fn group_num(&self, kb_def: &KbDef) -> Num<Group>;
}

impl<T, P> NGramEval<T, P>
    where Num<T>: HasGroup
{
    /// Initialize an evaluator for given n-grams and path cost
    pub fn new(kb_def: &KbDef, ngrams: NGrams<T>, costs: PathCost<P>) -> Self {
        NGramEval {
            group_ngrams: mk_group_ngrams(kb_def, &ngrams),
            intersections: mk_intersections(kb_def, &ngrams),
            ngrams: ngrams,
            costs: costs,
        }
    }
}

/// For all groups, construct the set of n-grams in which the group occurs.
fn mk_group_ngrams<T>(kb_def: &KbDef, ngrams: &NGrams<T>) -> GroupTable<NGrams<T>>
    where Num<T>: HasGroup
{
    let mut table = kb_def.group_num().map_nums(|_| NGramsSubsetBuilder::new());
    for (ngram_num, ngram) in ngrams.elements.enumerate() {
        for &item in ngram.iter() {
            table[item.group_num(kb_def)].push(ngram_num);
        }
    }
    return table.map_into(|b| b.build(ngrams)).compose(kb_def.group_num());
}

/// For all group pairs, construct the set of n-grams in which both groups occur.
fn mk_intersections<T>(kb_def: &KbDef, ngrams: &NGrams<T>) -> BagTable<Group, NGrams<T>>
    where Num<T>: HasGroup
{
    let seq_bag = SeqBag::new(kb_def.group_num().count(), ngrams.elements.seq_len());
    let mut builder = seq_bag
        .map_nums(|_| NGramsSubsetBuilder::new())
        .compose(seq_bag);
    for (ngram_num, ngram) in ngrams.elements.enumerate() {
        let mut subseqs = SubSeqs::new(ngram, 2);
        while subseqs.next() {
            let groups = subseqs.seq().map(|&t_num| t_num.group_num(kb_def));
            builder[groups].push(ngram_num);
        }
    }
    return builder.map_into(|b| b.build(ngrams));
}


// Implementations

// Group/Key n-gram evaluator

impl HasGroup for Num<Group> {
    fn group_num(&self, _: &KbDef) -> Num<Group> {
        self.clone()
    }
}

impl<'e> From<&'e Layout<'e>> for Table<Group, Num<Key>> {
    fn from(layout: &'e Layout<'e>) -> Self {
        layout.mk_group_map()
    }
}

impl<'w, 'e> HasMapping<Group, Key> for Walker<'w, 'e, NGramWalker<'e, Group, Key>> {
    fn mapping<'m>(&'m self) -> &'m Table<Group, Num<Key>> {
        self.driver.group_map()
    }
}

// Token/Location n-gram evaluator

impl HasGroup for Num<Token> {
    fn group_num(&self, kb_def: &KbDef) -> Num<Group> {
        kb_def.group_num().apply(kb_def.token_group[*self])
    }
}

impl<'e> From<&'e Layout<'e>> for Table<Token, Num<Loc>> {
    fn from(layout: &'e Layout<'e>) -> Self {
        // TODO: cloning here is not nice. Find a way to solve this.
        // Probably use control inversion, so that this method decides on lifetimes
        // (and thus allows for both passing a ref and allocating a new table).
        // The resulting abstraction could probably replace HasMapping as well.
        layout.token_map.clone()
    }
}

impl<'w, 'e> HasMapping<Token, Loc> for Walker<'w, 'e, NGramWalker<'e, Token, Loc>> {
    fn mapping<'m>(&'m self) -> &'m Table<Token, Num<Loc>> {
        self.driver.token_map()
    }
}
