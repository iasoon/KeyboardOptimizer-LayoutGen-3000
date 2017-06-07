use std::marker::PhantomData;

use cat::domain::*;
use cat::mapping::*;

struct Pre<'m, 'x, 't, S, X, T, M, D>
    where M: Mapping<'m, 'x, S, X::Type> + 'm,
          D: Dict<'t, X, T> + 't,
          S: Domain,
          X: Domain,
          X::Type: 'x,
          T: 't
{
    mapping: M,
    dict: D,
    p: PhantomData<&'x ()>,
    phantom_x: PhantomData<X>,
    phantom_s: PhantomData<S>,
    phantom_t: PhantomData<&'t T>,
    phantom_m: PhantomData<&'m M>,
}

impl<'m, 'x, 't: 'm, S, X, T, M, D> Mapping<'t, 't, S, &'t T> for Pre<'m, 'x, 't, S, X, T, M, D>
    where M: Mapping<'m, 'x, S, X::Type> + 'm,
          D: Dict<'t, X, T> + 't,
          S: Domain,
          X: Domain,
          X::Type: 'x,
          T: 't
{
    fn map(&'t self, elem: S::Type) -> &'t T {
        let d = self.mapping.map(elem);
        return self.dict.get(d);
    }
}

impl<'m, 'x, 't: 'm, S, X, T, M, D> Dict<'t, S, T> for Pre<'m, 'x, 't, S, X, T, M, D>
    where M: Mapping<'m, 'x, S, X::Type> + 'm,
          D: Dict<'t, X, T> + 't,
          S: Domain,
          X: Domain,
          X::Type: 't,
          T: 't
{
    fn get_mut(&'t mut self, elem: S::Type) -> &'t mut T {
        let d = self.mapping.map(elem);
        return self.dict.get_mut(d);
    }
}
