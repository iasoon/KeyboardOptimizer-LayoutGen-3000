use cat::*;
use cat::internal::*;

pub fn choose(n: usize, k: usize) -> usize {
    let mut num = 1;
    for i in 1..k+1 {
        num *= n - (k-i);
        num /= i;
    }
    return num;
}

pub fn choose_repeat(n: usize, k: usize) -> usize {
    return choose(n + k - 1, k);
}


/// A bag (multiset) of elements.
pub struct Bag<T> {
    /// elements are stored in descending order
    elems: Vec<T>,
}

impl<T> Bag<T> {
    pub fn new(mut vec: Vec<T>) -> Self
        where T: Ord
    {
        vec.sort_by(|a, b| a.cmp(b).reverse());
        return Bag::from_sorted(vec);
    }

    pub fn from_sorted(vec: Vec<T>) -> Self {
        return Bag { elems: vec };
    }
}

impl<D> Domain for Bag<D>
    where D: Domain
{
    type Type = Bag<D::Type>;
}

impl<D> FiniteDomain for Bag<D>
    where D: FiniteDomain
{}

pub struct BagNum<D>
    where D: FiniteDomain
{
    elemtype_count: Count<D>,
    size: usize,
}

impl<D> BagNum<D>
    where D: FiniteDomain
{
    pub fn new(count: Count<D>, size: usize) -> Self {
        BagNum {
            elemtype_count: count,
            size: size,
        }
    }
}

impl<D> HasCount<Bag<D>> for BagNum<D>
    where D: FiniteDomain
{
    fn count(&self) -> Count<Bag<D>> {
        let count = choose_repeat(self.elemtype_count.as_usize(), self.size);
        return to_count(count);
    }
}

impl<D> Mapping<Bag<Num<D>>> for BagNum<D>
    where D: FiniteDomain
{
    type Result = Num<Bag<D>>;

    fn apply(&self, bag: Bag<Num<D>>) -> Num<Bag<D>> {
        let mut num = 0;
        for i in 0..bag.elems.len() {
            let k = bag.elems.len() - i;
            let c = bag.elems[i].as_usize();
            if c > 0 {
                num += choose_repeat(c, k)
            }
        }
        return to_num(num);
    }
}

impl<D> Mapping<Num<Bag<D>>> for BagNum<D>
    where D: FiniteDomain
{
    type Result = Bag<Num<D>>;

    fn apply(&self, num: Num<Bag<D>>) -> Bag<Num<D>> {
        let mut num = num.as_usize();
        let mut rem = self.count().as_usize();
        let mut n = self.elemtype_count.as_usize();
        let mut k = self.size;
        let mut vec = Vec::with_capacity(self.size);
        loop {
            while rem > num {
                n -= 1;
                rem *= n;
                rem /= k + n;
            }

            vec.push(to_num(n));
            if k == 1 {
                return Bag::from_sorted(vec);
            }

            num -= rem;
            rem *= k;
            rem /= k + n - 1;
            k -= 1;
        }
    }
}

pub struct SeqBag<D>
    where D: FiniteDomain
{
    count: Count<D>,
    bag_size: usize,
    seq_bag: Table<Seq<D>, Num<Bag<D>>>,
}

impl<D> SeqBag<D>
    where D: FiniteDomain
{
    pub fn new(count: Count<D>, bag_size: usize) -> Self {
        let bag_num = BagNum::new(count, bag_size);
        let seq_bag = Table::from_vec(
            SeqIter::new(count, bag_size)
                .map(|seq| bag_num.apply(Bag::new(seq)))
                .collect());
        return SeqBag {
            count: count,
            bag_size: bag_size,
            seq_bag: seq_bag,
        }
    }
}

impl<D> HasCount<Bag<D>> for SeqBag<D>
    where D: FiniteDomain
{
    fn count(&self) -> Count<Bag<D>> {
        BagNum::new(self.count, self.bag_size).count()
    }
}

impl<D, I> Mapping<I> for SeqBag<D>
    where I: Iterator<Item = Num<D>>,
          D: FiniteDomain
{
    type Result = Num<Bag<D>>;

    fn apply(&self, seq: I) -> Num<Bag<D>> {
        *self.seq_bag.get(SeqNum::new(self.count, self.bag_size).apply(seq))
    }
}

pub type BagTable<D, T> = Composed<SeqBag<D>, Table<Bag<D>, T>>;
