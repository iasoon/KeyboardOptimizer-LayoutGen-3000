use std::mem;
use std::ops::{Index, IndexMut};
use rand::{thread_rng, Rng};
use cat::*;

pub type Subset<D> = IndexedList<Num<D>, Table<D, Option<usize>>>;

pub struct IndexedList<T, I> {
    // elements currently in this subset
    pub elems: Vec<T>,
    // maps an element to its index
    pub idxs: I,
}

impl<D> IndexedList<Num<D>, Table<D, Option<usize>>> {

    pub fn complete(count: Count<D>) -> Self {
        IndexedList {
            elems: count.nums().collect(),
            // nums are yielded in order
            idxs: count.map_nums(|num| Some(num.as_usize())),
        }
    }

    pub fn empty(count: Count<D>) -> Self {
        let index = count.map_nums(|_| None);
        IndexedList {
            elems: Vec::with_capacity(count.as_usize()),
            idxs: index,
        }
    }
}

impl<T, I> IndexedList<T, I>
    where I: Index<T, Output = Option<usize>> + IndexMut<T>,
          T: Copy,
{
    pub fn add(&mut self, mut elem: T, pos: usize) {
        if self.idxs[elem].is_none() {
            // swap elem and element in target position
            if pos < self.elems.len() {
                self.idxs[elem] = Some(pos);
                mem::swap(&mut elem, &mut self.elems[pos]);
            }
            // push elem to elems
            self.idxs[elem] = Some(self.elems.len());
            self.elems.push(elem);
        }
    }

    pub fn next(&self) -> Option<T> {
        if self.elems.len() > 0 {
            Some(self.get(0))
        } else {
            None
        }
    }

    pub fn get(&self, pos: usize) -> T {
        return self.elems[pos];
    }

    pub fn contains(&self, elem: T) -> bool {
        self.idxs[elem].is_some()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.elems.iter().cloned()
    }

    // returns index the element used to have
    pub fn remove(&mut self, elem: T) -> Option<usize> {
        if let Some(idx) = self.idxs[elem].take() {
            self.elems.swap_remove(idx);
            if idx < self.elems.len() {
                self.idxs[self.elems[idx]] = Some(idx);
            }
            return Some(idx);
        } else {
            return None;
        }
    }

    pub fn size(&self) -> usize {
        self.elems.len()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(self.elems.as_mut_slice());
        self.fix_index();
    }

    fn fix_index(&mut self) {
        for (idx, &elem) in self.elems.iter().enumerate() {
            self.idxs[elem] = Some(idx);
        }
    }
}
