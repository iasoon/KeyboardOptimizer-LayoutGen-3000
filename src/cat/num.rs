use std::marker::PhantomData;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct Num<D> {
    num: usize,
    phantom: PhantomData<D>,
}

impl<D> Clone for Num<D> {
    fn clone(&self) -> Self {
        Num {
            num: self.num,
            phantom: PhantomData,
        }
    }
}

impl<D> Copy for Num<D> {}

impl<D> Num<D> {
    pub fn as_usize(&self) -> usize {
        self.num
    }
}

impl<D> PartialEq for Num<D> {
    fn eq(&self, other: &Num<D>) -> bool {
        self.as_usize() == other.as_usize()
    }
}

impl<D> Eq for Num<D> {}

impl<D> PartialOrd for Num<D> {
    fn partial_cmp(&self, rhs: &Num<D>) -> Option<Ordering> {
        self.as_usize().partial_cmp(&rhs.as_usize())
    }
}

impl<D> Ord for Num<D> {
    fn cmp(&self, rhs: &Num<D>) -> Ordering {
        self.as_usize().cmp(&rhs.as_usize())
    }
}

impl<D> fmt::Debug for Num<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_usize())
    }
}

impl<D> Hash for Num<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.num.hash(state)
    }
}


pub fn to_num<D>(num: usize) -> Num<D> {
    Num {
        num: num,
        phantom: PhantomData,
    }
}
