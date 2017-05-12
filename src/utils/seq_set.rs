use utils::{Countable, SeqIter};

use std::vec::Vec;
use std::ops::Index;

pub struct SeqSet<C: Countable> {
    seq_len: usize,
    vec: Vec<C>,
}

impl<C: Countable> SeqSet<C> {
    pub fn from_vec(vec: Vec<C>, seq_len: usize) -> Self {
        SeqSet {
            seq_len: seq_len,
            vec: vec,
        }
    }

    pub fn get_seq<'a>(&'a self, idx: SeqNum) -> SeqIter<'a, C> {
        let offset = self.seq_len * idx.to_num(&self.seq_count());
        SeqIter::from_slice(&self.vec[offset..offset+self.seq_len])
    }

    pub fn seq_count(&self) -> SeqCount {
        SeqCount { count: self.vec.len() / self.seq_len }
    }

    pub fn seq_len(&self) -> usize {
        return self.seq_len;
    }
}

#[derive(Clone, Copy)]
pub struct SeqNum(usize);

pub struct SeqCount {
    count: usize,
}

impl Countable for SeqNum {
    type Data = SeqCount;

    fn from_num(_: &SeqCount, num: usize) -> Self {
        SeqNum(num)
    }

    fn to_num(&self, _: &SeqCount) -> usize {
        let &SeqNum(num) = self;
        num
    }

    fn count(data: &SeqCount) -> usize {
        data.count
    }
}
