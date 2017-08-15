use data::*;

use eval::{Eval, Evaluator};
use eval::ngram_eval::*;

mod seqs;

use json::reader::{Reader, EvalReader};
use json::errors::*;

use self::seqs::SeqsData;

#[derive(Deserialize)]
pub enum EvalData<'s> {
    Seqs(#[serde(borrow)] SeqsData<&'s str, &'s str>)
}

impl<'s> Reader<Eval> for EvalReader<'s> {
    type Repr = Vec<EvalData<'s>>;

    fn read(&self, data: Vec<EvalData<'s>>) -> Result<Eval> {
        let mut evals = Vec::with_capacity(data.len());
        for eval_data in data.into_iter() {
            let eval = try!(self.read(eval_data));
            evals.push(eval);
        }
        return Ok(Eval { evaluators: evals });
    }
}

impl<'s> Reader<Box<Evaluator>> for EvalReader<'s> {
    type Repr = EvalData<'s>;

    fn read(&self, repr: EvalData<'s>) -> Result<Box<Evaluator>> {
        match repr {
            EvalData::Seqs(data) => {
                let eval: NGramEval<Group, Key> = try!(self.read(data));
                return Ok(Box::new(eval));
            }
        }
    }
}
