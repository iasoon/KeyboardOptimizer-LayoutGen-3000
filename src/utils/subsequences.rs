pub struct SubSequences<'a, T: 'a> {
    slice: &'a [T],
    idxs: Vec<usize>,
}

impl<'a, T> SubSequences<'a, T> {
    pub fn new(slice: &'a [T], len: usize) -> Self {
        SubSequences {
            slice: slice,
            idxs: (0..len).collect(),
        }
    }

    fn pos<'b>(&'b self) -> Vec<&'a T> {
        self.idxs.iter().map(|&idx| &self.slice[idx]).collect()
    }

    fn increment(&mut self) {
        let mut pos = self.idxs.len() - 1;

        while pos > 0 && self.idxs[pos] >= self.slice.len() - self.idxs.len() + pos {
            pos -= 1;
        }

        self.idxs[pos] += 1;
        for i in 1..(self.idxs.len() - pos) {
            self.idxs[pos + i] = self.idxs[pos] + i;
        }
    }

}

impl<'a, T> Iterator for SubSequences<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        if self.idxs[self.idxs.len() - 1] >= self.slice.len() {
            None
        } else {
            let item = self.pos();
            self.increment();
            Some(item)
        }
    }
}
