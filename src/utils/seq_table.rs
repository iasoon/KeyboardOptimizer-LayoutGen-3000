use std::vec::Vec;

use utils::Countable;

pub struct SeqTable<C: Countable, T> {
    data: C::Data,
    vec: Vec<T>,
}

impl<C: Countable, T> SeqTable<C, T> {
    pub fn new(data: C::Data, seq_len: usize, default: T) -> Self
        where T: Copy
    {
        SeqTable {
            vec: vec![default; C::count(&data).pow(seq_len as u32)],
            data: data,
        }
    }

    fn calc_index<Iter, E>(&self, iter: Iter) -> Result<usize, E>
        where Iter: Iterator<Item = Result<C, E>>
    {
        reduce_results(iter, 0, |acc, elem| {
            acc * C::count(&self.data) + elem.to_num(&self.data)
        })
    }

    pub fn get<Iter, E>(&self, iter: Iter) -> Result<T, E>
        where Iter: Iterator<Item = Result<C, E>>, T: Copy
    {
        self.calc_index(iter).map(|idx| self.vec[idx])
    }

    pub fn set<Iter, E>(&mut self, iter: Iter, value: T) -> Result<(), E>
        where Iter: Iterator<Item = Result<C, E>>
    {
        self.calc_index(iter).map(|idx| self.vec[idx] = value)
    }
}

fn reduce_results<A, B, E, Iter, F>(iter: Iter, init: B, fun: F) -> Result<B, E>
    where Iter: Iterator<Item = Result<A, E>>,
          F: Fn(B, A) -> B
{
    let mut acc = init;
    for item in iter {
        match item {
            Ok(a) => acc = fun(acc, a),
            Err(err) => return Err(err),
        }
    }
    return Ok(acc);
}
