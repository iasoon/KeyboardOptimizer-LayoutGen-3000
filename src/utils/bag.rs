use utils::Countable;

pub fn choose(n: usize, k: usize) -> usize {
    (1..k+1).fold(1, |num, i| {
        (num * n - (k-i)) / i
    })
}

pub fn choose_repeat(n: usize, k: usize) -> usize {
    return choose(n + k - 1, k);
}

pub struct Bag<T> {
    elems: Vec<T>,
}

impl<T> Bag<T> {
    pub fn new(mut vec: Vec<T>) -> Self
        where T: Ord
    {
        vec.sort_by(|a, b| b.cmp(a));
        return Bag::from_sorted(vec);
    }

    pub fn elems<'a>(&'a self) -> &'a [T] {
        &self.elems
    }

    pub fn from_sorted(vec: Vec<T>) -> Self {
        Bag {
            elems: vec
        }
    }
}

pub struct BagData<C: Countable> {
    data: C::Data,
    len: usize,
}

impl<C: Countable> Countable for Bag<C> {
    type Data = BagData<C>;

    fn to_num(&self, data: &BagData<C>) -> usize {
        self.elems.iter().enumerate().map(|(pos, ref elem)| {
            choose_repeat(elem.to_num(&data.data), pos+1)
        }).sum()
    }

    fn from_num(data: &BagData<C>, mut num: usize) -> Bag<C> {
        let mut rem = Bag::count(data);
        let mut n = C::count(&data.data);
        let mut k = data.len;
        let mut vec = Vec::with_capacity(data.len);
        loop {
            while rem > num {
                n -= 1;
                rem *= n;
                rem /= k + n;
            }

            vec.push(C::from_num(&data.data, n));
            if k == 1 {
                return Bag::from_sorted(vec);
            }

            num -= rem;
            rem *= k;
            rem /= k + n - 1;
            k -= 1;
        }
    }

    fn count(data: &BagData<C>) -> usize {
        choose_repeat(C::count(&data.data), data.len)
    }
}
