use std::marker::PhantomData;

use cat::types::*;
use cat::has_count::*;

/// General composition struct.
/// A: First object to be composed
/// D: Intermediary domain: A maps to this, B maps from this
/// B: Second object to be composed
pub struct Composed<A, B>
{
    fst: A,
    snd: B,
}

impl<A, B> Composed<A, B> {
    pub fn new(fst: A, snd: B) -> Self {
        Composed {
            fst: fst,
            snd: snd,
        }
    }
}

impl<S, T, M, D> Dict<S, T> for Composed<M, D>
    where M: Mapping<S>,
          D: Dict<M::Result, T>,
          S: Domain,
{
    fn get<'t>(&'t self, elem: S) -> &'t T {
        let d = self.fst.apply(elem);
        return self.snd.get(d);
    }

    fn get_mut<'t>(&'t mut self, elem: S) -> &'t mut T {
        let d = self.fst.apply(elem);
        return self.snd.get_mut(d);
    }
}
