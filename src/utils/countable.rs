pub trait Countable {
    type Data;

    fn from_num(data: &Self::Data, num: usize) -> Self;
    fn to_num(&self, data: &Self::Data) -> usize;

    fn count(data: &Self::Data) -> usize;

    fn enumerate<'a>(data: &'a Self::Data) -> Enumerator<'a, Self>
        where Self: Sized
    {
        Enumerator::new(data)
    }
}

pub struct Enumerator<'a, T: Countable + 'a> {
    pos: usize,
    data: &'a T::Data,
}

impl<'a, T: Countable> Enumerator<'a, T> {
    fn new(data: &'a T::Data) -> Self {
        Enumerator {
            pos: 0,
            data: data,
        }
    }
}

impl<'a, T: Countable> Iterator for Enumerator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.pos < T::count(self.data) {
            let item = T::from_num(self.data, self.pos);
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}
