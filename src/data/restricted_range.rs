use super::*;
use cat::*;

use std::ops::{Index, Range};


struct Permutation<T> {
    items: Vec<Num<T>>,
    positions: Table<T, usize>,
}

impl<T> Permutation<T> {
    pub fn new(t_count: Count<T>) -> Self {
        Permutation {
            items: t_count.nums().collect(),
            positions: Table::from_vec((0..t_count.as_usize()).collect()),
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.items.swap(a, b);
        self.positions[self.items[a]] = a;
        self.positions[self.items[b]] = b;
    }

    pub fn pos(&self, value: Num<T>) -> usize {
        self.positions[value]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    fn update_pos(&mut self, pos: usize) {
        self.positions[self.items[pos]] = pos;
    }
}

impl<T> Index<usize> for Permutation<T> {
    type Output = Num<T>;

    fn index<'a>(&'a self, idx: usize) -> &'a Num<T> {
        &self.items[idx]
    }
}

impl<T> Index<Range<usize>> for Permutation<T> {
    type Output = [Num<T>];

    fn index<'a>(&'a self, range: Range<usize>) -> &'a [Num<T>] {
        &self.items[range]
    }
}

#[derive(Debug)]
struct Segment {
    offset: usize,
    num_rejected: usize,
}

impl Segment {
    fn empty(offset: usize) -> Self {
        Segment {
            offset: offset,
            num_rejected: 0,
        }
    }

    fn frontier(&self) -> usize {
        self.offset + self.num_rejected
    }

    fn accepts(&self, pos: usize) -> bool {
        pos >= self.frontier()
    }
}

pub struct SegmentedPermutation<T> {
    items: Permutation<T>,
    segments: Vec<Segment>,
    item_segment: Table<T, usize>,

}

impl<T> SegmentedPermutation<T> {
    pub fn new(t_count: Count<T>) -> Self {
        SegmentedPermutation {
            items: Permutation::new(t_count),
            segments: vec![Segment::empty(0)],
            item_segment: Table::from_vec(vec![0; t_count.as_usize()]),
        }
    }

    pub fn accept(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.accept_pos(segment, pos);
    }

    pub fn reject(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.reject_pos(segment, pos);
    }

    pub fn promote(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.promote_pos(segment, pos);
        self.item_segment[item_num] = segment + 1;
    }

    pub fn demote(&mut self, item_num: Num<T>) {
        let segment = self.item_segment[item_num];
        let pos = self.items.pos(item_num);
        self.demote_pos(segment, pos);
        self.item_segment[item_num] = segment - 1;
    }

    pub fn segments<'a>(&'a self) -> &'a [Segment] {
        &self.segments
    }

    pub fn push_segment(&mut self) {
        self.segments.push(Segment::empty(self.items.len()));
    }

    pub fn pop_segment(&mut self) {
        self.segments.pop();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    fn promote_pos(&mut self, segment: usize, pos: usize) {
        let promoting_pos = self.segments[segment + 1].offset - 1;

        if self.segments[segment].accepts(pos) {
            self.items.swap(pos, promoting_pos);
            self.extend_rejected(segment + 1);
            self.accept_pos(segment + 1, promoting_pos);
        } else {
            let accepted_pos = self.accept_pos(segment, pos);
            self.items.swap(accepted_pos, promoting_pos);
            self.extend_rejected(segment + 1);
        }
    }

    fn demote_pos(&mut self, segment: usize, pos: usize) {
        let demoting_pos = self.segments[segment].offset;

        if self.segments[segment].accepts(pos) {
            let rejected_pos = self.reject_pos(segment, pos);
            self.items.swap(rejected_pos, demoting_pos);
            self.extend_accepted(segment - 1);
        } else {
            self.items.swap(pos, demoting_pos);
            self.extend_accepted(segment - 1);
            self.reject_pos(segment - 1, demoting_pos);
        }
    }

    fn extend_accepted(&mut self, segment: usize) {
        self.segments[segment + 1].offset += 1;
        self.segments[segment + 1].num_rejected -= 1;
    }

    fn extend_rejected(&mut self, segment: usize) {
        self.segments[segment].offset -= 1;
        self.segments[segment].num_rejected += 1;
    }

    fn accept_pos(&mut self, segment: usize, pos: usize) -> usize {
        self.segments[segment].num_rejected -= 1;
        let frontier = self.segments[segment].frontier();
        self.items.swap(pos, frontier);
        return frontier;
    }

    fn reject_pos(&mut self, segment: usize, pos: usize) -> usize {
        let frontier = self.segments[segment].frontier();
        self.items.swap(pos, frontier);
        self.segments[segment].num_rejected += 1;
        return frontier;
    }
}

impl<T> Index<usize> for SegmentedPermutation<T> {
    type Output = Num<T>;

    fn index<'a>(&'a self, idx: usize) -> &'a Num<T> {
        &self.items[idx]
    }
}

impl<T> Index<Range<usize>> for SegmentedPermutation<T> {
    type Output = [Num<T>];

    fn index<'a>(&'a self, range: Range<usize>) -> &'a [Num<T>] {
        &self.items[range]
    }
}


pub struct RestrictedRange {
    values: SegmentedPermutation<Value>,
    times_rejected: Table<Value, usize>,
}

impl RestrictedRange {
    pub fn new(value_count: Count<Value>) -> Self {
        RestrictedRange {
            values: SegmentedPermutation::new(value_count),
            times_rejected: value_count.map_nums(|_| 0),
        }
    }

    pub fn accepted<'a>(&'a self) -> &'a [Num<Value>] {
        let segment = self.values.segments().last().unwrap();
        &self.values[(segment.frontier()..self.values.len())]
    }

    pub fn add_restriction(&mut self, restriction: &Restriction) {
        match restriction {
            &Restriction::Not(ref rejected_values) => {
                for &value_num in rejected_values {
                    self.reject(value_num);
                }
            }
            &Restriction::Only(ref accepted_values) => {
                self.values.push_segment();
                for &value_num in accepted_values {
                    self.values.promote(value_num);
                }
            }
        }
    }

    pub fn remove_restriction(&mut self, restriction: &Restriction) {
        match restriction {
            &Restriction::Not(ref rejected_values) => {
                for &value_num in rejected_values {
                    self.unreject(value_num);
                }
            }
            &Restriction::Only(ref accepted_values) => {
                for &value_num in accepted_values {
                    self.values.demote(value_num);
                }
                self.values.pop_segment();
            }
        }
    }

    fn reject(&mut self, value_num: Num<Value>) {
        if self.times_rejected[value_num] == 0 {
            self.values.reject(value_num);
        }
        self.times_rejected[value_num] += 1;
    }

    fn unreject(&mut self, value_num: Num<Value>) {
        self.times_rejected[value_num] -= 1;
        if self.times_rejected[value_num] == 0 {
            self.values.accept(value_num);
        }
    }
}