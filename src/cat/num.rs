use cat::types::*;

use std::marker::PhantomData;

pub struct Num<D: FiniteDomain> {
    num: usize,
    phantom: PhantomData<D>,
}

impl<D: FiniteDomain> Clone for Num<D> {
    fn clone(&self) -> Self {
        Num {
            num: self.num,
            phantom: PhantomData,
        }
    }
}

impl<D: FiniteDomain> Copy for Num<D> {}

impl<D: FiniteDomain> Domain for Num<D> {
    type Type = Num<D>;
}

impl<D: FiniteDomain> FiniteDomain for Num<D> {}

impl<D: FiniteDomain> Num<D> {
    pub fn as_usize(&self) -> usize {
        self.num
    }
}

impl<D: FiniteDomain> PartialEq for Num<D> {
    fn eq(&self, other: &Num<D>) -> bool {
        self.as_usize() == other.as_usize()
    }
}

impl<D: FiniteDomain> Eq for Num<D> {}

pub fn to_num<D: FiniteDomain>(num: usize) -> Num<D> {
    Num {
        num: num,
        phantom: PhantomData,
    }
}
