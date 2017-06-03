use std::marker::PhantomData;

use cat::domain::*;
use cat::mapping::*;

pub struct Seq<'e, D, I>
    where I: IntoIterator<Item = &'e D::Type>,
          D: Domain + 'e
{
    phantom_d: PhantomData<&'e D>,
    phantom_i: PhantomData<I>,
}

impl<'e, D, I> Domain for Seq<'e, D, I>
    where I: IntoIterator<Item = &'e D::Type>,
          D: Domain + 'e
{
    type Type = I;
}

impl<'e, D, I> FiniteDomain for Seq<'e, D, I>
    where I: IntoIterator<Item = &'e D::Type>,
          D: FiniteDomain + 'e
{
}

pub struct SeqNum<D: FiniteDomain> {
    count: Count<D>,
}

impl<'s1, 's2, D, I1, I2> Mapping<'s1, 's2, Seq<'s1, Num<D>, I1>, Num<Seq<'s2, D, I2>>> for SeqNum<D>
    where I1: IntoIterator<Item = &'s1 Num<D>> + 's1,
          I2: IntoIterator<Item = &'s2 D::Type> + 's2,
          D: FiniteDomain + 's1 + 's2
{
    fn map(&'s1 self, seq: I1) -> Num<Seq<'s2, D, I2>> {
        let count = from_count(self.count);
        let num = seq.into_iter().fold(0, |acc, &num| acc * count + from_num(num));
        return to_num(num);
    }
}
