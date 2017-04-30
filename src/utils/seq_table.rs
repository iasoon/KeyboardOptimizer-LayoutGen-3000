use utils::{Countable, SeqAssocList};

use std::vec::Vec;

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

    fn calc_index<Iter>(&self, iter: Iter) -> usize
        where Iter: Iterator<Item = C>
    {
        iter.fold(0,
                  |acc, elem| acc * C::count(&self.data) + elem.to_num(&self.data))
    }

    pub fn get<'a, Iter>(&'a self, iter: Iter) -> &'a T
        where Iter: Iterator<Item = C>
    {
        &self.vec[self.calc_index(iter)]
    }

    pub fn get_mut<'a, Iter>(&'a mut self, iter: Iter) -> &'a mut T
        where Iter: Iterator<Item = C>
    {
        let idx = self.calc_index(iter);
        &mut self.vec[idx]
    }

    fn try_calc_index<Iter, E>(&self, iter: Iter) -> Result<usize, E>
        where Iter: Iterator<Item = Result<C, E>>
    {
        reduce_results(iter,
                       0,
                       |acc, elem| acc * C::count(&self.data) + elem.to_num(&self.data))
    }

    pub fn try_get<Iter, E>(&self, iter: Iter) -> Result<T, E>
        where Iter: Iterator<Item = Result<C, E>>,
              T: Copy
    {
        self.try_calc_index(iter).map(|idx| self.vec[idx])
    }

    fn seq_len(&self) -> usize {
        self.vec.len() / C::count(&self.data)
    }

    fn num_to_seq(&self, mut num: usize) -> Vec<C> {
        let mut vec = Vec::with_capacity(self.seq_len());
        for _ in 0..self.seq_len() {
            vec.push(C::from_num(&self.data, num % C::count(&self.data)));
            num /= C::count(&self.data)
        }
        vec.reverse();
        return vec;
    }

    pub fn filter_entries<F>(&self, pred: F) -> SeqAssocList<C, T>
        where F: Fn(&Vec<C>, T) -> bool,
              T: Clone
    {
        let mut seqs = Vec::new();
        let mut values = Vec::new();
        for (num, value) in self.vec.iter().cloned().enumerate() {
            let seq = self.num_to_seq(num);
            if pred(&seq, value.clone()) {
                seqs.extend(seq.into_iter());
                values.push(value);
            }
        }
        return SeqAssocList::from_vecs(seqs, self.seq_len(), values);
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
