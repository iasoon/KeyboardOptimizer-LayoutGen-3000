use cat::*;
use data::*;

use eval::ngram_eval::*;

use json::reader::{Reader, EvalReader};
use json::errors::*;

#[derive(Serialize, Deserialize)]
pub struct EvalData<'s> {
    #[serde(borrow)]
    costs: Vec<CostData<'s>>,
    #[serde(borrow)]
    freqs: Vec<FreqData<'s>>,
}

#[derive(Serialize, Deserialize)]
struct CostData<'s> {
    #[serde(borrow)]
    key_seq: Vec<&'s str>,
    cost: f64,
}

#[derive(Serialize, Deserialize)]
struct FreqData<'s> {
    #[serde(borrow)]
    token_seq: Vec<&'s str>,
    freq: f64,
}

impl<'s> Reader<NGramEval<Group, Key>> for EvalReader<'s> {
    type Repr = EvalData<'s>;

    fn read(&self, repr: EvalData<'s>) -> Result<NGramEval<Group, Key>> {
        Ok(NGramEval::new(
            self.kb_def.group_num().count(),
            try!(self.read(&repr)),
            try!(self.read(&repr))))
    }
}

impl<'s> Reader<NGrams<Group>> for EvalReader<'s> {
    type Repr = &'s EvalData<'s>;

    fn read(&self, repr: &EvalData<'s>) -> Result<NGrams<Group>> {
        // TODO: read length
        let mut vec = Vec::with_capacity(repr.freqs.len() * 3);
        for ngram in repr.freqs.iter() {
            for &elem_repr in ngram.token_seq.iter() {
                let token: Num<Token> = self.read(elem_repr)?;
                let group = self.kb_def.
                    group_num()
                    .apply(self.kb_def.token_group[token]);
                vec.push(group);
            }
        }
        let elements = SeqTable::from_elem_vec(vec, 3);
        Ok(NGrams {
            freqs: elements.map_nums(|num| repr.freqs[num.as_usize()].freq),
            elements: elements,
        })
    }
}

impl<'s> Reader<PathCost<Key>> for EvalReader<'s> {
    type Repr = &'s EvalData<'s>;

    fn read(&self, repr: &'s EvalData<'s>) -> Result<PathCost<Key>> {
        // TODO: read length
        let seq_num = SeqNum::new(self.kb_def.keys.count(), 3);
        let mut table = seq_num.map_nums(|_| 0.0).compose(seq_num);
        let mut path_buf = Vec::with_capacity(3);

        for cost in repr.costs.iter() {
            path_buf.clear();
            for &elem in cost.key_seq.iter() {
                path_buf.push(try!(self.read(elem)));
            }
            table[path_buf.iter().cloned()] += cost.cost;
        }
        Ok(table)
    }
}
