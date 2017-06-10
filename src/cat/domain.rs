use std::marker::PhantomData;
use std::ops::Index;

use cat::mapping::Dict;

pub trait Domain {
    type Type;
}

pub trait FiniteDomain: Domain {}

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

pub fn to_num<D: FiniteDomain>(num: usize) -> Num<D> {
    Num {
        num: num,
        phantom: PhantomData,
    }
}
