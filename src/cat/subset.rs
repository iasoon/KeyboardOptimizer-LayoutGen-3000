use std::marker::PhantomData;
use std::collections::HashMap;
use std::ops::Index;

use cat::domain::*;
use cat::mapping::*;
use cat::table::*;


impl Domain for String {
    type Type = String;
}

pub struct Subset<'d, D: 'd, M, E>
    where M: PartialDict<'d, D, Num<D>>,
          E: Elements<D>,
          D: FiniteDomain
{
    pub dict: M,
    pub elems: E,
    pub phantom: PhantomData<&'d D>,
}

impl<'d, D: 'd, M, E> Subset<'d, D, M, E>
    where M: PartialDict<'d, D, Num<D>>,
          E: Elements<D>,
          D: FiniteDomain,
{
    pub fn from_elem_vec(vec: Vec<D::Type>) -> Self {
        let elems = E::from_vec(vec);
        let dict = M::construct(&elems, |num, _| Some(num));

        return Subset {
            dict: dict,
            elems: elems,
            phantom: PhantomData,
        };
    }
}
