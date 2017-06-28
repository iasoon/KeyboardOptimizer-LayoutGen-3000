use std::marker::PhantomData;

use cat::domain::*;
use cat::mapping::*;

/// Precomposition of a Mapping with a Dict
/// i.e. first apply mapping, then lookup in dict
pub struct ComposedDict<S, X, T: ?Sized, M, D>
    where M: Mapping<S, X::Type>,
          D: Dict<X, T>,
          S: Domain,
          X: Domain
{
    mapping: M,
    dict: D,
    phantom_s: PhantomData<S>,
    phantom_x: PhantomData<X>,
    phantom_t: PhantomData<T>,
}

impl<S, X, T: ?Sized, M, D> ComposedDict<S, X, T, M, D>
    where M: Mapping<S, X::Type>,
          D: Dict<X, T>,
          S: Domain,
          X: Domain
{
    pub fn new(mapping: M, dict: D) -> Self {
        ComposedDict {
            mapping: mapping,
            dict: dict,
            phantom_s: PhantomData,
            phantom_x: PhantomData,
            phantom_t: PhantomData,
        }
    }
}

impl<S, X, T, M, D> Dict<S, T> for ComposedDict<S, X, T, M, D>
    where M: Mapping<S, X::Type>,
          D: Dict<X, T>,
          S: Domain,
          X: Domain
{
    fn get<'t>(&'t self, elem: S::Type) -> &'t T {
        let d = self.mapping.apply(elem);
        return self.dict.get(d);
    }

    fn get_mut<'t>(&'t mut self, elem: S::Type) -> &'t mut T {
        let d = self.mapping.apply(elem);
        return self.dict.get_mut(d);
    }
}
