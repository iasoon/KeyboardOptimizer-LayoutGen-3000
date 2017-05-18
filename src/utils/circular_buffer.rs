use std::vec::Vec;
use std::ops::Index;

pub struct CircularBuffer<T> {
    vec: Vec<T>,
    start: usize,
    size: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            vec: Vec::with_capacity(size),
            start: 0,
            size: size,
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, t: T) {
        if self.vec.len() < self.size {
            self.vec.push(t);
        } else {
            self.vec[self.start] = t;
            self.start += 1;
            self.start %= self.size;
        }
    }

    pub fn iter<'a>(&'a self) -> CircularBufferIterator<'a, T> {
        CircularBufferIterator::new(self)
    }
}

impl<T> Index<usize> for CircularBuffer<T> {
    type Output =T;

    fn index<'a>(&'a self, idx: usize) -> &'a T {
        &self.vec[(self.start + idx) % self.size]
    }
}

pub struct CircularBufferIterator<'a, T: 'a> {
    buffer: &'a CircularBuffer<T>,
    start: usize,
    end: usize,
}

impl<'a, T> CircularBufferIterator<'a, T> {
    fn new(buf: &'a CircularBuffer<T>) -> Self {
        CircularBufferIterator {
            start: 0,
            end: buf.len(),
            buffer: buf,
        }
    }
}

impl<'a, T> Iterator for CircularBufferIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.start == self.end {
            None
        } else {
            let item = &self.buffer[self.start];
            self.start += 1;
            Some(item)
        }
    }
}

impl<'a, T> DoubleEndedIterator for CircularBufferIterator<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(&self.buffer[self.end])
        }
    }
}
