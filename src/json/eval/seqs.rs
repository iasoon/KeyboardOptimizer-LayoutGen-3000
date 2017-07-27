use cat::*;
use data::*;
use eval::ngram_eval::*;

use json::errors::*;
use json::reader::{Reader, EvalReader};

impl<'s> Reader<Num<Group>> for EvalReader<'s> {
    type Repr = &'s str;

    fn read(&self, token_name: &'s str) -> Result<Num<Group>> {
        self.read(token_name).map(|token_num: Num<Token>| {
            self.kb_def.group_num().apply(self.kb_def.token_group[token_num])
        })
    }
}

impl<'s> Reader<NGramEval<Group, Key>> for EvalReader<'s> {
    type Repr = SeqsData<&'s str, &'s str>;

    fn read(&self, repr: Self::Repr) -> Result<NGramEval<Group, Key>> {
        let reader = NGramEvalReader {
            reader: self,
            seq_len: repr.seq_len,
        };
        Ok(NGramEval::new(
            self.kb_def,
            try!(reader.read(repr.freqs)),
            try!(reader.read(repr.costs))
        ))
    }
}

struct NGramEvalReader<'s> {
    seq_len: usize,
    reader: &'s EvalReader<'s>,
}

impl<'s> HasCount<Key> for NGramEvalReader<'s> {
    fn count(&self) -> Count<Key> {
        self.reader.kb_def.keys.count()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SeqsData<T, S> {
    seq_len: usize,
    freqs: Vec<Freq<T>>,
    costs: Vec<Cost<S>>,
}

#[derive(Serialize, Deserialize)]
struct Freq<T> {
    seq: Vec<T>,
    freq: f64,
}

impl<'s, T> Reader<NGrams<T>> for NGramEvalReader<'s>
    where EvalReader<'s>: Reader<Num<T>>
{
    type Repr = Vec<Freq<<EvalReader<'s> as Reader<Num<T>>>::Repr>>;

    fn read(&self, repr: Self::Repr) -> Result<NGrams<T>> {
        let mut elems = Vec::with_capacity(repr.len() * self.seq_len);
        let mut freqs = Vec::with_capacity(repr.len());
        for freq in repr.into_iter() {
            for elem_repr in freq.seq.into_iter() {
                let elem = try!(self.reader.read(elem_repr));
                elems.push(elem);
            }
            freqs.push(freq.freq);
        }
        Ok(NGrams {
            elements: SeqTable::from_elem_vec(elems, self.seq_len),
            freqs: Table::from_vec(freqs),
        })
    }
}

#[derive(Serialize, Deserialize)]
struct Cost<S> {
    path: Vec<S>,
    cost: f64,
}

impl<'s, S> Reader<PathCost<S>> for NGramEvalReader<'s>
    where EvalReader<'s>: Reader<Num<S>>,
          Self: HasCount<S>
{
    type Repr = Vec<Cost<<EvalReader<'s> as Reader<Num<S>>>::Repr>>;

    fn read(&self, repr: Self::Repr) -> Result<PathCost<S>> {
        let seq_num = SeqNum::new(self.count(), self.seq_len);
        let mut table = seq_num.map_nums(|_| 0.0).compose(seq_num);
        let mut path_buf = Vec::with_capacity(self.seq_len);

        for cost in repr.into_iter() {
            path_buf.clear();
            for elem_repr in cost.path.into_iter() {
                let elem = try!(self.reader.read(elem_repr));
                path_buf.push(elem);
            }
            table[path_buf.iter().cloned()] += cost.cost;
        }
        Ok(table)
    }
}
