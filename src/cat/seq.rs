use std::marker::PhantomData;

use cat::*;
use cat::internal::{to_num, to_count};

/// A sequence of values.
/// Since it is not possible to parametrically fixate a length for these
/// sequences, impls for this type are a bit liberal and instead rely on
/// programmer discipline.
pub struct Seq<D> {
    phantom: PhantomData<D>,
}

impl<D> Domain for Seq<D>
    where D: Domain
{
    type Type = Vec<D>;
}

/// This implementation is purposely left a bit vague; for the domain of
/// sequences to be finite, one should constrain it, for example by using a
/// fixed length, or a maximum length.
impl<D> FiniteDomain for Seq<D>
    where D: FiniteDomain
{}

impl<D> Seq<D>
    where D: FiniteDomain
{
    pub fn iter(count: Count<D>, len: usize) -> SeqIter<D> {
        SeqIter::new(count, len)
    }
}

/// Enumerates, in order, all seqs over this domain with given length.
pub struct SeqIter<D: FiniteDomain> {
    idxs: Vec<usize>,
    count: Count<D>,
}

impl<D: FiniteDomain> SeqIter<D> {
    pub fn new(count: Count<D>, len: usize) -> Self {
        SeqIter {
            idxs: vec![0; len],
            count: count,
        }
    }

    fn increment(&mut self) {
        for i in (0..self.idxs.len()).rev() {
            self.idxs[i] += 1;

            if i > 0 && self.idxs[i] >= self.count.as_usize() {
                self.idxs[i] = 0;
            } else {
                return;
            }
        }
    }
}

impl<D: FiniteDomain> Iterator for SeqIter<D> {
    type Item = Vec<Num<D>>;

    fn next(&mut self) -> Option<Vec<Num<D>>> {
        if self.idxs[0] >= self.count.as_usize() {
            return None;
        } else {
            let item = self.idxs.iter().map(|&num| to_num(num)).collect();
            self.increment();
            return Some(item);
        }
    }
}


/// Maps a seq to its number in the domain of sequences of length len.
/// Providing a sequence of a differing length to this mapping is a programmer
/// error.
pub struct SeqNum<D: FiniteDomain> {
    count: Count<D>,
    len: usize,
}

impl<D: FiniteDomain> SeqNum<D> {
    pub fn new(count: Count<D>, len: usize) -> Self {
        SeqNum {
            count: count,
            len: len,
        }
    }
}

impl<D> HasCount<Seq<D>> for SeqNum<D>
    where D: FiniteDomain
{
    fn count(&self) -> Count<Seq<D>> {
        to_count(self.count.as_usize().pow(self.len as u32))
    }
}

impl<D, I> Mapping<I> for SeqNum<D>
    where I: Iterator<Item = Num<D>>,
          D: FiniteDomain
{
    type Result = Num<Seq<D>>;

    fn apply(&self, seq: I) -> Num<Seq<D>> {
        let num = seq.into_iter().fold(0, |acc, num| acc * self.count.as_usize() + num.as_usize());
        return to_num(num);
    }
}
