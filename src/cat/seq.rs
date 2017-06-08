use std::marker::PhantomData;

use cat::domain::*;
use cat::mapping::*;

/// A sequence of values.
/// Since it is not possible to parametrically fixate a length for these
/// sequences, impls for this type are a bit liberal and instead rely on
/// programmer discipline.
pub struct Seq<'e, D, I>
    where I: IntoIterator<Item = D::Type>,
          D: Domain + 'e
{
    phantom_d: PhantomData<&'e D>,
    phantom_i: PhantomData<I>,
}

impl<'e, D, I> Domain for Seq<'e, D, I>
    where I: IntoIterator<Item = D::Type>,
          D: Domain + 'e
{
    type Type = I;
}

/// This implementation is purposely left a bit vague; for the domain of
/// sequences to be finite, one should constrain it, for example by using a
/// fixed length, or a maximum length.
impl<'e, D, I> FiniteDomain for Seq<'e, D, I>
    where I: IntoIterator<Item = D::Type>,
          D: FiniteDomain + 'e
{
}

impl<'e, D> Seq<'e, D, Vec<D::Type>>
    where D: FiniteDomain + 'e
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
    fn new(count: Count<D>, len: usize) -> Self {
        SeqIter {
            idxs: vec![0; len],
            count: count,
        }
    }

    fn increment(&mut self) {
        for i in (0..self.idxs.len()).rev() {
            self.idxs[i] += 1;

            if i > 0 && self.idxs[i] >= from_count(self.count) {
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
        if self.idxs[0] > from_count(self.count) {
            return None;
        } else {
            let item = self.idxs.iter().map(|&num| to_num(num)).collect();
            self.increment();
            return Some(item);
        }
    }
}


/// Maps a seq to its number, for the corresponding fixed-length Seq domain.
pub struct SeqNum<D: FiniteDomain> {
    count: Count<D>,
}

impl<'s1, 's2, D, I1, I2> Mapping<'s1, 's2, Seq<'s1, Num<D>, I1>, Num<Seq<'s2, D, I2>>> for SeqNum<D>
    where I1: IntoIterator<Item = Num<D>> + 's1,
          I2: IntoIterator<Item = D::Type> + 's2,
          D: FiniteDomain + 's1 + 's2
{
    fn map(&'s1 self, seq: I1) -> Num<Seq<'s2, D, I2>> {
        let count = from_count(self.count);
        let num = seq.into_iter().fold(0, |acc, num| acc * count + from_num(num));
        return to_num(num);
    }
}
