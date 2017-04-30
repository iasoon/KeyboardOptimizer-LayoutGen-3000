use utils::Countable;

use std::vec::Vec;
use std::ops::Index;

pub struct SeqSet<T> {
    seq_len: usize,
    vec: Vec<T>,
}

impl<T> SeqSet<T> {
    pub fn from_vec(vec: Vec<T>, seq_len: usize) -> Self {
        SeqSet {
            seq_len: seq_len,
            vec: vec,
        }
    }

    pub fn get_seq<'a>(&'a self, idx: SeqId) -> Seq<'a, T> {
        let offset = self.seq_len * idx.to_num(&self.seq_count());
        Seq {
            slice: &self.vec,
            pos: offset,
            end: offset + self.seq_len,
        }
    }

    pub fn seq_count(&self) -> SeqCount {
        SeqCount { count: self.vec.len() / self.seq_len }
    }
}

pub struct Seq<'a, T: 'a> {
    slice: &'a [T],
    pos: usize,
    end: usize,
}

impl<'a, T: 'a> Iterator for Seq<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.pos >= self.end {
            None
        } else {
            let item = &self.slice[self.pos];
            self.pos += 1;
            Some(item)
        }
    }
}

#[derive(Clone, Copy)]
pub struct SeqId(usize);

pub struct SeqCount {
    count: usize,
}

impl Countable for SeqId {
    type Data = SeqCount;

    fn from_num(_: &SeqCount, num: usize) -> Self {
        SeqId(num)
    }

    fn to_num(&self, _: &SeqCount) -> usize {
        let &SeqId(num) = self;
        num
    }

    fn count(data: &SeqCount) -> usize {
        data.count
    }
}
