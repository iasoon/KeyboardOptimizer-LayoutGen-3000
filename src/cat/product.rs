use std::marker::PhantomData;

use cat;
use cat::*;

pub struct Product<Maj, Min> {
    phantom_major: PhantomData<Maj>,
    phantom_minor: PhantomData<Min>,
}

pub struct ProductNum<Maj, Min> {
    pub major_count: Count<Maj>,
    pub minor_count: Count<Min>,
}

impl<Maj, Min> Mapping<(Num<Maj>, Num<Min>)> for ProductNum<Maj, Min> {
    type Result = Num<Product<Maj, Min>>;

    fn apply(&self, elem: (Num<Maj>, Num<Min>)) -> Num<Product<Maj, Min>> {
        let (maj_num, min_num) = elem;
        let maj_component = self.minor_count.as_usize() * maj_num.as_usize();
        let min_component = min_num.as_usize();
        return cat::internal::to_num(maj_component + min_component);
    }
}

impl<Maj, Min> Mapping<Num<Product<Maj, Min>>> for ProductNum<Maj, Min> {
    type Result = (Num<Maj>, Num<Min>);

    fn apply(&self, num: Num<Product<Maj, Min>>) -> (Num<Maj>, Num<Min>) {
        let maj_num = num.as_usize() / self.minor_count.as_usize();
        let min_num = num.as_usize() % self.minor_count.as_usize();
        return (cat::internal::to_num(maj_num), cat::internal::to_num(min_num));
    }
}

impl<Maj, Min> HasCount<Product<Maj, Min>> for ProductNum<Maj, Min> {
    fn count(&self) -> Count<Product<Maj, Min>> {
        let count = self.major_count.as_usize() * self.minor_count.as_usize();
        return cat::internal::to_count(count);
    }
}
