use errors::*;

pub trait Countable {
    type Data;

    fn from_num(data: &Self::Data, num: usize) -> Self;
    fn to_num(&self, data: &Self::Data) -> usize;

    fn count(data: &Self::Data) -> usize;

    fn enumerate<'a>(data: Self::Data) -> Enumerator<Self>
        where Self: Sized
    {
        Enumerator::new(data)
    }

    fn from_num_checked(data: &Self::Data, num: usize) -> Result<Self>
        where Self: Sized
    {
        if num < Self::count(data) {
            Ok(Self::from_num(data, num))
        } else {
            bail!(format!("Countable out of bounds: count is {} but number is {}",
                          Self::count(data),
                          num))
        }
    }
}

pub struct Enumerator<T: Countable> {
    pos: usize,
    data: T::Data,
}

impl<T: Countable> Enumerator<T> {
    fn new(data: T::Data) -> Self {
        Enumerator {
            pos: 0,
            data: data,
        }
    }
}

impl<T: Countable> Iterator for Enumerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.pos < T::count(&self.data) {
            let item = T::from_num(&self.data, self.pos);
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}
