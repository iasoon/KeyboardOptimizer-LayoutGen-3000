use std::marker::PhantomData;

use cat::domain::{FiniteDomain, Num, HasCount, Count, to_num};
use cat::mapping::Dict;

pub struct ElemEnumerator<'e, D, E>
    where E: Dict<'e, Num<D>, D::Type> + 'e,
          D: FiniteDomain + 'e
{
    elems: &'e E,
    pos: usize,
    phantom: PhantomData<D>,
}

impl<'e, D, E> Iterator for ElemEnumerator<'e, D, E>
    where E: Dict<'e, Num<D>, D::Type> + HasCount<D>,
          D: FiniteDomain + 'e
{
    type Item = (Num<D>, &'e D::Type);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.elems.count().as_usize() {
            return None;
        } else {
            let num = to_num(self.pos);
            self.pos += 1;
            return Some((num, self.elems.get(num)));
        }
    }
}
