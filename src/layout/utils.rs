use std::mem;
use rand::{thread_rng, Rng};
use cat::*;
use cat::ops::*;

pub type Subset<D> = IndexedList<Num<D>, Table<D, Option<usize>>>;

pub struct IndexedList<D, I>
    where I: Dict<D, Option<usize>>,
          D: FiniteDomain
{
    // elements currently in this subset
    pub elems: Vec<D::Type>,
    // maps an element to its index
    pub idxs: I,
}

impl<D> IndexedList<Num<D>, Table<D, Option<usize>>>
    where D: FiniteDomain
{
    pub fn empty(count: Count<D>) -> Self {
        let index = count.map_nums(|_| None);
        IndexedList {
            elems: Vec::with_capacity(count.as_usize()),
            idxs: index,
        }
    }
}

impl<D, I> IndexedList<D, I>
    where I: Dict<D, Option<usize>>,
          D::Type: Copy,
          D: FiniteDomain
{
    pub fn add(&mut self, mut elem: D::Type, pos: usize) {
        if self.idxs.get(elem).is_none() {
            // swap elem and element in target position
            if pos < self.elems.len() {
                *self.idxs.get_mut(elem.clone()) = Some(pos);
                mem::swap(&mut elem, &mut self.elems[pos]);
            }
            // push elem to elems
            *self.idxs.get_mut(elem) = Some(self.elems.len());
            self.elems.push(elem);
        }
    }

    pub fn get(&self, pos: usize) -> D::Type {
        return self.elems[pos];
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = D::Type> + 'a {
        self.elems.iter().cloned()
    }

    // returns index the element used to have
    pub fn remove(&mut self, elem: D::Type) -> Option<usize> {
        if let Some(idx) = self.idxs.get_mut(elem).take() {
            self.elems.swap_remove(idx);
            if idx < self.elems.len() {
                *self.idxs.get_mut(self.elems[idx]) = Some(idx);
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
            *self.idxs.get_mut(elem) = Some(idx);
        }
    }
}
