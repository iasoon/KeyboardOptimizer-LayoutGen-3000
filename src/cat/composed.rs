use std::marker::PhantomData;

use cat::types::*;

/// General composition struct.
/// A: First object to be composed
/// D: Intermediary domain: A maps to this, B maps from this
/// B: Second object to be composed
pub struct Composed<A, D, B>
{
    fst: A,
    phantom_d: PhantomData<D>,
    snd: B,
}

impl<A, D, B> Composed<A, D, B>
    where D: Domain,
{
    pub fn new(fst: A, snd: B) -> Self {
        Composed {
            fst: fst,
            phantom_d: PhantomData,
            snd: snd,
        }
    }
}

impl<S, T, X, M, D> Dict<S, T> for Composed<M, X, D>
    where M: Mapping<S, X::Type>,
          D: Dict<X, T>,
          X: Domain,
          S: Domain,
{
    fn get<'t>(&'t self, elem: S::Type) -> &'t T {
        let d = self.fst.apply(elem);
        return self.snd.get(d);
    }

    fn get_mut<'t>(&'t mut self, elem: S::Type) -> &'t mut T {
        let d = self.fst.apply(elem);
        return self.snd.get_mut(d);
    }
}
