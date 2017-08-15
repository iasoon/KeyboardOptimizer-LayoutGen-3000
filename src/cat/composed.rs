use std::ops::{Index, IndexMut};

use cat::types::*;
use cat::ops::*;

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

impl<K, A, B> Mapping<K> for Composed<A, B>
    where A: Mapping<K>,
          B: Mapping<A::Result>
{
    type Result = B::Result;

    fn apply(&self, elem: K) -> B::Result {
        self.snd.apply(self.fst.apply(elem))
    }
}

impl<K, A, B> Index<K> for Composed<A, B>
    where A: Mapping<K>,
          B: Index<A::Result>
{
    type Output = B::Output;

    fn index<'t>(&'t self, key: K) -> &'t B::Output {
        let idx = self.fst.apply(key);
        return &self.snd[idx];
    }
}

impl<K, A, B> IndexMut<K> for Composed<A, B>
    where A: Mapping<K>,
          B: IndexMut<A::Result>
{
    fn index_mut<'t>(&'t mut self, key: K) -> &'t mut B::Output {
        let idx = self.fst.apply(key);
        return &mut self.snd[idx];
    }
}

impl<A, B, S, T, R> MapInto<S, T, Composed<A, R>> for Composed<A, B>
    where B: MapInto<S, T, R>
{
    fn map_into<F>(self, fun: F) -> Composed<A, R>
        where F: FnMut(S) -> T
    {
        Composed {
            fst: self.fst,
            snd: self.snd.map_into(fun),
        }
    }
}
